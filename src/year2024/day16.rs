//! Advent of Code 2024 Day 16
//!
//! Link: <https://adventofcode.com/2024/day/16>

use crate::utils::grid::{Grid, Pos};
use anyhow::Result;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let grid = Grid::<char>::from_lines(input_data.lines())?;
    Ok((part1(&grid), part2()))
}

fn part1(grid: &Grid<char>) -> usize {
    let start = grid.position(|&c| c == 'S').expect("No start found");
    let end = grid.position(|&c| c == 'E').expect("No end found");

    // We wrap in Reverse for a Min-Heap
    let mut heap: BinaryHeap<Reverse<HeapItem>> = BinaryHeap::new();

    // Track the best (lowest) cost to reach a specific position facing a specific direction
    let mut distances: HashMap<(Pos, Direction), usize> = HashMap::new();

    // Starting state: At 'S', facing East, with 0 cost
    distances.insert((start, Direction::East), 0);
    heap.push(Reverse(HeapItem {
        cost: 0,
        pos: start,
        dir: Direction::East,
    }));

    while let Some(Reverse(HeapItem { cost, pos, dir })) = heap.pop() {
        // If we've already found a cheaper way to this exact state, skip processing
        if let Some(&best_cost) = distances.get(&(pos, dir)) {
            if cost > best_cost {
                continue;
            }
        }

        // Dijkstra's property: The first time we pop the 'end' position,
        // it is guaranteed to be the minimum cost.
        if pos == end {
            return cost;
        }

        // Explore neighbours: Moving in all 4 directions
        for &next_dir in &ALL_DIRECTIONS {
            let next_pos = pos.get_next(next_dir);

            // 1. Check bounds and walls
            if grid[next_pos] == '#' {
                continue;
            }

            // 2. Calculate the cost to move to the next tile
            // move_cost = 1 (step) + rotation penalty
            let move_cost = 1 + dir.cost_changed(next_dir);
            let next_total_cost = cost + move_cost;

            // 3. Relaxation: Only push to the heap if this path is better than any found so far
            let current_best = distances
                .get(&(next_pos, next_dir))
                .copied()
                .unwrap_or(usize::MAX);

            if next_total_cost < current_best {
                distances.insert((next_pos, next_dir), next_total_cost);
                heap.push(Reverse(HeapItem {
                    cost: next_total_cost,
                    pos: next_pos,
                    dir: next_dir,
                }));
            }
        }
    }

    0 // Should not be reached if there is a path
}

fn part2() -> usize {
    0
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

impl Direction {
    fn is_rotated(self, other: Direction) -> bool {
        match self {
            Direction::North | Direction::South => {
                [Direction::East, Direction::West].contains(&other)
            }
            Direction::East | Direction::West => {
                [Direction::North, Direction::South].contains(&other)
            }
        }
    }
    fn cost_changed(self, other: Direction) -> usize {
        if self == other {
            0
        } else if self.is_rotated(other) {
            1000
        } else {
            2000
        }
    }
}

impl Pos {
    fn get_next(self, direction: Direction) -> Pos {
        let Pos(i, j) = self;
        match direction {
            Direction::North => Pos(i - 1, j),
            Direction::South => Pos(i + 1, j),
            Direction::East => Pos(i, j + 1),
            Direction::West => Pos(i, j - 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct HeapItem {
    cost: usize,
    pos: Pos,
    dir: Direction,
}

// Implement Ord so that it sorts by cost ascending
impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part1() {
        let grid = Grid::<char>::from_lines(EXAMPLE.lines()).unwrap();
        assert_eq!(part1(&grid), 7036);

        let grid1 = Grid::<char>::from_lines(EXAMPLE2.lines()).unwrap();
        assert_eq!(part1(&grid1), 11048);
    }
}
