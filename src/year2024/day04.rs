//! Advent of Code 2024 Day 4
//!
//! Link: <https://adventofcode.com/2024/day/4>

use crate::utils::grid::{Grid, GridError};
use crate::utils::point::Point;
use anyhow::Result;
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let input = parse_input(input_data)?;

    Ok((part1(&input), part2(&input)))
}

fn parse_input(input_data: &str) -> Result<Grid<char>, GridError> {
    Grid::<char>::from_str(input_data)
}

fn part1(input: &Grid<char>) -> usize {
    let search_term = vec!['X', 'M', 'A', 'S'];
    let start_positions = input.all_positions(|&c| c == 'X');
    let mut num_occurrences = 0;

    // Using Point-based offsets for all 8 directions
    let directions = [
        Point::UP,
        Point::DOWN,
        Point::LEFT,
        Point::RIGHT,
        Point::UP + Point::LEFT,
        Point::UP + Point::RIGHT,
        Point::DOWN + Point::LEFT,
        Point::DOWN + Point::RIGHT,
    ];

    for start_pos in start_positions {
        for &dir in &directions {
            // Use the new ray_cast utility to grab 4 characters in a line
            let path = input.ray_cast(start_pos, dir, 4);
            if path == search_term {
                num_occurrences += 1;
            }
        }
    }
    num_occurrences
}

fn part2(input: &Grid<char>) -> usize {
    let mut num_occurrences = 0;

    // Filter for 'A' positions that have room for a 3x3 X-shape
    let start_positions = input.all_positions(|&c| c == 'A').filter(|&p| {
        p.x > 0 && p.x < (input.width as i32 - 1) && p.y > 0 && p.y < (input.height as i32 - 1)
    });

    for p in start_positions {
        // Define relative corners using Point addition/subtraction
        let tl = p + Point::new(-1, -1);
        let br = p + Point::new(1, 1);
        let tr = p + Point::new(1, -1);
        let bl = p + Point::new(-1, 1);

        // Checking the two diagonals for "MAS" or "SAM"
        // We can simplify the logic by checking if opposite corners are M/S or S/M
        let diag1 = (input[tl], input[br]);
        let diag2 = (input[tr], input[bl]);

        let is_mas = |pair| matches!(pair, ('M', 'S') | ('S', 'M'));

        if is_mas(diag1) && is_mas(diag2) {
            num_occurrences += 1;
        }
    }
    num_occurrences
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 9);
    }
}
