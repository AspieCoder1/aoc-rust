use aoc::{year2024, year2025};
use criterion::{Criterion, criterion_group, criterion_main};
use std::fs;
use std::hint::black_box;

fn benchmark_solutions(c: &mut Criterion) {
    // Load all registered solutions
    let mut all_solutions = year2024::get_solutions();
    all_solutions.extend(year2025::get_solutions());

    for sol in all_solutions {
        let path = format!("input/year{}/day{:02}.txt", sol.year, sol.day);

        // Only bench if the input file exists
        if let Ok(data) = fs::read_to_string(&path) {
            let group_name = format!("Year {} Day {:02}", sol.year, sol.day);

            c.bench_function(&group_name, |b| {
                b.iter(|| {
                    // black_box prevents the compiler from optimising away
                    // the code if it thinks the result isn't being used.
                    black_box((sol.wrapper)(black_box(&data)))
                });
            });
        }
    }
}

criterion_group!(benches, benchmark_solutions);
criterion_main!(benches);
