// https://algo-logic.info/tree-dp/

// 問題ごとに書き換え
#[derive(Clone, Copy, Debug)]
struct Dp {
    value: i64,
}

impl Dp {
    fn new(value: i64) -> Dp {
        Dp { value }
    }
}

impl Monoid for Dp {
    const IDENTITY: Self = Dp { value: -1 };
}

impl Rerootable for Dp {
    fn add_root(&self) -> Dp {
        Dp::new(self.value + 1)
    }
}
// 書き換えここまで

pub trait Monoid: Clone + Copy {
    const IDENTITY: Self;
}

pub trait Rerootable: Monoid {
    fn add_root(&self) -> Self;
}

pub struct Graph {
    g: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(n: usize) -> Graph {
        Graph { g: vec![vec![]; n] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.g[from].push(Edge::new(to));
    }

    fn get_edges(&self, v: usize) -> &[Edge] {
        &self.g[v]
    }

    fn get_degree(&self, v: usize) -> usize {
        *&self.g[v].len()
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    to: usize,
}

impl Edge {
    fn new(to: usize) -> Edge {
        Edge { to }
    }
}

struct Rerooting<'a, T, F> {
    dp: Vec<Vec<T>>,
    ans: Vec<T>,
    graph: &'a Graph,
    merge: F,
}

impl<'a, T, F> Rerooting<'a, T, F>
where
    T: Rerootable,
    F: FnMut(T, T) -> T,
{
    fn new(n: usize, graph: &Graph, merge: F) -> Rerooting<T, F> {
        let dp = vec![vec![]; n];
        let ans = vec![T::IDENTITY; n];
        Rerooting {
            dp,
            ans,
            graph,
            merge,
        }
    }

    fn build(&mut self) {
        self.dfs(0, None);
        self.bfs(0, T::IDENTITY, None);
    }

    fn dfs(&mut self, v: usize, p: Option<usize>) -> T {
        let mut dp_cum = T::IDENTITY;
        let deg = self.graph.get_degree(v);
        self.dp[v] = vec![T::IDENTITY; deg];
        for (i, edge) in self.graph.get_edges(v).iter().copied().enumerate() {
            let next = edge.to;
            if matches!(p, Some(p) if next == p) {
                continue;
            }
            self.dp[v][i] = self.dfs(next, Some(v));
            dp_cum = (self.merge)(dp_cum, self.dp[v][i]);
        }

        dp_cum.add_root()
    }

    fn bfs(&mut self, v: usize, dp_p: T, p: Option<usize>) {
        let deg = self.graph.get_degree(v);
        if let Some(p) = p {
            for (i, edge) in self.graph.get_edges(v).iter().copied().enumerate() {
                if edge.to == p {
                    self.dp[v][i] = dp_p;
                }
            }
        }

        let mut dp_l = vec![T::IDENTITY; deg + 1];
        let mut dp_r = vec![T::IDENTITY; deg + 1];
        for i in 0..deg {
            dp_l[i + 1] = (self.merge)(dp_l[i], self.dp[v][i]);
        }
        for i in (0..deg).rev() {
            dp_r[i] = (self.merge)(dp_r[i + 1], self.dp[v][i]);
        }

        self.ans[v] = dp_l[deg].add_root();

        for (i, edge) in self.graph.get_edges(v).iter().copied().enumerate() {
            let next = edge.to;
            if matches!(p, Some(p) if next == p) {
                continue;
            }
            let merged = (self.merge)(dp_l[i], dp_r[i + 1]);
            self.bfs(next, merged.add_root(), Some(v));
        }
    }
}

pub fn rerooting<T, F>(n: usize, graph: &Graph, merge: F) -> Vec<T>
where
    T: Rerootable,
    F: FnMut(T, T) -> T,
{
    let mut rerooting = Rerooting::new(n, graph, merge);
    rerooting.build();
    rerooting.ans
}

#[cfg(test)]
mod tests {
    use std::cmp;

    use super::*;

    #[test]
    fn rerooting1_test() {
        let n = 6;
        let edges = vec![(1, 2), (1, 3), (3, 4), (3, 5), (5, 6)];
        let mut graph = Graph::new(n);
        for (a, b) in edges {
            let a = a - 1;
            let b = b - 1;
            graph.add_edge(a, b);
            graph.add_edge(b, a);
        }
        let merge = |x, y| {
            let Dp { value: v1 } = x;
            let Dp { value: v2 } = y;
            Dp::new(cmp::max(v1, v2))
        };

        let ans: Vec<Dp> = rerooting(n, &graph, merge);
        assert_eq!(
            vec![3, 4, 2, 3, 3, 4],
            ans.iter().map(|dp| dp.value).collect::<Vec<_>>()
        );
    }
}
