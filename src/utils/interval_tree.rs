//! # Interval Tree Utility
//!
//! An augmented interval tree for $O(\log N)$ range-overlap and point queries.
//! Includes utilities for merging, subtracting, and deleting intervals.

use std::cmp::{max, min};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval<T> {
    pub low: T,
    pub high: T,
}

impl<T: Ord + Copy> Interval<T> {
    pub fn new(low: T, high: T) -> Self {
        Self { low, high }
    }

    /// Checks if this interval overlaps with another.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.low <= other.high && other.low <= self.high
    }

    /// Checks if a point is within the interval.
    pub fn contains(&self, p: T) -> bool {
        p >= self.low && p <= self.high
    }

    /// Returns the difference (self - other).
    /// Note: This is a discrete difference. For AoC puzzles (i32/usize),
    /// you may need to adjust the boundaries by +/- 1 depending on whether
    /// the intervals are inclusive or exclusive.
    pub fn difference(&self, other: &Self) -> Vec<Self> {
        if !self.overlaps(other) {
            return vec![*self];
        }

        let mut results = Vec::new();
        if self.low < other.low {
            results.push(Self::new(self.low, other.low));
        }
        if self.high > other.high {
            results.push(Self::new(other.high, self.high));
        }
        results
    }

    /// Merges a list of intervals into the smallest possible set of disjoint intervals.
    pub fn merge_all(mut intervals: Vec<Self>) -> Vec<Self> {
        if intervals.is_empty() {
            return Vec::new();
        }
        intervals.sort_unstable_by_key(|i| i.low);

        let mut merged = Vec::with_capacity(intervals.len());
        let mut current = intervals[0];

        for next in intervals.into_iter().skip(1) {
            if next.low <= current.high {
                current.high = max(current.high, next.high);
            } else {
                merged.push(current);
                current = next;
            }
        }
        merged.push(current);
        merged
    }
}

impl<T: Ord + Copy> From<RangeInclusive<T>> for Interval<T> {
    fn from(r: RangeInclusive<T>) -> Self {
        Self::new(*r.start(), *r.end())
    }
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    interval: Interval<T>,
    max_high: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy> Node<T> {
    fn new(interval: Interval<T>) -> Self {
        let high = interval.high;
        Node {
            interval,
            max_high: high,
            left: None,
            right: None,
        }
    }

    fn update_max_high(&mut self) {
        let mut m = self.interval.high;
        if let Some(ref l) = self.left { m = max(m, l.max_high); }
        if let Some(ref r) = self.right { m = max(m, r.max_high); }
        self.max_high = m;
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct IntervalTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy> IntervalTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Build a tree from a list of intervals, merging them first to ensure disjoint ranges.
    pub fn from_merged(intervals: Vec<Interval<T>>) -> Self {
        let merged = Interval::merge_all(intervals);
        merged.into_iter().collect()
    }

    pub fn insert(&mut self, low: T, high: T) {
        let interval = Interval::new(low, high);
        self.root = Self::insert_rec(self.root.take(), interval);
    }

    fn insert_rec(node: Option<Box<Node<T>>>, interval: Interval<T>) -> Option<Box<Node<T>>> {
        let mut n = match node {
            Some(n) => n,
            None => return Some(Box::new(Node::new(interval))),
        };

        if interval.low < n.interval.low {
            n.left = Self::insert_rec(n.left.take(), interval);
        } else {
            n.right = Self::insert_rec(n.right.take(), interval);
        }

        n.update_max_high();
        Some(n)
    }

    /// Removes a specific interval from the tree.
    pub fn delete(&mut self, low: T, high: T) {
        self.root = Self::delete_rec(self.root.take(), low, high);
    }

    fn delete_rec(node: Option<Box<Node<T>>>, low: T, high: T) -> Option<Box<Node<T>>> {
        let mut n = node?;

        if low < n.interval.low {
            n.left = Self::delete_rec(n.left.take(), low, high);
        } else if low > n.interval.low || n.interval.high != high {
            n.right = Self::delete_rec(n.right.take(), low, high);
        } else {
            if n.left.is_none() { return n.right; }
            if n.right.is_none() { return n.left; }

            let (successor_iv, new_right) = Self::pop_min(n.right.take().unwrap());
            n.interval = successor_iv;
            n.right = new_right;
        }

        n.update_max_high();
        Some(n)
    }

    fn pop_min(mut node: Box<Node<T>>) -> (Interval<T>, Option<Box<Node<T>>>) {
        if let Some(left) = node.left.take() {
            let (min_iv, new_left) = Self::pop_min(left);
            node.left = new_left;
            node.update_max_high();
            (min_iv, Some(node))
        } else {
            (node.interval, node.right)
        }
    }

    pub fn find_at_point(&self, p: T) -> Vec<Interval<T>> {
        self.find_all_overlapping(Interval::new(p, p))
    }

    pub fn find_all_overlapping(&self, query: Interval<T>) -> Vec<Interval<T>> {
        let mut results = Vec::new();
        Self::find_all_overlapping_rec(&self.root, query, &mut results);
        results
    }

    fn find_all_overlapping_rec(node: &Option<Box<Node<T>>>, query: Interval<T>, results: &mut Vec<Interval<T>>) {
        let n = match node {
            Some(n) if n.max_high >= query.low => n,
            _ => return,
        };
        if n.interval.overlaps(&query) {
            results.push(n.interval);
        }
        Self::find_all_overlapping_rec(&n.left, query, results);
        Self::find_all_overlapping_rec(&n.right, query, results);
    }
}

impl<T: Ord + Copy> FromIterator<Interval<T>> for IntervalTree<T> {
    fn from_iter<I: IntoIterator<Item = Interval<T>>>(iter: I) -> Self {
        let mut tree = Self::new();
        for interval in iter {
            tree.insert(interval.low, interval.high);
        }
        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_basics() {
        let iv = Interval::new(10, 20);
        assert!(iv.contains(15));
        assert!(iv.overlaps(&Interval::new(15, 25)));
        assert!(!iv.overlaps(&Interval::new(25, 30)));
    }

    #[test]
    fn test_merge_all() {
        let ivs = vec![
            Interval::new(1, 5),
            Interval::new(10, 15),
            Interval::new(3, 7),
            Interval::new(20, 25),
            Interval::new(-5, 2),
        ];
        let merged = Interval::merge_all(ivs);
        // Expected: [-5, 7], [10, 15], [20, 25]
        assert_eq!(merged.len(), 3);
        assert_eq!(merged[0], Interval::new(-5, 7));
    }

    #[test]
    fn test_difference() {
        let base = Interval::new(10, 20);
        // Internal cut
        assert_eq!(base.difference(&Interval::new(12, 18)).len(), 2);
        // Complete overlap
        assert_eq!(base.difference(&Interval::new(0, 30)).len(), 0);
        // Partial left
        let left = base.difference(&Interval::new(5, 15));
        assert_eq!(left, vec![Interval::new(15, 20)]);
    }

    #[test]
    fn test_tree_insertion_and_search() {
        let mut tree = IntervalTree::new();
        tree.insert(15, 20);
        tree.insert(10, 30);
        tree.insert(17, 19);
        tree.insert(5, 20);

        let results = tree.find_all_overlapping(Interval::new(14, 16));
        // Finds: 15-20, 10-30, 5-20
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_deletion_integrity() {
        let mut tree = IntervalTree::new();
        tree.insert(10, 20);
        tree.insert(5, 30); // Sets max_high to 30
        tree.insert(15, 25);

        assert_eq!(tree.root.as_ref().unwrap().max_high, 30);

        // Delete the node providing the max_high
        tree.delete(5, 30);

        // Root's max_high should drop to 25
        assert_eq!(tree.root.as_ref().unwrap().max_high, 25);
        assert_eq!(tree.find_at_point(28).len(), 0);
    }

    #[test]
    fn test_delete_root_two_children() {
        let mut tree = IntervalTree::new();
        tree.insert(10, 20); // Root
        tree.insert(5, 5);   // Left
        tree.insert(20, 25); // Right

        tree.delete(10, 20);

        // Successor (20, 25) should move to root
        assert_eq!(tree.root.as_ref().unwrap().interval, Interval::new(20, 25));
    }

    #[test]
    fn test_point_intervals_and_negatives() {
        let tree: IntervalTree<_> = vec![
            Interval::new(-10, -10),
            Interval::new(0, 0),
            Interval::new(10, 10),
        ].into_iter().collect();

        assert_eq!(tree.find_at_point(-10).len(), 1);
        assert_eq!(tree.find_at_point(0).len(), 1);
        assert_eq!(tree.find_at_point(-5).len(), 0);
    }
}