extern crate day3_lib;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day3_lib::*;
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {

    let f_data = fs::read_to_string("../input").expect("didn't find the file");
    let (paths1, paths2) = process_data(f_data).unwrap();

    c.bench_function("gd", |b| b.iter(|| gd(black_box(&paths1), black_box(&paths2))));
    c.bench_function("day3", |b| b.iter(|| day3(black_box("../input"))));

    let f2_data = fs::read_to_string("../input").expect("didn't find the file");
    c.bench_function("proc", |b| b.iter(|| process_data(black_box(f2_data.to_owned()))));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);