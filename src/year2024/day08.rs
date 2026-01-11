//! Advent of Code 2024 Day 8
//!
//! Link: <https://adventofcode.com/2024/day/8>

use crate::utils::grid::{Grid, Pos};
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let grid = Grid::<char>::from_str(input_data)?;

    Ok((part1(&grid), part2(&grid)))
}

fn part1(grid: &Grid<char>) -> usize {
    let antenna_positions = get_antenna_positions(grid);
    let mut antinodes: HashSet<Pos> = HashSet::new();

    for (_, positions) in antenna_positions.iter() {
        for (&a, &b) in positions.iter().tuple_combinations() {
            // Calculate delta using isize
            let di = a.0 as isize - b.0 as isize;
            let dj = a.1 as isize - b.1 as isize;

            // Antinode A is a + delta, Antinode B is b - delta
            if let Some(p_a) = a + (di, dj)
                && grid.in_bounds(p_a) {
                    antinodes.insert(p_a);
                }

            if let Some(p_b) = b + (-di, -dj)
                && grid.in_bounds(p_b) {
                    antinodes.insert(p_b);
                }
        }
    }
    antinodes.len()
}

fn part2(grid: &Grid<char>) -> usize {
    let antenna_positions = get_antenna_positions(grid);
    let mut antinodes: HashSet<Pos> = HashSet::new();

    for (_, positions) in antenna_positions.iter() {
        for (&a, &b) in positions.iter().tuple_combinations() {
            let di = a.0 as isize - b.0 as isize;
            let dj = a.1 as isize - b.1 as isize;

            // Resonant harmonics in direction 1 (starting from a)
            let mut curr = Some(a);
            while let Some(p) = curr {
                if !grid.in_bounds(p) { break; }
                antinodes.insert(p);
                curr = p + (di, dj);
            }

            // Resonant harmonics in direction 2 (starting from b)
            let mut curr = Some(b);
            while let Some(p) = curr {
                if !grid.in_bounds(p) { break; }
                antinodes.insert(p);
                curr = p + (-di, -dj);
            }
        }
    }
    antinodes.len()
}

fn get_antenna_positions(grid: &Grid<char>) -> HashMap<char, Vec<Pos>> {
    let mut antenna_positions: HashMap<char, Vec<Pos>> = HashMap::new();
    // Use the grid utility to find all non-empty tiles
    for r in 0..grid.height {
        for c in 0..grid.width {
            let pos = Pos(r, c);
            let cell = grid[pos];
            if cell != '.' {
                antenna_positions.entry(cell).or_default().push(pos);
            }
        }
    }
    antenna_positions
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
        let input = Grid::<char>::from_str(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 14);
    }

    #[test]
    fn test_part2() {
        let input = Grid::<char>::from_str(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 34);
    }
}