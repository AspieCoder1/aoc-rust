//! Advent of Code 2024 Day 14
//!
//! Link: <https://adventofcode.com/2024/day/14>

use crate::utils::point::Point;
use anyhow::{Context, Result};
use itertools::*;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

pub fn main(input: &str) -> Result<(usize, usize)> {
    let input = parse_input(input)?;
    Ok((part1(&input, (101, 103)), part2(&input)))
}

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    input.lines().map(Robot::from_str).collect()
}

fn part1(input: &[Robot], grid_size: (i32, i32)) -> usize {
    let mut num_per_quadrant = HashMap::new();
    input.iter().for_each(|robot| {
        let new_pos = robot.move_robot(100, grid_size.1, grid_size.0);
        let quadrant = map_coord_to_quadrant(new_pos, grid_size.0, grid_size.1);
        if quadrant > 0 {
            *num_per_quadrant.entry(quadrant).or_insert(0) += 1;
        }
    });
    num_per_quadrant.values().product()
}

fn part2(input: &[Robot]) -> usize {
    let mut robots = input.to_vec();
    let mut robot_map = [[0; 101]; 103];
    let mut time = 1;

    loop {
        for robot in robots.iter_mut() {
            robot.move_robot_mut(1, 103, 101);
        }
        project_robots_to_map(&robots, &mut robot_map);
        if find_straight_line_of_ten(&robot_map) {
            break;
        }
        time += 1;
    }
    time
}

fn reset_map(map: &mut [[i32; 101]; 103]) {
    for row in map.iter_mut() {
        for cell in row.iter_mut() {
            *cell = 0;
        }
    }
}

fn project_robots_to_map(locations: &[Robot], map: &mut [[i32; 101]; 103]) {
    reset_map(map);
    for robot in locations {
        map[robot.start.y as usize][robot.start.x as usize] += 1;
    }
}

fn find_straight_line_of_ten(map: &[[i32; 101]; 103]) -> bool {
    // Check horizontal lines
    let found_horizontal_line = map.iter().any(|row| {
        row.windows(10)
            .any(|window| window.iter().all(|cell| *cell > 0))
    });

    // Check vertical lines
    let found_vertical_line = (0..101).any(|x| {
        (0..103)
            .map(|y| map[y][x] > 0)
            .chunk_by(|&is_positive| is_positive)
            .into_iter()
            .any(|(is_positive, group)| is_positive && group.count() >= 10)
    });
    found_horizontal_line || found_vertical_line
}

fn map_coord_to_quadrant(point: Point, tile_width: i32, tile_height: i32) -> usize {
    let middle_width = tile_width / 2;
    let middle_heigh = tile_height / 2;

    if point.x < middle_width && point.y < middle_heigh {
        1
    } else if point.x > middle_width && point.y < middle_heigh {
        2
    } else if point.x < middle_width && point.y > middle_heigh {
        3
    } else if point.x > middle_width && point.y > middle_heigh {
        4
    } else {
        0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Robot {
    start: Point,
    velocity: Point,
}

impl Robot {
    fn move_robot(&self, steps: i32, tile_tall: i32, tile_wide: i32) -> Point {
        let new_x = (self.start.x + (self.velocity.x * steps)).rem_euclid(tile_wide);
        let new_y = (self.start.y + (self.velocity.y * steps)).rem_euclid(tile_tall);
        Point::new(new_x, new_y)
    }

    fn move_robot_mut(&mut self, steps: i32, tile_tall: i32, tile_wide: i32) {
        self.start.x = (self.start.x + (self.velocity.x * steps)).rem_euclid(tile_wide);
        self.start.y = (self.start.y + (self.velocity.y * steps)).rem_euclid(tile_tall);
    }
}

static ROBOT_REGEX: OnceLock<Regex> = OnceLock::new();

impl FromStr for Robot {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let integer_regex =
            ROBOT_REGEX.get_or_init(|| Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap());
        let capture = integer_regex.captures(s).context("Invalid robot format")?;

        let pos_x = capture.get(1).unwrap().as_str().parse::<i32>()?;
        let pos_y = capture.get(2).unwrap().as_str().parse::<i32>()?;
        let vel_x = capture.get(3).unwrap().as_str().parse::<i32>()?;
        let vel_y = capture.get(4).unwrap().as_str().parse::<i32>()?;

        Ok(Self {
            start: Point::new(pos_x, pos_y),
            velocity: Point::new(vel_x, vel_y),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_parse_input() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(input.len(), 12);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();

        assert_eq!(part1(&input, (11, 7)), 12);
    }
}
