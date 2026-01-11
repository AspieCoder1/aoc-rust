use std::fmt::Debug;
use std::num::ParseIntError;
use std::ops::{Add, Index, IndexMut};
use std::str::FromStr;
use std::{fmt, io};
use thiserror::Error;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub g: Vec<T>,
}

#[derive(Error, Debug)]
pub enum GridError {
    #[error("empty grid")]
    EmptyGrid,
    #[error("inconsistent row lengths")]
    Inconsistent,
    #[error("{0}")]
    IOError(#[from] io::Error),
    #[error("{0}")]
    ParseError(#[from] ParseIntError),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos(pub usize, pub usize);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Pos {
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl Add<(isize, isize)> for Pos {
    type Output = Option<Pos>;
    fn add(self, (di, dj): (isize, isize)) -> Self::Output {
        let ni = self.0 as isize + di;
        let nj = self.1 as isize + dj;
        if ni >= 0 && nj >= 0 {
            Some(Pos(ni as usize, nj as usize))
        } else {
            None
        }
    }
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
        assert_eq!(vals.len(), width * height);
        Self {
            width,
            height,
            g: vals,
        }
    }

    pub fn in_bounds(&self, pos: Pos) -> bool {
        pos.0 < self.height && pos.1 < self.width
    }

    pub fn position(&self, f: impl Fn(&T) -> bool) -> Option<Pos> {
        let ind = self.g.iter().position(f)?;
        Some(Pos(ind / self.width, ind % self.width))
    }

    pub fn all_positions<'a>(
        &'a self,
        f: impl Fn(&T) -> bool + 'a,
    ) -> impl Iterator<Item = Pos> + 'a {
        self.g.iter().enumerate().filter_map(move |(i, val)| {
            if f(val) {
                Some(Pos(i / self.width, i % self.width))
            } else {
                None
            }
        })
    }

    pub fn swap(&mut self, a: Pos, b: Pos) {
        let idx_a = a.0 * self.width + a.1;
        let idx_b = b.0 * self.width + b.1;
        self.g.swap(idx_a, idx_b);
    }
}

impl<'a, T> Grid<T>
where
    T: Clone + 'a,
{
    pub fn from_iter<I>(it: I, width: usize, height: usize) -> Self
    where
        I: Iterator<Item = &'a T>,
    {
        Self::from_vals(it.cloned().collect(), width, height)
    }

    pub fn transpose(&self) -> Self {
        let mut g = Vec::with_capacity(self.g.len());
        for j in 0..self.width {
            for i in 0..self.height {
                g.push(self[Pos(i, j)].clone());
            }
        }
        Self {
            width: self.height,
            height: self.width,
            g,
        }
    }

    pub fn rotate_right(&self) -> Self {
        let mut g = Vec::with_capacity(self.g.len());
        for j in 0..self.width {
            for i in (0..self.height).rev() {
                g.push(self[Pos(i, j)].clone());
            }
        }
        Self {
            width: self.height,
            height: self.width,
            g,
        }
    }

    pub fn rotate_left(&self) -> Self {
        let mut g = Vec::with_capacity(self.g.len());
        for j in (0..self.width).rev() {
            for i in 0..self.height {
                g.push(self[Pos(i, j)].clone());
            }
        }
        Self {
            width: self.height,
            height: self.width,
            g,
        }
    }

    pub fn flip_lr(&self) -> Self {
        let mut g = Vec::with_capacity(self.g.len());
        for row in self.g.chunks(self.width) {
            for val in row.iter().rev() {
                g.push(val.clone());
            }
        }
        Self {
            width: self.width,
            height: self.height,
            g,
        }
    }

    pub fn flip_ud(&self) -> Self {
        let mut g = Vec::with_capacity(self.g.len());
        for i in (0..self.height).rev() {
            for j in 0..self.width {
                g.push(self[Pos(i, j)].clone());
            }
        }
        Self {
            width: self.width,
            height: self.height,
            g,
        }
    }

    pub fn expand(&self, fill: T) -> Self {
        let width = self.width + 2;
        let mut g = Vec::with_capacity(width * (self.height + 2));
        g.extend(std::iter::repeat_n(fill.clone(), width));
        for i in 0..self.height {
            g.push(fill.clone());
            g.extend(self.row(i).cloned());
            g.push(fill.clone());
        }
        g.extend(std::iter::repeat_n(fill.clone(), width));
        Self {
            width,
            height: self.height + 2,
            g,
        }
    }

    pub fn subgrid(&'a self, from_pos: Pos, to_pos: Pos) -> Self {
        Self::from_iter(
            self.subgrid_elements(from_pos, to_pos),
            to_pos.1 - from_pos.1 + 1,
            to_pos.0 - from_pos.0 + 1,
        )
    }

    pub fn subgrid_elements(&'a self, from_pos: Pos, to_pos: Pos) -> impl Iterator<Item = &'a T> {
        (from_pos.0..=to_pos.0)
            .flat_map(move |i| self.row(i).skip(from_pos.1).take(to_pos.1 - from_pos.1 + 1))
    }

    pub fn row(&self, i: usize) -> impl Iterator<Item = &T> {
        self.g.chunks(self.width).nth(i).unwrap().iter()
    }
    pub fn col(&self, j: usize) -> impl Iterator<Item = &T> {
        (0..self.height).map(move |i| &self[Pos(i, j)])
    }

    pub fn map<U, F>(&self, f: F) -> Grid<U>
    where
        F: Fn(&T) -> U,
    {
        Grid {
            width: self.width,
            height: self.height,
            g: self.g.iter().map(f).collect(),
        }
    }

    pub fn dfs_one_direction(
        &self,
        start_pos: Pos,
        direction: (isize, isize),
        n_elements: usize,
    ) -> Vec<T> {
        let (di, dj) = direction;
        let mut elements = Vec::with_capacity(n_elements);

        let mut curr_i = start_pos.0 as isize;
        let mut curr_j = start_pos.1 as isize;

        for _ in 0..n_elements {
            if curr_i < 0
                || curr_i >= self.height as isize
                || curr_j < 0
                || curr_j >= self.width as isize
            {
                break;
            }

            let pos = Pos(curr_i as usize, curr_j as usize);
            elements.push(self[pos].clone());

            curr_i += di;
            curr_j += dj;
        }
        elements
    }
}

impl<T: Clone + Eq> Grid<T> {
    pub fn flood_fill<F>(&mut self, start_pos: Pos, fill: T, is_blocked: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
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

    pub fn is_inside_polygon<F>(&self, pos: Pos, boundary_fn: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        if boundary_fn(&self[pos]) {
            return false;
        }
        let mut crossings = 0;
        for j in pos.1..self.width {
            let p = Pos(pos.0, j);
            if boundary_fn(&self[p])
                && (pos.0 == 0
                    || boundary_fn(&self[Pos(pos.0 - 1, j)])
                    || (pos.0 < self.height - 1 && boundary_fn(&self[Pos(pos.0 + 1, j)])))
            {
                crossings += 1;
            }
        }
        crossings % 2 == 1
    }
}

impl<T> Grid<T> {
    pub fn cardinal_neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |off| (pos + off).filter(|&p| self.in_bounds(p)))
    }
    pub fn all_neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .filter_map(move |off| (pos + off).filter(|&p| self.in_bounds(p)))
    }
}

impl<T: From<char>> FromStr for Grid<T> {
    type Err = GridError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_lines(s.lines())
    }
}

impl<T: From<char>> Grid<T> {
    pub fn from_lines<'a>(mut lines: impl Iterator<Item = &'a str>) -> Result<Self, GridError> {
        let first = lines.next().ok_or(GridError::EmptyGrid)?;
        let width = first.len();
        let mut g: Vec<T> = first.chars().map(T::from).collect();
        for line in lines {
            if line.is_empty() {
                continue;
            }
            if line.len() != width {
                return Err(GridError::Inconsistent);
            }
            g.extend(line.chars().map(T::from));
        }
        Ok(Self {
            width,
            height: g.len() / width,
            g,
        })
    }
}

impl<T: FromStr> Grid<T>
where
    GridError: From<T::Err>,
{
    pub fn from_space_sep(s: &str) -> Result<Self, GridError> {
        let mut g = Vec::new();
        let mut lines = s.lines().peekable();
        let width = lines
            .peek()
            .ok_or(GridError::EmptyGrid)?
            .split_whitespace()
            .count();
        for line in lines {
            let row: Vec<T> = line
                .split_whitespace()
                .map(|v| v.parse::<T>())
                .collect::<Result<_, _>>()?;
            if row.len() != width {
                return Err(GridError::Inconsistent);
            }
            g.extend(row);
        }
        Ok(Self {
            width,
            height: g.len() / width,
            g,
        })
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;
    fn index(&self, index: Pos) -> &Self::Output {
        &self.g[index.0 * self.width + index.1]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.g[index.0 * self.width + index.1]
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
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

impl<T: fmt::Display> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid {}x{}:", self.width, self.height)?;
        for row in self.g.chunks(self.width) {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// --- Original Unit Tests Preserved ---

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_grid() -> Grid<char> {
        Grid::from_lines("abc\ndef\nghi".lines()).unwrap()
    }

    #[test]
    fn test_cardinal_neighbors() {
        let g = sample_grid();
        let n: Vec<_> = g.cardinal_neighbors(Pos(0, 0)).collect();
        assert_eq!(n.len(), 2);
        assert!(n.contains(&Pos(1, 0)));
        assert!(n.contains(&Pos(0, 1)));
    }

    #[test]
    fn test_all_neighbors() {
        let g = sample_grid();
        let n: Vec<_> = g.all_neighbors(Pos(1, 1)).collect();
        assert_eq!(n.len(), 8);
    }

    #[test]
    fn test_grid_index() {
        let g = sample_grid();
        assert_eq!(g[Pos(0, 0)], 'a');
        assert_eq!(g[Pos(1, 1)], 'e');
    }

    #[test]
    fn test_transpose() {
        let g = sample_grid();
        let gt = g.transpose();
        assert_eq!(gt[Pos(0, 1)], 'd');
        assert_eq!(gt[Pos(1, 0)], 'b');
    }

    #[test]
    fn test_rotations() {
        let g = sample_grid();
        assert_eq!(g.rotate_right()[Pos(0, 0)], 'g');
        assert_eq!(g.rotate_left()[Pos(0, 0)], 'c');
    }

    #[test]
    fn test_flips() {
        let g = sample_grid();
        assert_eq!(g.flip_lr()[Pos(0, 0)], 'c');
        assert_eq!(g.flip_ud()[Pos(0, 0)], 'g');
    }

    #[test]
    fn test_flood_fill() {
        let mut g = Grid::from_lines("....\n.##.\n....".lines()).unwrap();
        g.flood_fill(Pos(0, 0), 'x', |&c| c == '#');
        assert_eq!(g[Pos(0, 0)], 'x');
        assert_eq!(g[Pos(1, 1)], '#');
    }

    #[test]
    fn test_subgrid() {
        let g = sample_grid();
        let sub = g.subgrid(Pos(0, 1), Pos(1, 2));
        assert_eq!(sub.width, 2);
        assert_eq!(sub.g, vec!['b', 'c', 'e', 'f']);
    }

    #[test]
    fn test_expand() {
        let g = sample_grid();
        let exp = g.expand('.');
        assert_eq!(exp.width, 5);
        assert_eq!(exp[Pos(1, 1)], 'a');
    }

    #[test]
    fn test_inside_polygon() {
        let g = Grid::<char>::from_lines("#####\n#...#\n#####".lines()).unwrap();
        assert!(g.is_inside_polygon(Pos(1, 1), |&c| c == '#'));
    }

    #[test]
    fn test_dfs_one_direction() {
        let g = Grid::<char>::from_lines("abc\ndef\nghi".lines()).unwrap();
        // Start at (0,0) 'a', go Down (1,0) for 3 elements
        let res = g.dfs_one_direction(Pos(0, 0), (1, 0), 3);
        assert_eq!(res, vec!['a', 'd', 'g']);

        // Start at (1,1) 'e', go Right (0,1) but expect it to hit the wall
        let res_limited = g.dfs_one_direction(Pos(1, 1), (0, 1), 5);
        assert_eq!(res_limited, vec!['e', 'f']);
    }
}
