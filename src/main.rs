use clap::Parser;
use colored::Colorize;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

struct Solution {
    year: u32,
    day: u32,
    wrapper: fn(String) -> (String, String),
}

/// CLI to run Advent of Code solutions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Year to run
    #[arg(short, long)]
    year: Option<u32>,

    /// Day to run
    #[arg(short, long)]
    day: Option<u32>,
}

fn main() {
    let args = Args::parse();

    let year = args.year;
    let day = args.day;

    let solutions = [year2025()];

    let (star, duration) = solutions
        .iter()
        .flatten()
        .filter(|s| year.is_none_or(|y| y == s.year))
        .filter(|s| day.is_none_or(|d| d == s.day))
        .fold((0, Duration::ZERO), run_solution);

    println!("⭐ {}", star);
    println!("🕓 {} ms", duration.as_millis());
}

fn run_solution((stars, duration): (u32, Duration), solution: &Solution) -> (u32, Duration) {
    let Solution { year, day, wrapper } = solution;
    let data = read_to_string(Path::new(&format!("input/year{}/day{:02}.txt", year, day))).unwrap();
    let instant = Instant::now();
    let (part1, part2) = wrapper(data);
    let elapsed = instant.elapsed();

    println!("{}", format!("{year} Day {day}").green().bold());
    println!("    Part 1: {part1}");
    println!("    Part 2: {part2}");

    (stars + 2, duration + elapsed)
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$(
                Solution {
                    year: stringify!($year).strip_prefix("year").expect("Invalid year").parse().unwrap(),
                    day: stringify!($day).strip_prefix("day").expect("Invalid day").parse().unwrap(),
                    wrapper: |data: String| {
                        if let Ok((part1, part2)) = aoc::$year::$day::main(data.as_str()) {
                            return (part1.to_string(), part2.to_string())
                        } else {
                            return (String::from("???"), String::from("???"))
                        }
                    }
                }
            ,)*]
        }
    }
}

run!(year2025 day01, day02, day03, day04, day05, day06, day07, day08, day09, day10);
