use std::collections::HashMap;
use std::path::Path;

use crate::kvstore::lsm_tree::LSMTree;
use crate::kvstore::storage::KvStorage;
use crate::Result;

mod log_entry;
mod lsm_tree;
mod storage;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `KvStorage`
///
/// Example:
///
/// ```rust
/// # use kvs::{KvStore, Result};
/// # fn try_main() -> Result<()> {
/// use std::env::current_dir;
/// let mut store = KvStore::open(current_dir()?)?;
/// store.set("key".to_owned(), "value".to_owned())?;
/// let val = store.get("key".to_owned())?;
/// assert_eq!(val, Some("value".to_owned()));
/// # Ok(())
/// # }
/// ```
pub struct KvStore {
    storage: Box<dyn KvStorage>,
}

impl Default for KvStore {
    /// Creates a default `KvStore`.
    /// By now, the background storage would be a HashMap
    fn default() -> Self {
        Self {
            storage: Box::new(HashMap::new()),
        }
    }
}

impl KvStore {
    /// Open/Create files in `path` to store the data
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(KvStore {
            storage: Box::new(LSMTree::new(path)?),
        })
    }

    // The following three methods just proxy its request to storage
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.storage.insert(key, value)
    }

    // In my opinion, get should not take a mutable reference
    // of self, because get should not change the "state" a store
    // use interior mutability pattern if necessary
    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        self.storage.get(&key)
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.storage.remove(&key)
    }
}
