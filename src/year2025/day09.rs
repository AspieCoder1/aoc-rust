//! Advent of Code 2024 Day 9 (Alternative)
//!
//! Solving for max area rectangle inside a non-convex polygon using
//! Coordinate Compression and 2D Prefix Sums.

use crate::utils::grid::Grid;
use crate::utils::point::Point;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

pub fn main(data: &str) -> Result<(usize, usize)> {
    let input = parse_input(data)?;
    Ok((part1(&input), part2(&input)))
}

type Tile = Point;

pub fn parse_input(input: &str) -> Result<Vec<Tile>> {
    Ok(input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').expect("Invalid format: x,y");
            Point::new(
                x.trim().parse::<i32>().expect("X parse error"),
                y.trim().parse::<i32>().expect("Y parse error"),
            )
        })
        .collect())
}

/// Part 1: Simplistic max bounding box between any two vertices
pub fn part1(input: &[Tile]) -> usize {
    let mut max_area = 0;
    for (i, p1) in input.iter().enumerate() {
        for p2 in input.iter().skip(i + 1) {
            let width = (p1.x.abs_diff(p2.x) + 1) as usize;
            let height = (p1.y.abs_diff(p2.y) + 1) as usize;
            max_area = max_area.max(width * height);
        }
    }
    max_area
}

struct Compressed {
    coords: Vec<i32>,
    map: HashMap<i32, usize>,
}

impl Compressed {
    fn new(points: &[i32]) -> Self {
        let mut coords = Vec::new();
        for &p in points {
            // Include neighbors to ensure we represent the space between points
            coords.push(p);
            coords.push(p + 1);
        }
        coords.sort_unstable();
        coords.dedup();
        let map = coords.iter().enumerate().map(|(i, &p)| (p, i)).collect();
        Self { coords, map }
    }

    fn get(&self, point: i32) -> usize {
        *self.map.get(&point).expect("Coord missing")
    }
}

pub fn part2(input: &[Tile]) -> usize {
    if input.len() < 3 {
        return 0;
    }

    let x_coords = input.iter().map(|p| p.x).collect::<Vec<_>>();
    let y_coords = input.iter().map(|p| p.y).collect::<Vec<_>>();

    let xcomp = Compressed::new(&x_coords);
    let ycomp = Compressed::new(&y_coords);

    let mut g = Grid::<bool>::new(false, xcomp.coords.len(), ycomp.coords.len());

    // 1. Draw boundary lines using Point math
    for (a, b) in input
        .iter()
        .chain(std::iter::once(&input[0]))
        .tuple_windows()
    {
        let (cx1, cy1) = (xcomp.get(a.x), ycomp.get(a.y));
        let (cx2, cy2) = (xcomp.get(b.x), ycomp.get(b.y));

        for y in cy1.min(cy2)..=cy1.max(cy2) {
            for x in cx1.min(cx2)..=cx1.max(cx2) {
                g[Point::new(x as i32, y as i32)] = true;
            }
        }
    }

    // 2. Interior Detection using Flood Fill
    // We look for a point guaranteed to be inside. A simple heuristic is to check
    // points near the boundary.
    let mut fill_start = None;
    'outer: for y in 0..g.height {
        for x in 0..g.width {
            let p = Point::new(x as i32, y as i32);
            // Ray casting is expensive, so we only do it until we find one seed point
            if !g[p] && g.is_inside_polygon(p, |&b| b) {
                fill_start = Some(p);
                break 'outer;
            }
        }
    }

    if let Some(start_node) = fill_start {
        g.flood_fill(start_node, true, |&is_wall| is_wall);
    }

    // 3. 2D Prefix Sum on the Compressed Grid
    let mut psum = Grid::<usize>::new(0, g.width, g.height);
    for y in 0..g.height {
        for x in 0..g.width {
            let p = Point::new(x as i32, y as i32);
            let val = g[p] as usize;
            let left = if x > 0 {
                psum[Point::new(x as i32 - 1, y as i32)]
            } else {
                0
            };
            let up = if y > 0 {
                psum[Point::new(x as i32, y as i32 - 1)]
            } else {
                0
            };
            let diag = if x > 0 && y > 0 {
                psum[Point::new(x as i32 - 1, y as i32 - 1)]
            } else {
                0
            };
            psum[p] = val + left + up - diag;
        }
    }

    // 4. Find max area rectangle that is fully "filled" (all true in g)
    let mut max_area = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let (p1, p2) = (input[i], input[j]);
            let (x_min, x_max) = (p1.x.min(p2.x), p1.x.max(p2.x));
            let (y_min, y_max) = (p1.y.min(p2.y), p1.y.max(p2.y));

            let (cx1, cx2) = (xcomp.get(x_min), xcomp.get(x_max));
            let (cy1, cy2) = (ycomp.get(y_min), ycomp.get(y_max));

            // Query the 2D prefix sum for the compressed region
            let rect_sum = query_psum(&psum, cx1, cy1, cx2, cy2);
            let expected_area = (cx2 - cx1 + 1) * (cy2 - cy1 + 1);

            if rect_sum == expected_area {
                let actual_area = (x_max - x_min + 1) * (y_max - y_min + 1);
                max_area = max_area.max(actual_area as usize);
            }
        }
    }

    max_area
}

fn query_psum(psum: &Grid<usize>, x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let total = psum[Point::new(x2 as i32, y2 as i32)];
    let left = if x1 > 0 {
        psum[Point::new(x1 as i32 - 1, y2 as i32)]
    } else {
        0
    };
    let up = if y1 > 0 {
        psum[Point::new(x2 as i32, y1 as i32 - 1)]
    } else {
        0
    };
    let diag = if x1 > 0 && y1 > 0 {
        psum[Point::new(x1 as i32 - 1, y1 as i32 - 1)]
    } else {
        0
    };
    total + diag - left - up
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 50); // Corrected bounding box for 11,1 and 2,5
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 24);
    }
}
