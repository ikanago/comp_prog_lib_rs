pub mod dijkstra;

/// 二次元配列で表現されたグリッドにおいて,ある点に隣接する4方向の節点を返す.
/// グリッドからはみ出るような節点は無視される.
pub fn adjacent4<T>(
    graph: &Vec<Vec<T>>,
    row: usize,
    col: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let height = graph.len() as isize;
    let width = graph[0].len() as isize;
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .filter_map(move |&(dx, dy)| {
            let nx = row as isize + dx;
            let ny = col as isize + dy;
            if nx >= 0 && nx < height && ny >= 0 && ny < width {
                Some((nx as usize, ny as usize))
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::graph::adjacent4;

    #[test]
    fn test_adjacent4_corner() {
        let graph = vec![vec![0; 3]; 3];
        let expected = vec![(1, 0), (0, 1)];
        let actual = adjacent4(&graph, 0, 0).collect::<Vec<(usize, usize)>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_adjacent4_side() {
        let graph = vec![vec![0; 3]; 3];
        let expected = vec![(1, 1), (0, 2), (0, 0)];
        let actual = adjacent4(&graph, 0, 1).collect::<Vec<(usize, usize)>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_adjacent4_center() {
        let graph = vec![vec![0; 3]; 3];
        let expected = vec![(2, 1), (1, 2), (0, 1), (1, 0)];
        let actual = adjacent4(&graph, 1, 1).collect::<Vec<(usize, usize)>>();
        assert_eq!(expected, actual);
    }
}
