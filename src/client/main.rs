use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use rpc::minikv_grpc::MiniKvServerClient;
use rpc::minikv::{GetRequest, PutRequest, DeleteRequest, GetResponse, PutResponse, DeleteResponse, ScanResponse, ScanRequest};
use std::io::{stdin, BufRead, stdout};
use std::str::from_utf8;
use std::io::Write;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = MiniKvServerClient::new(ch);
    loop {
        let mut command_and_arg = String::new();
        print!(">>> ");
        stdout().flush().unwrap();
        stdin().lock().read_line(&mut command_and_arg).unwrap();
        command_and_arg = command_and_arg.to_ascii_lowercase();
        let mut command_and_arg_iter = command_and_arg.split(' ').map(|it| it.trim());
        let command = command_and_arg_iter.next().unwrap();
        match command {
            "put" => {
                let mut request = PutRequest::default();
                // todo: warn when key_str.len() != 8
                let key_str = command_and_arg_iter.next().unwrap().as_bytes();
                let mut key = vec![];
                for i in 0..8 {
                    key.push(key_str.get(i).cloned().unwrap_or(0x00u8))
                }
                request.set_key(key);
                let value_str = command_and_arg_iter.next().unwrap().as_bytes();
                let mut value = vec![];
                for i in 0..256 {
                    value.push(value_str.get(i).cloned().unwrap_or(0x00u8))
                }
                request.set_value(value);
                let response = client.put(&request).unwrap();
                // todo: better error handling
                assert!(response.success);
            }
            "get" => {
                let mut request = GetRequest::default();
                // todo: warn when key_str.len() != 8
                let key_str = command_and_arg_iter.next().unwrap().as_bytes();
                let mut key = vec![];
                for i in 0..8 {
                    key.push(key_str.get(i).cloned().unwrap_or(0x00u8))
                }
                request.set_key(key);
                let response = client.get(&request).unwrap();
                println!("{}", from_utf8(&response.value).unwrap());
            }
            "delete" => {
                let mut request = DeleteRequest::default();
                // todo: warn when key_str.len() != 8
                let key_str = command_and_arg_iter.next().unwrap().as_bytes();
                let mut key = vec![];
                for i in 0..8 {
                    key.push(key_str.get(i).cloned().unwrap_or(0x00u8))
                }
                request.set_key(key);
                let response = client.delete(&request).unwrap();
                assert!(response.success);
            }
            "scan" => {
                let mut request = ScanRequest::default();
                let key_str = command_and_arg_iter.next().unwrap().as_bytes();
                let mut key = vec![];
                for i in 0..8 {
                    key.push(key_str.get(i).cloned().unwrap_or(0x00u8))
                }
                request.set_cursor(key);
                let response = client.scan(&request).unwrap();
                println!("cursor: {}", from_utf8(&response.cursor).unwrap());
                for (i, key) in response.result.iter().enumerate() {
                    println!("({}): {}", i, from_utf8(key).unwrap());
                }
            }
            &_ => println!("Invalid command!")
        }
    }
    // let mut request = PutRequest::default();
    // request.set_key(vec![0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    // request.set_value(vec![0x01u8].repeat(256));
    // let response = client.put(&request).unwrap();
    // println!("{:?}", response);
    // let mut request = GetRequest::default();
    // request.set_key(vec![0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    // let response = client.get(&request).unwrap();
    // println!("{:?}", response);
    // let mut request = PutRequest::default();
    // request.set_key(vec![0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    // request.set_value(vec![0x02u8].repeat(256));
    // let response = client.put(&request).unwrap();
    // println!("{:?}", response);
    // let mut request = GetRequest::default();
    // request.set_key(vec![0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    // let response = client.get(&request).unwrap();
    // println!("{:?}", response);
    // let mut request = PutRequest::default();
    // request.set_key(vec![0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x09]);
    // request.set_value(vec![0x03u8].repeat(256));
    // let response = client.put(&request).unwrap();
    // println!("{:?}", response);
    // let mut request = ScanRequest::default();
    // request.set_cursor(vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);
    // let response = client.scan(&request).unwrap();
    // println!("{:?}", response);
}
