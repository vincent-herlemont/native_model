use bincode::{Decode, Encode};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use native_model_macro::native_model;

fn native_model_encode_body<T: Encode>(obj: &T) -> Result<Vec<u8>, bincode::error::EncodeError> {
    bincode::encode_to_vec(obj, bincode::config::standard())
}

fn native_model_decode_body<T: Decode>(data: Vec<u8>) -> Result<T, bincode::error::DecodeError> {
    bincode::decode_from_slice(&data, bincode::config::standard()).map(|(result, _)| result)
}

#[derive(Encode, Decode)]
#[native_model(id = 1, version = 1)]
struct Data(Vec<u8>);

fn wrapper(data: &mut Vec<u8>) {
    native_model::wrapper::native_model_encode(data, 1, 1);
}

fn unwrap(data: &mut Vec<u8>) {
    native_model::wrapper::Wrapper::deserialize(&data[..]).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");

    // 1 byte, 1KB, 1MB, 10MB, 100MB
    for nb_bytes in [1, 1024, 1024 * 1024, 10 * 1024 * 1024, 100 * 1024 * 1024].into_iter() {
        group.throughput(criterion::Throughput::Bytes(nb_bytes as u64));

        // encode
        let data = Data(vec![1; nb_bytes]);
        let encode_body = native_model_encode_body(&data).unwrap();
        group.bench_function(BenchmarkId::new("encode", nb_bytes), |b| {
            b.iter(|| wrapper(&mut encode_body.clone()))
        });

        // decode
        let data = Data(vec![1; nb_bytes]);
        let encode_body = native_model::encode(&data).unwrap();
        group.bench_function(BenchmarkId::new("decode", nb_bytes), |b| {
            b.iter(|| unwrap(&mut encode_body.clone()))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
