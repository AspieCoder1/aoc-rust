//! Advent of Code 2024 Day 16
//!
//! Link: <https://adventofcode.com/2024/day/16>

use crate::utils::grid::Grid;
use crate::utils::point::Point;
use anyhow::Result;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let grid = Grid::<char>::from_str(input_data)?;
    Ok(solve(&grid))
}

fn solve(grid: &Grid<char>) -> (usize, usize) {
    let start = grid.find_pos(|&c| c == 'S').expect("No start");
    let end = grid.find_pos(|&c| c == 'E').expect("No end");

    let mut heap = BinaryHeap::new();
    // Distance map stores: (Position, Direction) -> Min Cost
    let mut distances: HashMap<(Point, Point), usize> = HashMap::new();
    // Predecessors stores: (Position, Direction) -> List of optimal previous states
    let mut predecessors: HashMap<(Point, Point), Vec<(Point, Point)>> = HashMap::new();

    // Start state: Position S, facing East
    let start_state = (start, Point::RIGHT);
    distances.insert(start_state, 0);
    heap.push(State {
        cost: 0,
        pos: start,
        dir: Point::RIGHT,
    });

    let mut best_total_cost = usize::MAX;

    while let Some(State { cost, pos, dir }) = heap.pop() {
        if cost > *distances.get(&(pos, dir)).unwrap_or(&usize::MAX) {
            continue;
        }

        if pos == end {
            best_total_cost = best_total_cost.min(cost);
        }

        // Possible next states: Forward, Turn Left, Turn Right
        let moves = [
            (pos + dir, dir, 1),                // Move forward
            (pos, dir.rotate_left_90(), 1000),  // Turn CCW
            (pos, dir.rotate_right_90(), 1000), // Turn CW
        ];

        for (next_pos, next_dir, step_cost) in moves {
            if !grid.in_bounds(next_pos) || grid[next_pos] == '#' {
                continue;
            }

            let next_cost = cost + step_cost;
            let state = (next_pos, next_dir);
            let current_best = *distances.get(&state).unwrap_or(&usize::MAX);

            if next_cost < current_best {
                distances.insert(state, next_cost);
                predecessors.insert(state, vec![(pos, dir)]);
                heap.push(State {
                    cost: next_cost,
                    pos: next_pos,
                    dir: next_dir,
                });
            } else if next_cost == current_best {
                predecessors.entry(state).or_default().push((pos, dir));
            }
        }
    }

    // --- Part 2: Backtracking ---
    let mut best_tiles = HashSet::new();
    let mut queue = VecDeque::new();
    let mut seen_states = HashSet::new();

    // Any direction that reaches 'end' with the global best cost is a valid starting point for backtracking
    for &d in &[Point::UP, Point::DOWN, Point::LEFT, Point::RIGHT] {
        if let Some(&cost) = distances.get(&(end, d))
            && cost == best_total_cost {
                queue.push_back((end, d));
            }
    }

    while let Some(state) = queue.pop_front() {
        if !seen_states.insert(state) {
            continue;
        }
        best_tiles.insert(state.0); // state.0 is the Point (position)

        if let Some(preds) = predecessors.get(&state) {
            for &prev in preds {
                queue.push_back(prev);
            }
        }
    }

    (best_total_cost, best_tiles.len())
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: Point,
    dir: Point,
}

// BinaryHeap is a max-heap, so we implement Ord such that lower cost has higher priority
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_day16() {
        let (p1, p2) = solve(&Grid::from_str(EXAMPLE).unwrap());
        assert_eq!(p1, 7036);
        assert_eq!(p2, 45);
    }
}
