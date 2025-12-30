//! Advent of Code 2025 Day 3
//! Link: <https://adventofcode.com/2025/day/3>
//!
use anyhow::Result;

pub fn main(data: &str) -> Result<(u64, u64)> {
    let input = parse_input(data).unwrap();
    let (part1, part2) = solve(input);

    Ok((part1, part2))
}

pub fn solve(input: Vec<Vec<u32>>) -> (u64, u64) {
    let part1 = part1(&input);
    let part2 = part2(&input);

    (part1, part2)
}

type NestedU32 = Vec<Vec<u32>>;

pub fn parse_input(input: &str) -> Result<Vec<Vec<u32>>, String> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| format!("Parsing error: Invalid digit '{}' found.", c))
                })
                .collect::<Result<Vec<u32>, String>>()
        })
        .collect::<Result<NestedU32, String>>()
}

pub fn part1(input: &[Vec<u32>]) -> u64 {
    max_joltage::<2>(input)
}

pub fn part2(input: &[Vec<u32>]) -> u64 {
    max_joltage::<12>(input)
}

pub fn max_joltage<const DIGITS: usize>(input: &[Vec<u32>]) -> u64 {
    let mut answer = 0;
    for bank in input {
        let mut start = 0;
        let mut acc = 0;
        for remaining_digits in (0..DIGITS).rev() {
            let end = bank.len() - remaining_digits;

            let slice = &bank[start..end];
            let (mut max_idx, mut max_digit) = (0, 0);

            for (idx, &digit) in slice.iter().enumerate() {
                if digit > max_digit {
                    max_idx = idx;
                    max_digit = digit;
                }
            }

            acc = acc * 10 + u64::from(max_digit);
            start += max_idx + 1;
        }
        answer += acc;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        let part1 = part1(&input);

        assert_eq!(part1, 357);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        let part2 = part2(&input);

        assert_eq!(part2, 3121910778619);
    }
}
