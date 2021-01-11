// https://algo-logic.info/tree-dp/

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
    const IDENTITY: Dp = Dp { value: -1 };

    fn new(value: i64) -> Dp {
        Dp { value }
    }

    fn add_root(&self) -> Dp {
        Dp::new(self.value + 1)
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
// 書き換えここまで

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

struct Rerooting<'a> {
    dp: Vec<Vec<Dp>>,
    ans: Vec<Dp>,
    graph: &'a Graph,
}

impl<'a> Rerooting<'a> {
    fn new(n: usize, graph: &Graph) -> Rerooting {
        let dp = vec![vec![]; n];
        let ans = vec![Dp::IDENTITY; n];
        Rerooting { dp, ans, graph }
    }

    fn build(&mut self) {
        self.dfs(0, None);
        self.bfs(0, Dp::IDENTITY, None);
    }

    fn dfs(&mut self, v: usize, p: Option<usize>) -> Dp {
        let mut dp_cum = Dp::IDENTITY;
        let deg = self.graph.get_edges(v).len();
        self.dp[v] = vec![Dp::IDENTITY; deg];
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

    fn bfs(&mut self, v: usize, dp_p: Dp, p: Option<usize>) {
        let deg = self.graph.get_edges(v).len();
        if let Some(p) = p {
            for i in 0..deg {
                if self.graph.get_edges(v)[i].to == p {
                    self.dp[v][i] = dp_p;
                }
            }
        }

        let mut dp_l = vec![Dp::IDENTITY; deg + 1];
        let mut dp_r = vec![Dp::IDENTITY; deg + 1];
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

fn rerooting(n: usize, graph: &Graph) -> Vec<Dp> {
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

        let ans = rerooting(n, &graph);
        assert_eq!(
            vec![3, 4, 2, 3, 3, 4],
            ans.iter().map(|dp| dp.value).collect::<Vec<_>>()
        );
    }
}
