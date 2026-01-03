//! Advent of Code 2024 - Day 13
//!
//! Link: <https://adventofcode.com/2024/day/13>

use crate::utils::simplex::{LPBuilder, LPOps, branch_and_bound};
use anyhow::{Error, Result};
use regex::Regex;
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(i64, i64)> {
    let input = parse_input(input_data)?;
    Ok((part1(&input), part2(&input)))
}

fn parse_input(input: &str) -> Result<Vec<ClawMachine>> {
    input.split("\n\n").map(ClawMachine::from_str).collect()
}

fn part1(input: &[ClawMachine]) -> i64 {
    let mut acc = 0;
    for claw_machine in input {
        match branch_and_bound(claw_machine.to_part1_lp(), 2) {
            Some(solution) => {
                acc += solution;
            }
            None => continue,
        }
    }
    acc
}

fn part2(input: &[ClawMachine]) -> i64 {
    let mut acc = 0;
    for claw_machine in input {
        match branch_and_bound(claw_machine.to_part2_lp(), 2) {
            Some(solution) => {
                acc += solution;
            }
            None => continue,
        }
    }
    acc
}

#[derive(Debug)]
struct ClawMachine {
    button_a: [i64; 2],
    button_b: [i64; 2],
    prize: [i64; 2],
}

impl ClawMachine {
    fn to_part1_lp(&self) -> LPBuilder {
        let mut builder = LPBuilder::new();

        builder.add_constraint(
            vec![self.button_a[0], self.button_b[0]],
            LPOps::Eq,
            self.prize[0],
        );
        builder.add_constraint(
            vec![self.button_a[1], self.button_b[1]],
            LPOps::Eq,
            self.prize[1],
        );
        builder.add_constraint(vec![1, 0], LPOps::Lte, 100);
        builder.add_constraint(vec![0, 1], LPOps::Lte, 100);
        builder.add_objective(vec![3, 1]);
        builder
    }

    fn to_part2_lp(&self) -> LPBuilder {
        let mut builder = LPBuilder::new();

        builder.add_constraint(
            vec![self.button_a[0], self.button_b[0]],
            LPOps::Eq,
            self.prize[0] + 10000000000000,
        );
        builder.add_constraint(
            vec![self.button_a[1], self.button_b[1]],
            LPOps::Eq,
            self.prize[1] + 10000000000000,
        );
        builder.add_constraint(vec![1, 0], LPOps::Gte, 100);
        builder.add_constraint(vec![0, 1], LPOps::Gte, 100);
        builder.add_objective(vec![3, 1]);
        builder
    }
}

impl FromStr for ClawMachine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digit_regex = Regex::new(r"\d+")?;

        let nums: Vec<i64> = digit_regex
            .find_iter(s)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();

        if nums.len() < 6 {
            return Err(Error::msg("Invalid input"));
        }

        Ok(Self {
            button_a: [nums[0], nums[1]],
            button_b: [nums[2], nums[3]],
            prize: [nums[4], nums[5]],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_parse_input() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(input.len(), 4);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 480);
    }
}
