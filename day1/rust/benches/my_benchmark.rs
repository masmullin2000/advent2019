extern crate day1_lib;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day1_lib::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| b.iter(|| day1(black_box("input"))));
    c.bench_function("fc", |b| b.iter(|| fc(black_box(42))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);