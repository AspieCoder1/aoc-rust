use criterion::{ criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use aoc::utils::grid::Grid;
use aoc::utils::point::Point;

fn bench_grid_ops(c: &mut Criterion) {
    // Setup a 100x100 grid for a realistic AoC workload
    let size = 100;
    let grid = Grid::new('.', size, size);
    let center = Point::new(50, 50);

    let mut group = c.benchmark_group("Grid Access");

    // Test indexing speed (crucial for BFS/DFS)
    group.bench_function("indexing", |b| {
        b.iter(|| {
            let val = grid[black_box(center)];
            black_box(val);
        })
    });

    // Test neighbor generation
    group.bench_function("cardinal_neighbors", |b| {
        b.iter(|| {
            for p in grid.cardinal_neighbors(black_box(center)) {
                black_box(p);
            }
        })
    });
    group.finish();

    let mut trans_group = c.benchmark_group("Grid Transformations");

    // Test memory-heavy operations
    trans_group.bench_function("rotate_right_100x100", |b| {
        b.iter(|| {
            black_box(grid.rotate_right());
        })
    });

    trans_group.bench_function("expand_100x100", |b| {
        b.iter(|| {
            black_box(grid.expand('#'));
        })
    });

    trans_group.finish();
}

criterion_group!(benches, bench_grid_ops);
criterion_main!(benches);