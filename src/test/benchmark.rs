use std::collections::HashSet;
use rand::seq::IteratorRandom;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Instant;
use std::sync::Arc;
use std::sync::mpsc::channel;
use grpcio::{EnvBuilder, ChannelBuilder};
use rpc::minikv_grpc::MiniKvServerClient;
use rpc::minikv::{PutRequest, GetRequest, DeleteRequest, ScanRequest};
use rand::Rng;
use std::{thread, io};
use std::io::Read;

/// simulate pressure for the server
fn main() {
    // assume disturb of request
    let thread = 3;
    let put = 0.4;
    let get = 0.4;
    let delete = 0.1;
    let scan = 0.1;
    let get_fail_rate = 0.05;
    let mut txs = vec![];
    let start_time = Instant::now();
    let query_count = Arc::new(AtomicU64::new(0));
    for _ in 0..thread {
        let (tx, rx) = channel();
        txs.push(tx);
        let query_count = query_count.clone();
        thread::spawn(move || {
            let mut already_putted = HashSet::new();
            let mut rng = rand::thread_rng();
            let env = Arc::new(EnvBuilder::new().build());
            let address = "localhost:5884".to_string();
            let ch = ChannelBuilder::new(env).connect(&address);
            let client = MiniKvServerClient::new(ch);
            let mut key = [0u8; 8];
            let mut value = [0u8; 256];
            // fill a few values to prevent first few gets got nothing
            for _ in 0..10 {
                let mut put_request = PutRequest::new();
                key = rng.gen();
                rng.fill(&mut value);
                put_request.set_key(key.to_vec());
                put_request.set_value(value.to_vec());
                client.put(&put_request).unwrap();
                already_putted.insert(key);
            }
            while rx.try_recv().is_err() {
                query_count.fetch_add(1, Ordering::Relaxed);
                let select_function: f64 = rng.gen();
                if select_function < put {
                    // put
                    let mut put_request = PutRequest::new();
                    key = rng.gen();
                    rng.fill(&mut value);
                    put_request.set_key(key.to_vec());
                    put_request.set_value(value.to_vec());
                    already_putted.insert(key);
                    client.put(&put_request).unwrap();
                } else if select_function < put + get {
                    // get
                    let non_existing: f64 = rng.gen();
                    key = if non_existing < get_fail_rate {
                        // get a non-existing key
                        rng.gen() // this does not guarantee non-existing, but good enough
                    } else {
                        *already_putted.iter().choose(&mut rng).unwrap()
                    };
                    let mut get_request = GetRequest::new();
                    get_request.set_key(key.to_vec());
                    client.get(&get_request).unwrap();
                } else if select_function < put + get + delete {
                    // delete
                    let non_existing: f64 = rng.gen();
                    key = if non_existing < get_fail_rate {
                        // get a non-existing key
                        rng.gen() // this is not really non-existing, but good enough
                    } else {
                        *already_putted.iter().choose(&mut rng).unwrap()
                    };
                    let mut delete_request = DeleteRequest::new();
                    delete_request.set_key(key.to_vec());
                    client.delete(&delete_request).unwrap();
                    already_putted.remove(&key);
                } else {
                    let scan_at = rng.gen_range(0, already_putted.len());
                    let mut scan_request = ScanRequest::new();
                    scan_request.set_cursor(scan_at as _);
                    client.scan(&scan_request).unwrap();
                }
            }
        });
    }
    let mut buffer = [0u8; 1];
    println!("Press ENTER to exit...");
    io::stdin().lock().read_exact(&mut buffer).unwrap();
    txs.iter().for_each(|it| it.send(0).unwrap());
    let query_count = query_count.load(Ordering::Relaxed);
    println!("{:?} in {:?}", query_count, start_time.elapsed().as_secs());
    println!("QPS: {:?}", query_count / start_time.elapsed().as_secs());
}