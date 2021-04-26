extern crate day2_lib;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day2_lib::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| b.iter(|| day2(black_box("../input"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);