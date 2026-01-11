use aoc::utils::disjointset::{DisjointSet, MappingDisjointSet};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_comparisons(c: &mut Criterion) {
    let mut group = c.benchmark_group("DSU Comparison");
    let iterations = 1000;

    // 1. Raw DisjointSet (Integer based)
    group.bench_function("Raw DisjointSet (usize)", |b| {
        b.iter_batched(
            || DisjointSet::new(iterations),
            |mut dsu| {
                for i in 0..iterations - 1 {
                    dsu.union(black_box(i), black_box(i + 1));
                }
                dsu.find(black_box(0))
            },
            BatchSize::SmallInput,
        )
    });

    // 2. Mapping DisjointSet (String based)
    // We pre-generate strings to ensure we're measuring DSU/Hashing logic,
    // not string allocation time.
    let strings: Vec<String> = (0..iterations).map(|i| i.to_string()).collect();

    group.bench_function("Mapping DisjointSet (String)", |b| {
        b.iter_batched(
            MappingDisjointSet::new,
            |mut dsu| {
                for i in 0..iterations - 1 {
                    dsu.union(
                        black_box(strings[i].clone()),
                        black_box(strings[i + 1].clone()),
                    );
                }
                dsu.is_connected(black_box(&strings[0]), black_box(&strings[iterations - 1]))
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_path_compression(c: &mut Criterion) {
    let size = 100_000;
    let mut dsu = DisjointSet::new(size);
    // Create one massive chain
    for i in 0..size - 1 {
        dsu.union(i, i + 1);
    }

    c.bench_function("DSU Path Compression (Deep Tree)", |b| {
        // We use a separate find for each iteration to see the
        // transition from O(N) to O(alpha(N))
        b.iter(|| black_box(dsu.find(black_box(0))))
    });
}

criterion_group!(benches, bench_comparisons, bench_path_compression);
criterion_main!(benches);
