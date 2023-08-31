use std::vec::Vec;

pub struct UnionFind {
    pub parents: Vec<usize>,
    pub length: usize,
}

// Implementation of the Union Find algorithm to obtain all the connected duplicates
impl UnionFind {
    pub fn new(length: usize) -> Self {
        Self {
            parents: (0..length).collect(),
            length: length,
        }
    }

    // find the parent of a node
    // after finding the uppermost parent, we set the direct parent of x, to that parent
    // so we widen the tree and subsequent finds will be much faster (only one jump)
    // doing mutable self because it's called from union, who has mutable self
    pub fn find(&mut self, x: usize) -> usize {
        let mut p = x;
        while self.parents[p] != p {
            p = self.parents[p];
        }
        self.parents[x] = p; // path compression
        return p;
    }

    pub fn union(&mut self, x: usize, y: usize) {
        if x == y {
            return
        }
        let par_x = self.find(x);
        let par_y = self.find(y);

        // The lowest always the parent
        if par_x > par_y {
            self.parents[par_x] = par_y;
        } else {
            self.parents[par_y] = par_x;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find() {
        let mut uf = UnionFind::new(6);
        uf.union(3,2);
        uf.union(4,2);

        assert_eq!(uf.parents, [0, 1, 2, 2, 2, 5]);
    }

    #[test]
    fn union_find_path_compression() {
        let mut uf = UnionFind::new(10);
        uf.union(2,8);
        uf.union(4,2);
        uf.union(1,2);

        assert_eq!(uf.find(4), 1);
        assert_eq!(uf.parents, [0, 1, 1, 3, 1, 5, 6, 7, 2, 9]);
    }
}
