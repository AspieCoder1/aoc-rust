use clap::Parser;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use aoc::{year2024, year2025, Solution};

#[derive(Parser, Debug)]
#[command(version, about = "🎄 Advent of Code Dashboard")]
struct Args {
    #[arg(short, long)]
    year: Option<u32>,
    #[arg(short, long)]
    day: Option<u32>,
}

fn main() {
    let args = Args::parse();

    // 1. Pretty Header
    println!("\n{}", " ❄  ADVENT OF CODE RUNNER  ❄ ".bold().white().on_blue());
    println!("{}", "=".repeat(32).blue());

    let all_solutions: Vec<Solution> = [
        year2024::get_solutions(),
        year2025::get_solutions(),
    ]
        .into_iter()
        .flatten()
        .filter(|s| args.year.is_none_or(|y| y == s.year))
        .filter(|s| args.day.is_none_or(|d| d == s.day))
        .collect();

    if all_solutions.is_empty() {
        println!("{}", "  No solutions matched your filters.".dimmed());
        return;
    }

    let multi = MultiProgress::new();
    let pb = multi.add(ProgressBar::new(all_solutions.len() as u64));

    // 2. Prettier Bar Style
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.magenta} {elapsed_precise} [{bar:30.white/blue}] {pos}/{len} {msg}",
        )
            .unwrap()
            .progress_chars("❄- "), // Using a snowflake as the progress char
    );

    let mut total_stars = 0;
    let mut total_duration = Duration::ZERO;
    let mut current_year = 0;

    for solution in all_solutions {
        // Grouping by Year
        if solution.year != current_year {
            current_year = solution.year;
            pb.println(format!("\n📅 {}", current_year.to_string().bold().underline().blue()));
        }

        pb.set_message(format!("Day {:02}", solution.day));

        let (stars, duration) = run_solution(&solution, &pb);

        total_stars += stars;
        total_duration += duration;
        pb.inc(1);
    }

    pb.finish_and_clear();

    // 3. Final Summary Table-style
    println!("\n{}", "📊 FINAL STATS".bold().magenta());
    println!("{}", "─".repeat(20).magenta());
    println!("⭐ Stars:    {}", total_stars.to_string().yellow().bold());
    println!("🕓 Time:     {}", format_duration(total_duration));
    println!("{}\n", "─".repeat(20).magenta());
}

fn run_solution(sol: &Solution, pb: &ProgressBar) -> (u32, Duration) {
    let path_str = format!("input/year{}/day{:02}.txt", sol.year, sol.day);
    let Ok(data) = fs::read_to_string(Path::new(&path_str)) else {
        pb.println(format!("  {} Day {:02}: {}", "⚠".red(), sol.day, "Input missing".dimmed()));
        return (0, Duration::ZERO);
    };

    let start = Instant::now();
    let (p1, p2) = (sol.wrapper)(&data);
    let elapsed = start.elapsed();

    // 4. Color-coded timing (Heatmap style)
    let time_color = if elapsed.as_millis() < 100 {
        elapsed.as_millis().to_string().green()
    } else if elapsed.as_millis() < 1000 {
        elapsed.as_millis().to_string().yellow()
    } else {
        elapsed.as_millis().to_string().red()
    };

    pb.println(format!(
        "  {} Day {:02} {} {} ms\n    {} {}\n    {} {}",
        "✨".blue(),
        sol.day,
        "─".dimmed(),
        time_color,
        "▪ Part 1:".dimmed(), p1.white().bold(),
        "▪ Part 2:".dimmed(), p2.white().bold(),
    ));

    (2, elapsed)
}

fn format_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        format!("{:.2}s", d.as_secs_f32()).red().bold().to_string()
    } else {
        format!("{}ms", d.as_millis()).cyan().bold().to_string()
    }
}