extern crate core;

macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

library!(utils "Utility functions" grid, disjointset, read_lines, simplex);
library!(year2025 "Advent of Code 2025" day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12);
library!(year2024 "Advent of Code 2024" day01, day02, day03, day04, day05, day06);
