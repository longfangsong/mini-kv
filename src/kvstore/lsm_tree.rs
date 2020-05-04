//! This is a simplified version of lsm-tree
//! There're only two levels
//! one is the unordered, read/append-only, may contains put/delete operation with duplicated keys, small tree
//! the other is the ordered, read-only, unique key-value only, large tree
//! when the small tree's size reach some certain limit and compaction triggered
//! merge-sort would be used to compact the small tree into the large one
//! Since the length of the input is not a fixed value, a "meta" file is need to
//! know the offset of each k-v pair in large tree, this approach looks a little like WiscKey
//! maybe replace with a real lsm-tree in the future

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use bincode::deserialize_from;

use crate::error::Error;
use crate::kvstore::log_entry::LogEntry;
use crate::kvstore::storage::KvStorage;
use crate::Result;

/// if small_tree's size is larger than `COMPAT_LIMIT`, a compaction will occur
/// use a relatively small value to make it easy to debug
/// need profiling in real use
const COMPAT_LIMIT: u64 = 4096;

/// a simplified lsm-tree
pub struct LSMTree {
    path: PathBuf,
    small_tree: RefCell<File>,
    large_tree: RefCell<File>,
    large_tree_meta: RefCell<File>,
}

impl LSMTree {
    /// create a new `LSMTree` from data in `path`
    /// Will create the folder and necessary files if necessary
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        fs::create_dir_all(&path)?;
        let small_tree = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path.as_ref().join("small.sst"))?;
        let large_tree = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path.as_ref().join("large.sst"))?;
        let meta = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path.as_ref().join("meta.sst"))?;
        Ok(Self {
            path: path.as_ref().to_path_buf(),
            small_tree: RefCell::new(small_tree),
            large_tree: RefCell::new(large_tree),
            large_tree_meta: RefCell::new(meta),
        })
    }

    /// compact the small tree into the large one if necessary(size of small tree larger than `COMPAT_LIMIT`)
    fn compact_if_necessary(&mut self) -> Result<()> {
        if self
            .small_tree
            .borrow()
            .metadata()
            .map(|it| it.len())
            .unwrap_or_else(|_| 0)
            > COMPAT_LIMIT
        {
            self.compact()?;
        }
        Ok(())
    }

    // compact the small tree into the large tree with merge
    /// do the compaction job
    fn compact(&mut self) -> Result<()> {
        // small-tree content
        let mut buffer = BTreeMap::new();
        // removed keys in small tree
        let mut removed = HashSet::new();
        // when doing write, small_tree's cursor would be in the end of the file, move it to the start
        self.small_tree.borrow_mut().seek(SeekFrom::Start(0))?;
        // read each operation in the small tree
        while let Ok(entry) = bincode::deserialize_from::<_, LogEntry>(self.small_tree.get_mut()) {
            match entry {
                LogEntry::Set(k, v) => {
                    buffer.insert(k, v);
                }
                LogEntry::Remove(k) => {
                    removed.insert(k);
                }
            }
        }
        // iterate though the buffer
        let mut buffer_iter = buffer.iter().peekable();
        // new large tree
        let mut new_large_tree = File::create(self.path.join("large"))?;
        let new_large_tree_meta = File::create(self.path.join("meta"))?;
        // write the (`key`, `value`) pair if not deleted
        let mut write_if_not_deleted = |key: &str, value: &str| -> Result<()> {
            if !removed.contains(key) {
                bincode::serialize_into(
                    &new_large_tree_meta,
                    &new_large_tree.seek(SeekFrom::Current(0))?,
                )?;
                bincode::serialize_into(&new_large_tree, key)?;
                bincode::serialize_into(&new_large_tree, value)?;
            }
            Ok(())
        };
        self.large_tree_meta.borrow_mut().seek(SeekFrom::Start(0))?;
        // merge occurs in this loop
        loop {
            let buffer_next = buffer_iter.peek();
            // all value in the buffer is written to the file
            // dump all undeleted remaining data in old large tree to the new one
            if buffer_next == None {
                while let Ok(large_tree_next_offset) =
                bincode::deserialize_from(self.large_tree_meta.get_mut())
                {
                    let key = self.read_key_in_large_tree(large_tree_next_offset)?;
                    let value: String = bincode::deserialize_from(self.large_tree.get_mut())?;
                    write_if_not_deleted(&key, &value)?;
                }
                break;
            }
            let &(buffer_next_key, buffer_next_value) = buffer_next.unwrap();
            let large_tree_next_offset = bincode::deserialize_from(self.large_tree_meta.get_mut());
            if large_tree_next_offset.is_err() {
                // all value in the old large_tree is written to the file
                // dump all undeleted remaining data in buffer to the new one
                for (k, v) in buffer_iter {
                    write_if_not_deleted(k, v)?;
                }
                break;
            }
            let large_tree_next_offset = large_tree_next_offset.unwrap();
            let large_tree_next_key = self.read_key_in_large_tree(large_tree_next_offset)?;
            match buffer_next_key.cmp(&large_tree_next_key) {
                Ordering::Less => {
                    write_if_not_deleted(&buffer_next_key, &buffer_next_value)?;
                    // move the cursor in the large_tree_meta back
                    // todo: maybe use a cache here to speed up
                    self.large_tree_meta.borrow_mut().seek(SeekFrom::Current(
                        -(bincode::serialized_size(&large_tree_next_offset)? as i64),
                    ))?;
                    buffer_iter.next();
                }
                Ordering::Equal => {
                    // when the keys are equal
                    // since the data in the buffer is always newer
                    // there's no need to move the cursor in the large_tree_meta back
                    write_if_not_deleted(&buffer_next_key, &buffer_next_value)?;
                    buffer_iter.next();
                }
                Ordering::Greater => {
                    let large_tree_next_value: String =
                        bincode::deserialize_from(self.large_tree.get_mut())?;
                    write_if_not_deleted(&large_tree_next_key, &large_tree_next_value)?;
                }
            }
        }
        // rename old files and rename the new files
        fs::remove_file(self.path.join("large.sst"))?;
        fs::rename(self.path.join("large"), self.path.join("large.sst"))?;
        fs::remove_file(self.path.join("meta.sst"))?;
        fs::rename(self.path.join("meta"), self.path.join("meta.sst"))?;
        // truncate the small tree file
        self.small_tree.borrow_mut().seek(SeekFrom::Start(0))?;
        self.small_tree.borrow_mut().set_len(0)?;
        // reopen the large tree
        let large_tree = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(self.path.join("large.sst"))?;
        self.large_tree.replace(large_tree);
        let meta = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(self.path.join("meta.sst"))?;
        self.large_tree_meta.replace(meta);
        Ok(())
    }

    /// find a value for a key in the small tree
    /// Return (the value found, whether the k-v pair is deleted)
    fn find_in_small_tree(&self, key: &str) -> Result<(Option<String>, bool)> {
        self.small_tree.borrow_mut().seek(SeekFrom::Start(0))?;
        let mut result = None;
        let mut removed = false;
        while let Ok(entry) =
        bincode::deserialize_from::<_, LogEntry>(self.small_tree.borrow_mut().by_ref())
        {
            match entry {
                LogEntry::Set(k, v) => {
                    if key == k {
                        result = Some(v);
                        removed = false;
                    }
                }
                LogEntry::Remove(k) => {
                    if key == k {
                        result = None;
                        removed = true;
                    }
                }
            }
        }
        Ok((result, removed))
    }

    /// read the key from `offset` in the large tree
    /// read large_tree directly after calling this function will
    /// get the value associated with the key in big tree
    fn read_key_in_large_tree(&self, offset: u64) -> Result<String> {
        self.large_tree.borrow_mut().seek(SeekFrom::Start(offset))?;
        Ok(bincode::deserialize_from(
            self.large_tree.borrow_mut().by_ref(),
        )?)
    }

    /// find the value for a key in the large three
    fn find_in_large_tree(&self, key: &str) -> Result<Option<String>> {
        let offset_serialize_size = bincode::serialized_size(&0u64)?;
        let mut left = 0;
        let mut right = self.large_tree_meta.borrow().metadata()?.len() / offset_serialize_size;
        while left < right {
            let mid = (left + right) / 2;
            self.large_tree_meta
                .borrow_mut()
                .seek(SeekFrom::Start(mid * offset_serialize_size))?;
            let offset = deserialize_from(self.large_tree_meta.borrow_mut().by_ref())?;
            let key_found = self.read_key_in_large_tree(offset)?;
            match key.cmp(&key_found) {
                Ordering::Equal => {
                    return Ok(Some(bincode::deserialize_from(
                        self.large_tree.borrow_mut().by_ref(),
                    )?));
                }
                Ordering::Less => right = mid,
                Ordering::Greater => left = mid + 1,
            }
        }
        Ok(None)
    }
}

#[test]
fn test_lsm_tree() {
    let dir = tempfile::tempdir().unwrap();
    let mut lsm_tree = LSMTree::new(dir.path()).unwrap();
    for i in 0..1024 {
        lsm_tree
            .insert(format!("key:{}", i), format!("value:{}", i))
            .unwrap();
    }
    for i in (0..1024).step_by(7) {
        lsm_tree.remove(&format!("key:{}", i)).unwrap();
    }
    assert_eq!(lsm_tree.get("key:127").unwrap().unwrap(), "value:127");
    assert_eq!(lsm_tree.get("key:7").unwrap(), None);
    for i in (0..1024).step_by(3) {
        lsm_tree
            .remove(&format!("key:{}", i))
            .unwrap_or_else(|_| ());
    }
    assert_eq!(lsm_tree.get("key:131").unwrap().unwrap(), "value:131");
    assert_eq!(lsm_tree.get("key:3").unwrap(), None);
    assert_eq!(lsm_tree.get("key:999").unwrap(), None);
    // force compact
    lsm_tree.compact().unwrap();
    let old_large_size = lsm_tree.large_tree.get_mut().metadata().unwrap().len();
    for i in 0..1024 {
        lsm_tree
            .insert(format!("key:{}", 1), format!("value:{}", i))
            .unwrap();
    }
    lsm_tree.compact().unwrap();
    assert_eq!(lsm_tree.get("key:1").unwrap().unwrap(), "value:1023");
    let new_large_size = lsm_tree.large_tree.get_mut().metadata().unwrap().len();
    assert_eq!(new_large_size, old_large_size + 3);
}

impl KvStorage for LSMTree {
    fn insert(&mut self, key: String, value: String) -> Result<()> {
        bincode::serialize_into(
            self.small_tree.borrow_mut().by_ref(),
            &LogEntry::Set(key, value),
        )?;
        self.compact_if_necessary()?;
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<String>> {
        let (in_small_tree, removed) = self.find_in_small_tree(key)?;
        if removed {
            return Ok(None);
        }
        Ok(if let Some(v) = in_small_tree {
            Some(v)
        } else if let Some(v) = self.find_in_large_tree(key)? {
            Some(v)
        } else {
            None
        })
    }

    fn remove(&mut self, key: &str) -> Result<()> {
        if let Ok(None) = self.get(key) {
            Err(Error::KeyNotFound)
        } else {
            bincode::serialize_into(
                self.small_tree.borrow_mut().by_ref(),
                &LogEntry::Remove(key.into()),
            )?;
            self.compact_if_necessary()?;
            Ok(())
        }
    }
}
