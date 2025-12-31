//! Advent of Code 2024 Day 1
//!
//! Link: <https://adventofcode.com/2024/day/1>

use anyhow::{Error, Result};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

pub fn main(input: &str) -> Result<(usize, usize)> {
    let input = parse_input(input)?;
    Ok((part1(&input), part2(&input)))
}

fn parse_input(input: &str) -> Result<Input> {
    input.parse()
}

fn part1(input: &Input) -> usize {
    let mut acc = 0;
    let mut left_heap = BinaryHeap::from_iter(input.left_list.iter().copied().map(Reverse));
    let mut right_heap = BinaryHeap::from_iter(input.right_list.iter().copied().map(Reverse));

    while let (Some(Reverse(l)), Some(Reverse(r))) = (left_heap.pop(), right_heap.pop()) {
        acc += l.abs_diff(r);
    }
    acc
}

fn part2(input: &Input) -> usize {
    let mut counts = HashMap::new();

    for right_id in input.right_list.iter() {
        *counts.entry(right_id).or_default() += 1;
    }

    input
        .left_list
        .iter()
        .map(|&l| l * counts.get(&l).unwrap_or(&0))
        .sum()
}

#[derive(Debug, PartialEq)]
struct Input {
    left_list: Vec<usize>,
    right_list: Vec<usize>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left_list = Vec::new();
        let mut right_list = Vec::new();

        for line in s.lines() {
            let (left, right) = line.split_once("   ").unwrap();
            left_list.push(left.parse::<usize>()?);
            right_list.push(right.parse::<usize>()?);
        }

        Ok(Self {
            left_list,
            right_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_parse_input() {
        let input = parse_input(EXAMPLE).unwrap();
        let expected = Input {
            left_list: vec![3, 4, 2, 1, 3, 3],
            right_list: vec![4, 3, 5, 3, 9, 3],
        };

        assert_eq!(input, expected);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 31);
    }
}
