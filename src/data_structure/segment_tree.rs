use crate::math::monoid::Monoid;

pub struct SegmentTree<T>
where
    T: Monoid,
{
    n_leaves: usize,
    nodes: Vec<T>,
}

impl<T> SegmentTree<T>
where
    T: Monoid,
{
    pub fn new(n: usize) -> Self {
        let n_leaves = n.next_power_of_two();
        let nodes = vec![T::mempty(); 2 * n_leaves - 1];
        SegmentTree { n_leaves, nodes }
    }

    pub fn update(&mut self, i: usize, x: T) {
        let mut i = i + self.n_leaves - 1;
        self.nodes[i] = x;
        while i > 0 {
            i = (i - 1) / 2;
            let left_child = &self.nodes[2 * i + 1];
            let right_child = &self.nodes[2 * i + 2];
            self.nodes[i] = left_child.mappend(&right_child);
        }
    }

    pub fn query(&self, start: usize, end: usize) -> T {
        self.query_rec(start, end, 0, 0, self.n_leaves)
    }

    fn query_rec(&self, start: usize, end: usize, i: usize, left: usize, right: usize) -> T {
        if right <= start || end <= left {
            T::mempty()
        } else if start <= left && right <= end {
            self.nodes[i].clone()
        } else {
            let mid = (left + right) / 2;
            let left_child = self.query_rec(start, end, 2 * i + 1, left, mid);
            let right_child = self.query_rec(start, end, 2 * i + 2, mid, right);
            left_child.mappend(&right_child)
        }
    }
}

impl<T> FromIterator<T> for SegmentTree<T>
where
    T: Monoid,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let v: Vec<T> = iter.into_iter().collect();
        v.into()
    }
}

impl<T> From<Vec<T>> for SegmentTree<T>
where
    T: Monoid,
{
    fn from(v: Vec<T>) -> Self {
        let n = v.len();
        let mut st = SegmentTree::new(n);
        for (i, x) in v.into_iter().enumerate() {
            st.update(i, x);
        }
        st
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest::proptest! {
        #[test]
        fn test_segment_tree_max_i64(v in proptest::collection::vec(-100..100i64, 1..100)) {
            let st = v.iter().cloned().map(crate::math::monoid::Max).collect::<SegmentTree<_>>();
            for i in 0..v.len() {
                for j in i..v.len() {
                    let max = (i..j).map(|k| v[k]).max().unwrap_or(std::i64::MIN);
                    assert_eq!(st.query(i, j).0, max);
                }
            }
        }
    }

    proptest::proptest! {
        #[test]
        fn test_segment_tree_max_u64(v in proptest::collection::vec(0..100u64, 1..100)) {
            let st = v.iter().cloned().map(crate::math::monoid::Max).collect::<SegmentTree<_>>();
            for i in 0..v.len() {
                for j in i..v.len() {
                    let max = (i..j).map(|k| v[k]).max().unwrap_or(std::u64::MIN);
                    assert_eq!(st.query(i, j).0, max);
                }
            }
        }
    }

    proptest::proptest! {
        #[test]
        fn test_segment_tree_min_i64(v in proptest::collection::vec(-100..100i64, 1..100)) {
            let st = v.iter().cloned().map(crate::math::monoid::Min).collect::<SegmentTree<_>>();
            for i in 0..v.len() {
                for j in i..v.len() {
                    let min = (i..j).map(|k| v[k]).min().unwrap_or(std::i64::MAX);
                    assert_eq!(st.query(i, j).0, min);
                }
            }
        }
    }

    proptest::proptest! {
        #[test]
        fn test_segment_tree_min_u64(v in proptest::collection::vec(0..100u64, 1..100)) {
            let st = v.iter().cloned().map(crate::math::monoid::Min).collect::<SegmentTree<_>>();
            for i in 0..v.len() {
                for j in i..v.len() {
                    let min = (i..j).map(|k| v[k]).min().unwrap_or(std::u64::MAX);
                    assert_eq!(st.query(i, j).0, min);
                }
            }
        }
    }
}
