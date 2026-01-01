//! Advent of Code 2024 Day 9
//!
//! Link: <https://adventofcode.com/2024/day/9>

use anyhow::Result;
use std::iter::repeat_n;

pub fn main(input: &str) -> Result<(usize, usize)> {
    let data = parse_input(input)?;
    Ok((part1(&data), part2()))
}

fn parse_input(input: &str) -> Result<Vec<isize>> {
    let mut id = 0_isize;
    let mut data_block: Vec<isize> = Vec::new();

    for (i, c) in input.chars().enumerate() {
        let digit = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            data_block.extend(repeat_n(id, digit as usize));
            id += 1;
        } else {
            data_block.extend(repeat_n(-1, digit as usize));
        }
    }
    Ok(data_block)
}

fn part1(input: &[isize]) -> usize {
    let mut disk = input.to_vec();

    for i in (0..disk.len()).rev() {
        let empty_idx = disk.iter().position(|&x| x == -1).unwrap();
        if i <= empty_idx {
            break;
        }
        disk.swap(i, empty_idx);
    }

    disk.iter()
        .enumerate()
        .filter(|&(_, &x)| x > 0)
        .map(|(i, &file_id)| i * file_id as usize)
        .sum()
}

fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input() {
        let input = parse_input("2333133121414131402").unwrap();
        assert_eq!(
            input,
            vec![
                0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5,
                5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = parse_input("2333133121414131402").unwrap();

        assert_eq!(part1(&input), 1928);
    }
}
