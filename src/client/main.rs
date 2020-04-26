use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use rpc::minikv_grpc::MiniKvServerClient;
use rpc::minikv::{GetRequest, PutRequest, DeleteRequest, GetResponse, PutResponse, DeleteResponse, ScanResponse};

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = MiniKvServerClient::new(ch);
    let mut request = GetRequest::default();
    request.set_key(vec![0x01]);
    let response = client.get(&request).unwrap();
    println!("{:?}", response);
}
