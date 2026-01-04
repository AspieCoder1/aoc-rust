//! Advent of Code 2024 Day 14
//!
//! Link: <https://adventofcode.com/2024/day/14>

use crate::utils::point::Point;
use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::OnceLock;

pub fn main(input: &str) -> Result<(usize, usize)> {
    let robots = parse_input(input)?;

    Ok((part1(&robots, (101, 103)), part2(&robots, (101, 103))))
}

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    input.lines().map(Robot::from_str).collect()
}

fn part1(robots: &[Robot], grid_size: (i32, i32)) -> usize {
    let (w, h) = grid_size;
    let mid_x = w / 2;
    let mid_y = h / 2;
    let mut quads = [0usize; 4];

    for r in robots {
        let p = r.at_time(100, w, h);

        // Use Ordering to determine which quadrant the robot falls into
        match (p.x.cmp(&mid_x), p.y.cmp(&mid_y)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quads[0] += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quads[1] += 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quads[2] += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quads[3] += 1,
            _ => {} // Robots on the exact middle lines are ignored
        }
    }
    quads.iter().product()
}

fn part2(robots: &[Robot], grid_size: (i32, i32)) -> usize {
    let (w, h) = grid_size;
    // Part 2's Christmas tree logic is specific to the 101x103 challenge.
    if w < 20 {
        return 0;
    }

    let mut current_robots = robots.to_vec();
    for t in 1..10_000 {
        let mut seen = HashSet::with_capacity(robots.len());
        let mut overlap = false;

        for r in current_robots.iter_mut() {
            r.step(w, h);
            // If two robots occupy the same space, the "tree" isn't formed yet
            if !seen.insert((r.pos.x, r.pos.y)) {
                overlap = true;
            }
        }

        if !overlap {
            return t;
        }
    }
    0
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn at_time(&self, t: i32, w: i32, h: i32) -> Point {
        Point::new(
            (self.pos.x + self.vel.x * t).rem_euclid(w),
            (self.pos.y + self.vel.y * t).rem_euclid(h),
        )
    }

    fn step(&mut self, w: i32, h: i32) {
        self.pos.x = (self.pos.x + self.vel.x).rem_euclid(w);
        self.pos.y = (self.pos.y + self.vel.y).rem_euclid(h);
    }
}

impl FromStr for Robot {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let caps = RE
            .get_or_init(|| Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap())
            .captures(s)
            .context("Parse error")?;

        Ok(Self {
            pos: Point::new(caps[1].parse()?, caps[2].parse()?),
            vel: Point::new(caps[3].parse()?, caps[4].parse()?),
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
