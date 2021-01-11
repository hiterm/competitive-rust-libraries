// https://algo-logic.info/tree-dp/

use std::{
    cmp,
    ops::{Mul, MulAssign},
};

// TODO: Dpのメソッドにする
const IDENTITY: Dp = Dp { value: -1 };
const INF: usize = std::usize::MAX >> 2;

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

fn add_root(d: Dp) -> Dp {
    Dp::new(d.value + 1)
}
// 書き換えここまで

type Graph = Vec<Vec<Edge>>;

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
}

impl Edge {
    fn new(to: usize) -> Edge {
        Edge { to }
    }
}

struct Rerooting {
    dp: Vec<Vec<Dp>>,
    ans: Vec<Dp>,
    g: Graph,
}

impl Rerooting {
    fn new(n: usize) -> Rerooting {
        let dp = vec![vec![]; n];
        let ans = vec![IDENTITY; n];
        let g = vec![vec![]; n];
        Rerooting { dp, ans, g }
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        self.g[a].push(Edge::new(b));
    }

    fn build(&mut self) {
        self.dfs(0, INF);
        self.bfs(0, IDENTITY, INF);
    }

    fn dfs(&mut self, v: usize, p: usize) -> Dp {
        let mut dp_cum = IDENTITY;
        let deg = self.g[v].len();
        self.dp[v] = vec![IDENTITY; deg];
        for i in 0..deg {
            let u = self.g[v][i].to;
            if u == p {
                continue;
            }
            self.dp[v][i] = self.dfs(u, v);
            dp_cum *= self.dp[v][i];
        }

        add_root(dp_cum)
    }

    fn bfs(&mut self, v: usize, dp_p: Dp, p: usize) {
        let deg = self.g[v].len();
        for i in 0..deg {
            if self.g[v][i].to == p {
                self.dp[v][i] = dp_p;
            }
        }

        let mut dp_l = vec![IDENTITY; deg + 1];
        let mut dp_r = vec![IDENTITY; deg + 1];
        for i in 0..deg {
            dp_l[i + 1] = dp_l[i] * self.dp[v][i];
        }
        for i in (0..deg).rev() {
            dp_r[i] = dp_r[i + 1] * self.dp[v][i];
        }

        self.ans[v] = add_root(dp_l[deg]);

        for i in 0..deg {
            let u = self.g[v][i].to;
            if u == p {
                continue;
            }
            self.bfs(u, add_root(dp_l[i] * dp_r[i + 1]), v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rerooting1_test() {
        let n = 6;
        let edges = vec![(1, 2), (1, 3), (3, 4), (3, 5), (5, 6)];
        let mut reroot = Rerooting::new(n);
        for (a, b) in edges {
            let a = a - 1;
            let b = b - 1;
            reroot.add_edge(a, b);
            reroot.add_edge(b, a);
        }
        reroot.build();
        assert_eq!(
            vec![3, 4, 2, 3, 3, 4],
            reroot.ans.iter().map(|dp| dp.value).collect::<Vec<_>>()
        );
    }
}
