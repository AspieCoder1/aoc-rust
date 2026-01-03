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

    Ok(grid.expand('.'))
}

fn part1(input: &Grid<char>) -> usize {
    let mut total_price = 0;
    let regions = find_regions(input);

    for (_, region) in regions.iter() {
        let area = region.len();
        let perimeter = region
            .iter()
            .map(|p| {
                let pos = Pos(p / input.width, p % input.width);
                input
                    .cardinal_neighbors(pos)
                    .filter(|&n| input[n] != input[pos])
                    .count()
            })
            .sum::<usize>();
        total_price += area * perimeter
    }
    total_price
}

fn part2(input: &Grid<char>) -> usize {
    let mut total_price = 0;
    let regions = find_regions(input);
    for (&region_parent, region) in regions.iter() {
        let value = input.g[region_parent];
        let area = region.len();
        // Number of sides is equal to the number of corners
        let num_corners = region
            .iter()
            .map(|&p| {
                let (y, x) = (p / input.width, p % input.width);
                let mut corners = 0;

                // We check the 4 potential corner directions around a cell:
                // (Vertical, Horizontal, Diagonal)
                let checks = [
                    ((-1, 0), (0, -1), (-1, -1)), // Top-Left
                    ((-1, 0), (0, 1), (-1, 1)),   // Top-Right
                    ((1, 0), (0, -1), (1, -1)),   // Bottom-Left
                    ((1, 0), (0, 1), (1, 1)),     // Bottom-Right
                ];

                for (v, h, d) in checks {
                    let is_v_diff = is_different(input, y, x, v, value);
                    let is_h_diff = is_different(input, y, x, h, value);
                    let is_d_diff = is_different(input, y, x, d, value);

                    // 1. Outer Corner: Both cardinal neighbours are different
                    if is_v_diff && is_h_diff {
                        corners += 1;
                    }
                    // 2. Inner Corner: Both cardinal neighbours are the same,
                    //    but the diagonal between them is different
                    if !is_v_diff && !is_h_diff && is_d_diff {
                        corners += 1;
                    }
                }
                corners
            })
            .sum::<usize>();
        total_price += area * num_corners
    }
    total_price
}

/// Implementation of the Hoshen–Kopelman algorithm to perform connected component detection.
fn find_regions(input: &Grid<char>) -> HashMap<usize, HashSet<usize>> {
    let mut regions = DisjointSet::from_iter(input.g.iter().cloned());
    for x in 1..input.width - 1 {
        for y in 1..input.height - 1 {
            let curr = input[(y, x)];
            let left = input[(y, x - 1)];
            let above = input[(y - 1, x)];
            let curr_idx = y * input.width + x;
            let left_idx = y * input.width + x - 1;
            let above_idx = (y - 1) * input.width + x;
            if curr != left && curr != above {
                // No neighbours, so this is a new region.
                continue;
            } else if curr == left && curr != above {
                // One neighbour to the left
                regions.union(curr_idx, left_idx);
            } else if curr != left && curr == above {
                // One neighbour above
                regions.union(curr_idx, above_idx);
            } else {
                // Neighbour left and above
                regions.union(left_idx, above_idx);
                regions.union(curr_idx, left_idx);
            }
        }
    }

    // Get map of connected components and their indexes
    let mut sets = HashMap::new();

    for i in 0..regions.nodes.len() {
        if regions.nodes[i].data == '.' {
            continue;
        }

        // Find the root of the current node
        let root = regions.find(i);

        // Get the data (requires Clone) and push to the corresponding group
        sets.entry(root).or_insert_with(HashSet::new).insert(i);
    }

    sets
}

fn is_different(grid: &Grid<char>, y: usize, x: usize, offset: (i32, i32), value: char) -> bool {
    let ny = y as i32 + offset.0;
    let nx = x as i32 + offset.1;

    if ny < 0 || ny >= grid.height as i32 || nx < 0 || nx >= grid.width as i32 {
        return true;
    }
    grid.g[ny as usize * grid.width + nx as usize] != value
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

    const SMALL_EXAMPLE: &str = "\
AAAA
BBCD
BBCC
EEEC";

    #[test]
    fn test_find_regions() {
        let input = parse_input(SMALL_EXAMPLE).unwrap();
        let regions = find_regions(&input);
        let expected_regions = HashMap::from([
            (16, HashSet::from([16])),
            (19, HashSet::from([13, 14, 19, 20])),
            (21, HashSet::from([15, 21, 22, 28])),
            (8, HashSet::from([7, 8, 9, 10])),
            (26, HashSet::from([25, 26, 27])),
        ]);
        assert_eq!(regions, expected_regions);
    }

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
