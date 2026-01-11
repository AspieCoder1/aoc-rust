//! Advent of Code 2024 Day 12
//!
//! Link: <https://adventofcode.com/2024/day/12>

use crate::utils::grid::Grid;
use crate::utils::point::Point;
use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let grid = Grid::<char>::from_str(input_data)?;

    let mut part1_total = 0;
    let mut part2_total = 0;

    // Track which tiles we've already assigned to a region
    let mut global_visited = HashSet::new();

    for p in grid.all_positions(|_| true) {
        if global_visited.contains(&p) {
            continue;
        }

        // Identify a new region using flood fill
        let region_char = grid[p];
        let mut region_points = HashSet::new();
        let mut stack = vec![p];
        global_visited.insert(p);

        while let Some(curr) = stack.pop() {
            region_points.insert(curr);
            for next in grid.cardinal_neighbors(curr) {
                if grid[next] == region_char && !global_visited.contains(&next) {
                    global_visited.insert(next);
                    stack.push(next);
                }
            }
        }

        // Calculate metrics for this specific region
        let area = region_points.len();
        let perimeter = calculate_perimeter(&grid, &region_points);
        let corners = calculate_corners(&grid, &region_points);

        part1_total += area * perimeter;
        part2_total += area * corners;
    }

    Ok((part1_total, part2_total))
}

fn calculate_perimeter(grid: &Grid<char>, region: &HashSet<Point>) -> usize {
    let mut perimeter = 0;
    for &p in region {
        let mut neighbors_in_region = 0;
        for next in grid.cardinal_neighbors(p) {
            if region.contains(&next) {
                neighbors_in_region += 1;
            }
        }
        perimeter += 4 - neighbors_in_region;
    }
    perimeter
}

fn calculate_corners(grid: &Grid<char>, region: &HashSet<Point>) -> usize {
    let mut corners = 0;
    let region_char = grid[*region.iter().next().unwrap()];

    for &p in region {
        // We check 4 quadrants around the point
        // Quadrant directions: (Vertical, Horizontal, Diagonal)
        let quadrants = [
            (Point::UP, Point::LEFT, Point::UP + Point::LEFT),    // Top-Left
            (Point::UP, Point::RIGHT, Point::UP + Point::RIGHT), // Top-Right
            (Point::DOWN, Point::LEFT, Point::DOWN + Point::LEFT), // Bottom-Left
            (Point::DOWN, Point::RIGHT, Point::DOWN + Point::RIGHT), // Bottom-Right
        ];

        for (v, h, d) in quadrants {
            let v_diff = !grid.in_bounds(p + v) || grid[p + v] != region_char;
            let h_diff = !grid.in_bounds(p + h) || grid[p + h] != region_char;
            let d_diff = !grid.in_bounds(p + d) || grid[p + d] != region_char;

            // Outer Corner: Both cardinal directions are different
            if v_diff && h_diff {
                corners += 1;
            }
            // Inner Corner: Both cardinal are same, but diagonal is different
            if !v_diff && !h_diff && d_diff {
                corners += 1;
            }
        }
    }
    corners
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_day12() {
        let (p1, p2) = main(EXAMPLE).unwrap();
        assert_eq!(p1, 1930);
        assert_eq!(p2, 1206);
    }
}