//! Advent of Code 2024 - Day 15
//!
//! Link: <https://adventofcode.com/2024/day/15>

use crate::utils::grid::{Grid, Pos};
use anyhow::{Error, Result};
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::str::FromStr;

pub fn main(input: &str) -> Result<(usize, usize)> {
    let input = Input::from_str(input)?;
    Ok((part1(&input), part2(&input)))
}

fn part1(input: &Input) -> usize {
    let mut grid = input.grid.clone();
    let mut robot = grid
        .position(|&el| el == Element::Robot)
        .expect("No robot found");

    for &dir in &input.moves {
        let next = robot.add_dir(dir);
        match grid[next] {
            Element::Empty => {
                grid.swap(robot, next);
                robot = next;
            }
            Element::Wall => continue,
            Element::Box => {
                let mut scan = next;
                while grid[scan] == Element::Box {
                    scan = scan.add_dir(dir);
                }
                if grid[scan] == Element::Empty {
                    grid.swap(scan, next);
                    grid.swap(next, robot);
                    robot = next;
                }
            }
            _ => unreachable!(),
        }
    }
    score(&grid, Element::Box)
}

fn part2(input: &Input) -> usize {
    let mut grid = expand_grid(&input.grid);
    let mut robot = grid
        .position(|&el| el == Element::Robot)
        .expect("No robot found");

    for &dir in &input.moves {
        let mut affected = HashSet::new();
        if can_move(&grid, robot, dir, &mut affected) {
            // Sort affected positions so we move from the "front" of the push backwards
            let mut sorted: Vec<Pos> = affected.into_iter().collect();
            sorted.sort_by_key(|p| match dir {
                Direction::Up => p.0 as isize,
                Direction::Down => -(p.0 as isize),
                Direction::Left => p.1 as isize,
                Direction::Right => -(p.1 as isize),
            });

            for pos in sorted {
                let target = pos.add_dir(dir);
                grid[target] = grid[pos];
                grid[pos] = Element::Empty;
            }
            robot = robot.add_dir(dir);
        }
    }
    score(&grid, Element::BoxLeft)
}

fn can_move(grid: &Grid<Element>, pos: Pos, dir: Direction, seen: &mut HashSet<Pos>) -> bool {
    if !seen.insert(pos) {
        return true;
    }

    let next = pos.add_dir(dir);
    match grid[next] {
        Element::Empty => true,
        Element::Wall => false,
        Element::Box => can_move(grid, next, dir, seen),
        Element::BoxLeft | Element::BoxRight => {
            let mut possible = can_move(grid, next, dir, seen);
            // In Part 2, vertical moves must also check the "other half" of the wide box
            if matches!(dir, Direction::Up | Direction::Down) {
                let other_half = if grid[next] == Element::BoxLeft {
                    next.add_dir(Direction::Right)
                } else {
                    next.add_dir(Direction::Left)
                };
                possible &= can_move(grid, other_half, dir, seen);
            }
            possible
        }
        _ => true,
    }
}

fn score(grid: &Grid<Element>, target: Element) -> usize {
    grid.all_positions(|&el| el == target)
        .map(|Pos(y, x)| 100 * y + x)
        .sum()
}

fn expand_grid(old: &Grid<Element>) -> Grid<Element> {
    let mut g = Vec::with_capacity(old.g.len() * 2);
    for &el in &old.g {
        match el {
            Element::Wall => {
                g.push(Element::Wall);
                g.push(Element::Wall);
            }
            Element::Box => {
                g.push(Element::BoxLeft);
                g.push(Element::BoxRight);
            }
            Element::Robot => {
                g.push(Element::Robot);
                g.push(Element::Empty);
            }
            Element::Empty => {
                g.push(Element::Empty);
                g.push(Element::Empty);
            }
            _ => unreachable!(),
        }
    }
    Grid {
        width: old.width * 2,
        height: old.height,
        g,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Wall,
    Empty,
    Robot,
    Box,
    BoxLeft,
    BoxRight,
}

impl Element {
    fn to_char(self) -> char {
        match self {
            Self::Wall => '#',
            Self::Empty => '.',
            Self::Robot => '@',
            Self::Box => 'O',
            Self::BoxLeft => '[',
            Self::BoxRight => ']',
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl From<char> for Element {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '@' => Self::Robot,
            'O' => Self::Box,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            _ => Self::Empty,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => Self::Right,
        }
    }
}

impl Pos {
    fn add_dir(&self, dir: Direction) -> Self {
        let (di, dj) = dir.offset();
        Pos(
            (self.0 as isize + di) as usize,
            (self.1 as isize + dj) as usize,
        )
    }
}

struct Input {
    grid: Grid<Element>,
    moves: Vec<Direction>,
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (g_str, m_str) = s
            .split_once("\n\n")
            .ok_or_else(|| Error::msg("Invalid input"))?;
        let grid = Grid::<Element>::from_lines(g_str.lines())?;
        let moves = m_str
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(Direction::from)
            .collect();
        Ok(Self { grid, moves })
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
