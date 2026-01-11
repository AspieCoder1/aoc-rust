//! Advent of Code 2024 Day 16
//!
//! Link: <https://adventofcode.com/2024/day/16>

use crate::utils::grid::{Grid, Pos};
use anyhow::Result;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let grid = Grid::<char>::from_lines(input_data.lines())?;
    Ok((part1(&grid), part2(&grid)))
}

fn part1(grid: &Grid<char>) -> usize {
    let start = grid.position(|&c| c == 'S').expect("No start found");
    let end = grid.position(|&c| c == 'E').expect("No end found");

    let mut heap: BinaryHeap<Reverse<HeapItem>> = BinaryHeap::new();
    let mut distances: HashMap<(Pos, Direction), usize> = HashMap::new();

    distances.insert((start, Direction::East), 0);
    heap.push(Reverse(HeapItem {
        cost: 0,
        pos: start,
        dir: Direction::East,
    }));

    while let Some(Reverse(HeapItem { cost, pos, dir })) = heap.pop() {
        if let Some(&best) = distances.get(&(pos, dir))
            && cost > best { continue; }

        if pos == end {
            return cost;
        }

        // 1. Forward Move
        let next_pos = pos.get_next(dir);
        if grid.in_bounds(next_pos) && grid[next_pos] != '#' {
            let next_cost = cost + 1;
            if next_cost < *distances.get(&(next_pos, dir)).unwrap_or(&usize::MAX) {
                distances.insert((next_pos, dir), next_cost);
                heap.push(Reverse(HeapItem { cost: next_cost, pos: next_pos, dir }));
            }
        }

        // 2. Turns
        for next_dir in [dir.turn_left(), dir.turn_right()] {
            let next_cost = cost + 1000;
            if next_cost < *distances.get(&(pos, next_dir)).unwrap_or(&usize::MAX) {
                distances.insert((pos, next_dir), next_cost);
                heap.push(Reverse(HeapItem { cost: next_cost, pos, dir: next_dir }));
            }
        }
    }
    0
}

fn part2(grid: &Grid<char>) -> usize {
    let start = grid.position(|&c| c == 'S').expect("No start found");
    let end = grid.position(|&c| c == 'E').expect("No end found");

    let mut heap: BinaryHeap<Reverse<HeapItem>> = BinaryHeap::new();
    let mut distances: HashMap<(Pos, Direction), usize> = HashMap::new();
    let mut predecessors: HashMap<(Pos, Direction), Vec<(Pos, Direction)>> = HashMap::new();

    distances.insert((start, Direction::East), 0);
    heap.push(Reverse(HeapItem { cost: 0, pos: start, dir: Direction::East }));

    let mut best_total_cost = usize::MAX;

    while let Some(Reverse(HeapItem { cost, pos, dir })) = heap.pop() {
        if cost > *distances.get(&(pos, dir)).unwrap_or(&usize::MAX) {
            continue;
        }

        if pos == end {
            best_total_cost = best_total_cost.min(cost);
        }

        // Define valid transitions: (NextPos, NextDir, CostIncrement)
        let moves = [
            (pos.get_next(dir), dir, 1),
            (pos, dir.turn_left(), 1000),
            (pos, dir.turn_right(), 1000),
        ];

        for (next_pos, next_dir, step_cost) in moves {
            if !grid.in_bounds(next_pos) || grid[next_pos] == '#' {
                continue;
            }

            let next_cost = cost + step_cost;
            let current_best = *distances.get(&(next_pos, next_dir)).unwrap_or(&usize::MAX);

            if next_cost < current_best {
                distances.insert((next_pos, next_dir), next_cost);
                predecessors.insert((next_pos, next_dir), vec![(pos, dir)]);
                heap.push(Reverse(HeapItem { cost: next_cost, pos: next_pos, dir: next_dir }));
            } else if next_cost == current_best {
                predecessors.entry((next_pos, next_dir)).or_default().push((pos, dir));
            }
        }
    }

    // Backtrack to find all optimal tiles
    let mut best_tiles = HashSet::new();
    let mut queue = VecDeque::new();
    let mut seen_states = HashSet::new();

    for dir in ALL_DIRECTIONS {
        if let Some(&cost) = distances.get(&(end, dir))
            && cost == best_total_cost {
                queue.push_back((end, dir));
            }
    }

    while let Some(state) = queue.pop_front() {
        if !seen_states.insert(state) { continue; }
        best_tiles.insert(state.0);

        if let Some(preds) = predecessors.get(&state) {
            for &prev in preds {
                queue.push_back(prev);
            }
        }
    }

    best_tiles.len()
}



#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction { North, South, East, West }

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North, Direction::South, Direction::East, Direction::West
];

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
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

    #[test]
    fn test_part2() {
        let grid = Grid::<char>::from_lines(EXAMPLE.lines()).unwrap();
        assert_eq!(part2(&grid), 45);

        let grid1 = Grid::<char>::from_lines(EXAMPLE2.lines()).unwrap();
        assert_eq!(part2(&grid1), 64);
    }
}