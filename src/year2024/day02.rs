//! Advent of Code 2024 Day 2
//!
//! Link: <https://adventofcode.com/2024/day/2>

use anyhow::Result;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let input = parse_input(input_data);

    Ok((part1(&input), part2(&input)))
}

fn parse_input(input_data: &str) -> Vec<Vec<i32>> {
    input_data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<i32>]) -> usize {
    let mut safe_reports = 0;
    for report in input.iter() {
        safe_reports += check_report_is_safe(report) as usize;
    }
    safe_reports
}

fn part2(input: &[Vec<i32>]) -> usize {
    let mut safe_reports = 0;
    for report in input.iter() {
        if check_report_is_safe(report) {
            safe_reports += 1;
            continue;
        }

        // Otherwise try removing each level to see if that is valid
        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);

            if check_report_is_safe(&new_report) {
                safe_reports += 1;
                break;
            }
        }
    }
    safe_reports
}

fn check_report_is_safe(report: &[i32]) -> bool {
    let diffs = report
        .windows(2)
        .map(|chunk| chunk[0] - chunk[1])
        .collect::<Vec<_>>();
    let invalid_changes = diffs.iter().by_ref().any(|&x| x.abs() == 0 || 3 < x.abs());
    let all_pos_changes = diffs.iter().by_ref().all(|&x| x > 0);
    let all_neg_changes = diffs.iter().by_ref().all(|&x| x < 0);

    !invalid_changes && (all_pos_changes || all_neg_changes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE);
        let part1 = part1(&input);
        assert_eq!(part1, 2);
    }
}
