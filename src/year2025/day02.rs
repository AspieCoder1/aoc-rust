//! Advent of Code 2025 Day 2
//! Link: <https://adventofcode.com/2025/day/3>
//!
use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
use std::str::FromStr;

pub fn main(input: &str) -> Result<(u64, u64)> {
    let input = parse_input(input)?;

    let (part1, part2) = solve(input);

    Ok((part1, part2))
}

pub fn solve(input: Vec<IdRange>) -> (u64, u64) {
    let part1 = part1(&input);
    let part2 = part2(&input);

    (part1, part2)
}

pub fn parse_input(input: &str) -> Result<Vec<IdRange>> {
    input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect()
}

#[derive(Debug, Clone)]
pub struct IdRange {
    start: u64,
    end: u64,
}

impl FromStr for IdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s_start, s_end) = s
            .split_once('-')
            .ok_or_else(|| anyhow!("Invalid range format: {}", s))?;

        Ok(Self {
            start: s_start.trim().parse().context("Failed to parse start")?,
            end: s_end.trim().parse().context("Failed to parse end")?,
        })
    }
}

pub fn part1(inp: &[IdRange]) -> u64 {
    let mut total: u64 = 0;

    for range in inp {
        let mut r_start = range.start;
        let mut r_end = range.end;

        let mut start_digits = r_start.ilog10() + 1;
        let mut end_digits = r_end.ilog10() + 1;

        // Skip odd digit counts as they cannot be split into two equal halves
        if start_digits % 2 != 0 {
            r_start = 10_u64.pow(start_digits);
            start_digits += 1;
        }
        if end_digits % 2 != 0 {
            end_digits -= 1;
            r_end = 10_u64.pow(end_digits) - 1;
        }

        let half = start_digits / 2;
        let divisor = 10_u64.pow(half);

        let mut start_prefix = r_start / divisor;
        let mut end_prefix = r_end / divisor;

        // Ensure the full constructed number is within range
        if start_prefix * divisor + start_prefix < r_start {
            start_prefix += 1;
        }
        if end_prefix * divisor + end_prefix > r_end {
            end_prefix -= 1;
        }

        if start_prefix <= end_prefix {
            let count = end_prefix - start_prefix + 1;
            let sum_prefixes = (count * (start_prefix + end_prefix)) / 2;
            total += sum_prefixes * (divisor + 1);
        }
    }
    total
}

pub fn part2(inp: &[IdRange]) -> u64 {
    let mut part2_total: u64 = 0;
    let mut seen = HashSet::new();

    for range in inp {
        let max_digits = range.end.ilog10() + 1;

        for total_len in 2..=max_digits {
            for pattern_len in 1..=(total_len / 2) {
                if total_len % pattern_len == 0 {
                    let repetitions = total_len / pattern_len;
                    let p_start = 10_u64.pow(pattern_len - 1);
                    let p_end = 10_u64.pow(pattern_len) - 1;

                    for pattern in p_start..=p_end {
                        let mut full_num: u64 = 0;
                        let multiplier = 10_u64.pow(pattern_len);

                        for _ in 0..repetitions {
                            if let Some(next) = full_num.checked_mul(multiplier) {
                                full_num = next + pattern;
                            } else {
                                break;
                            }
                        }

                        if full_num >= range.start && full_num <= range.end {
                            if seen.insert(full_num) {
                                part2_total += full_num;
                            }
                        }
                    }
                }
            }
        }
        seen.clear();
    }
    part2_total
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 4174379265);
    }
}