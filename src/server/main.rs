use rpc::minikv::{ScanRequest, GetRequest, PutRequest, DeleteRequest, GetResponse, PutResponse, DeleteResponse, ScanResponse};
use std::io::{Read, Write};
use std::sync::Arc;
use std::{io, thread};
use std::path::Path;
use futures::{
    channel::oneshot,
    executor::block_on,
    compat::Future01CompatExt,
};
use futures01::future::Future;
use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, UnarySink};
use futures_locks::Mutex;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::env::args;
use mini_kv::shared::bytes::copy_bytes;
use std::str::FromStr;

#[derive(Clone)]
struct KVServer {
    // todo: lock lines instead of whole table
    store: Arc<Mutex<HashMap<[u8; 8], [u8; 256]>>>,
    log: Arc<Mutex<File>>,
}

fn check_write_log_err<T, E>(result: Result<T, E>, op_description: &str) {
    if result.is_err() {
        eprintln!("Write log failed! {} won't be available in log!", op_description);
    }
}

impl rpc::minikv_grpc::MiniKvServer for KVServer {
    fn get(&mut self, ctx: RpcContext<'_>, req: GetRequest, sink: UnarySink<GetResponse>) {
        let mut response = GetResponse::default();
        let mut key = [0u8; 8];
        // todo: return error when req.get_key().len() != 8
        copy_bytes(req.get_key(), &mut key);
        let f = self.store.lock()
            .then(move |it| {
                let guard = it.unwrap();
                if let Some(value) = guard.get(&key) {
                    response.set_success(true);
                    response.set_value(value.to_vec());
                } else {
                    response.set_success(false);
                    response.set_errorMessage("key not found".to_string())
                }
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
            let log = self.log.clone();
            let f = self.store.lock()
                .map(move |mut it| {
                    it.insert(key, value);
                }).then(move |_| {
                // fixme: currently, this log system contains consistency problem
                // eg. request comes and handled in order put (A, 1), (A, 2), thus the A in memory is 2
                // but log written in order (A, 2), (A, 1), thus the recovered state of A is 1
                // one way to fix this problem is to use a Coarse-grained lock
                // to lock both memory data and log file
                log.lock().map(move |mut it| {
                    let err_string = format!("put {:?}", key);
                    check_write_log_err(it.write_all(b"   put"), &err_string);
                    check_write_log_err(it.write_all(&key), &err_string);
                    check_write_log_err(it.write_all(&value), &err_string);
                })
            }).then(|_| {
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
        let log = self.log.clone();
        let f = self.store.lock()
            .map(move |mut it| {
                it.remove(&key)
            })
            .then(move |_| {
                log.lock().map(move |mut it| {
                    let err_string = format!("delete {:?}", key);
                    check_write_log_err(it.write_all(b"delete"), &err_string);
                    check_write_log_err(it.write_all(&key), &err_string);
                })
            })
            .then(|removed| {
                if removed.is_ok() {
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
            .then(move |it| {
                let guard = it.unwrap();
                let value: Vec<Vec<u8>> = guard.keys()
                    .skip(req.cursor as _)
                    .take(16)
                    .map(|it| it.to_vec()).collect();
                if value.len() + req.cursor as usize >= guard.keys().len() {
                    response.set_cursor(0)
                } else {
                    response.set_cursor(req.cursor + value.len() as u64);
                }
                response.set_result(value.into());
                sink.success(response)
                    .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
                    .map(|_| ())
            });
        ctx.spawn(f)
    }
}

impl KVServer {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
            log: Arc::new(Mutex::new(OpenOptions::new().create(true).write(true).open("./minikv.log").unwrap())),
        }
    }
    pub fn from_log<P: AsRef<Path>>(path: P) -> Option<Self> {
        let mut file = File::open(&path).ok()?;
        let mut store = HashMap::new();
        let mut op = [0u8; 6];
        while file.read_exact(&mut op).is_ok() {
            match &op {
                b"   put" => {
                    let mut key = [0u8; 8];
                    file.read_exact(&mut key).ok()?;
                    let mut value = [0u8; 256];
                    file.read_exact(&mut value).ok()?;
                    store.insert(key, value);
                }
                b"delete" => {
                    let mut key = [0u8; 8];
                    file.read_exact(&mut key).ok()?;
                    store.remove(&key);
                }
                _ => {
                    return None;
                }
            }
        }
        drop(file);
        Some(Self {
            store: Arc::new(Mutex::new(store)),
            log: Arc::new(Mutex::new(OpenOptions::new().write(true).append(true).open(path).unwrap())),
        })
    }
}

fn main() {
    // todo: maybe make cq_count configurable
    let env = Arc::new(Environment::new(1));
    let mut args = args();
    let server = if let Some(path) = args.nth(1) {
        KVServer::from_log(path).unwrap_or_else(|| {
            println!("warn: load log failed, use new store");
            KVServer::new()
        })
    } else {
        KVServer::new()
    };
    let service = rpc::minikv_grpc::create_mini_kv_server(server);
    let quota = ResourceQuota::new(Some("MiniKVServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .map(|s| u16::from_str(&s).unwrap_or_else(|_| {
            eprintln!("PORT is not valid");
            panic!("PORT is not valid");
        }))
        .unwrap_or_else(|_| 5884);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind(host, port)
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
