use crate::graph_struct::{Edge, Graph};

// https://algo-logic.info/tree-dp/

pub trait Monoid: Clone + Copy {
    fn identity() -> Self;
}

struct Rerooting<'a, T, E: Edge, F1, F2> {
    dp: Vec<Vec<T>>,
    ans: Vec<T>,
    graph: &'a Graph<E>,
    merge: F1,
    add_root: F2,
}

impl<'a, T, E, F1, F2> Rerooting<'a, T, E, F1, F2>
where
    T: Monoid,
    E: Edge,
    F1: FnMut(T, T) -> T,
    F2: FnMut(T) -> T,
{
    fn new(n: usize, graph: &Graph<E>, merge: F1, add_root: F2) -> Rerooting<T, E, F1, F2> {
        let dp = vec![vec![]; n];
        let ans = vec![T::identity(); n];
        Rerooting {
            dp,
            ans,
            graph,
            merge,
            add_root,
        }
    }

    fn build(&mut self) {
        self.dfs(0, None);
        self.bfs(0, T::identity(), None);
    }

    fn dfs(&mut self, v: usize, p: Option<usize>) -> T {
        let mut dp_cum = T::identity();
        let deg = self.graph.degree(v);
        self.dp[v] = vec![T::identity(); deg];
        for (i, edge) in self.graph.edges(v).iter().cloned().enumerate() {
            let next = edge.to();
            if matches!(p, Some(p) if next == p) {
                continue;
            }
            self.dp[v][i] = self.dfs(next, Some(v));
            dp_cum = (self.merge)(dp_cum, self.dp[v][i]);
        }

        (self.add_root)(dp_cum)
    }

    fn bfs(&mut self, v: usize, dp_p: T, p: Option<usize>) {
        let deg = self.graph.degree(v);
        if let Some(p) = p {
            for (i, edge) in self.graph.edges(v).iter().cloned().enumerate() {
                if edge.to() == p {
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

        self.ans[v] = (self.add_root)(dp_l[deg]);

        for (i, edge) in self.graph.edges(v).iter().cloned().enumerate() {
            let next = edge.to();
            if matches!(p, Some(p) if next == p) {
                continue;
            }
            let merged = (self.merge)(dp_l[i], dp_r[i + 1]);
            let add_rooted = (self.add_root)(merged);
            self.bfs(next, add_rooted, Some(v));
        }
    }
}

pub fn rerooting<T, E, F1, F2>(n: usize, graph: &Graph<E>, merge: F1, add_root: F2) -> Vec<T>
where
    T: Monoid,
    E: Edge,
    F1: FnMut(T, T) -> T,
    F2: FnMut(T) -> T,
{
    let mut rerooting = Rerooting::new(n, graph, merge, add_root);
    rerooting.build();
    rerooting.ans
}

#[cfg(test)]
mod tests {
    use std::cmp;

    use crate::graph_struct::SimpleEdge;

    use super::*;

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
            Dp::new(-1)
        }
    }

    #[test]
    fn rerooting1_test() {
        let n = 6;
        let edges = vec![(1, 2), (1, 3), (3, 4), (3, 5), (5, 6)];
        let mut graph = Graph::new(n);
        for (a, b) in edges {
            let a = a - 1;
            let b = b - 1;
            graph.add_edge(a, SimpleEdge::new(b));
            graph.add_edge(b, SimpleEdge::new(a));
        }
        let merge = |x, y| {
            let Dp { value: v1 } = x;
            let Dp { value: v2 } = y;
            Dp::new(cmp::max(v1, v2))
        };
        let add_root = |x: Dp| Dp::new(x.value + 1);

        let ans: Vec<Dp> = rerooting(n, &graph, merge, add_root);
        assert_eq!(
            vec![3, 4, 2, 3, 3, 4],
            ans.iter().map(|dp| dp.value).collect::<Vec<_>>()
        );
    }
}
