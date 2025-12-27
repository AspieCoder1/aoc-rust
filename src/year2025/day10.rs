use crate::utils::simplex::{LPBuilder, LPOps};
use anyhow::{Error, Result};
use regex::RegexBuilder;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

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

fn branch_and_bound(root: LPBuilder, n: usize) -> Option<i64> {
    let mut best: Option<i64> = None;
    let mut stack = vec![root];
    while let Some(b) = stack.pop() {
        let mut lp = b.clone().build();
        let Some(obj) = lp.minimize() else {
            continue; // infeasible/unbounded node
        };
        let node_lb = obj.ceil();
        if let Some(best_val) = best
            && node_lb >= best_val.into() {
                continue;
            }
        let x = lp.solution_x();
        if let Some((k, xk)) = x.iter().enumerate().find(|(_, v)| !v.is_integer()) {
            let lo = xk.floor().to_integer();
            let hi = xk.ceil().to_integer();
            let mut b_le = b.clone();
            let mut v = vec![0; n];
            v[k] = 1;
            b_le.add_constraint(v.clone(), LPOps::Lte, lo);
            let mut b_ge = b;
            b_ge.add_constraint(v, LPOps::Gte, hi);
            stack.push(b_le);
            stack.push(b_ge);
        } else {
            let obj_i = obj.to_integer();
            best = Some(best.map_or(obj_i, |cur| cur.min(obj_i)));
        }
    }
    best
}

// We can recast each problem as ILP and then use the revised simplex algorithm to solve it.
fn part2(_input: &[Input]) -> usize {
    let mut acc = 0;

    for input in _input {
        match branch_and_bound(input.lpbuilder.clone(), input.wiring.len()) {
            Some(solution) => {
                acc += solution;
            }
            None => continue
        }
    }
    println!("Total num of button presses: {}", acc);
    acc as usize
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Input {
    pattern: usize,
    wiring: Vec<usize>,
    joltage_required: Vec<usize>,
    lpbuilder: LPBuilder,
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

        let num_buttons = wiring_str.len();
        let mut lp_builder = LPBuilder::new();
        let num_machines = joltage_required.len();
        let mut constraints = vec![vec![0; num_buttons]; num_machines];

        for (button, wiring) in wiring_str.iter().enumerate() {
            for machine in wiring[1].split(',').map(|s| s.parse::<usize>().unwrap()) {
                constraints[machine][button] = 1;
            }
        }
        for (ind, constraint) in constraints.iter().enumerate() {
            lp_builder.add_constraint(constraint.clone(), LPOps::Eq, joltage_required[ind] as i64);
        }
        lp_builder.add_objective(vec![1; num_buttons]);

        Ok(Self {
            pattern,
            wiring,
            joltage_required,
            lpbuilder: lp_builder,
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
