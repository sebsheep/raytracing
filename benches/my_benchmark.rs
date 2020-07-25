use criterion::{black_box, criterion_group, criterion_main, Criterion};



fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create image", |b| b.iter(|| raytracing::create_image()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);