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
    let mut antenna_positions: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (Pos(y, x), &cell) in grid.enumerate_by_pos().filter(|&(_, cell)| *cell != '.') {
        antenna_positions
            .entry(cell)
            .or_default()
            .push((y as isize, x as isize))
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_, positions) in antenna_positions.iter() {
        for (a, b) in positions.iter().tuple_combinations() {
            let (di, dj) = (a.0 - b.0, a.1 - b.1);
            let antinode_a = (a.0 + di, a.1 + dj);
            let antinode_b = (b.0 - di, b.1 - dj);

            if antinode_a.0 >= 0
                && antinode_a.0 < grid.height as isize
                && antinode_a.1 >= 0
                && antinode_a.1 < grid.width as isize
            {
                antinodes.insert(antinode_a);
            }

            if antinode_b.0 >= 0
                && antinode_b.0 < grid.height as isize
                && antinode_b.1 >= 0
                && antinode_b.1 < grid.width as isize
            {
                antinodes.insert(antinode_b);
            }
        }
    }
    antinodes.len()
}

fn part2(_grid: &Grid<char>) -> usize {
    0
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
}
