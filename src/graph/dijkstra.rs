use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
pub struct Edge {
    to: usize,
    cost: usize,
}

pub struct Dijkstra {
    // i番目の要素はi番目から出る`Edge`のを集めた`Vec`をもつ.
    graph: Vec<Vec<Edge>>,
    // i番目の要素はスタート地点からi番目の節点までの最短経路.
    pub distance: Vec<usize>,
}

impl Dijkstra {
    pub fn new(size: usize) -> Self {
        Self {
            graph: vec![vec![]; size],
            distance: vec![usize::MAX; size],
        }
    }

    /// `from` から `to` へコスト `cost` の辺を張る．
    /// `from` と `to` は 0-indexed でなければならない.
    pub fn add_edge(&mut self, from: usize, to: usize, cost: usize) {
        self.graph[from].push(Edge { to, cost });
    }

    /// 最短経路を求める．
    pub fn solve(&mut self, start: usize) {
        // (i番目の節点への最小コスト, i番目の節点) を格納する
        let mut bin_heap = BinaryHeap::new();
        self.distance[start] = 0;
        bin_heap.push(Reverse((0, start)));
        while !bin_heap.is_empty() {
            let (cost, current_edge_index) = bin_heap.pop().unwrap().0;
            if self.distance[current_edge_index] < cost {
                continue;
            }
            for next_edge in &self.graph[current_edge_index] {
                let candidate = self.distance[current_edge_index] + next_edge.cost;
                if self.distance[next_edge.to] > candidate {
                    self.distance[next_edge.to] = candidate;
                    bin_heap.push(Reverse((self.distance[next_edge.to], next_edge.to)))
                }
            }
        }
    }
}
