//! Advent of Code 2025 Day 9
//!
//! Final Source with robust Ray-Casting Fill and 2D Prefix Sums.

use crate::utils::grid::{Grid, Pos};
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

pub fn main(data: &str) -> Result<(usize, usize)> {
    let input = parse_input(data)?;
    Ok((part1(&input), part2(&input)))
}

type Tile = (usize, usize);

pub fn parse_input(input: &str) -> Result<Vec<Tile>> {
    Ok(input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').expect("Invalid format: x,y");
            (
                x.trim().parse::<usize>().expect("X parse error"),
                y.trim().parse::<usize>().expect("Y parse error"),
            )
        })
        .collect())
}

pub fn part1(input: &[Tile]) -> usize {
    let mut acc = 0;
    for (i, &(x0, y0)) in input.iter().enumerate() {
        for &(x1, y1) in input.iter().skip(i + 1) {
            let length = x0.abs_diff(x1) + 1;
            let height = y0.abs_diff(y1) + 1;
            acc = acc.max(length * height);
        }
    }
    acc
}

struct Compressed {
    coords: Vec<usize>,
    map: HashMap<usize, usize>,
}

impl Compressed {
    fn new(points: &[usize]) -> Self {
        let mut coords = Vec::new();
        for &p in points {
            if p > 0 { coords.push(p - 1); }
            coords.push(p);
            coords.push(p + 1);
        }
        coords.sort_unstable();
        coords.dedup();

        let map = coords.iter().enumerate().map(|(i, &p)| (p, i)).collect();
        Self { coords, map }
    }

    fn get(&self, point: usize) -> usize {
        *self.map.get(&point).expect("Coord missing")
    }
}

pub fn part2(input: &[Tile]) -> usize {
    if input.len() < 3 { return 0; }

    let x_coords = input.iter().map(|p| p.0).collect::<Vec<_>>();
    let y_coords = input.iter().map(|p| p.1).collect::<Vec<_>>();

    let xcomp = Compressed::new(&x_coords);
    let ycomp = Compressed::new(&y_coords);

    let mut g = Grid::<bool>::new(false, xcomp.coords.len(), ycomp.coords.len());

    // 1. Draw boundary lines
    for (&(ax, ay), &(bx, by)) in input.iter().chain(std::iter::once(&input[0])).tuple_windows() {
        let (cx1, cy1) = (xcomp.get(ax), ycomp.get(ay));
        let (cx2, cy2) = (xcomp.get(bx), ycomp.get(by));

        for i in cy1.min(cy2)..=cy1.max(cy2) {
            for j in cx1.min(cx2)..=cx1.max(cx2) {
                g[Pos(i, j)] = true;
            }
        }
    }

    // 2. Interior Detection & Fill (Borrow-safe)
    let fill_start = {
        let mut found = None;
        // Search for a point that is NOT a boundary but IS inside the polygon
        'search: for r in 0..g.height {
            for c in 0..g.width {
                let p = Pos(r, c);
                if !g[p] && g.is_inside_polygon(p, |&b| b) {
                    found = Some(p);
                    break 'search;
                }
            }
        }
        found
    };

    if let Some(pos) = fill_start {
        g.flood_fill(pos, true, |&b| b);
    }

    // 3. Functional 2D Prefix Sum
    let mut psum = Grid::<usize>::new(0, g.width, g.height);
    for i in 0..g.height {
        let mut row_acc = 0;
        for j in 0..g.width {
            row_acc += g[Pos(i, j)] as usize;
            psum[Pos(i, j)] = row_acc;
        }
    }
    for j in 0..g.width {
        let mut col_acc = 0;
        for i in 0..g.height {
            col_acc += psum[Pos(i, j)];
            psum[Pos(i, j)] = col_acc;
        }
    }

    // 4. Find max area rectangle
    let mut max_area = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let (p1, p2) = (input[i], input[j]);
            let (x_min, x_max) = (p1.0.min(p2.0), p1.0.max(p2.0));
            let (y_min, y_max) = (p1.1.min(p2.1), p1.1.max(p2.1));

            let (cx1, cx2) = (xcomp.get(x_min), xcomp.get(x_max));
            let (cy1, cy2) = (ycomp.get(y_min), ycomp.get(y_max));

            let corner = psum[Pos(cy2, cx2)];
            let up = if cy1 > 0 { psum[Pos(cy1 - 1, cx2)] } else { 0 };
            let left = if cx1 > 0 { psum[Pos(cy2, cx1 - 1)] } else { 0 };
            let diag = if cy1 > 0 && cx1 > 0 { psum[Pos(cy1 - 1, cx1 - 1)] } else { 0 };

            let filled_cells = (corner + diag).saturating_sub(up + left);
            let expected_cells = (cy2 - cy1 + 1) * (cx2 - cx1 + 1);

            if filled_cells == expected_cells {
                max_area = max_area.max((x_max - x_min + 1) * (y_max - y_min + 1));
            }
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 50);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 24);
    }
}