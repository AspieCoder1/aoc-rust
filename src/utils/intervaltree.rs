//! # Interval tree
//!
//! Implementation of an augmented interval tree

use num_traits::{Bounded, ConstOne};
use std::cmp::max;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval<T> {
    pub low: T,
    pub high: T,
}

impl<T: Ord + Copy + Bounded + Sub<Output = T> + ConstOne + Add<Output = T>> Interval<T> {
    pub fn size(&self) -> T {
        self.high - self.low + T::one()
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    interval: Interval<T>,
    max_high: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy + Bounded> Node<T> {
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
        let left_max = self.left.as_ref().map_or(T::min_value(), |n| n.max_high);
        let right_max = self.right.as_ref().map_or(T::min_value(), |n| n.max_high);
        self.max_high = max(self.max_high, max(left_max, right_max));
    }
}

#[derive(Debug, PartialEq)]
pub struct IntervalTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy + Bounded> IntervalTree<T> {
    /// Create a new empty interval tree
    pub fn new() -> Self {
        IntervalTree { root: None }
    }

    /// Insert a new interval into the tree
    pub fn insert(&mut self, low: T, high: T) {
        let interval = Interval { low, high };
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

    /// Delete an interval from the tree
    pub fn delete(&mut self, low: T, high: T) {
        self.root = Self::delete_rec(self.root.take(), low, high);
    }

    fn delete_rec(node: Option<Box<Node<T>>>, low: T, high: T) -> Option<Box<Node<T>>> {
        let mut n = match node {
            None => return None,
            Some(n) => n,
        };

        if low < n.interval.low {
            n.left = Self::delete_rec(n.left.take(), low, high);
        } else if low > n.interval.low || n.interval.high != high {
            n.right = Self::delete_rec(n.right.take(), low, high);
        } else {
            // Case 1 & 2: 0 or 1 child
            if n.left.is_none() {
                return n.right;
            }
            if n.right.is_none() {
                return n.left;
            }

            // Case 3: 2 children - replace with inorder successor
            let (successor_iv, new_right) = Self::pop_min(n.right.take().unwrap());
            n.interval = successor_iv;
            n.right = new_right;
        }

        // Critical: Update augmentation after structure change
        n.update_max_high();
        Some(n)
    }

    fn pop_min(mut node: Box<Node<T>>) -> (Interval<T>, Option<Box<Node<T>>>) {
        if let Some(left) = node.left.take() {
            let (min_iv, new_left) = Self::pop_min(left);
            node.left = new_left;
            node.update_max_high(); // Update max while recursing back up
            (min_iv, Some(node))
        } else {
            (node.interval, node.right)
        }
    }

    /// Find all intervals that contain a given point
    pub fn find_at_point(&self, p: T) -> Vec<Interval<T>> {
        let mut results = Vec::new();
        Self::query_rec(&self.root, p, &mut results);
        results
    }

    fn query_rec(node: &Option<Box<Node<T>>>, p: T, results: &mut Vec<Interval<T>>) {
        let n = match node {
            Some(n) if n.max_high >= p => n,
            _ => return,
        };

        if n.interval.low <= p && n.interval.high >= p {
            results.push(n.interval);
        }

        if n.left.is_some() {
            Self::query_rec(&n.left, p, results);
        }
        if n.right.is_some() && p >= n.interval.low {
            Self::query_rec(&n.right, p, results);
        }
    }

    /// Find all intervals that end before a given point
    pub fn find_all_before(&self, p: T) -> Vec<Interval<T>> {
        let mut results = Vec::new();
        Self::find_all_before_rec(&self.root, p, &mut results);
        results
    }

    fn find_all_before_rec(node: &Option<Box<Node<T>>>, p: T, results: &mut Vec<Interval<T>>) {
        let n = match node {
            Some(n) => n,
            _ => return,
        };

        Self::find_all_before_rec(&n.left, p, results);
        if n.interval.high < p {
            results.push(n.interval);
            Self::find_all_before_rec(&n.right, p, results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_insert() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 3);

        assert_eq!(
            tree,
            IntervalTree {
                root: Some(Box::new(Node {
                    interval: Interval { low: 1, high: 3 },
                    max_high: 3,
                    left: None,
                    right: None
                }))
            }
        );
    }

    #[test]
    fn test_delete() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 3);
        tree.delete(1, 3);
        assert_eq!(tree, IntervalTree::new());
    }

    #[test]
    fn test_find_at_point() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 3);
        tree.insert(5, 6);
        assert_eq!(tree.find_at_point(2), vec![Interval { low: 1, high: 3 }]);
    }

    #[test]
    fn test_find_all_before() {
        let mut tree = IntervalTree::new();
        tree.insert(1, 3);
        tree.insert(5, 6);
        tree.insert(10, 11);
        assert_eq!(
            tree.find_all_before(7),
            vec![Interval { low: 1, high: 3 }, Interval { low: 5, high: 6 }]
        );
    }
}
