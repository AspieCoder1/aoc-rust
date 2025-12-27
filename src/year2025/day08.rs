use crate::utils::disjointset::DisjointSet;
use anyhow::Error;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

const INPUT_NUM: usize = 0;

pub fn main() -> Result<(usize, usize)> {
    let (num_pairs, points) = parse_input(INPUT_NUM)?;

    Ok((part1(&points, num_pairs), part2(&points)))
}

pub fn parse_input(input_num: usize) -> Result<(usize, Vec<Point>)> {
    let points: Vec<Point> = [
        include_str!("inputs/day08.inp"),
        include_str!("test_inputs/day08.inp1"),
    ][input_num]
        .lines()
        .map(str::parse::<Point>)
        .collect::<Result<Vec<_>>>()?;
    let num_pairs: usize = if input_num == 1 { 10 } else { 1000 };

    Ok((num_pairs, points))
}

pub fn part1(input: &[Point], num_pairs: usize) -> usize {
    let nearest_neighbours = get_closest_pairs(input).into_iter().take(num_pairs);
    let mut adj_table: HashMap<u16, HashSet<u16>> = HashMap::new();

    // Construct adjacency table
    for (i, j) in nearest_neighbours {
        adj_table.entry(i).or_default().insert(j);
        adj_table.entry(j).or_default().insert(i);
    }

    // We now need to create the disjoint sets
    let mut disjoint_set: DisjointSet<usize> = DisjointSet::from_iter(0..input.len());
    for (&i, neighbours) in adj_table.iter() {
        for j in neighbours.iter().copied() {
            let (i, j) = (i as usize, j as usize);
            disjoint_set.union(i, j);
        }
    }

    // Now take the top 3 larget sets and multiply their sizes
    let mut nodes = disjoint_set.nodes;
    nodes.sort_unstable_by_key(|node| node.size);
    nodes.iter().rev().take(3).map(|node| node.size).product()
}

type NearestNeighbour = (u16, u16);

/// Gets the top N nearest neighbours
fn get_closest_pairs(points: &[Point]) -> Vec<NearestNeighbour> {
    // Doing this incredibly naively by raw looping
    // Using matric algebra is much more efficient
    let mut distances: Vec<(usize, u16, u16)> = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate().skip(i + 1) {
            let distance = p1.euclidean_distance(*p2);
            distances.push((distance, i as u16, j as u16));
        }
    }
    distances.sort_unstable_by_key(|(dist, _, _)| *dist);

    distances
        .into_iter()
        .map(|(_, i, j)| (i, j))
        .collect::<Vec<_>>()
}

// Need to just perform kruskal's algorithm here
pub fn part2(input: &[Point]) -> usize {
    let mut mst: Vec<(usize, usize)> = Vec::new();
    let mut union_find: DisjointSet<usize> = DisjointSet::from_iter(0..input.len());

    for (u, v) in get_closest_pairs(input).iter() {
        if union_find.find(*u as usize) != union_find.find(*v as usize) {
            mst.push((*u as usize, *v as usize));
            union_find.union(*u as usize, *v as usize);
        }
    }

    let last_edge = mst.last().unwrap();
    input[last_edge.0].x * input[last_edge.1].x
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
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
        let split = s.split(',').take(3).collect::<Vec<_>>();
        let [x, y, z] = split.as_slice() else {
            return Err(Error::msg(format!("Received an invalid point: {}", s)));
        };
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        let (num_pairs, points) = parse_input(1).unwrap();
        assert_eq!(part1(&points, num_pairs), 40);
    }

    #[test]
    fn test_part2() {
        let points = parse_input(1).unwrap().1;
        assert_eq!(part2(&points), 25272);
    }
}
