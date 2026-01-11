//! A 2D coordinate utility for grid-based puzzles.
//!
//! Optimized for a y-down coordinate system (standard in grids).
//! Provides vector arithmetic and rotation logic.

use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// A point or vector in 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    // Standard cardinal directions
    pub const UP: Point = Point { x: 0, y: -1 };
    pub const DOWN: Point = Point { x: 0, y: 1 };
    pub const LEFT: Point = Point { x: -1, y: 0 };
    pub const RIGHT: Point = Point { x: 1, y: 0 };

    /// Creates a new point.
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns the Manhattan distance between two points.
    /// Uses `abs_diff` to ensure correct results without overflow during subtraction.
    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// Returns the squared Euclidean distance between two points.
    ///
    /// Useful for comparing distances without the overhead of a square root.
    /// Formula: $d^2 = (x_1 - x_2)^2 + (y_1 - y_2)^2$
    pub fn euclidean_squared(&self, other: &Self) -> u32 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx * dx + dy * dy
    }

    /// Returns the Euclidean distance between two points as a float.
    ///
    /// Formula: $d = \sqrt{(x_1 - x_2)^2 + (y_1 - y_2)^2}$
    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        (self.euclidean_squared(other) as f64).sqrt()
    }

    /// Rotates the vector 90 degrees clockwise (in a y-down system).
    ///
    /// Formula: (x, y) -> (-y, x)
    #[inline]
    pub fn rotate_right_90(&self) -> Self {
        Point::new(-self.y, self.x)
    }

    /// Rotates the vector 90 degrees counter-clockwise (in a y-down system).
    ///
    /// Formula: (x, y) -> (y, -x)
    #[inline]
    pub fn rotate_left_90(&self) -> Self {
        Point::new(self.y, -self.x)
    }

    /// Rotates the vector 180 degrees.
    #[inline]
    pub fn reverse(&self) -> Self {
        Point::new(-self.x, -self.y)
    }

    /// Returns an iterator of all integer points on a straight line between
    /// self and other (inclusive). Handles horizontal, vertical, and 45-degree lines.
    pub fn points_between(&self, other: Point) -> Vec<Point> {
        let mut points = Vec::new();

        // Calculate the step direction for both axes (-1, 0, or 1)
        let dx = (other.x - self.x).signum();
        let dy = (other.y - self.y).signum();

        let mut curr = *self;
        points.push(curr);

        while curr != other {
            curr.x += dx;
            curr.y += dy;
            points.push(curr);
        }

        points
    }

    pub fn wrap(&self, width: i32, height: i32) -> Self {
        Point::new(
            ((self.x % width) + width) % width,
            ((self.y % height) + height) % height,
        )
    }
}

// --- Operator Overloads ---

impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, scalar: i32) -> Self {
        Point::new(self.x * scalar, self.y * scalar)
    }
}

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Point::new(5, -3);
        assert_eq!(p.x, 5);
        assert_eq!(p.y, -3);
    }

    #[test]
    fn test_arithmetic() {
        let mut p1 = Point::new(10, 20);
        let p2 = Point::new(5, 5);

        assert_eq!(p1 + p2, Point::new(15, 25));
        assert_eq!(p1 - p2, Point::new(5, 15));

        p1 += p2;
        assert_eq!(p1, Point::new(15, 25));
        p1 -= p2;
        assert_eq!(p1, Point::new(10, 20));
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(4, 5);
        // |1-4| + |1-5| = 3 + 4 = 7
        assert_eq!(p1.manhattan_distance(&p2), 7);

        // Test with negative coordinates
        let p3 = Point::new(-2, -2);
        let p4 = Point::new(2, 2);
        // |(-2)-2| + |(-2)-2| = 4 + 4 = 8
        assert_eq!(p3.manhattan_distance(&p4), 8);
    }

    #[test]
    fn test_rotation_clockwise_cycle() {
        let mut dir = Point::UP;

        dir = dir.rotate_right_90();
        assert_eq!(dir, Point::RIGHT, "Up -> Right");

        dir = dir.rotate_right_90();
        assert_eq!(dir, Point::DOWN, "Right -> Down");

        dir = dir.rotate_right_90();
        assert_eq!(dir, Point::LEFT, "Down -> Left");

        dir = dir.rotate_right_90();
        assert_eq!(dir, Point::UP, "Left -> Up");
    }

    #[test]
    fn test_rotation_counter_clockwise_cycle() {
        let mut dir = Point::UP;

        dir = dir.rotate_left_90();
        assert_eq!(dir, Point::LEFT, "Up -> Left");

        dir = dir.rotate_left_90();
        assert_eq!(dir, Point::DOWN, "Left -> Down");

        dir = dir.rotate_left_90();
        assert_eq!(dir, Point::RIGHT, "Down -> Right");

        dir = dir.rotate_left_90();
        assert_eq!(dir, Point::UP, "Right -> Up");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(Point::UP.reverse(), Point::DOWN);
        assert_eq!(Point::LEFT.reverse(), Point::RIGHT);
        assert_eq!(Point::new(5, -10).reverse(), Point::new(-5, 10));
    }

    #[test]
    fn test_zero_vector() {
        let zero = Point::new(0, 0);
        assert_eq!(zero.rotate_right_90(), zero);
        assert_eq!(zero.manhattan_distance(&zero), 0);
        assert_eq!(zero + Point::UP, Point::UP);
    }

    #[test]
    fn test_euclidean_distance() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 4);

        // 3-4-5 triangle
        assert_eq!(p1.euclidean_squared(&p2), 25);
        assert_eq!(p1.euclidean_distance(&p2), 5.0);

        // Test with negatives
        let p3 = Point::new(-1, -1);
        let p4 = Point::new(1, 1);
        // dist squared = (2^2) + (2^2) = 8
        assert_eq!(p3.euclidean_squared(&p4), 8);
    }

    #[test]
    fn test_points_between_horizontal() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(4, 1);
        let pts = p1.points_between(p2);
        assert_eq!(pts.len(), 4);
        assert_eq!(pts[1], Point::new(2, 1));
        assert_eq!(pts[3], Point::new(4, 1));
    }

    #[test]
    fn test_points_between_vertical() {
        let p1 = Point::new(2, 5);
        let p2 = Point::new(2, 2); // Testing backwards
        let pts = p1.points_between(p2);
        assert_eq!(pts.len(), 4);
        assert_eq!(pts[1], Point::new(2, 4));
        assert_eq!(pts[3], Point::new(2, 2));
    }

    #[test]
    fn test_points_between_diagonal() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(2, 2);
        let pts = p1.points_between(p2);
        // Should catch (0,0), (1,1), (2,2)
        assert_eq!(pts.len(), 3);
        assert_eq!(pts[1], Point::new(1, 1));
    }

    #[test]
    fn test_arithmetic_operators() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(5, -5);

        // Test Add
        assert_eq!(p1 + p2, Point::new(15, 15));

        // Test Mul (Scalar)
        assert_eq!(p1 * 3, Point::new(30, 60));
        assert_eq!(p2 * -2, Point::new(-10, 10));
    }

    #[test]
    fn test_wrap_logic() {
        // Test wrapping positive out-of-bounds
        let p1 = Point::new(105, 10);
        assert_eq!(p1.wrap(100, 100), Point::new(5, 10));

        // Test wrapping negative coordinates (the tricky part of modulo)
        let p2 = Point::new(-1, -5);
        assert_eq!(p2.wrap(100, 100), Point::new(99, 95));

        // Test exact boundary
        let p3 = Point::new(100, 100);
        assert_eq!(p3.wrap(100, 100), Point::new(0, 0));
    }
}
