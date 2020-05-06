use crate::error::Error;
use crate::server::storage::KvStorage;
use crate::Result;
use sled::{Db, Tree};

/// Wrapper of `sled::Db`
#[derive(Clone)]
pub struct SledKvsStorage(Db);

impl SledKvsStorage {
    /// Creates a `SledKvsStorage` from `sled::Db`.
    pub fn new(db: Db) -> Self {
        SledKvsStorage(db)
    }
}

impl KvStorage for SledKvsStorage {
    fn insert(&mut self, key: String, value: String) -> Result<()> {
        let tree: &Tree = &self.0;
        tree.insert(key, value.into_bytes()).map(|_| ())?;
        tree.flush()?;
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<String>> {
        let tree: &Tree = &self.0;
        Ok(tree
            .get(key)?
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&mut self, key: &str) -> Result<()> {
        let tree: &Tree = &self.0;
        tree.remove(key)?.ok_or(Error::KeyNotFound)?;
        tree.flush()?;
        Ok(())
    }
}
