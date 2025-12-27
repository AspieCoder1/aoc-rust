#[allow(unused)]
#[derive(Debug)]
pub(crate) struct Node<T> {
    data: T,
    parent: usize,
    pub(crate) size: usize,
}

impl<T> Node<T> {
    fn new(data: T, ind: usize) -> Self {
        Self {
            data,
            parent: ind,
            size: 1,
        }
    }
}

#[derive(Debug)]
pub(crate) struct DisjointSet<T> {
    pub(crate) nodes: Vec<Node<T>>,
    size: usize,
}

impl<T> FromIterator<T> for DisjointSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for item in iter {
            set.add_node(item);
        }
        set
    }
}

impl<T> DisjointSet<T> {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            size: 0,
        }
    }

    fn add_node(&mut self, data: T) -> usize {
        let i = self.nodes.len();
        self.nodes.push(Node::new(data, i));
        self.size += 1;
        i
    }

    pub(crate) fn find(&mut self, mut x: usize) -> usize {
        while self.nodes[x].parent != x {
            let parent = self.nodes[x].parent;
            (x, self.nodes[x].parent) = (parent, self.nodes[parent].parent);
        }
        x
    }

    pub(crate) fn union(&mut self, mut x: usize, mut y: usize) -> usize {
        x = self.find(x);
        y = self.find(y);

        if x != y {
            if self.nodes[x].size < self.nodes[y].size {
                (x, y) = (y, x);
            }
            self.nodes[y].parent = x;
            self.nodes[x].size += self.nodes[y].size;
        }
        self.nodes[x].size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn sample_disjoint_set() -> DisjointSet<usize> {
        DisjointSet::from_iter(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn test_find() {
        let mut union_find = sample_disjoint_set();

        assert_eq!(union_find.find(0), 0);
        assert_eq!(union_find.find(1), 1);
    }

    #[test]
    fn test_union() {
        let mut union_find = sample_disjoint_set();
        union_find.union(0, 1);

        assert_eq!(union_find.find(0), 0);
        assert_eq!(union_find.find(1), 0);
    }
}
