//! A 2D grid utility optimized for Advent of Code.
//!
//! This module provides the [`Grid`] struct, which uses a flat `Vec<T>` for
//! memory efficiency and cache-friendliness. It uses [`Point`] for all
//! coordinate-based operations.

use crate::utils::point::Point;
use std::fmt::{self, Debug, Display};
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GridError {
    #[error("empty grid")]
    EmptyGrid,
    #[error("inconsistent row lengths")]
    Inconsistent,
}

/// A 2D grid stored in row-major order.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub g: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(fill: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Self {
            width,
            height,
            g: vec![fill; width * height],
        }
    }

    pub fn from_vals(vals: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(
            vals.len(),
            width * height,
            "Buffer size must match dimensions"
        );
        Self {
            width,
            height,
            g: vals,
        }
    }

    #[inline]
    pub fn in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.x < self.width as i32 && p.y >= 0 && p.y < self.height as i32
    }

    #[inline]
    fn to_idx(&self, p: Point) -> usize {
        (p.y as usize * self.width) + p.x as usize
    }

    pub fn find_pos(&self, f: impl Fn(&T) -> bool) -> Option<Point> {
        let ind = self.g.iter().position(f)?;
        Some(Point::new(
            (ind % self.width) as i32,
            (ind / self.width) as i32,
        ))
    }

    pub fn all_positions<'a>(
        &'a self,
        f: impl Fn(&T) -> bool + 'a,
    ) -> impl Iterator<Item = Point> + 'a {
        self.g.iter().enumerate().filter_map(move |(i, val)| {
            if f(val) {
                Some(Point::new((i % self.width) as i32, (i / self.width) as i32))
            } else {
                None
            }
        })
    }

    pub fn cardinal_neighbors(&self, p: Point) -> impl Iterator<Item = Point> + '_ {
        [Point::UP, Point::DOWN, Point::LEFT, Point::RIGHT]
            .into_iter()
            .map(move |dir| p + dir)
            .filter(move |&pos| self.in_bounds(pos))
    }

    pub fn all_neighbors(&self, p: Point) -> impl Iterator<Item = Point> + '_ {
        (-1..=1)
            .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .map(move |(dx, dy)| p + Point::new(dx, dy))
            .filter(move |&pos| self.in_bounds(pos))
    }
}

impl<T: Clone + Eq> Grid<T> {
    pub fn flood_fill<F>(&mut self, start_pos: Point, fill: T, is_blocked: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        if !self.in_bounds(start_pos) || is_blocked(&self[start_pos]) || self[start_pos] == fill {
            return 0;
        }

        let mut stack = vec![start_pos];
        let mut changed = 0;
        while let Some(pos) = stack.pop() {
            if self[pos] != fill && !is_blocked(&self[pos]) {
                self[pos] = fill.clone();
                changed += 1;
                for n in self.cardinal_neighbors(pos) {
                    stack.push(n);
                }
            }
        }
        changed
    }

    pub fn is_inside_polygon<F>(&self, pos: Point, boundary_fn: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        if boundary_fn(&self[pos]) {
            return false;
        }
        let mut crossings = 0;
        for x in pos.x..self.width as i32 {
            let p = Point::new(x, pos.y);
            if boundary_fn(&self[p]) {
                if pos.y > 0 && boundary_fn(&self[p + Point::UP]) {
                    crossings += 1;
                }
            }
        }
        crossings % 2 == 1
    }
}

impl<T: Clone> Grid<T> {
    pub fn ray_cast(&self, start: Point, dir: Point, steps: usize) -> Vec<T> {
        let mut elements = Vec::with_capacity(steps);
        let mut curr = start;

        for _ in 0..steps {
            if !self.in_bounds(curr) {
                break;
            }
            elements.push(self[curr].clone());
            curr += dir;
        }
        elements
    }

    pub fn rotate_right(&self) -> Self {
        let mut g = Vec::with_capacity(self.g.len());
        for x in 0..self.width {
            for y in (0..self.height).rev() {
                g.push(self[Point::new(x as i32, y as i32)].clone());
            }
        }
        Self::from_vals(g, self.height, self.width)
    }

    pub fn flip_lr(&self) -> Self {
        let g = self
            .g
            .chunks(self.width)
            .flat_map(|row| row.iter().rev().cloned())
            .collect();
        Self::from_vals(g, self.width, self.height)
    }

    pub fn expand(&self, fill: T) -> Self {
        let new_w = self.width + 2;
        let mut g = Vec::with_capacity(new_w * (self.height + 2));
        g.extend(std::iter::repeat(fill.clone()).take(new_w));
        for row in self.g.chunks(self.width) {
            g.push(fill.clone());
            g.extend(row.iter().cloned());
            g.push(fill.clone());
        }
        g.extend(std::iter::repeat(fill.clone()).take(new_w));
        Self::from_vals(g, new_w, self.height + 2)
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;
    #[inline]
    fn index(&self, p: Point) -> &Self::Output {
        &self.g[self.to_idx(p)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        let idx = self.to_idx(p);
        &mut self.g[idx]
    }
}

impl FromStr for Grid<char> {
    type Err = GridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Use a peekable iterator to avoid collecting into a Vec<&str> first
        let mut lines = s.lines().filter(|l| !l.is_empty()).peekable();

        let first_line = lines.peek().ok_or(GridError::EmptyGrid)?;
        let width = first_line.len();

        // Pre-allocate space if possible (estimation)
        let mut g = Vec::new();
        let mut height = 0;

        for line in lines {
            if line.len() != width {
                return Err(GridError::Inconsistent);
            }
            g.extend(line.chars());
            height += 1;
        }

        // Now height is explicitly tracked and g owns the chars
        Ok(Self { width, height, g })
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.g.chunks(self.width) {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// --- EXTENDED UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    fn rect_grid() -> Grid<char> {
        // 4x2 grid
        Grid::from_str("abcd\nefgh").unwrap()
    }

    #[test]
    fn test_initialization_and_metadata() {
        let g = rect_grid();
        assert_eq!(g.width, 4);
        assert_eq!(g.height, 2);
        assert_eq!(g.g.len(), 8);
    }

    #[test]
    fn test_indexing_edge_cases() {
        let g = rect_grid();
        assert_eq!(g[Point::new(0, 0)], 'a');
        assert_eq!(g[Point::new(3, 0)], 'd');
        assert_eq!(g[Point::new(0, 1)], 'e');
        assert_eq!(g[Point::new(3, 1)], 'h');
    }

    #[test]
    fn test_in_bounds_extremes() {
        let g = rect_grid();
        assert!(g.in_bounds(Point::new(0, 0)));
        assert!(g.in_bounds(Point::new(3, 1)));
        assert!(!g.in_bounds(Point::new(4, 1)));
        assert!(!g.in_bounds(Point::new(3, 2)));
        assert!(!g.in_bounds(Point::new(-1, 0)));
        assert!(!g.in_bounds(Point::new(0, -1)));
    }

    #[test]
    fn test_neighbor_bounds_logic() {
        let g = rect_grid();
        // Top-right corner (3,0)
        let neighbors: Vec<_> = g.cardinal_neighbors(Point::new(3, 0)).collect();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Point::new(2, 0))); // Left
        assert!(neighbors.contains(&Point::new(3, 1))); // Down

        // Edge point (1,0)
        let neighbors_8: Vec<_> = g.all_neighbors(Point::new(1, 0)).collect();
        assert_eq!(neighbors_8.len(), 5); // 2 on current row, 3 below
    }

    #[test]
    fn test_flood_fill_blocked_and_oob() {
        let mut g = Grid::from_str("....\n####\n....").unwrap();
        // Start in bounds
        assert_eq!(g.flood_fill(Point::new(0, 0), 'X', |&c| c == '#'), 4);
        // Start on blocked
        assert_eq!(g.flood_fill(Point::new(0, 1), 'X', |&c| c == '#'), 0);
        // Start out of bounds
        assert_eq!(g.flood_fill(Point::new(0, 10), 'X', |&c| c == '#'), 0);
    }

    #[test]
    fn test_ray_cast_directions() {
        let g = rect_grid();
        // Diagonal ray cast (if we allow it via Point)
        assert_eq!(
            g.ray_cast(Point::new(0, 0), Point::new(1, 1), 2),
            vec!['a', 'f']
        );
        // Ray cast that hits wall immediately
        assert_eq!(g.ray_cast(Point::new(0, 0), Point::UP, 5), vec!['a']);
        // Length 0 ray
        let empty_vec: Vec<char> = vec![];
        assert_eq!(g.ray_cast(Point::new(0, 0), Point::RIGHT, 0), empty_vec);
    }

    #[test]
    fn test_non_square_rotation() {
        let g = rect_grid(); // 4 wide, 2 high
        let rotated = g.rotate_right();
        assert_eq!(rotated.width, 2);
        assert_eq!(rotated.height, 4);
        assert_eq!(rotated[Point::new(0, 0)], 'e'); // Old (0,1)
        assert_eq!(rotated[Point::new(1, 3)], 'd'); // Old (3,0)
    }

    #[test]
    fn test_expand_with_different_types() {
        let g = Grid::new(1, 1, 1);
        let expanded = g.expand(0);
        assert_eq!(expanded.width, 3);
        assert_eq!(expanded.g, vec![0, 0, 0, 0, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_find_positions_none() {
        let g = rect_grid();
        assert_eq!(g.find_pos(|&c| c == 'z'), None);
        assert_eq!(g.all_positions(|&c| c == 'z').count(), 0);
    }

    #[test]
    fn test_parsing_malformed_input() {
        // Inconsistent rows
        let res = Grid::<char>::from_str("abc\ndefg");
        assert_eq!(res.err(), Some(GridError::Inconsistent));

        // Empty string
        let res = Grid::<char>::from_str("");
        assert_eq!(res.err(), Some(GridError::EmptyGrid));

        // Only newlines
        let res = Grid::<char>::from_str("\n\n");
        assert_eq!(res.err(), Some(GridError::EmptyGrid));
    }
}
