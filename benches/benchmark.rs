use criterion::{Criterion, criterion_group, criterion_main};

macro_rules! benchmark {
    ($year:ident $($day:ident),*) => {
        $(
        paste::item! {
            fn [<bench_ $year _ $day>](c: &mut Criterion){
                let mut group = c.benchmark_group(format!("{}/{}", stringify!($year), stringify!($day)));
                for i in 0..=1 {
                    let input = aoc::$year::$day::parse_input(i).unwrap();
                    group.bench_with_input(format!("parse_input_{}", i).as_str(), &i, |b, &i| b.iter(|| aoc::$year::$day::parse_input(i)));
                    group.bench_with_input(format!("part1_{}", i).as_str(), &input, |b, input| b.iter(|| aoc::$year::$day::part1(input)));
                    group.bench_with_input(format!("part2_{}", i).as_str(), &input, |b, input| b.iter(|| aoc::$year::$day::part2(input)));
                }
                group.finish();
            }
        }
        )*

        paste::item! {
            criterion_group!($year, $([<bench_ $year _ $day>]),*);
        }
    };
}

benchmark!(year2025 day01, day02, day03, day04, day05, day06, day07);
criterion_main!(year2025);
