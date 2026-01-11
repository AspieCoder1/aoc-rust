//! Advent of Code 2025 Day 8
//! Link: <https://adventofcode.com/2025/day/8>

use crate::utils::disjointset::DisjointSet;
use anyhow::{Context, Error, Result};
use std::str::FromStr;

pub fn main(data: &str) -> Result<(usize, usize)> {
    let input = parse_input(data)?;
    Ok((part1(&input), part2(&input)))
}

pub fn parse_input(input: &str) -> Result<Input> {
    input.parse::<Input>()
}

pub fn part1(input: &Input) -> usize {
    let n = input.points.len();
    let mut ds = DisjointSet::new(n);

    // Construct DSU from the top N closest pairs
    let nearest_neighbours = get_closest_pairs(&input.points);
    for (i, j) in nearest_neighbours.into_iter().take(input.num_pairs) {
        ds.union(i as usize, j as usize);
    }

    // To find the top 3 largest sets:
    // Identify all roots and collect their sizes
    let mut root_sizes = Vec::new();
    for i in 0..n {
        // Only consider the element if it's the representative of its set
        if ds.find(i) == i {
            root_sizes.push(ds.size_of(i));
        }
    }

    root_sizes.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending
    root_sizes.iter().take(3).product()
}

// Kruskal's Algorithm for Minimum Spanning Tree
pub fn part2(input: &Input) -> usize {
    let n = input.points.len();
    let mut ds = DisjointSet::new(n);
    let mut last_edge = (0, 0);

    // Kruskal's: Iterate through edges sorted by weight
    for (u, v) in get_closest_pairs(&input.points) {
        let u = u as usize;
        let v = v as usize;
        if ds.union(u, v) {
            last_edge = (u, v);
            // If we've reduced the forest to a single tree, we're done
            if ds.num_sets == 1 {
                break;
            }
        }
    }

    input.points[last_edge.0].x * input.points[last_edge.1].x
}

type NearestNeighbour = (u16, u16);

fn get_closest_pairs(points: &[Point]) -> Vec<NearestNeighbour> {
    let mut distances: Vec<(usize, u16, u16)> = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate().skip(i + 1) {
            let distance = p1.euclidean_distance(*p2);
            distances.push((distance, i as u16, j as u16));
        }
    }
    // Sort by distance ascending
    distances.sort_unstable_by_key(|(dist, _, _)| *dist);

    distances.into_iter().map(|(_, i, j)| (i, j)).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Point {
    pub fn euclidean_distance(self, other: Self) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx * dx + dy * dy + dz * dz
    }
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().context("Missing x")?.trim().parse()?;
        let y = split.next().context("Missing y")?.trim().parse()?;
        let z = split.next().context("Missing z")?.trim().parse()?;
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    num_pairs: usize,
    points: Vec<Point>,
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines.next().context("Empty input")?;
        let num_pairs = first_line
            .split('=')
            .last()
            .context("Invalid header")?
            .parse()?;
        let points = lines.map(|line| line.parse()).collect::<Result<Vec<_>>>()?;
        Ok(Self { num_pairs, points })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
num_neighbours=10
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 25272);
    }
}
