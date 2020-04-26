use rpc::minikv::{ScanRequest, GetRequest, PutRequest, DeleteRequest, GetResponse, PutResponse, DeleteResponse, ScanResponse};
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::compat::{Future01CompatExt, Compat};
use futures01::future::Future;
use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, UnarySink};

#[derive(Clone)]
struct MockKVServer;

impl rpc::minikv_grpc::MiniKvServer for MockKVServer {
    fn get(&mut self, ctx: RpcContext<'_>, req: GetRequest, sink: UnarySink<GetResponse>) {
        let mut response = GetResponse::default();
        let result = vec![0x01u8].iter().cycle().take(256).copied().collect();
        response.set_value(result);
        let f = sink
            .success(response)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn put(&mut self, ctx: RpcContext<'_>, req: PutRequest, sink: UnarySink<PutResponse>) {
        let mut response = PutResponse::default();
        response.set_success(true);
        let f = sink
            .success(response)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn delete(&mut self, ctx: RpcContext<'_>, req: DeleteRequest, sink: UnarySink<DeleteResponse>) {
        let mut response = DeleteResponse::default();
        response.set_success(true);
        let f = sink
            .success(response)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn scan(&mut self, ctx: RpcContext<'_>, req: ScanRequest, sink: UnarySink<ScanResponse>) {
        let mut response = ScanResponse::default();
        response.set_cursor(vec![0x0]);
        response.set_result(vec![vec![0x1], vec![0x2]].into());
        let f = sink
            .success(response)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = rpc::minikv_grpc::create_mini_kv_server(MockKVServer);
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
