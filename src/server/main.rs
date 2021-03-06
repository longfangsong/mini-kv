#[macro_use]
extern crate log;

mod store;
mod kv_server;

use std::io::{Read, Write};
use std::sync::Arc;
use std::{io, thread};
use futures::{
    channel::oneshot,
    executor::block_on,
    compat::Future01CompatExt,
};
use grpcio::{ChannelBuilder, Environment, ResourceQuota, ServerBuilder};
use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::env::args;
use std::str::FromStr;
use crate::store::Store;
use crate::kv_server::KVServer;
use pprof::protos::Message;

fn main() {
    let guard = if cfg!(profile) {
        Some(pprof::ProfilerGuard::new(100).unwrap())
    } else {
        None
    };
    env_logger::init();
    // todo: maybe make cq_count configurable
    let env = Arc::new(Environment::new(1));
    let mut args = args();
    let log_path = args.nth(1).unwrap_or_else(|| "./minikv.log".to_string());
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .write(true)
        .open(log_path)
        .unwrap_or_else(|_| {
            panic!("Cannot read or create log file!")
        });
    let store = Store::new(HashMap::new(), log_file);
    let server = KVServer::new(store);
    let service = rpc::minikv_grpc::create_mini_kv_server(server);
    let quota = ResourceQuota::new(Some("MiniKVServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .map(|s| u16::from_str(&s).unwrap_or_else(|_| {
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
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown().compat());
    if cfg!(profile) {
        let guard = guard.unwrap();
        if let Ok(report) = guard.report().build() {
            let mut file = File::create("profile.pb").unwrap();
            let profile = report.pprof().unwrap();

            let mut content = Vec::new();
            profile.encode(&mut content).unwrap();
            file.write_all(&content).unwrap();
        };
    }
}
