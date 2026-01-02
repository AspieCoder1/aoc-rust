//! Advent of Code 2024 Day 10
//!
//! Link: <https://adventofcode.com/2024/day/10>

use crate::utils::grid::Grid;
use anyhow::Result;
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let grid = Grid::<char>::from_str(input_data)?;

    Ok((part1(&grid), part2(&grid)))
}

fn part1(input: &Grid<char>) -> usize {
    input
        .all_positions(|&c| c == '0')
        .par_bridge()
        .map(|pos| {
            let mut visited = HashSet::from([pos]);
            let mut queue = VecDeque::from([pos]);
            let mut trailheads = HashSet::new();

            while let Some(pos) = queue.pop_front() {
                visited.insert(pos);
                if input[pos] == '9' {
                    trailheads.insert(pos);
                    continue;
                }
                for neighbour in input.cardinal_neighbors(pos) {
                    let a = input[pos].to_digit(10).unwrap_or(0);
                    let b = input[neighbour].to_digit(10).unwrap_or(0);

                    if b > a && a.abs_diff(b) == 1 && !visited.contains(&neighbour) {
                        queue.push_back(neighbour);
                    }
                }
            }
            trailheads.len()
        })
        .sum()
}

fn part2(input: &Grid<char>) -> usize {
    input
        .all_positions(|&c| c == '0')
        .par_bridge()
        .map(|pos| {
            let mut visited = HashSet::from([pos]);
            let mut queue = VecDeque::from([pos]);
            let mut acc = 0;

            while let Some(pos) = queue.pop_front() {
                visited.insert(pos);
                if input[pos] == '9' {
                    acc += 1;
                    continue;
                }
                for neighbour in input.cardinal_neighbors(pos) {
                    let a = input[pos].to_digit(10).unwrap_or(0);
                    let b = input[neighbour].to_digit(10).unwrap_or(0);

                    if b > a && a.abs_diff(b) == 1 && !visited.contains(&neighbour) {
                        queue.push_back(neighbour);
                    }
                }
            }
            acc
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        let input = Grid::<char>::from_str(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 36);
    }

    #[test]
    fn test_part2() {
        let input = Grid::<char>::from_str(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 81);
    }
}
