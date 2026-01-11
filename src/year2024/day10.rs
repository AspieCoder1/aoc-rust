//! Advent of Code 2024 Day 10: Hoof It
//!
//! This solution uses a Breadth-First Search (BFS) for Part 1 to find unique reachable peaks,
//! and a Memoized Depth-First Search (DFS) for Part 2 to count all distinct hiking trails.

use crate::utils::grid::Grid;
use crate::utils::point::Point;
use anyhow::Result;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    // 1. Initial parse into Grid<char>
    let char_grid = Grid::<char>::from_str(input_data)?;

    // 2. Optimization: Convert to Grid<u8> immediately to avoid char-to-digit
    // conversions in the performance-critical path.
    let grid = Grid::from_vals(
        char_grid
            .g
            .iter()
            .map(|&c| c.to_digit(10).unwrap_or(99) as u8)
            .collect(),
        char_grid.width,
        char_grid.height,
    );

    Ok((part1(&grid), part2(&grid)))
}

/// Part 1: Score is the number of '9' height peaks reachable from each trailhead.
fn part1(grid: &Grid<u8>) -> usize {
    grid.all_positions(|&h| h == 0)
        .par_bridge()
        .map(|start| {
            let mut queue = VecDeque::from([start]);
            let mut visited = HashSet::from([start]);
            let mut reachable_peaks = 0;

            while let Some(curr) = queue.pop_front() {
                if grid[curr] == 9 {
                    reachable_peaks += 1;
                    continue;
                }

                for next in grid.cardinal_neighbors(curr) {
                    // Gradual incline: next step must be exactly current + 1
                    if grid[next] == grid[curr] + 1 && visited.insert(next) {
                        queue.push_back(next);
                    }
                }
            }
            reachable_peaks
        })
        .sum()
}

/// Part 2: Rating is the number of distinct hiking trails (unique paths) to any '9'.
fn part2(grid: &Grid<u8>) -> usize {
    grid.all_positions(|&h| h == 0)
        .par_bridge()
        .map(|start| {
            let mut memo = HashMap::new();
            count_trails_memo(grid, start, &mut memo)
        })
        .sum()
}

/// A recursive DFS with memoization to count all unique paths from `curr` to any peak.
fn count_trails_memo(grid: &Grid<u8>, curr: Point, memo: &mut HashMap<Point, usize>) -> usize {
    // Base case: we reached a peak
    if grid[curr] == 9 {
        return 1;
    }

    // Check cache to avoid re-calculating sub-paths
    if let Some(&count) = memo.get(&curr) {
        return count;
    }

    let mut total_paths = 0;
    let target_height = grid[curr] + 1;

    for next in grid.cardinal_neighbors(curr) {
        if grid[next] == target_height {
            total_paths += count_trails_memo(grid, next, memo);
        }
    }

    // Store in cache before returning
    memo.insert(curr, total_paths);
    total_paths
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_day10_full() {
        let char_grid = Grid::<char>::from_str(EXAMPLE).unwrap();
        let grid = Grid::from_vals(
            char_grid
                .g
                .iter()
                .map(|&c| c.to_digit(10).unwrap_or(99) as u8)
                .collect(),
            char_grid.width,
            char_grid.height,
        );
        assert_eq!(part1(&grid), 36);
        assert_eq!(part2(&grid), 81);
    }
}
