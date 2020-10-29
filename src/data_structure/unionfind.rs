use std::mem;

#[derive(Clone, Debug)]
pub struct UnionFind {
    root: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    /// Union Find Tree を初期化する. `size` は要素数.
    pub fn new(size: usize) -> Self {
        Self {
            root: (0..size).collect(),
            size: vec![0; size],
        }
    }

    /// 指定した要素の根を返す(経路圧縮もする)．
    pub fn root(&mut self, x: usize) -> usize {
        if self.root[x] == x {
            x
        } else {
            let parent = self.root[x];
            let root = self.root(parent);
            self.root[x] = root;
            root
        }
    }

    /// 二つの要素が同じ木にあるかを返す．
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// 二つの要素が含まれる木を併合する．
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x_root = self.root(x);
        let mut y_root = self.root(y);
        if x_root == y_root {
            return false;
        }
        if self.size(x_root) < self.size(y_root) {
            mem::swap(&mut x_root, &mut y_root);
        }
        self.root[y_root] = x_root;
        self.size[x_root] += self.size[y_root];
        true
    }

    /// 指定した要素が属する木の大きさを返す．
    pub fn size(&self, x: usize) -> usize {
        self.size[x]
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::unionfind;

    #[test]
    fn random_unite() {
        let size = 5;
        let mut uf = unionfind::UnionFind::new(size);

        uf.unite(1, 2);
        uf.unite(3, 2);
        assert!(uf.same(0, 0));
        assert!(uf.same(1, 2));
        assert!(uf.same(1, 3));
        assert!(uf.same(2, 3));
        assert!(!uf.same(0, 1));
        assert!(!uf.same(0, 4));

        uf.unite(2, 4);
        assert!(uf.same(1, 4));
        assert!(uf.same(2, 4));
        assert!(uf.same(3, 4));
        assert!(!uf.same(0, 1));
        assert!(!uf.same(0, 4));
    }
}
