//! Advent of Code 2025 Day 12
//! Link: <https://adventofcode.com/2025/day/12>
//!
use crate::utils::grid::Grid;
use anyhow::{Error, Result};
use std::str::FromStr;

pub fn main(input_data: &str) -> Result<(usize, usize)> {
    let input = parse_input(input_data)?;

    Ok((part1(&input), 0))
}

fn parse_input(data: &str) -> Result<Input> {
    data.parse()
}

fn part1(input: &Input) -> usize {
    let mut acc = 0;

    for region in &input.regions {
        let area_required = region
            .required_presents
            .iter()
            .enumerate()
            .map(|(ind, num_presents)| num_presents * input.shape_areas[ind])
            .sum::<usize>();
        // Trivially impossible as insufficient total area
        if area_required > region.width * region.height {
            continue;
        }

        // Trivially possible as no overlap required
        if region.required_presents.iter().sum::<usize>()
            <= (region.width / 3) * (region.height / 3)
        {
            acc += 1
        }
    }
    acc
}

#[derive(Debug, PartialEq)]
struct Region {
    width: usize,
    height: usize,
    required_presents: Vec<usize>,
}

#[derive(Debug, PartialEq)]
struct Input {
    shapes: Vec<Grid<char>>,
    shape_areas: Vec<usize>,
    regions: Vec<Region>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split = s.split("\n\n");
        let shapes: Vec<Grid<char>> = split
            .by_ref()
            .take(6)
            .map(|s| {
                let (_, grid) = s.split_once("\n").unwrap();
                Grid::<char>::from_str(grid).expect("Unable to parse grid")
            })
            .collect::<Vec<_>>();
        let shape_areas = shapes
            .iter()
            .by_ref()
            .map(|g| g.g.iter().filter(|&c| *c == '#').count())
            .collect();
        let regions = split
            .last()
            .unwrap()
            .lines()
            .map(|s| {
                let (region, required_presents) = s
                    .split_once(": ")
                    .expect("Unable to split region and required presents");
                let (width, height) = region.split_once("x").unwrap();
                let width = width.parse::<usize>().unwrap();
                let height = height.parse::<usize>().unwrap();
                let required_presents = required_presents
                    .split(' ')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                Region {
                    width,
                    height,
                    required_presents,
                }
            })
            .collect();

        Ok(Self {
            shapes,
            shape_areas,
            regions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_parse_input() {
        let input = parse_input(EXAMPLE).unwrap();
        let expected = Input {
            shapes: [
                "###\n##.\n##.",
                "###\n##.\n.##",
                ".##\n###\n##.",
                "##.\n###\n##.",
                "###\n#..\n###",
                "###\n.#.\n###",
            ]
            .iter()
            .map(|s| Grid::<char>::from_str(s).unwrap())
            .collect(),
            shape_areas: vec![7; 6],
            regions: vec![
                Region {
                    width: 4,
                    height: 4,
                    required_presents: vec![0, 0, 0, 0, 2, 0],
                },
                Region {
                    width: 12,
                    height: 5,
                    required_presents: vec![1, 0, 1, 0, 2, 2],
                },
                Region {
                    width: 12,
                    height: 5,
                    required_presents: vec![1, 0, 1, 0, 3, 2],
                },
            ],
        };

        assert_eq!(input, expected);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();

        assert_eq!(part1(&input), 0);
    }
}
