//! Advent of Code 2024 Day 4
//!
//! Link: <https://adventofcode.com/2024/day/4>

use crate::utils::grid::{Grid, GridError, Pos};
use anyhow::Result;
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let input = parse_input(input_data)?;

    Ok((part1(&input), part2(&input)))
}

fn parse_input(input_data: &str) -> Result<Grid<char>, GridError> {
    Grid::<char>::from_str(input_data)
}

fn part1(input: &Grid<char>) -> usize {
    let search_term = ['X', 'M', 'A', 'S'];
    let start_positions = input.all_positions(|&c| c == 'X').collect::<Vec<_>>();
    let mut num_occurrences = 0;
    let offsets: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for start_position in start_positions {
        for &offset in offsets.iter() {
            for (ind, &c) in input
                .clone()
                .dfs_one_direction(start_position, offset, 4)
                .iter()
                .enumerate()
            {
                if c != search_term[ind] {
                    break;
                }
                if ind == 3 {
                    num_occurrences += 1;
                }
            }
        }
    }
    num_occurrences
}

fn part2(input: &Grid<char>) -> usize {
    let mut num_occurrences = 0;
    let start_positions = input
        .all_positions(|&c| c == 'A')
        .filter(|&Pos(y, x)| y > 0 && y < input.height - 1 && x > 0 && x < input.width - 1)
        .collect::<Vec<_>>();

    for Pos(y, x) in start_positions {
        match (
            input[(y - 1, x - 1)],
            input[(y + 1, x + 1)],
            input[(y - 1, x + 1)],
            input[(y + 1, x - 1)],
        ) {
            ('M', 'S', 'M', 'S') => num_occurrences += 1,
            ('S', 'M', 'S', 'M') => num_occurrences += 1,
            ('M', 'S', 'S', 'M') => num_occurrences += 1,
            ('S', 'M', 'M', 'S') => num_occurrences += 1,
            _ => continue,
        }
    }
    num_occurrences
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 9);
    }
}
