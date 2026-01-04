//! Implementation of a 2-dimensional point in rust which can be used with the [`Grid`] struct.
//!
//! [`Grid`]: crate::utils::grid::Grid
use num::integer::Roots;
use std::cmp::max;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul, Rem, RemAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[allow(unused)]
    fn euclidean_distance(&self, other: &Self) -> i32 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2)).sqrt()
    }

    #[allow(unused)]
    fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[allow(unused)]
    fn chebyshev_distance(&self, other: &Self) -> i32 {
        max((self.x - other.x).abs(), (self.y - other.y).abs())
    }
}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, other: i32) -> Self::Output {
        Self::new(self.x * other, self.y * other)
    }
}

impl Rem<i32> for Point {
    type Output = Self;
    fn rem(self, other: i32) -> Self::Output {
        Self::new(self.x % other, self.y % other)
    }
}

impl RemAssign<i32> for Point {
    fn rem_assign(&mut self, other: i32) {
        self.x %= other;
        self.y %= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new() {
        assert_eq!(Point::new(1, 2), Point { x: 1, y: 2 });
    }

    #[test]
    fn test_distance() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);

        assert_eq!(p1.euclidean_distance(&p2), 2);
        assert_eq!(p1.manhattan_distance(&p2), 4);
        assert_eq!(p1.chebyshev_distance(&p2), 2);
    }

    #[test]
    fn test_add_assign() {
        let mut p1 = Point::new(1, 2);
        p1 += Point::new(3, 4);
        assert_eq!(p1, Point::new(4, 6));
    }

    #[test]
    fn test_sub_assign() {
        let mut p1 = Point::new(1, 2);
        p1 -= Point::new(3, 4);
        assert_eq!(p1, Point::new(-2, -2));
    }
}
