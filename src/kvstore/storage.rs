use std::collections::{BTreeMap, HashMap};

use crate::error::Error;
use crate::Result;

/// an abstraction over key-value storage
pub trait KvStorage {
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn insert(&mut self, key: String, value: String) -> Result<()>;

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&self, key: &str) -> Result<Option<String>>;

    /// Remove a given key.
    fn remove(&mut self, key: &str) -> Result<()>;
}

/// `HashMap` is one of the implementations of KvStorage
/// and it is also the default storage
///
/// All its methods are direct proxies to `HashMap`'s own method with same name
impl<S: std::hash::BuildHasher> KvStorage for HashMap<String, String, S> {
    fn insert(&mut self, key: String, value: String) -> Result<()> {
        HashMap::insert(self, key, value);
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<String>> {
        Ok(HashMap::get(self, key).cloned())
    }

    fn remove(&mut self, key: &str) -> Result<()> {
        if let Some(_removed) = HashMap::remove(self, key) {
            Ok(())
        } else {
            Err(Error::KeyNotFound)
        }
    }
}

// It's sad that Rust itself doesn't provide its own abstraction over Hash and BTree map
/// `BTreeMap` is another implementations of KvStorage
///
/// All its methods are direct proxies to `BTreeMap`'s own method with same name
impl KvStorage for BTreeMap<String, String> {
    fn insert(&mut self, key: String, value: String) -> Result<()> {
        BTreeMap::insert(self, key, value);
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<String>> {
        Ok(BTreeMap::get(self, key).cloned())
    }

    fn remove(&mut self, key: &str) -> Result<()> {
        if let Some(_removed) = BTreeMap::remove(self, key) {
            Ok(())
        } else {
            Err(Error::KeyNotFound)
        }
    }
}
