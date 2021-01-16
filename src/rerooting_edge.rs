// abc187

use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use rorooting::{rerooting, Graph, Monoid, Rerootable};

//{{{
#[allow(unused_macros)]
macro_rules! multi_vec {
    ( $elem:expr; $num:expr ) => (vec![$elem; $num]);
    ( $elem:expr; $num:expr, $($rest:expr),* ) => (vec![multi_vec![$elem; $($rest),*]; $num]);
}
//}}}

#[allow(unused)]
fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
        q: usize,
        queries: [(Usize1, Usize1, i64); q],
    }

    let mut values = vec![vec![0; 2]; n - 1];
    for (t, e, x) in queries {
        values[e][t] += x;
    }

    let mut graph = Graph::new(n);
    for (i, (a, b)) in edges.iter().copied().enumerate() {
        graph.add_edge(a, b, values[i][0], values[i][1]);
        graph.add_edge(b, a, values[i][1], values[i][0]);
    }

    let merge = |x: Dp, y: Dp| Dp::new(x.value + y.value);
    let ans = rerooting(n, &graph, merge);
    for ans in ans {
        println!("{}", ans.value);
    }
}

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
    fn identity() -> Self {
        Dp { value: 0 }
    }
}

impl Rerootable for Dp {
    fn add_root(&self, edge: i64) -> Dp {
        Dp::new(self.value + edge)
    }
}
// 書き換えここまで

mod rorooting {
    // https://algo-logic.info/tree-dp/

    use std::fmt::Debug;

    pub trait Monoid: Clone + Copy {
        fn identity() -> Self;
    }

    pub trait Rerootable: Monoid + Debug {
        fn add_root(&self, edge: i64) -> Self;
    }

    #[derive(Debug)]
    pub struct Graph {
        adj_list: Vec<Vec<Edge>>,
    }

    impl Graph {
        pub fn new(n: usize) -> Graph {
            Graph {
                adj_list: vec![vec![]; n],
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, value: i64, rev_value: i64) {
            self.adj_list[from].push(Edge::new(to, value, rev_value));
        }

        fn edges(&self, v: usize) -> &[Edge] {
            &self.adj_list[v]
        }

        fn degree(&self, v: usize) -> usize {
            *&self.adj_list[v].len()
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Edge {
        pub to: usize,
        pub value: i64,
        pub rev_value: i64,
    }

    fn edge_value_identity() -> i64 {
        0
    }

    impl Edge {
        fn new(to: usize, value: i64, rev_value: i64) -> Edge {
            Edge {
                to,
                value,
                rev_value,
            }
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
            let ans = vec![T::identity(); n];
            Rerooting {
                dp,
                ans,
                graph,
                merge,
            }
        }

        fn build(&mut self) {
            let start = 0;
            self.dfs(start, None, edge_value_identity());
            self.bfs(start, T::identity(), None);
        }

        fn dfs(&mut self, v: usize, p: Option<usize>, p_edge: i64) -> T {
            let mut dp_cum = T::identity();
            let deg = self.graph.degree(v);
            self.dp[v] = vec![T::identity(); deg];
            for (i, edge) in self.graph.edges(v).iter().copied().enumerate() {
                let next = edge.to;
                if matches!(p, Some(p) if next == p) {
                    continue;
                }
                self.dp[v][i] = self.dfs(next, Some(v), edge.value);
                dp_cum = (self.merge)(dp_cum, self.dp[v][i]);
            }

            dp_cum.add_root(p_edge)
        }

        fn bfs(&mut self, v: usize, dp_p: T, p: Option<usize>) {
            let deg = self.graph.degree(v);

            if let Some(p) = p {
                for (i, edge) in self.graph.edges(v).iter().copied().enumerate() {
                    if edge.to == p {
                        self.dp[v][i] = dp_p;
                    }
                }
            }

            let mut dp_l = vec![T::identity(); deg + 1];
            let mut dp_r = vec![T::identity(); deg + 1];
            for i in 0..deg {
                dp_l[i + 1] = (self.merge)(dp_l[i], self.dp[v][i]);
            }
            for i in (0..deg).rev() {
                dp_r[i] = (self.merge)(dp_r[i + 1], self.dp[v][i]);
            }

            self.ans[v] = dp_l[deg].add_root(edge_value_identity());

            for (i, edge) in self.graph.edges(v).iter().copied().enumerate() {
                let next = edge.to;
                if matches!(p, Some(p) if next == p) {
                    continue;
                }
                let dp_p = (self.merge)(dp_l[i], dp_r[i + 1]).add_root(edge.rev_value);
                self.bfs(next, dp_p, Some(v));
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
}
