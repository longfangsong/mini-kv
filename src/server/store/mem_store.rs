use std::collections::{HashMap, BTreeMap};
use std::cmp::min;

pub trait MemStore: Send {
    fn put(&mut self, key: [u8; 8], value: [u8; 256]);
    // don't remove & here, need this to infer lifetime
    fn get(&self, key: &[u8; 8]) -> Option<&[u8; 256]>;
    fn delete(&mut self, key: [u8; 8]) -> Option<[u8; 256]>;
    fn scan(&self, at: usize, count: usize) -> (usize, Vec<[u8; 8]>);
}

impl MemStore for HashMap<[u8; 8], [u8; 256]> {
    fn put(&mut self, key: [u8; 8], value: [u8; 256]) {
        self.insert(key, value);
    }

    fn get(&self, key: &[u8; 8]) -> Option<&[u8; 256]> {
        self.get(key)
    }

    fn delete(&mut self, key: [u8; 8]) -> Option<[u8; 256]> {
        self.remove(&key)
    }

    fn scan(&self, at: usize, count: usize) -> (usize, Vec<[u8; 8]>) {
        let iter = self.keys().skip(at);
        let mut next_cursor = at + min(iter.clone().count(), count);
        if next_cursor >= self.len() {
            next_cursor = 0;
        }
        let result: Vec<_> = iter.take(count).cloned().collect();
        (next_cursor, result)
    }
}

#[cfg(test)]
fn do_test<T: MemStore>(store: &mut T) {
    use mini_kv::shared::bytes::copy_bytes;
    use mini_kv::shared::bytes::bytes_equal;
    let key = *b"00000001";
    let mut value = [0u8; 256];
    copy_bytes(b"a", &mut value);
    store.put(key, value);
    let key = *b"00000002";
    let mut value = [0u8; 256];
    copy_bytes(b"b", &mut value);
    store.put(key, value);
    let key = *b"00000001";
    let mut value = [0u8; 256];
    copy_bytes(b"c", &mut value);
    store.put(key, value);

    let key = *b"00000002";
    let mut value = [0u8; 256];
    copy_bytes(b"b", &mut value);
    assert!(bytes_equal(store.get(&key).unwrap(), &value));
    let key = *b"00000001";
    let mut value = [0u8; 256];
    copy_bytes(b"c", &mut value);
    assert!(bytes_equal(store.get(&key).unwrap(), &value));
    let scan_result = store.scan(0, 16);
    assert_eq!(scan_result.0, 0);
    assert_eq!(scan_result.1.len(), 2);

    store.delete(key);
    assert!(store.get(&key).is_none());
    let scan_result = store.scan(0, 16);
    assert_eq!(scan_result.0, 0);
    assert_eq!(scan_result.1.len(), 1);
}

#[test]
fn test_hashmap_store() {
    let mut store = HashMap::new();
    do_test(&mut store);
}

impl MemStore for BTreeMap<[u8; 8], [u8; 256]> {
    fn put(&mut self, key: [u8; 8], value: [u8; 256]) {
        self.insert(key, value);
    }

    fn get(&self, key: &[u8; 8]) -> Option<&[u8; 256]> {
        self.get(key)
    }

    fn delete(&mut self, key: [u8; 8]) -> Option<[u8; 256]> {
        self.remove(&key)
    }

    fn scan(&self, at: usize, count: usize) -> (usize, Vec<[u8; 8]>) {
        let iter = self.keys().skip(at);
        let next_cursor = at + min(iter.clone().count(), count);
        let result: Vec<_> = iter.take(count).cloned().collect();
        (next_cursor, result)
    }
}

#[test]
fn test_btreemap_store() {
    let mut store = HashMap::new();
    do_test(&mut store);
}