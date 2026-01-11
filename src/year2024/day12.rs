//! Advent of Code 2024 Day 12
//!
//! Link: <https://adventofcode.com/2024/day/12>

use crate::utils::disjointset::DisjointSet;
use crate::utils::grid::{Grid, Pos};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let grid = parse_input(input_data)?;
    Ok((part1(&grid), part2(&grid)))
}

fn parse_input(input_data: &str) -> Result<Grid<char>> {
    let grid = Grid::<char>::from_lines(input_data.lines())?;
    // Expanding simplifies bounds checking for perimeter/corners
    Ok(grid.expand('.'))
}

fn part1(input: &Grid<char>) -> usize {
    let mut total_price = 0;
    let regions = find_regions(input);

    for (_, region_indices) in regions.iter() {
        let area = region_indices.len();
        let mut perimeter = 0;

        for &idx in region_indices {
            let pos = Pos(idx / input.width, idx % input.width);
            let val = input[pos];
            // Perimeter is count of cardinal neighbors that have a different value
            perimeter += input.cardinal_neighbors(pos)
                .filter(|&n| input[n] != val)
                .count();

            // Account for neighbors outside the "expand" border which are always different
            // (Standard cardinal_neighbors only returns in_bounds positions)
            let in_bounds_count = input.cardinal_neighbors(pos).count();
            perimeter += 4 - in_bounds_count;
        }
        total_price += area * perimeter;
    }
    total_price
}

fn part2(input: &Grid<char>) -> usize {
    let mut total_price = 0;
    let regions = find_regions(input);

    for (&root_idx, region_indices) in regions.iter() {
        let value = input.g[root_idx];
        let area = region_indices.len();
        let mut total_corners = 0;

        for &idx in region_indices {
            let y = idx / input.width;
            let x = idx % input.width;
            let pos = Pos(y, x);

            // Corner checks: (Vertical, Horizontal, Diagonal)
            let checks = [
                ((-1, 0), (0, -1), (-1, -1)), // Top-Left
                ((-1, 0), (0, 1), (-1, 1)),   // Top-Right
                ((1, 0), (0, -1), (1, -1)),   // Bottom-Left
                ((1, 0), (0, 1), (1, 1)),     // Bottom-Right
            ];

            for (v_off, h_off, d_off) in checks {
                let is_v_diff = is_different(input, pos, v_off, value);
                let is_h_diff = is_different(input, pos, h_off, value);
                let is_d_diff = is_different(input, pos, d_off, value);

                // 1. Outer Corner: Both adjacent cardinal sides are different
                if is_v_diff && is_h_diff {
                    total_corners += 1;
                }
                // 2. Inner Corner: Both cardinal sides are same, but the diagonal is different
                if !is_v_diff && !is_h_diff && is_d_diff {
                    total_corners += 1;
                }
            }
        }
        total_price += area * total_corners;
    }
    total_price
}

fn find_regions(input: &Grid<char>) -> HashMap<usize, HashSet<usize>> {
    let mut ds = DisjointSet::from_iter(input.g.iter().cloned());

    for y in 0..input.height {
        for x in 0..input.width {
            let pos = Pos(y, x);
            let curr_val = input[pos];
            if curr_val == '.' { continue; }

            let curr_idx = y * input.width + x;

            // Only need to check right and down to form all unions
            for offset in [(0, 1), (1, 0)] {
                if let Some(neighbor) = pos + offset
                    && input.in_bounds(neighbor) && input[neighbor] == curr_val {
                        let neighbor_idx = neighbor.0 * input.width + neighbor.1;
                        ds.union(curr_idx, neighbor_idx);
                    }
            }
        }
    }

    let mut sets = HashMap::new();
    for i in 0..ds.nodes.len() {
        if ds.nodes[i].data == '.' { continue; }
        let root = ds.find(i);
        sets.entry(root).or_insert_with(HashSet::new).insert(i);
    }
    sets
}

fn is_different(grid: &Grid<char>, pos: Pos, offset: (isize, isize), value: char) -> bool {
    if let Some(next_pos) = pos + offset
        && grid.in_bounds(next_pos) {
            return grid[next_pos] != value;
        }
    true // If out of bounds, it's "different" (the border)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 1930);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 1206);
    }
}