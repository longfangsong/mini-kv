#[macro_use]
extern crate criterion;

use criterion::{BatchSize, Criterion, ParameterizedBenchmark};
use rand::rngs::StdRng;
use tempfile::TempDir;
use kvs::server::storage::KvStorage;
use kvs::server::storage::lsm_tree::LSMTree;
use rand::{SeedableRng, Rng};
use serde::export::from_utf8_lossy;
use std::iter;
use kvs::server::storage::sled::SledKvsStorage;
use std::time::Duration;

fn kvs_write_bench(c: &mut Criterion) {
    let bench = ParameterizedBenchmark::new(
        "kvs-write",
        |b: &mut criterion::Bencher, _| {
            b.iter_batched(|| {
                let mut buffer: Vec<(String, String)> = Vec::with_capacity(100);
                let mut rng = StdRng::from_seed([0; 32]);
                for _ in 0..100 {
                    let key_length: usize = rng.gen_range(1, 10_0000);
                    let value_length: usize = rng.gen_range(1, 10_0000);
                    let mut key: Vec<u8> = vec![];
                    key.resize_with(key_length, || rng.gen());
                    let mut value: Vec<u8> = vec![];
                    value.resize_with(value_length, || rng.gen());
                    buffer.push((from_utf8_lossy(&key).to_string(), from_utf8_lossy(&value).to_string()));
                }
                buffer
            }, |kv| {
                let temp_dir = TempDir::new().unwrap();
                let mut tree = LSMTree::new(temp_dir.path()).unwrap();
                for (key, value) in kv {
                    tree.insert(key, value).unwrap();
                }
            }, BatchSize::SmallInput)
        },
        iter::once(()),
    )
        .sample_size(10)
        .measurement_time(Duration::from_secs(120));
    c.bench("kvs-write_bench", bench);
}

fn sled_write_bench(c: &mut Criterion) {
    let bench = ParameterizedBenchmark::new(
        "sled-write",
        |b: &mut criterion::Bencher, _| {
            b.iter_batched(|| {
                let mut buffer: Vec<(String, String)> = Vec::with_capacity(100);
                let mut rng = StdRng::from_seed([0; 32]);
                for _ in 0..100 {
                    let key_length: usize = rng.gen_range(1, 10_0000);
                    let value_length: usize = rng.gen_range(1, 10_0000);
                    let mut key: Vec<u8> = vec![];
                    key.resize_with(key_length, || rng.gen());
                    let mut value: Vec<u8> = vec![];
                    value.resize_with(value_length, || rng.gen());
                    buffer.push((from_utf8_lossy(&key).to_string(), from_utf8_lossy(&value).to_string()));
                }
                buffer
            }, |kv| {
                let temp_dir = TempDir::new().unwrap();
                let mut tree = SledKvsStorage::new(sled::open(temp_dir.path()).unwrap());
                for (key, value) in kv {
                    tree.insert(key, value).unwrap();
                }
            }, BatchSize::SmallInput)
        },
        iter::once(()),
    )
        .sample_size(10)
        .measurement_time(Duration::from_secs(120));
    c.bench("sled-write_bench", bench);
}

fn kvs_read_bench(c: &mut Criterion) {
    let bench = ParameterizedBenchmark::new(
        "kvs-read",
        |b: &mut criterion::Bencher, _| {
            b.iter_batched(|| {
                let temp_dir = TempDir::new().unwrap();
                let mut tree = LSMTree::new(temp_dir.path()).unwrap();
                let mut keys: Vec<String> = Vec::with_capacity(100);
                let mut rng = StdRng::from_seed([0; 32]);
                for _ in 0..100 {
                    let key_length: usize = rng.gen_range(1, 10_0000);
                    let value_length: usize = rng.gen_range(1, 10_0000);
                    let mut key: Vec<u8> = vec![];
                    key.resize_with(key_length, || rng.gen());
                    let mut value: Vec<u8> = vec![];
                    value.resize_with(value_length, || rng.gen());
                    let key = from_utf8_lossy(&key).to_string();
                    let value = from_utf8_lossy(&value).to_string();
                    keys.push(key.clone());
                    tree.insert(key, value).unwrap();
                }
                let mut test_seq: Vec<String> = Vec::with_capacity(1000);
                for _ in 0..1000 {
                    test_seq.push(keys[rng.gen_range(0, 100)].clone());
                }
                (tree, test_seq)
            }, |(tree, test_seq)| {
                for item in test_seq {
                    tree.get(&item).unwrap();
                }
            }, BatchSize::SmallInput)
        },
        iter::once(()),
    )
        .sample_size(10)
        .measurement_time(Duration::from_secs(360));
    c.bench("kvs-read_bench", bench);
}

fn sled_read_bench(c: &mut Criterion) {
    let bench = ParameterizedBenchmark::new(
        "sled-read",
        |b: &mut criterion::Bencher, _| {
            b.iter_batched(|| {
                let temp_dir = TempDir::new().unwrap();
                let mut tree = SledKvsStorage::new(sled::open(temp_dir.path()).unwrap());
                let mut keys: Vec<String> = Vec::with_capacity(100);
                let mut rng = StdRng::from_seed([0; 32]);
                for _ in 0..100 {
                    let key_length: usize = rng.gen_range(1, 10_0000);
                    let value_length: usize = rng.gen_range(1, 10_0000);
                    let mut key: Vec<u8> = vec![];
                    key.resize_with(key_length, || rng.gen());
                    let mut value: Vec<u8> = vec![];
                    value.resize_with(value_length, || rng.gen());
                    let key = from_utf8_lossy(&key).to_string();
                    let value = from_utf8_lossy(&value).to_string();
                    keys.push(key.clone());
                    tree.insert(key, value).unwrap();
                }
                let mut test_seq: Vec<String> = Vec::with_capacity(1000);
                for _ in 0..1000 {
                    test_seq.push(keys[rng.gen_range(0, 100)].clone());
                }
                (tree, test_seq)
            }, |(tree, test_seq)| {
                for item in test_seq {
                    tree.get(&item).unwrap();
                }
            }, BatchSize::SmallInput)
        },
        iter::once(()),
    )
        .sample_size(10)
        .measurement_time(Duration::from_secs(360));
    c.bench("sled-read_bench", bench);
}

criterion_group!(benches, kvs_write_bench, sled_write_bench, kvs_read_bench, sled_read_bench);
criterion_main!(benches);
