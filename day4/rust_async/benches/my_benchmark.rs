extern crate day4_lib;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day4_lib::*;
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("1", |b| {
        b.iter(|| day4(black_box("256310-732736"), black_box(1)))
    });

    c.bench_function("6", |b| {
        b.iter(|| day4(black_box("256310-732736"), black_box(6)))
    });

    c.bench_function("12", |b| {
        b.iter(|| day4(black_box("256310-732736"), black_box(12)))
    });

    c.bench_function("18", |b| {
        b.iter(|| day4(black_box("256310-732736"), black_box(18)))
    });

    c.bench_function("24", |b| {
        b.iter(|| day4(black_box("256310-732736"), black_box(24)))
    });

    c.bench_function("48", |b| {
        b.iter(|| day4(black_box("256310-732736"), black_box(48)))
    });

    c.bench_function("96", |b| {
        b.iter(|| day4(black_box("256310-732736"), black_box(96)))
    });

    c.bench_function("10000", |b| {
        b.iter(|| day4(black_box("256310-732736"), black_box(10000)))
    });

    c.bench_function("50000", |b| b.iter(|| day4(black_box("256310-732736"), black_box(50000))));

    c.bench_function("100000", |b| b.iter(|| day4(black_box("256310-732736"), black_box(100000))));

    c.bench_function("1000000", |b| b.iter(|| day4(black_box("256310-732736"), black_box(1000000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
