use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use onebrc::parse_decimal_to_integer_optimized;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_decimal_str_to_integer_optimized", |b| {
        b.iter(|| parse_decimal_to_integer_optimized(b"-12345.6"))
    });
    c.bench_function("parse_decial_v2", |b| {
        b.iter(|| onebrc::_parse_decimal_v1("-12345.6"))
    });
    c.bench_function("parse_decimal_str_to_float_std_lib", |b| {
        b.iter(|| "-12345.6".parse::<f32>().unwrap())
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}

criterion_main!(benches);
