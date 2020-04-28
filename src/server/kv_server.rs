use crate::store::Store;
use futures_locks::Mutex;
use grpcio::{RpcContext, UnarySink};
use rpc::minikv::{ScanRequest, DeleteResponse, GetRequest, ScanResponse, PutRequest, PutResponse, GetResponse, DeleteRequest};
use mini_kv::shared::bytes::copy_bytes;
use futures01::future::Future;

#[derive(Clone)]
pub struct KVServer {
    // todo: use rwlock
    // todo: lock lines instead of whole table
    // note: the Mutex in futures_locks has a built-in Arc
    store: Mutex<Store>,
}

impl rpc::minikv_grpc::MiniKvServer for KVServer {
    fn get(&mut self, ctx: RpcContext<'_>, req: GetRequest, sink: UnarySink<GetResponse>) {
        let mut response = GetResponse::default();
        let mut key = [0u8; 8];
        // todo: return error when req.get_key().len() != 8
        copy_bytes(req.get_key(), &mut key);
        let lock = self.store.clone();
        let f = lock.lock()
            .map(move |guard| {
                if let Some(value) = guard.get(&key) {
                    response.set_success(true);
                    response.set_value(value.to_vec());
                } else {
                    response.set_success(false);
                    response.set_errorMessage("key not found".to_string())
                }
                response
            }).then(move |response| {
            let response = response.unwrap();
            sink.success(response)
                .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
                .map(|_| ())
        });
        ctx.spawn(f)
    }

    fn put(&mut self, ctx: RpcContext<'_>, req: PutRequest, sink: UnarySink<PutResponse>) {
        let mut response = PutResponse::default();
        response.set_success(true);
        let mut error_messages = vec![];
        if req.get_key().len() != 8 {
            response.set_success(false);
            error_messages.push("the key must be 8 bytes, padding if necessary".to_string());
        }
        if req.get_value().len() != 256 {
            response.set_success(false);
            error_messages.push("the key must be 256 bytes, padding if necessary".to_string());
        }
        if !error_messages.is_empty() {
            let error_message = error_messages.join("\n");
            response.set_errorMessage(error_message);
        }
        let mut key = [0u8; 8];
        let mut value = [0u8; 256];
        copy_bytes(req.get_key(), &mut key);
        copy_bytes(req.get_value(), &mut value);

        if response.get_success() {
            let f = self.store.lock()
                .map(move |mut it| {
                    it.put(key, value);
                })
                .then(|_| {
                    sink.success(response)
                        .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
                        .map(|_| ())
                });
            ctx.spawn(f);
        } else {
            let f = sink.success(response)
                .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
                .map(|_| ());
            ctx.spawn(f);
        }
    }

    fn delete(&mut self, ctx: RpcContext<'_>, req: DeleteRequest, sink: UnarySink<DeleteResponse>) {
        let mut response = DeleteResponse::default();
        let mut key = [0u8; 8];
        // todo: return error when req.get_key().len() != 8
        copy_bytes(req.get_key(), &mut key);
        let f = self.store.lock()
            .map(move |mut guard| {
                guard.delete(key)
            })
            .then(|removed| {
                if removed.is_ok() && removed.unwrap().is_some() {
                    response.set_success(true);
                } else {
                    response.set_success(false);
                    response.set_errorMessage("key not found".to_string());
                }
                sink.success(response)
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
                    .map(|_| ())
            });
        ctx.spawn(f)
    }

    fn scan(&mut self, ctx: RpcContext<'_>, req: ScanRequest, sink: UnarySink<ScanResponse>) {
        let mut response = ScanResponse::default();
        let f = self.store.lock()
            .map(move |guard| {
                guard.scan(req.cursor as _, 16)
            }).then(|result| {
            let result = result.unwrap();
            response.set_cursor(result.0 as _);
            let keys: Vec<_> = result.1.iter()
                .map(|it| it.to_vec())
                .collect();
            response.set_result(keys.into());
            sink.success(response)
                .map_err(move |e| println!("failed to reply: {:?}", e))
                .map(|_| ())
        });
        ctx.spawn(f)
    }
}

impl KVServer {
    pub fn new(store: Store) -> Self {
        Self {
            store: Mutex::new(store)
        }
    }
}