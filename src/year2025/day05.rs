use anyhow::{Error, Result};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::str::FromStr;

const INPUT_NUM: usize = 0;

pub fn main() -> Result<(u32, u64)> {
    let input = parse_input(INPUT_NUM)?;
    Ok((part1(&input), part2(&input)))
}

pub fn parse_input(input_num: usize) -> Result<Input> {
    [
        include_str!("inputs/day05.inp"),
        include_str!("test_inputs/day05.inp1"),
    ][input_num]
        .parse()
}

pub fn part1(input: &Input) -> u32 {
    let intervals = merge_intervals(&input.intervals);
    let mut acc = 0;

    for ingredient_id in &input.ingredient_ids {
        for interval in &intervals {
            if ingredient_id >= &interval.start && ingredient_id <= &interval.end {
                acc += 1;
            }
        }
    }
    acc
}

pub fn part2(input: &Input) -> u64 {
    let intervals = merge_intervals(&input.intervals);
    let mut acc = 0;

    for interval in &intervals {
        acc += interval.end - interval.start + 1;
    }
    acc
}

fn merge_intervals(intervals: &[Interval]) -> Vec<Interval> {
    // Add intervals to the heap
    let mut min_heap = BinaryHeap::new();
    for interval in intervals.iter().cloned() {
        min_heap.push(Reverse(interval));
    }

    if let Some(Reverse(initial_interval)) = min_heap.pop() {
        let mut merged_intervals = vec![initial_interval];

        while let Some(Reverse(interval)) = min_heap.pop() {
            let previous_interval = merged_intervals.last_mut().unwrap();
            if interval.start <= previous_interval.end {
                previous_interval.end = u64::max(previous_interval.end, interval.end);
            } else {
                merged_intervals.push(interval);
            }
        }
        merged_intervals
    } else {
        intervals.to_vec()
    }
}

#[derive(Debug, Clone)]
struct Interval {
    start: u64,
    end: u64,
}

#[derive(Debug, Clone)]
pub struct Input {
    intervals: Vec<Interval>,
    ingredient_ids: Vec<u64>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (intervals, ingredient_ids) = s.split_once("\n\n").unwrap();
        Ok(Self {
            intervals: intervals
                .lines()
                .map(|line| {
                    let (a, b) = line.split_once("-").unwrap();
                    Interval {
                        start: a.parse().unwrap(),
                        end: b.parse().unwrap(),
                    }
                })
                .collect(),
            ingredient_ids: ingredient_ids
                .lines()
                .map(|line| line.parse().unwrap())
                .collect(),
        })
    }
}

impl Eq for Interval {}

impl PartialEq<Self> for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.start == self.end && other.start == other.end
    }
}

impl PartialOrd<Self> for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        let input = parse_input(1).unwrap();
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(1).unwrap();
        assert_eq!(part2(&input), 14);
    }
}
