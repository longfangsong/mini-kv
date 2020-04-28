use crate::store::Store;
use futures_locks::{RwLock};
use grpcio::{RpcContext, UnarySink};
use rpc::minikv::{ScanRequest, DeleteResponse, GetRequest, ScanResponse, PutRequest, PutResponse, GetResponse, DeleteRequest};
use mini_kv::shared::bytes::copy_bytes;
use futures01::future::Future;

#[derive(Clone)]
pub struct KVServer {
    // todo: lock lines instead of whole table
    // note: the Mutex in futures_locks has a built-in Arc
    store: RwLock<Store>,
}

impl rpc::minikv_grpc::MiniKvServer for KVServer {
    fn get(&mut self, ctx: RpcContext<'_>, req: GetRequest, sink: UnarySink<GetResponse>) {
        debug!("GET {:?}", req.key);
        let mut response = GetResponse::default();
        let mut key = [0u8; 8];
        if req.key.len() != 8 {
            warn!("Get is called with a key {:?} which length is not 8, will padding/truncate it to 8 bytes", req.key)
        }
        copy_bytes(req.get_key(), &mut key);
        let lock = self.store.clone();
        let f = lock.read()
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
                .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e))
                .map(|_| ())
        });
        ctx.spawn(f)
    }

    fn put(&mut self, ctx: RpcContext<'_>, req: PutRequest, sink: UnarySink<PutResponse>) {
        debug!("PUT {:?}", req.key);
        let mut response = PutResponse::default();
        response.set_success(true);
        let mut key = [0u8; 8];
        let mut value = [0u8; 256];
        if req.key.len() != 8 {
            warn!("Put is called with a key {:?} which length is not 8, will padding/truncate it to 8 bytes", req.key)
        }
        copy_bytes(&req.key, &mut key);
        if req.value.len() != 8 {
            warn!("Put is called with a value {:?} which length is not 256, will padding/truncate it to 8 bytes", req.value)
        }
        copy_bytes(&req.value, &mut value);
        let f = self.store.write()
            .map(move |mut it| {
                it.put(key, value);
            })
            .then(move |_| {
                sink.success(response)
                    .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e))
                    .map(|_| ())
            });
        ctx.spawn(f);
    }

    fn delete(&mut self, ctx: RpcContext<'_>, req: DeleteRequest, sink: UnarySink<DeleteResponse>) {
        debug!("DELETE {:?}", req.key);
        let mut response = DeleteResponse::default();
        let mut key = [0u8; 8];
        if req.key.len() != 8 {
            warn!("Delete is called with a key {:?} which length is not 8, will padding/truncate it to 8 bytes", req.key)
        }
        copy_bytes(req.get_key(), &mut key);
        let f = self.store.write()
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
                    .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e))
                    .map(|_| ())
            });
        ctx.spawn(f)
    }

    fn scan(&mut self, ctx: RpcContext<'_>, req: ScanRequest, sink: UnarySink<ScanResponse>) {
        debug!("SCAN at index: {:?}", req.cursor);
        let mut response = ScanResponse::default();
        let f = self.store.read()
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
                .map_err(move |e| error!("failed to reply: {:?}", e))
                .map(|_| ())
        });
        ctx.spawn(f)
    }
}

impl KVServer {
    pub fn new(store: Store) -> Self {
        Self {
            store: RwLock::new(store)
        }
    }
}