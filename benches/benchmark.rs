use criterion::{Criterion, criterion_group, criterion_main};
use std::fs::read_to_string;

macro_rules! benchmark {
    ($year:ident $($day:ident),*) => {
        $(
        paste::item! {
            fn [<bench_ $year _ $day>](c: &mut Criterion){
                let mut group = c.benchmark_group(format!("{}/{}", stringify!($year), stringify!($day)));
                let path = format!("input/{}/{}.txt", stringify!($year), stringify!($day));
                let data = read_to_string(path).unwrap();
                let input = aoc::$year::$day::parse_input(data.as_str()).unwrap();
                group.bench_with_input("parse_input", &data.as_str(), |b, data| b.iter(|| aoc::$year::$day::parse_input(data)));
                group.bench_with_input("part_1", &input, |b, input| b.iter(|| aoc::$year::$day::part1(input)));
                group.bench_with_input("part_2", &input, |b, input| b.iter(|| aoc::$year::$day::part2(input)));
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
