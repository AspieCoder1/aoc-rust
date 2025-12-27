use anyhow::{Error, Result};
use regex::RegexBuilder;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

const INPUT_NUM: usize = 0;

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
    // Brute force DFS initial solution
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
    0
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Input {
    pattern: usize,
    wiring: Vec<usize>,
    joltage_required: Vec<usize>,
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Regex to parse each part of the input
        let pattern_regex = RegexBuilder::new(r"\[([#\.]+)\]").build()?;
        let wiring_regex = RegexBuilder::new(r"\(([\d\,?]+)\)").build()?;
        let joltage_regex = RegexBuilder::new(r"\{([\d|\,]+)\}").build()?;

        // Processing the capture group
        let pattern_str = &pattern_regex.captures(s).unwrap()[1];
        let wiring_str = &wiring_regex.captures_iter(s).collect::<Vec<_>>();
        let joltage_str = &joltage_regex.captures(s).unwrap()[1];

        // Converting the final values
        let pattern = pattern_str
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(|(i, _)| 1 << (pattern_str.len() - 1 - i))
            .sum::<usize>();
        let wiring = wiring_str
            .iter()
            .map(|row| {
                row[1]
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .map(|u| 1 << (pattern_str.len() - 1 - u))
                    .sum::<usize>()
            })
            .collect::<Vec<_>>();
        let joltage_required = joltage_str
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Ok(Self {
            pattern,
            wiring,
            joltage_required,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";

        let parsed = input.parse::<Input>().unwrap();
        let expected = Input {
            pattern: 0b0110,
            wiring: vec![0b0001, 0b0101, 0b0010, 0b0011, 0b1010, 0b1100],
            joltage_required: vec![3, 5, 4, 7],
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(1).unwrap();
        assert_eq!(part1(&input), 7);
    }
}
