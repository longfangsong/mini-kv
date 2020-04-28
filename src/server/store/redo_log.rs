use std::io::{Write, Read};
use crate::store::mem_store::MemStore;

pub trait RedoLog: Send {
    fn log_put(&mut self, key: &[u8; 8], value: &[u8; 256]);
    fn log_delete(&mut self, key: &[u8; 8]);
    fn redo(&mut self, store: &mut dyn MemStore);
}

impl<T: Read + Write + Send> RedoLog for T {
    fn log_put(&mut self, key: &[u8; 8], value: &[u8; 256]) {
        // todo: error handling
        self.write_all(b"   put").unwrap_or(());
        self.write_all(key).unwrap_or(());
        self.write_all(value).unwrap_or(());
        self.flush().unwrap_or(());
    }

    fn log_delete(&mut self, key: &[u8; 8]) {
        // todo: error handling
        self.write_all(b"delete").unwrap_or(());
        self.write_all(key).unwrap_or(());
        self.flush().unwrap_or(());
    }

    fn redo(&mut self, store: &mut dyn MemStore) {
        let mut op = [0u8; 6];
        while self.read_exact(&mut op).is_ok() {
            match &op {
                b"   put" => {
                    let mut key = [0u8; 8];
                    self.read_exact(&mut key).unwrap_or(());
                    let mut value = [0u8; 256];
                    self.read_exact(&mut value).unwrap_or(());
                    store.put(key, value);
                }
                b"delete" => {
                    let mut key = [0u8; 8];
                    self.read_exact(&mut key).unwrap_or(());
                    store.delete(key);
                }
                _ => {
                    // todo: log error
                }
            }
        }
    }
}

#[test]
fn test_file() {
    use tempfile::NamedTempFile;
    use mini_kv::shared::bytes::copy_bytes;
    use mini_kv::shared::bytes::bytes_equal;
    use std::collections::HashMap;

    let mut file = NamedTempFile::new().unwrap();

    let key = *b"00000001";
    let mut value = [0u8; 256];
    copy_bytes(b"a", &mut value);
    file.log_put(&key, &value);
    let key = *b"00000002";
    let mut value = [0u8; 256];
    copy_bytes(b"b", &mut value);
    file.log_put(&key, &value);
    let key = *b"00000001";
    let mut value = [0u8; 256];
    copy_bytes(b"c", &mut value);
    file.log_put(&key, &value);
    let key = *b"00000002";
    file.log_delete(&key);

    let mut store = HashMap::new();
    let mut file = file.reopen().unwrap();
    file.redo(&mut store);
    let scan_result = store.scan(0, 16);
    assert_eq!(scan_result.0, 0);
    assert_eq!(scan_result.1.len(), 1);
    let key = *b"00000001";
    let mut value = [0u8; 256];
    copy_bytes(b"c", &mut value);
    assert!(bytes_equal(store.get(&key).unwrap(), &value));
}