use rpc::minikv::{ScanRequest, GetRequest, PutRequest, DeleteRequest, GetResponse, PutResponse, DeleteResponse, ScanResponse};
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::{
    channel::oneshot,
    executor::block_on,
    compat::Future01CompatExt,
};
use futures01::future::Future;
use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, UnarySink};
use futures_locks::Mutex;
use std::collections::HashMap;

#[derive(Clone)]
struct KVServer {
    // todo: lock lines instead of whole table
    store: Arc<Mutex<HashMap<[u8; 8], [u8; 256]>>>
}

impl rpc::minikv_grpc::MiniKvServer for KVServer {
    fn get(&mut self, ctx: RpcContext<'_>, req: GetRequest, sink: UnarySink<GetResponse>) {
        let mut response = GetResponse::default();
        let mut key = [0u8; 8];
        for (i, v) in req.key.iter().enumerate() {
            if let Some(r) = key.get_mut(i) {
                *r = *v;
            } else {
                break;
            }
        }
        let f = self.store.lock()
            .then(move |it| {
                let guard = it.unwrap();
                let value = guard.get(&key).unwrap();
                response.set_value(value.to_vec());
                sink.success(response)
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
                    .map(|_| ())
            });
        ctx.spawn(f)
    }

    fn put(&mut self, ctx: RpcContext<'_>, req: PutRequest, sink: UnarySink<PutResponse>) {
        let mut response = PutResponse::default();
        let mut key = [0u8; 8];
        let mut value = [0u8; 256];
        response.set_success(true);
        for (i, v) in req.key.iter().enumerate() {
            if let Some(r) = key.get_mut(i) {
                *r = *v;
            } else {
                response.set_success(false);
                break;
            }
        }
        for (i, v) in req.value.iter().enumerate() {
            if let Some(r) = value.get_mut(i) {
                *r = *v;
            } else {
                response.set_success(false);
                break;
            }
        }
        let f = self.store.lock()
            .map(move |mut it| {
                it.insert(key, value);
            })
            .then(|_| {
                response.set_success(true);
                sink.success(response)
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
                    .map(|_| ())
            });
        ctx.spawn(f)
    }

    fn delete(&mut self, ctx: RpcContext<'_>, req: DeleteRequest, sink: UnarySink<DeleteResponse>) {
        let mut response = DeleteResponse::default();
        let mut key = [0u8; 8];
        for (i, v) in req.key.iter().enumerate() {
            if let Some(r) = key.get_mut(i) {
                *r = *v;
            } else {
                break;
            }
        }
        let f = self.store.lock()
            .map(move |mut it| {
                it.remove(&key)
            }).then(|_| {
            response.set_success(true);
            sink.success(response)
                .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
                .map(|_| ())
        });
        ctx.spawn(f)
    }

    fn scan(&mut self, ctx: RpcContext<'_>, req: ScanRequest, sink: UnarySink<ScanResponse>) {
        let mut response = ScanResponse::default();
        response.set_cursor(vec![0x0]);
        let f = self.store.lock()
            .then(move |it| {
                let guard = it.unwrap();
                let value: Vec<Vec<u8>> = guard.keys().map(|it| it.to_vec()).collect();
                response.set_result(value.into());
                sink.success(response)
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
                    .map(|_| ())
            });
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let server = KVServer {
        store: Arc::new(Mutex::new(HashMap::new()))
    };
    let service = rpc::minikv_grpc::create_mini_kv_server(server);
    let quota = ResourceQuota::new(Some("MiniKVServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server.start();
    for (host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown().compat());
}
