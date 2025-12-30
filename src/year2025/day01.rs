//! Advent of Code 2025 Day 1
//! Link: <https://adventofcode.com/2025/day/3>
//!
use anyhow::Result;
use std::str::FromStr;

pub fn main(input: &str) -> Result<(i64, i64)> {
    let input = parse_input(input)?;

    let (part1, part2) = solve(&input);

    Ok((part1, part2))
}

pub fn parse_input(input: &str) -> Result<Vec<Command>> {
    input.lines().map(str::parse).collect()
}

type Input = (i64, i64);

pub fn solve(input: &[Command]) -> Input {
    let mut dial: i64 = 50;
    let mut part1 = 0;
    let mut part2 = 0;

    for cmd in input {
        match cmd {
            Command::Left(d) => {
                let reversed = (100 - dial) % 100;
                part2 += (reversed + d) / 100;
                dial = (dial - d).rem_euclid(100);
            }
            Command::Right(d) => {
                part2 += (dial + d) / 100;
                dial = (dial + d) % 100;
            }
        }
        part1 += i64::from(dial == 0);
    }
    (part1, part2)
}

pub fn part1(input: &[Command]) -> i64 {
    solve(input).0
}

pub fn part2(input: &[Command]) -> i64 {
    solve(input).1
}

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Left(i64),
    Right(i64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;

        let (cmd, d) = s.split_at(1);
        let dist: i64 = d.parse()?;
        let c = cmd.chars().next().unwrap();

        Ok(match c {
            'L' => Left(dist),
            'R' => Right(dist),
            _ => unreachable!("Only 'L' or 'R' are valid first letters of commands."),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 6);
    }
}
