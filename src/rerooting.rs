use std::cmp;

// use proconio::input;
// #[allow(unused_imports)]
// use proconio::marker::*;

//{{{
#[allow(unused_macros)]
macro_rules! multi_vec {
    ( $elem:expr; $num:expr ) => (vec![$elem; $num]);
    ( $elem:expr; $num:expr, $($rest:expr),* ) => (vec![multi_vec![$elem; $($rest),*]; $num]);
}
//}}}

// fn main() {
//     input! {
//         n: usize,
//         edges: [(Usize1, Usize1); n - 1],
//     }

//     let mut reroot = Rerooting::new(n);
//     for (a, b) in edges {
//         reroot.add_edge(a, b);
//         reroot.add_edge(b, a);
//     }
//     reroot.build();

//     for i in 0..n {
//         println!("{}", reroot.ans[i].dp);
//     }
// }

// TODO: Dpのメソッドにする
const IDENTITY: Dp = Dp { dp: -1 };
const INF: usize = std::usize::MAX >> 2;

#[derive(Clone, Copy, Debug)]
struct Dp {
    dp: i64,
}

impl Dp {
    fn new(dp: i64) -> Dp {
        Dp { dp }
    }
}

fn merge(dp_cum: Dp, d: Dp) -> Dp {
    Dp::new(cmp::max(dp_cum.dp, d.dp))
}

fn add_root(d: Dp) -> Dp {
    Dp::new(d.dp + 1)
}

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
}
type Graph = Vec<Vec<Edge>>;
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
            dp_cum = merge(dp_cum, self.dp[v][i])
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
            dp_l[i + 1] = merge(dp_l[i], self.dp[v][i]);
        }
        for i in (0..deg).rev() {
            dp_r[i] = merge(dp_r[i + 1], self.dp[v][i]);
        }

        self.ans[v] = add_root(dp_l[deg]);

        for i in 0..deg {
            let u = self.g[v][i].to;
            if u == p {
                continue;
            }
            self.bfs(u, add_root(merge(dp_l[i], dp_r[i + 1])), v);
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
            reroot.ans.iter().map(|dp| dp.dp).collect::<Vec<_>>()
        );
    }
}
