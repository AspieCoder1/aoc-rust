//! Advent of Code 2024 Day 6
//!
//! Link: <https://adventofcode.com/2024/day/6>

use crate::utils::grid::{Grid, Pos};
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let input = Grid::<char>::from_str(input_data)?;

    Ok((part1(&input), part2(&input)))
}

fn part1(input: &Grid<char>) -> usize {
    let start_pos = input.all_positions(|&c| c == '^').next().unwrap();
    get_visited_locations(input, start_pos).len()
}

fn get_visited_locations(input: &Grid<char>, start_pos: Pos) -> HashSet<Pos> {
    let mut visited = HashSet::new();
    let mut curr = start_pos;
    let mut dir: (isize, isize) = (-1, 0); // North

    loop {
        visited.insert(curr);

        // Try to move forward
        if let Some(next_pos) = curr + dir {
            if !input.in_bounds(next_pos) {
                break;
            }
            if input[next_pos] == '#' {
                dir = rotate_right(dir);
            } else {
                curr = next_pos;
            }
        } else {
            break; // Out of bounds (negative)
        }
    }
    visited
}

fn check_does_loop(input: &Grid<char>, start_pos: Pos) -> bool {
    // Storing (Position, Direction) to detect cycles
    let mut seen = HashSet::new();
    let mut curr = start_pos;
    let mut dir: (isize, isize) = (-1, 0);

    loop {
        if !seen.insert((curr, dir)) {
            return true; // Loop detected
        }

        if let Some(next_pos) = curr + dir {
            if !input.in_bounds(next_pos) {
                return false;
            }
            if input[next_pos] == '#' {
                dir = rotate_right(dir);
            } else {
                curr = next_pos;
            }
        } else {
            return false;
        }
    }
}

fn rotate_right(curr_direction: (isize, isize)) -> (isize, isize) {
    match curr_direction {
        (-1, 0) => (0, 1),  // N -> E
        (0, 1) => (1, 0),   // E -> S
        (1, 0) => (0, -1),  // S -> W
        (0, -1) => (-1, 0), // W -> N
        _ => unreachable!(),
    }
}

fn part2(input: &Grid<char>) -> usize {
    let start_pos = input.all_positions(|&c| c == '^').next().unwrap();

    // Optimization: Only test positions on the original path
    let original_path = get_visited_locations(input, start_pos);

    original_path
        .into_par_iter()
        .filter(|&pos| pos != start_pos)
        .filter(|&pos| {
            let mut new_grid = input.clone();
            new_grid[pos] = '#';
            check_does_loop(&new_grid, start_pos)
        })
        .count()
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

    #[test]
    fn test_part2() {
        let input = Grid::<char>::from_str(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 6);
    }
}