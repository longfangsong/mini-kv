#![deny(missing_docs)]
//! A simple key/value store.

use std::collections::HashMap;

use crate::storage::KvStorage;

/// storage
pub mod storage;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `KvStorage`
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
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
    /// Creates a default `KvStore`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `KvStore` with certain backend.
    pub fn new_with_backend<T: KvStorage + 'static>(backend: T) -> Self {
        Self {
            storage: Box::new(backend),
        }
    }

    // The following three methods just proxy its request to storage
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Option<String> {
        self.storage.get(&key)
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) {
        self.storage.remove(&key);
    }
}
