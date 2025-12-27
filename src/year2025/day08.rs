use crate::utils::disjointset::DisjointSet;
use anyhow::Error;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

pub fn main(data: &str) -> Result<(usize, usize)> {
    let input = parse_input(data)?;

    Ok((part1(&input), part2(&input)))
}

pub fn parse_input(input: &str) -> Result<Input> {
    let input = input.parse::<Input>()?;
    Ok(input)
}

pub fn part1(input: &Input) -> usize {
    let nearest_neighbours = get_closest_pairs(&input.points)
        .into_iter()
        .take(input.num_pairs)
        .collect::<Vec<_>>();
    let mut adj_table: HashMap<u16, HashSet<u16>> = HashMap::new();

    // Construct adjacency table
    for (i, j) in nearest_neighbours {
        adj_table.entry(i).or_default().insert(j);
        adj_table.entry(j).or_default().insert(i);
    }

    // We now need to create the disjoint sets
    let mut disjoint_set: DisjointSet<usize> = DisjointSet::from_iter(0..input.points.len());
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
pub fn part2(input: &Input) -> usize {
    let mut mst: Vec<(usize, usize)> = Vec::new();
    let mut union_find: DisjointSet<usize> = DisjointSet::from_iter(0..input.points.len());

    for (u, v) in get_closest_pairs(&input.points).iter() {
        if union_find.find(*u as usize) != union_find.find(*v as usize) {
            mst.push((*u as usize, *v as usize));
            union_find.union(*u as usize, *v as usize);
        }
    }

    let last_edge = mst.last().unwrap();
    input.points[last_edge.0].x * input.points[last_edge.1].x
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

#[derive(Debug, Clone)]
struct Input {
    num_pairs: usize,
    points: Vec<Point>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let num_pairs = lines.next().unwrap().split("=").last().unwrap().parse()?;
        let points: Vec<Point> = lines.map(|line| line.parse()).collect::<Result<Vec<_>>>()?;
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
