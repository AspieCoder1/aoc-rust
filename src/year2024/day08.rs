//! Advent of Code 2024 Day 8
//!
//! Link: <https://adventofcode.com/2024/day/8>

use crate::utils::grid::Grid;
use crate::utils::point::Point;
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let grid = Grid::<char>::from_str(input_data)?;

    Ok((part1(&grid), part2(&grid)))
}

fn part1(grid: &Grid<char>) -> usize {
    let antenna_map = get_antenna_positions(grid);
    let mut antinodes: HashSet<Point> = HashSet::new();

    for positions in antenna_map.values() {
        for (&a, &b) in positions.iter().tuple_combinations() {
            // Point subtraction gives the vector between them
            let delta = a - b;

            // Antinode 1 is 'a' plus the vector
            let p1 = a + delta;
            if grid.in_bounds(p1) {
                antinodes.insert(p1);
            }

            // Antinode 2 is 'b' minus the vector
            let p2 = b - delta;
            if grid.in_bounds(p2) {
                antinodes.insert(p2);
            }
        }
    }
    antinodes.len()
}

fn part2(grid: &Grid<char>) -> usize {
    let antenna_map = get_antenna_positions(grid);
    let mut antinodes: HashSet<Point> = HashSet::new();

    for positions in antenna_map.values() {
        for (&a, &b) in positions.iter().tuple_combinations() {
            let delta = a - b;

            // Resonant harmonics: extend from 'a' in the positive direction
            let mut curr = a;
            while grid.in_bounds(curr) {
                antinodes.insert(curr);
                curr += delta;
            }

            // Resonant harmonics: extend from 'b' in the negative direction
            let mut curr = b;
            while grid.in_bounds(curr) {
                antinodes.insert(curr);
                curr -= delta;
            }
        }
    }
    antinodes.len()
}

/// Groups antenna positions by their frequency (character).
fn get_antenna_positions(grid: &Grid<char>) -> HashMap<char, Vec<Point>> {
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();

    // Use our grid utility to find all antennas (non-dots)
    for pos in grid.all_positions(|&c| c != '.') {
        map.entry(grid[pos]).or_default().push(pos);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        let grid = Grid::<char>::from_str(EXAMPLE).unwrap();
        assert_eq!(part1(&grid), 14);
    }

    #[test]
    fn test_part2() {
        let grid = Grid::<char>::from_str(EXAMPLE).unwrap();
        assert_eq!(part2(&grid), 34);
    }
}
