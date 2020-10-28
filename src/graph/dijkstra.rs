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
    distance: Vec<usize>,
}

impl Dijkstra {
    /// Dijkstra 法で最短経路を求めるための構造体を初期化する．
    /// `size` は節点数．
    pub fn new(size: usize) -> Self {
        Self {
            graph: vec![vec![]; size],
            distance: vec![usize::MAX; size],
        }
    }

    /// `from` 番目の節点から `to` 番目の節点へコスト `cost` の辺を張る．
    /// `from` と `to` は 0-indexed でなければならない.
    pub fn add_edge(&mut self, from: usize, to: usize, cost: usize) {
        self.graph[from].push(Edge { to, cost });
    }

    /// `to` 番目の節点への距離を返す
    pub fn distance(&self, to: usize) -> usize {
        self.distance[to]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cost_of_start_point_is_zero() {
        let mut dijkstra = Dijkstra::new(2);
        dijkstra.add_edge(0, 1, 1);
        dijkstra.solve(0);
        assert_eq!(0, dijkstra.distance(0));
    }

    #[test]
    fn shortest_path_by_dijkstra() {
        let mut dijkstra = Dijkstra::new(8);
        dijkstra.add_edge(0, 1, 1);
        dijkstra.add_edge(0, 3, 4);
        dijkstra.add_edge(0, 4, 5);
        dijkstra.add_edge(1, 2, 1);
        dijkstra.add_edge(2, 5, 4);
        dijkstra.add_edge(2, 7, 8);
        dijkstra.add_edge(3, 6, 4);
        dijkstra.add_edge(4, 5, 2);
        dijkstra.add_edge(4, 6, 2);
        dijkstra.add_edge(5, 7, 2);
        dijkstra.add_edge(6, 7, 5);
        dijkstra.solve(0);
        assert_eq!(8, dijkstra.distance(7));
    }
}
