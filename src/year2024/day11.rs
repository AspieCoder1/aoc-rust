//! Advent of Code 2024 Day 11
//!
//! Link: <https://adventofcode.com/2024/day/11>

use anyhow::Result;
use rustc_hash::FxHashMap;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let initial_state = parse_input(input_data);
    Ok((part1(&initial_state), part2(&initial_state)))
}

fn parse_input(input_data: &str) -> Vec<usize> {
    input_data
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .filter_map(Result::ok)
        .collect()
}

fn part1(initial_state: &[usize]) -> usize {
    get_num_stones(initial_state, 25)
}

fn get_num_stones(initial_state: &[usize], n_iter: usize) -> usize {
    // The cache stores: (stone_value, remaining_iters) -> total_stones_produced
    let mut cache: FxHashMap<(usize, usize), usize> = FxHashMap::default();

    initial_state
        .iter()
        .map(|&stone| get_num_stones_rec(stone, n_iter, &mut cache))
        .sum()
}

fn get_num_stones_rec(
    stone: usize,
    num_iters: usize,
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    // Base case: 0 iterations left means this stone represents exactly 1 stone
    if num_iters == 0 {
        return 1;
    }

    // Check if we have already calculated the result for this specific stone and depth
    if let Some(&count) = cache.get(&(stone, num_iters)) {
        return count;
    }

    // Apply rules and recurse
    let next_stones = apply_rules(stone);
    let total_count: usize = next_stones
        .0
        .into_iter()
        .take(next_stones.1)
        .map(|s| get_num_stones_rec(s, num_iters - 1, cache))
        .sum();

    // Store the result in the cache before returning
    cache.insert((stone, num_iters), total_count);
    total_count
}

fn part2(initial_state: &[usize]) -> usize {
    get_num_stones(initial_state, 75)
}

fn apply_rules(num: usize) -> ([usize; 2], usize) {
    if num == 0 {
        return ([1, 0], 1);
    }

    // Num digits is ⌊log₁₀(num)⌋ + 1
    let num_digits = usize::ilog10(num) + 1;
    if num_digits.is_multiple_of(2) {
        let k = num_digits / 2;
        return ([num / 10_usize.pow(k), num % 10_usize.pow(k)], 2);
    }
    ([2024 * num, 0], 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn test_apply_rules() {
        assert_eq!(apply_rules(0), ([1, 0], 1));
        assert_eq!(apply_rules(1), ([2024, 0], 1));
        assert_eq!(apply_rules(10), ([1, 0], 2));
        assert_eq!(apply_rules(99), ([9, 9], 2));
        assert_eq!(apply_rules(999), ([2021976, 0], 1));
        assert_eq!(apply_rules(1000), ([10, 0], 2));
    }

    #[test]
    fn test_get_num_stones() {
        let input = parse_input(EXAMPLE);
        assert_eq!(get_num_stones(&input, 25), 55312);
    }
}
