use std::collections::HashMap;
use std::hash::Hash;

/// Fast, index-based Disjoint Set Union (DSU) with Path Compression and Union by Size.
#[derive(Debug, Clone)]
pub struct DisjointSet {
    pub(crate) parent: Vec<usize>,
    pub(crate) size: Vec<usize>,
    pub num_sets: usize,
}

impl DisjointSet {
    /// Creates a new DSU with `n` elements, each in its own set.
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_sets: n,
        }
    }

    /// Dynamically adds a new element in its own set.
    pub fn push(&mut self) -> usize {
        let new_idx = self.parent.len();
        self.parent.push(new_idx);
        self.size.push(1);
        self.num_sets += 1;
        new_idx
    }

    /// Finds the representative (root) of the set containing `i` with full path compression.
    pub fn find(&mut self, i: usize) -> usize {
        let mut root = i;
        while self.parent[root] != root {
            root = self.parent[root];
        }

        // Path compression: make all nodes in path point to root
        let mut curr = i;
        while self.parent[curr] != root {
            let next = self.parent[curr];
            self.parent[curr] = root;
            curr = next;
        }
        root
    }

    /// Unions the sets containing `i` and `j`. Returns true if they were merged.
    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            self.num_sets -= 1;
            return true;
        }
        false
    }

    pub fn size_of(&mut self, i: usize) -> usize {
        let root = self.find(i);
        self.size[root]
    }
}

/// A wrapper around DisjointSet that allows using any Hashable type (Strings, Points, etc.)
pub struct MappingDisjointSet<T>
where
    T: Eq + Hash + Clone,
{
    inner: DisjointSet,
    mapping: HashMap<T, usize>,
    reverse_mapping: Vec<T>,
}

impl<T> MappingDisjointSet<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            inner: DisjointSet::new(0),
            mapping: HashMap::new(),
            reverse_mapping: Vec::new(),
        }
    }

    pub fn get_index(&mut self, item: &T) -> usize {
        if let Some(&idx) = self.mapping.get(item) {
            idx
        } else {
            let idx = self.inner.push();
            self.mapping.insert(item.clone(), idx);
            self.reverse_mapping.push(item.clone());
            idx
        }
    }

    pub fn union(&mut self, a: T, b: T) -> bool {
        let idx_a = self.get_index(&a);
        let idx_b = self.get_index(&b);
        self.inner.union(idx_a, idx_b)
    }

    pub fn is_connected(&mut self, a: &T, b: &T) -> bool {
        let idx_a = self.get_index(a);
        let idx_b = self.get_index(b);
        self.inner.find(idx_a) == self.inner.find(idx_b)
    }

    pub fn num_sets(&self) -> usize {
        self.inner.num_sets
    }

    pub fn get_all_sets(&mut self) -> Vec<Vec<T>> {
        let mut groups: HashMap<usize, Vec<T>> = HashMap::new();
        for i in 0..self.reverse_mapping.len() {
            let root = self.inner.find(i);
            groups
                .entry(root)
                .or_default()
                .push(self.reverse_mapping[i].clone());
        }
        groups.into_values().collect()
    }
}

// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_initial_state() {
        let mut dsu = DisjointSet::new(5);
        assert_eq!(dsu.num_sets, 5);
        for i in 0..5 {
            assert_eq!(dsu.find(i), i);
            assert_eq!(dsu.size_of(i), 1);
        }
    }

    #[test]
    fn test_simple_union() {
        let mut dsu = DisjointSet::new(10);
        assert!(dsu.union(0, 1));
        assert!(dsu.union(2, 3));
        assert_eq!(dsu.find(0), dsu.find(1));
        assert_eq!(dsu.find(2), dsu.find(3));
        assert_ne!(dsu.find(0), dsu.find(2));
        assert_eq!(dsu.num_sets, 8);
    }

    #[test]
    fn test_redundant_union() {
        let mut dsu = DisjointSet::new(5);
        dsu.union(0, 1);
        dsu.union(1, 2);
        assert!(!dsu.union(0, 2));
        assert_eq!(dsu.num_sets, 3);
    }

    #[test]
    fn test_size_tracking() {
        let mut dsu = DisjointSet::new(10);
        dsu.union(0, 1);
        dsu.union(1, 2);
        dsu.union(3, 4);
        assert_eq!(dsu.size_of(0), 3);
        assert_eq!(dsu.size_of(3), 2);
        dsu.union(0, 3);
        assert_eq!(dsu.size_of(0), 5);
    }

    #[test]
    fn test_path_compression_effect() {
        let mut dsu = DisjointSet::new(5);
        // Manually create chain: 4 -> 3 -> 2 -> 1 -> 0
        dsu.parent[4] = 3;
        dsu.parent[3] = 2;
        dsu.parent[2] = 1;
        dsu.parent[1] = 0;

        assert_eq!(dsu.parent[4], 3);
        let root = dsu.find(4);
        assert_eq!(root, 0);
        // After compression, 4 points directly to 0
        assert_eq!(dsu.parent[4], 0);
        assert_eq!(dsu.parent[3], 0);
        assert_eq!(dsu.parent[2], 0);
    }

    #[test]
    fn test_mapping_dsu() {
        let mut dsu = MappingDisjointSet::new();
        dsu.union("Alice", "Bob");
        dsu.union("Charlie", "David");
        dsu.union("Bob", "Charlie");

        assert!(dsu.is_connected(&"Alice", &"David"));
        assert_eq!(dsu.num_sets(), 1);

        let all_sets = dsu.get_all_sets();
        assert_eq!(all_sets.len(), 1);
        assert_eq!(all_sets[0].len(), 4);
    }

    #[test]
    fn test_dynamic_push() {
        let mut dsu = DisjointSet::new(2);
        dsu.union(0, 1);
        let idx = dsu.push(); // Adds index 2
        assert_eq!(dsu.num_sets, 2);
        assert_eq!(dsu.size_of(idx), 1);
        dsu.union(0, idx);
        assert_eq!(dsu.num_sets, 1);
        assert_eq!(dsu.size_of(idx), 3);
    }
}