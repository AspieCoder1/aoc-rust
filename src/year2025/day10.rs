//! Advent of Code 2025 Day 10
//! Link: <https://adventofcode.com/2025/day/10>
//!
use crate::utils::simplex::{LPBuilder, LPOps, branch_and_bound};
use anyhow::{Context, Error, Result};
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use std::sync::OnceLock;

pub fn main(data: &str) -> Result<(usize, usize)> {
    let input = parse_input(data)?;

    Ok((part1(&input), part2(&input)))
}

fn parse_input(input: &str) -> Result<Vec<Input>> {
    Ok(input
        .lines()
        .map(|line| line.parse::<Input>().unwrap())
        .collect::<Vec<_>>())
}

fn part1(input: &[Input]) -> usize {
    let mut acc = 0;
    // Brute force BFS initial solution
    for input in input {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);

        while let Some((btn_presses, state)) = queue.pop_front() {
            if state == input.pattern {
                acc += btn_presses;
                break;
            }
            for wiring in &input.wiring {
                let new_state = state ^ wiring;
                if visited.contains(&new_state) {
                    continue;
                }
                queue.push_back((btn_presses + 1, new_state));
            }
            visited.insert(state);
        }
    }
    acc
}

// We can recast each problem as ILP and then use the revised simplex algorithm to solve it.
fn part2(_input: &[Input]) -> usize {
    let mut acc = 0;

    for input in _input {
        match branch_and_bound(input.lpbuilder.clone(), input.wiring.len()) {
            Some(solution) => {
                acc += solution;
            }
            None => continue,
        }
    }
    acc as usize
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Input {
    pattern: usize,
    wiring: Vec<usize>,
    joltage_required: Vec<usize>,
    lpbuilder: LPBuilder,
}

static RE_PATTERN: OnceLock<Regex> = OnceLock::new();
static RE_WIRING: OnceLock<Regex> = OnceLock::new();
static RE_JOLTAGE: OnceLock<Regex> = OnceLock::new();

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Initialize Regexes
        let re_p = RE_PATTERN.get_or_init(|| Regex::new(r"\[([#.]+)\]").unwrap());
        let re_w = RE_WIRING.get_or_init(|| Regex::new(r"\(([\d,?]+)\)").unwrap());
        let re_j = RE_JOLTAGE.get_or_init(|| Regex::new(r"\{([\d|,]+)\}").unwrap());

        // Extract raw strings with context-rich errors
        let pattern_raw = &re_p.captures(s).context("Failed to find pattern [#...]")?[1];
        let joltage_raw = &re_j.captures(s).context("Failed to find joltage {...}")?[1];
        let wiring_caps: Vec<_> = re_w.captures_iter(s).collect();

        let p_len = pattern_raw.len();
        let to_mask = |i: usize| 1 << (p_len - 1 - i);

        // 1. Parse Pattern bitmask
        let pattern = pattern_raw
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(|(i, _)| to_mask(i))
            .sum();

        // 2. Parse Joltage requirements
        let joltage_required = joltage_raw
            .split(',')
            .map(|val| {
                val.parse::<usize>()
                    .with_context(|| format!("Invalid joltage: {val}"))
            })
            .collect::<Result<Vec<_>>>()?;

        // 3. Parse Wiring and build Constraints simultaneously
        let num_buttons = wiring_caps.len();
        let num_machines = joltage_required.len();
        let mut constraints = vec![vec![0; num_buttons]; num_machines];

        let wiring = wiring_caps
            .iter()
            .enumerate()
            .map(|(btn_idx, cap)| {
                let mut mask_sum = 0;
                for part in cap[1].split(',') {
                    let m_idx: usize = part
                        .parse()
                        .with_context(|| format!("Invalid wiring index: {part}"))?;

                    mask_sum += to_mask(m_idx);
                    if m_idx < num_machines {
                        constraints[m_idx][btn_idx] = 1;
                    }
                }
                Ok(mask_sum)
            })
            .collect::<Result<Vec<usize>>>()?;

        // 4. Build LP
        let mut lpbuilder = LPBuilder::new();
        for (i, constraint) in constraints.into_iter().enumerate() {
            lpbuilder.add_constraint(constraint, LPOps::Eq, joltage_required[i] as i64);
        }
        lpbuilder.add_objective(vec![1; num_buttons]);

        Ok(Self {
            pattern,
            wiring,
            joltage_required,
            lpbuilder,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(EXAMPLE).unwrap();
        let expected = Input {
            pattern: 0b0110,
            wiring: vec![0b0001, 0b0101, 0b0010, 0b0011, 0b1010, 0b1100],
            joltage_required: vec![3, 5, 4, 7],
            lpbuilder: LPBuilder {
                objective: vec![1; 6],
                constraints: vec![
                    vec![0, 0, 0, 0, 1, 1],
                    vec![0, 1, 0, 0, 0, 1],
                    vec![0, 0, 1, 1, 1, 0],
                    vec![1, 1, 0, 1, 0, 0],
                ],
                ops: vec![LPOps::Eq; 4],
                ans: vec![3, 5, 4, 7],
            },
        };

        println!("{:?}", parsed[0].lpbuilder);
        assert_eq!(parsed[0], expected);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 33);
    }
}
