use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use aoc::utils::point::Point;

fn bench_point_arithmetic(c: &mut Criterion) {
    let p1 = Point::new(123, 456);
    let p2 = Point::new(789, 101);

    let mut group = c.benchmark_group("Point Arithmetic");

    group.bench_function("add", |b| {
        b.iter(|| black_box(p1) + black_box(p2))
    });

    group.bench_function("mul_scalar", |b| {
        b.iter(|| black_box(p1) * black_box(10))
    });

    group.finish();
}

fn bench_point_geometry(c: &mut Criterion) {
    let p1 = Point::new(10, 20);
    let p2 = Point::new(100, 200);

    let mut group = c.benchmark_group("Point Geometry");

    group.bench_function("manhattan", |b| {
        b.iter(|| black_box(p1).manhattan_distance(black_box(&p2)))
    });

    group.bench_function("euclidean", |b| {
        b.iter(|| black_box(p1).euclidean_squared(black_box(&p2)))
    });

    group.bench_function("rotate_cw", |b| {
        b.iter(|| black_box(p1).rotate_right_90())
    });

    group.finish();
}

fn bench_point_utility(c: &mut Criterion) {
    let p = Point::new(-50, 150);

    c.bench_function("point_wrap", |b| {
        b.iter(|| black_box(p).wrap(black_box(100), black_box(100)))
    });
}

criterion_group!(benches, bench_point_arithmetic, bench_point_geometry, bench_point_utility);
criterion_main!(benches);