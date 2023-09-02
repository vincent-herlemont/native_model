use bincode::{Decode, Encode};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use native_model_macro::native_model;

#[derive(Encode, Decode)]
struct DataForBincode {
    x: i32,
    string: String,
}

// Encode 1 data with bincode
fn native_model_encode_body<T: Encode>(obj: &T) -> Result<Vec<u8>, bincode::error::EncodeError> {
    bincode::encode_to_vec(obj, bincode::config::standard())
}

fn native_model_decode_body<T: Decode>(data: Vec<u8>) -> Result<T, bincode::error::DecodeError> {
    bincode::decode_from_slice(&data, bincode::config::standard()).map(|(result, _)| result)
}

fn encode_with_bincode(data: &DataForBincode) -> Vec<u8> {
    native_model_encode_body(data).unwrap()
}

fn decode_with_bincode(data: Vec<u8>) -> DataForBincode {
    native_model_decode_body(data).unwrap()
}

fn encode_decode_with_bincode(data: &DataForBincode) -> DataForBincode {
    decode_with_bincode(encode_with_bincode(data))
}

#[derive(Encode, Decode)]
#[native_model(id = 1, version = 1)]
struct DataForNativeModel {
    x: i32,
    string: String,
}

fn encode_with_native_model(data: &DataForNativeModel) -> Vec<u8> {
    native_model::encode(data).unwrap()
}

fn decode_with_native_model(data: Vec<u8>) -> DataForNativeModel {
    let (data, _) = native_model::decode::<DataForNativeModel>(data).unwrap();
    data
}

fn encode_decode_with_native_model(data: &DataForNativeModel) -> DataForNativeModel {
    decode_with_native_model(encode_with_native_model(data))
}

fn criterion_benchmark(c: &mut Criterion) {
    // Bincode
    let data = DataForBincode {
        x: 1,
        // Set a very long string
        string: "Hello".repeat(10000),
    };
    c.bench_function("encode_with_bincode", |b| {
        b.iter(|| encode_with_bincode(black_box(&data)))
    });
    let encoded_data = encode_with_bincode(&data);
    c.bench_function("decode_with_bincode", |b| {
        b.iter(|| decode_with_bincode(black_box(encoded_data.clone())))
    });
    c.bench_function("encode_decode_with_bincode", |b| {
        b.iter(|| encode_decode_with_bincode(black_box(&data)))
    });

    // Native model
    let data = DataForNativeModel {
        x: 1,
        string: "Hello".repeat(10000),
    };
    c.bench_function("encode_with_native_model", |b| {
        b.iter(|| encode_with_native_model(black_box(&data)))
    });
    let encoded_data = native_model::encode(&data).unwrap();
    c.bench_function("decode_with_native_model", |b| {
        b.iter(|| decode_with_native_model(black_box(encoded_data.clone())))
    });
    c.bench_function("encode_decode_with_native_model", |b| {
        b.iter(|| encode_decode_with_native_model(black_box(&data)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
