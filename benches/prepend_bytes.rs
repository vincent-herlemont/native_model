/// Found a way to prepend bytes at the beginning of a Vec<u8> with a constant overhead.
use bincode::{Decode, Encode};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");

    // 1 byte, 1KB, 1MB, 10MB, 100MB
    for nb_bytes in [1, 1024, 1024 * 1024, 10 * 1024 * 1024, 100 * 1024 * 1024].into_iter() {
        group.throughput(criterion::Throughput::Bytes(nb_bytes as u64));

        let header: Vec<u8> = vec![0; 4];
        let mut data: Vec<u8> = vec![1; nb_bytes];
        group.bench_function(BenchmarkId::new("prepend_bytes", nb_bytes), |b| {
            b.iter(|| {
                // Fastest way to prepend bytes to data
                let mut header = header.clone();
                header.append(&mut data);
                // prepend bytes to data
                // let mut header = header.clone();
                // header.extend_from_slice(&data);
            });
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
