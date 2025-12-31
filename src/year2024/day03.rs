//! Advent of Code 2024 Day 3
//!
//! Link: <https://adventofcode.com/2024/day/3>
use anyhow::Result;
use regex::Regex;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let input = parse_input(input_data)?;
    Ok((part1(&input), part2(&input)))
}

fn parse_input(input_data: &str) -> Result<Vec<Instruction>> {
    let instruction_regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(don't\(\))|(do\(\))")?;
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    Ok(instruction_regex
        .captures_iter(input_data)
        .map(|cap| {
            if cap.get(1).is_some() {
                let mul_capture = mul_regex.captures(&cap[1]);
                if let Some(mul) = mul_capture {
                    let a = mul[1].parse::<usize>().unwrap();
                    let b = mul[2].parse::<usize>().unwrap();
                    Instruction::Mul(a, b)
                } else {
                    panic!("Invalid mul capture");
                }
            } else if cap.get(2).is_some() {
                Instruction::DoNot
            } else {
                Instruction::Do
            }
        })
        .collect::<Vec<_>>())
}

fn part1(input: &[Instruction]) -> usize {
    input
        .iter()
        .filter(|i| matches!(i, Instruction::Mul(_, _)))
        .map(|i| match i {
            Instruction::Mul(a, b) => *a * *b,
            _ => unreachable!(),
        })
        .sum()
}

fn part2(input: &[Instruction]) -> usize {
    let mut sum = 0;

    let mut should_add = true;
    for instruction in input {
        match instruction {
            Instruction::Mul(a, b) => {
                if should_add {
                    sum += a * b;
                }
            }
            Instruction::Do => should_add = true,
            Instruction::DoNot => should_add = false,
        }
    }
    sum
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(usize, usize),
    Do,
    DoNot,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse_input() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(
            input,
            vec![
                Instruction::Mul(2, 4),
                Instruction::DoNot,
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Do,
                Instruction::Mul(8, 5)
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 48);
    }
}
