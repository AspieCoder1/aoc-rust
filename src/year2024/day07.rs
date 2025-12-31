//! Advent of Code 2024 Day 7
//!
//! Link: <https://adventofcode.com/2024/day/7>

use anyhow::Result;
use std::collections::VecDeque;
use std::str::FromStr;
pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let input = parse_input(input_data)?;
    Ok((part1(&input), part2(&input)))
}

fn parse_input(input_data: &str) -> Result<Vec<Calibration>> {
    input_data.lines().map(Calibration::from_str).collect()
}

fn part1(input: &[Calibration]) -> usize {
    let mut acc = 0;
    for calibration in input {
        let mut queue = VecDeque::from([calibration.equation[0]]);
        for value in calibration.equation.iter().skip(1) {
            let mut next_vals = Vec::new();
            while let Some(val) = queue.pop_front() {
                next_vals.push(val + value);
                next_vals.push(val * value);
            }
            queue.extend(next_vals);
        }
        if queue.iter().any(|&x| x == calibration.value) {
            acc += calibration.value;
        }
    }
    acc
}

fn part2(input: &[Calibration]) -> usize {
    let mut acc = 0;
    for calibration in input {
        let mut queue = VecDeque::from([calibration.equation[0]]);
        for value in calibration.equation.iter().skip(1) {
            let mut next_vals = Vec::new();
            while let Some(val) = queue.pop_front() {
                next_vals.push(val + value);
                next_vals.push(val * value);
                let concat = val.to_string() + &value.to_string();
                next_vals.push(concat.parse::<usize>().unwrap());
            }
            queue.extend(next_vals);
        }
        if queue.iter().any(|&x| x == calibration.value) {
            acc += calibration.value;
        }
    }
    acc
}

struct Calibration {
    value: usize,
    equation: Vec<usize>,
}

impl FromStr for Calibration {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, equation) = s.split_once(": ").unwrap();
        let equation = equation
            .split(" ")
            .map(|x| x.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            value: value.parse()?,
            equation,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 3749);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 11387);
    }
}
