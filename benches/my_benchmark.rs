use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate ray_tracer;

use ray_tracer::create_image;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| create_image()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);