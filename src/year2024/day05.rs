//! Advent of Code 2024 Day 5
//!
//! Link: <https://adventofcode.com/2024/day/5>

use anyhow::{Error, Result};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub fn main(input_date: &str) -> Result<(usize, usize)> {
    let input = parse_input(input_date)?;
    Ok((part1(&input), part2(&input)))
}

fn parse_input(input: &str) -> Result<Input> {
    input.parse()
}

fn part1(input: &Input) -> usize {
    let mut acc = 0;
    for update in &input.updates {
        if !input.is_valid_update(update) {
            continue;
        }
        acc += update.get((update.len()) / 2).unwrap_or(&0);
    }
    acc
}

fn part2(input: &Input) -> usize {
    let mut acc = 0;
    for update in &input.updates {
        if !input.is_valid_update(update) {
            acc += update
                .iter()
                .sorted_by(|&a, &b| input.compare_page_ordering(a, b))
                .nth(update.len() / 2)
                .unwrap_or(&0);
        }
    }
    acc
}

#[derive(Debug, PartialEq)]
struct Input {
    ordering: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

impl Input {
    fn is_valid_update(&self, update: &[usize]) -> bool {
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                if let Some(value) = self.ordering.get(&update[j])
                    && value.contains(&update[i])
                {
                    return false;
                }
            }
        }
        true
    }

    fn compare_page_ordering(&self, page_a: &usize, page_b: &usize) -> Ordering {
        if let Some(ordering) = self.ordering.get(page_a)
            && ordering.contains(page_b)
        {
            Ordering::Less
        } else if let Some(ordering) = self.ordering.get(page_b)
            && ordering.contains(page_a)
        {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (orderings, updates) = input.split_once("\n\n").unwrap();
        let mut ordering: HashMap<usize, HashSet<usize>> = HashMap::new();

        for comparison in orderings.lines() {
            if let Some((left, right)) = comparison.split_once("|")
                && let (Ok(left), Ok(right)) =
                    (left.trim().parse::<usize>(), right.trim().parse::<usize>())
            {
                ordering.entry(left).or_default().insert(right);
            }
        }

        Ok(Self {
            ordering,
            updates: updates
                .lines()
                .map(|line| {
                    line.split(',')
                        .map(|num| num.parse().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_parse_input() {
        let input = parse_input(EXAMPLE).unwrap();
        let expected_ordering: HashMap<usize, HashSet<usize>> = HashMap::from([
            (47, HashSet::from([53, 13, 61, 29])),
            (97, HashSet::from([13, 61, 47, 29, 53, 75])),
            (75, HashSet::from([29, 53, 47, 61, 13])),
            (61, HashSet::from([13, 53, 29])),
            (29, HashSet::from([13])),
            (53, HashSet::from([29, 13])),
        ]);
        let expected_updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let expected_input = Input {
            ordering: expected_ordering,
            updates: expected_updates,
        };

        assert_eq!(input, expected_input);
    }

    #[test]
    fn test_compare_page_ordering() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(input.compare_page_ordering(&47, &53), Ordering::Less);
        assert_eq!(input.compare_page_ordering(&47, &29), Ordering::Less);
        assert_eq!(input.compare_page_ordering(&47, &13), Ordering::Less);
        assert_eq!(input.compare_page_ordering(&53, &61), Ordering::Greater);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 143);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 123);
    }
}
