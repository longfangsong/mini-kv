use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use rpc::minikv_grpc::MiniKvServerClient;
use rpc::minikv::{GetRequest, PutRequest, DeleteRequest, GetResponse, PutResponse, DeleteResponse, ScanResponse, ScanRequest};

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = MiniKvServerClient::new(ch);
    let mut request = PutRequest::default();
    request.set_key(vec![0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    request.set_value(vec![0x01u8].repeat(256));
    let response = client.put(&request).unwrap();
    println!("{:?}", response);
    let mut request = GetRequest::default();
    request.set_key(vec![0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    let response = client.get(&request).unwrap();
    println!("{:?}", response);
    let mut request = PutRequest::default();
    request.set_key(vec![0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    request.set_value(vec![0x02u8].repeat(256));
    let response = client.put(&request).unwrap();
    println!("{:?}", response);
    let mut request = GetRequest::default();
    request.set_key(vec![0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    let response = client.get(&request).unwrap();
    println!("{:?}", response);
    let mut request = PutRequest::default();
    request.set_key(vec![0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x09]);
    request.set_value(vec![0x03u8].repeat(256));
    let response = client.put(&request).unwrap();
    println!("{:?}", response);
    let mut request = ScanRequest::default();
    request.set_cursor(vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);
    let response = client.scan(&request).unwrap();
    println!("{:?}", response);
}
