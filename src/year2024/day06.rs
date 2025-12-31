//! Advent of Code 2024 Day 6
//!
//! Link: <https://adventofcode.com/2024/day/6>

use crate::utils::grid::Grid;
use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let input = Grid::<char>::from_str(input_data)?;

    Ok((part1(&input), part2(&input)))
}

fn part1(input: &Grid<char>) -> usize {
    let start_pos = input.all_positions(|&c| c == '^').next().unwrap();
    let mut visited_position = HashSet::from([(start_pos.0, start_pos.1)]);
    let mut curr_i = start_pos.0 as isize;
    let mut curr_j = start_pos.1 as isize;
    let mut direction: (isize, isize) = (-1, 0);

    while curr_i >= 0
        && curr_i < input.height as isize
        && curr_j >= 0
        && curr_j < input.width as isize
    {
        visited_position.insert((curr_i as usize, curr_j as usize));
        let new_i = curr_i + direction.0;
        let new_j = curr_j + direction.1;

        if new_i < 0 || new_i >= input.height as isize || new_j < 0 || new_j >= input.width as isize
        {
            break;
        }

        if input[(new_i as usize, new_j as usize)] == '#' {
            // Perform the right turn
            let new_direction = match direction {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!("Invalid direction"),
            };
            direction = new_direction;
        }
        curr_i += direction.0;
        curr_j += direction.1;
    }

    visited_position.len()
}

fn part2(_input: &Grid<char>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let input = Grid::<char>::from_str(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 41);
    }
}
