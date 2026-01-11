//! Advent of Code 2024 Day 6
//!
//! Link: <https://adventofcode.com/2024/day/6>

use crate::utils::grid::Grid;
use crate::utils::point::Point;
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let input = Grid::<char>::from_str(input_data)?;

    Ok((part1(&input), part2(&input)))
}

fn part1(input: &Grid<char>) -> usize {
    let start_pos = input.find_pos(|&c| c == '^').expect("Guard not found");
    get_visited_locations(input, start_pos).len()
}

/// Simulates the guard's path and returns all visited unique positions.
fn get_visited_locations(input: &Grid<char>, start_pos: Point) -> HashSet<Point> {
    let mut visited = HashSet::new();
    let mut curr = start_pos;
    let mut dir = Point::UP; // North

    loop {
        visited.insert(curr);
        let next = curr + dir;

        if !input.in_bounds(next) {
            break;
        }

        if input[next] == '#' {
            dir = dir.rotate_right_90();
        } else {
            curr = next;
        }
    }
    visited
}

/// Checks if placing an obstacle at `extra_obstacle` causes the guard to loop.
fn check_does_loop(input: &Grid<char>, start_pos: Point, extra_obstacle: Point) -> bool {
    // We use a simple 2D array or a BitSet for even more speed,
    // but HashSet of (Point, Direction) is robust.
    let mut seen = HashSet::new();
    let mut curr = start_pos;
    let mut dir = Point::UP;

    loop {
        // If we've been at this position facing this way before, it's a loop
        if !seen.insert((curr, dir)) {
            return true;
        }

        let next = curr + dir;

        if !input.in_bounds(next) {
            return false;
        }

        // Check original obstacles OR the new one we're testing
        if input[next] == '#' || next == extra_obstacle {
            dir = dir.rotate_right_90();
        } else {
            curr = next;
        }
    }
}

fn part2(input: &Grid<char>) -> usize {
    let start_pos = input.find_pos(|&c| c == '^').expect("Guard not found");

    // Optimization: Only test positions that the guard actually visits in Part 1.
    // An obstacle elsewhere cannot possibly affect the path.
    let original_path = get_visited_locations(input, start_pos);

    original_path
        .into_par_iter()
        .filter(|&pos| pos != start_pos)
        .filter(|&pos| check_does_loop(input, start_pos, pos))
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
