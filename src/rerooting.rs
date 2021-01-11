// https://algo-logic.info/tree-dp/

// TODO: for文を削れないか

use std::{
    cmp,
    ops::{Mul, MulAssign},
};

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

impl Mul for Dp {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let Dp { value: v1 } = self;
        let Dp { value: v2 } = rhs;
        Dp::new(cmp::max(v1, v2))
    }
}

impl MulAssign for Dp {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
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
//
trait Monoid: Clone + Copy + Mul<Output = Self> + MulAssign {
    const IDENTITY: Self;
}

trait Rerootable: Monoid {
    fn add_root(&self) -> Self;
}

struct Graph {
    g: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(n: usize) -> Graph {
        Graph { g: vec![vec![]; n] }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.g[from].push(Edge::new(to));
    }

    fn get_edges(&self, v: usize) -> &[Edge] {
        &self.g[v]
    }
}

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
}

impl Edge {
    fn new(to: usize) -> Edge {
        Edge { to }
    }
}

struct Rerooting<'a, T> {
    dp: Vec<Vec<T>>,
    ans: Vec<T>,
    graph: &'a Graph,
}

impl<'a, T> Rerooting<'a, T>
where
    T: Rerootable,
{
    fn new(n: usize, graph: &Graph) -> Rerooting<T> {
        let dp = vec![vec![]; n];
        let ans = vec![T::IDENTITY; n];
        Rerooting { dp, ans, graph }
    }

    fn build(&mut self) {
        self.dfs(0, None);
        self.bfs(0, T::IDENTITY, None);
    }

    fn dfs(&mut self, v: usize, p: Option<usize>) -> T {
        let mut dp_cum = T::IDENTITY;
        let deg = self.graph.get_edges(v).len();
        self.dp[v] = vec![T::IDENTITY; deg];
        for i in 0..deg {
            let u = self.graph.get_edges(v)[i].to;
            if matches!(p, Some(p) if u == p) {
                continue;
            }
            self.dp[v][i] = self.dfs(u, Some(v));
            dp_cum *= self.dp[v][i];
        }

        dp_cum.add_root()
    }

    fn bfs(&mut self, v: usize, dp_p: T, p: Option<usize>) {
        let deg = self.graph.get_edges(v).len();
        if let Some(p) = p {
            for i in 0..deg {
                if self.graph.get_edges(v)[i].to == p {
                    self.dp[v][i] = dp_p;
                }
            }
        }

        let mut dp_l = vec![T::IDENTITY; deg + 1];
        let mut dp_r = vec![T::IDENTITY; deg + 1];
        for i in 0..deg {
            dp_l[i + 1] = dp_l[i] * self.dp[v][i];
        }
        for i in (0..deg).rev() {
            dp_r[i] = dp_r[i + 1] * self.dp[v][i];
        }

        self.ans[v] = dp_l[deg].add_root();

        for i in 0..deg {
            let u = self.graph.get_edges(v)[i].to;
            if matches!(p, Some(p) if u == p) {
                continue;
            }
            self.bfs(u, (dp_l[i] * dp_r[i + 1]).add_root(), Some(v));
        }
    }
}

fn rerooting<T>(n: usize, graph: &Graph) -> Vec<T>
where
    T: Rerootable,
{
    let mut rerooting = Rerooting::new(n, graph);
    rerooting.build();
    rerooting.ans
}

#[cfg(test)]
mod tests {
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

        let ans: Vec<Dp> = rerooting(n, &graph);
        assert_eq!(
            vec![3, 4, 2, 3, 3, 4],
            ans.iter().map(|dp| dp.value).collect::<Vec<_>>()
        );
    }
}
