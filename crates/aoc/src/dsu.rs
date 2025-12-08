/// A Disjoint Set Union (Union-Find) structure using 0..n indices.
///
/// Uses path compression and union by size (attach smaller tree under larger).
#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    /// Create a `UnionFind` with `n` elements (0..n-1), each in its own set.
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    /// Returns the number of elements.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.parent.len()
    }

    /// Returns true if no elements.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }

    /// Find the representative (root) of `mut x` with path compression.
    ///
    /// # Panics
    /// Panics if `x` is out of bounds.
    pub fn find(&mut self, mut x: usize) -> usize {
        let n = self.len();
        assert!(x < n, "index out of bounds in UnionFind::find");

        // Find root
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }

        // Path compression
        while self.parent[x] != x {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }

        root
    }

    /// Returns true if `a` and `b` are in the same set.
    ///
    /// # Panics
    /// Panics if indices out of bounds.
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// Unite the sets containing `a` and `b`. Returns `true` if they were separate (merge happened).
    ///
    /// # Panics
    /// Panics if indices out of bounds.
    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return false;
        }

        // Union by size: ensure ra is the larger root
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        // Attach rb under ra
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.components -= 1;
        true
    }

    /// Size of the set containing `x`.
    ///
    /// # Panics
    /// Panics if `x` out of bounds.
    pub fn set_size(&mut self, x: usize) -> usize {
        let r = self.find(x);
        self.size[r]
    }

    /// Number of connected components (disjoint sets)
    #[must_use]
    pub const fn component_count(&self) -> usize {
        self.components
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn basic_union_find() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.component_count(), 5);
        assert!(!uf.same(0, 1));
        assert!(uf.unite(0, 1));
        assert!(uf.same(0, 1));
        assert_eq!(uf.set_size(0), 2);
        assert_eq!(uf.component_count(), 4);

        // idempotent union returns false
        assert!(!uf.unite(0, 1));

        // connect remaining
        uf.unite(1, 2);
        uf.unite(3, 4);
        assert_eq!(uf.component_count(), 2);
        assert!(uf.same(0, 2));
        assert!(uf.same(3, 4));
        assert!(!uf.same(2, 3));
    }

    #[test]
    #[should_panic]
    fn find_out_of_bounds() {
        let mut uf = UnionFind::new(3);
        let _ = uf.find(10);
    }
}
