use crate::store::mem_store::MemStore;
use crate::store::redo_log::RedoLog;

mod mem_store;
mod redo_log;

pub struct Store {
    mem_store: Box<dyn MemStore>,
    redo_log: Box<dyn RedoLog>,
}

impl Store {
    pub fn new<S: 'static + MemStore, L: 'static + RedoLog>(
        mem_store: S,
        redo_log: L,
    ) -> Self {
        let mut mem_store = mem_store;
        let mut redo_log = redo_log;
        // todo: remove this when impl based on lsm-tree is ready
        redo_log.redo(&mut mem_store);
        Self {
            mem_store: Box::new(mem_store),
            redo_log: Box::new(redo_log),
        }
    }

    pub fn put(&mut self, key: [u8; 8], value: [u8; 256]) {
        self.mem_store.put(key, value);
        self.redo_log.log_put(key, &value);
    }
    pub fn get(&self, key: &[u8; 8]) -> Option<&[u8; 256]> {
        self.mem_store.get(key)
    }
    pub fn delete(&mut self, key: [u8; 8]) -> Option<[u8; 256]> {
        let result = self.mem_store.delete(key);
        if result.is_some() {
            self.redo_log.log_delete(key);
        }
        result
    }
    pub fn scan(&self, at: usize, count: usize) -> (usize, Vec<[u8; 8]>) {
        self.mem_store.scan(at, count)
    }
}
