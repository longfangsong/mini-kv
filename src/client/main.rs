use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use rpc::minikv_grpc::MiniKvServerClient;
use rpc::minikv::{GetRequest, PutRequest, DeleteRequest, ScanRequest};
use std::io::{stdin, BufRead, stdout};
use std::str::{from_utf8, FromStr};
use std::io::Write;
use mini_kv::shared::bytes::get_bytes_with_fill;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:5884");
    let client = MiniKvServerClient::new(ch);
    loop {
        let mut command_and_arg = String::new();
        print!(">>> ");
        stdout().flush().unwrap();
        stdin().lock().read_line(&mut command_and_arg).unwrap();
        command_and_arg = command_and_arg;
        let mut command_and_arg_iter = command_and_arg.split(' ').map(|it| it.trim());
        let command = command_and_arg_iter.next().unwrap().to_ascii_lowercase();
        // todo: separate these commands into their own function
        match &command[..] {
            "" => {}
            "put" => {
                let mut request = PutRequest::default();
                if let Some(arg1) = command_and_arg_iter.next() {
                    let key_str = arg1.as_bytes();
                    if key_str.len() != 8 {
                        println!("waring: key must be 8 bytes long, will padding/truncate to 8 bytes")
                    }
                    let key = get_bytes_with_fill(key_str, 8, 0x00);
                    request.set_key(key);
                    if let Some(arg2) = command_and_arg_iter.next() {
                        let value_str = arg2.as_bytes();
                        if value_str.len() != 256 {
                            println!("waring: value must be 256 bytes long, will padding/truncate to 256 bytes");
                        }
                        let mut value = vec![];
                        for i in 0..256 {
                            value.push(value_str.get(i).cloned().unwrap_or(0x00u8))
                        }
                        request.set_value(value);
                        let response = client.put(&request);
                        if let Ok(resp) = response {
                            if !resp.get_success() {
                                eprintln!("error: {}", resp.get_errorMessage());
                            }
                        } else {
                            eprintln!("{}", response.unwrap_err());
                        }
                    } else {
                        eprintln!("error: Must provide a key and a value");
                    }
                } else {
                    eprintln!("error: Must provide a key and a value");
                }
            }
            "get" => {
                let mut request = GetRequest::default();
                if let Some(arg) = command_and_arg_iter.next() {
                    let key_str = arg.as_bytes();
                    let key = get_bytes_with_fill(key_str, 8, 0x00);
                    request.set_key(key);
                    let response = client.get(&request);
                    if let Ok(resp) = response {
                        if !resp.get_success() {
                            eprintln!("error: {}", resp.get_errorMessage());
                        } else {
                            println!("{}", from_utf8(resp.get_value()).unwrap_or("<non-printable>"))
                        }
                    } else {
                        eprintln!("{}", response.unwrap_err());
                    }
                } else {
                    println!("Get command needs a key argument!")
                }
            }
            "delete" => {
                let mut request = DeleteRequest::default();
                let key_str = command_and_arg_iter.next().unwrap().as_bytes();
                let mut key = vec![];
                for i in 0..8 {
                    key.push(key_str.get(i).cloned().unwrap_or(0x00u8))
                }
                request.set_key(key);
                let response = client.delete(&request);
                if let Ok(resp) = response {
                    if !resp.get_success() {
                        eprintln!("error: {}", resp.get_errorMessage());
                    }
                } else {
                    eprintln!("{}", response.unwrap_err());
                }
            }
            "scan" => {
                let mut request = ScanRequest::default();
                if let Some(key_str) = command_and_arg_iter.next() {
                    if let Ok(key) = u64::from_str(key_str) {
                        request.set_cursor(key);
                        let response = client.scan(&request);
                        if let Ok(resp) = response {
                            println!("cursor: {}", resp.cursor);
                            for (i, key) in resp.result.iter().enumerate() {
                                println!("({}): {}", i, from_utf8(key).unwrap_or("<non-printable>"));
                            }
                        } else {
                            eprintln!("{}", response.unwrap_err());
                        }
                    } else {
                        eprintln!("Scan's cursor must be a number! Use 0 if you want to scan from the start.");
                    }
                } else {
                    eprintln!("Scan needs a cursor! Use 0 if you want to scan from the start.");
                }
            }
            &_ => println!("Invalid command")
        }
    }
}
