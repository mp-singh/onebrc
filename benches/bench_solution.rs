use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use onebrc::soln;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solution_one_million", |b| b.iter(soln));
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}

criterion_main!(benches);
