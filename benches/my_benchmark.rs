use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
#[warn(unused_imports)]
use onebrc::solns::{soln1::soln1, soln2::soln2, soln3::soln3};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("soln");
    // group.bench_function("soln1", |b| b.iter(soln1));
    group.bench_function("soln2", |b| b.iter(soln2));
    // group.bench_function("soln3", |b| b.iter(soln3));
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}

criterion_main!(benches);
