//! Advent of Code 2024 - Day 15
//!
//! Link: <https://adventofcode.com/2024/day/15>
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use anyhow::{Error, Result};
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::str::FromStr;

pub fn main(input: &str) -> Result<(i32, i32)> {
    let input = Input::from_str(input)?;
    Ok((part1(&input), part2(&input)))
}

fn part1(input: &Input) -> i32 {
    let mut grid = input.grid.clone();
    let mut robot = grid
        .find_pos(|&el| el == Element::Robot)
        .expect("No robot found");

    for &dir in &input.moves {
        let delta = dir.to_point();
        let next = robot + delta;

        match grid[next] {
            Element::Empty => {
                grid[next] = Element::Robot;
                grid[robot] = Element::Empty;
                robot = next;
            }
            Element::Wall => continue,
            Element::Box => {
                let mut scan = next;
                while grid[scan] == Element::Box {
                    scan = scan + delta;
                }
                if grid[scan] == Element::Empty {
                    // Standard shift: move the whole line of boxes
                    grid[scan] = Element::Box;
                    grid[next] = Element::Robot;
                    grid[robot] = Element::Empty;
                    robot = next;
                }
            }
            _ => unreachable!(),
        }
    }
    score(&grid, Element::Box)
}

fn part2(input: &Input) -> i32 {
    let mut grid = expand_grid(&input.grid);
    let mut robot = grid
        .find_pos(|&el| el == Element::Robot)
        .expect("No robot found");

    for &dir in &input.moves {
        let mut affected = HashSet::new();
        if can_move(&grid, robot, dir, &mut affected) {
            let mut sorted: Vec<Point> = affected.into_iter().collect();
            let delta = dir.to_point();

            // Sort to move pieces furthest from robot first
            sorted.sort_by_key(|p| match dir {
                Direction::Up => p.y,
                Direction::Down => -p.y,
                Direction::Left => p.x,
                Direction::Right => -p.x,
            });

            for pos in sorted {
                let target = pos + delta;
                grid[target] = grid[pos];
                grid[pos] = Element::Empty;
            }
            robot = robot + delta;
        }
    }
    score(&grid, Element::BoxLeft)
}

fn can_move(grid: &Grid<Element>, pos: Point, dir: Direction, seen: &mut HashSet<Point>) -> bool {
    if !seen.insert(pos) {
        return true;
    }

    let delta = dir.to_point();
    let next = pos + delta;

    match grid[next] {
        Element::Empty => true,
        Element::Wall => false,
        Element::Box => can_move(grid, next, dir, seen),
        Element::BoxLeft | Element::BoxRight => {
            // Check the space directly in front
            if !can_move(grid, next, dir, seen) {
                return false;
            }
            // Vertical moves must pull the other side of the double-box
            if matches!(dir, Direction::Up | Direction::Down) {
                let other_side = if grid[next] == Element::BoxLeft {
                    next + Point::RIGHT
                } else {
                    next + Point::LEFT
                };
                if !can_move(grid, other_side, dir, seen) {
                    return false;
                }
            }
            true
        }
        _ => true,
    }
}

fn score(grid: &Grid<Element>, target: Element) -> i32 {
    grid.all_positions(|&el| el == target)
        .map(|p| 100 * p.y + p.x)
        .sum()
}

fn expand_grid(old: &Grid<Element>) -> Grid<Element> {
    let mut new_vec = Vec::with_capacity(old.g.len() * 2);
    for cell in &old.g {
        match cell {
            Element::Wall => { new_vec.push(Element::Wall); new_vec.push(Element::Wall); }
            Element::Box => { new_vec.push(Element::BoxLeft); new_vec.push(Element::BoxRight); }
            Element::Robot => { new_vec.push(Element::Robot); new_vec.push(Element::Empty); }
            Element::Empty => { new_vec.push(Element::Empty); new_vec.push(Element::Empty); }
            _ => unreachable!(),
        }
    }
    Grid::from_vals(new_vec, old.width * 2, old.height)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element { Wall, Empty, Robot, Box, BoxLeft, BoxRight }

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Wall => '#', Self::Empty => '.', Self::Robot => '@',
            Self::Box => 'O', Self::BoxLeft => '[', Self::BoxRight => ']',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction { Up, Down, Left, Right }

impl Direction {
    fn to_point(self) -> Point {
        match self {
            Self::Up => Point::UP,
            Self::Down => Point::DOWN,
            Self::Left => Point::LEFT,
            Self::Right => Point::RIGHT,
        }
    }
}

struct Input {
    grid: Grid<Element>,
    moves: Vec<Direction>,
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (g_str, m_str) = s.split_once("\n\n").ok_or_else(|| Error::msg("Invalid input"))?;

        // Parse grid manually to convert chars to Elements
        let lines: Vec<&str> = g_str.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut g = Vec::with_capacity(width * height);
        for line in lines {
            for c in line.chars() {
                g.push(match c {
                    '#' => Element::Wall,
                    'O' => Element::Box,
                    '@' => Element::Robot,
                    '[' => Element::BoxLeft,
                    ']' => Element::BoxRight,
                    _ => Element::Empty,
                });
            }
        }

        let moves = m_str.chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => Direction::Right,
            }).collect();

        Ok(Self { grid: Grid::from_vals(g, width, height), moves })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const SMALL_EXAMPLE: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const LARGE_EXAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_input_parsing() {
        let input = Input::from_str(SMALL_EXAMPLE).unwrap();
        let expected_grid = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########\n";

        assert_eq!(input.grid.to_string(), expected_grid);
        assert_eq!(
            input.moves,
            vec![
                Direction::Left,
                Direction::Up,
                Direction::Up,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Down,
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Left
            ]
        );
    }

    #[test]
    fn test_part1_small_example() {
        let input = Input::from_str(SMALL_EXAMPLE).unwrap();
        assert_eq!(part1(&input), 2028);
    }

    #[test]
    fn test_part1_large_example() {
        let input = Input::from_str(LARGE_EXAMPLE).unwrap();
        assert_eq!(part1(&input), 10092);
    }
}