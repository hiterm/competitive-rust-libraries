use lca::Lca;

//{{{
#[allow(unused_macros)]
macro_rules! getl {
    ( $( $t:ty ),* ) => {
        {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let s = s.trim_right();
            let mut ws = s.split_whitespace();
            ($(ws.next().unwrap().parse::<$t>().unwrap()),*)
        }
    };
}

#[allow(unused_macros)]
macro_rules! getl_vec {
    ( $t:ty ) => {{
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let s = s.trim_right();
        s.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<$t>>()
    }};
}
//}}}

fn main() {
    let n = getl!(usize);
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        let v = getl_vec!(usize);
        graph[i] = v[1..].to_owned();
    }
    let q = getl!(usize);
    let mut queries = vec![];
    for _ in 0..q {
        let (u, v) = getl!(usize, usize);
        queries.push((u, v));
    }

    let lca = Lca::new(0, &graph);

    for (u, v) in queries {
        let ans = lca.lca(u, v);
        println!("{}", ans);
    }
}

mod lca {
    pub struct Lca {
        id: Vec<usize>,
        segtree: SegmentTree<Option<usize>>,
    }

    impl<'a> Lca {
        pub fn new(root: usize, graph: &Vec<Vec<usize>>) -> Lca {
            let n = graph.len();
            let mut k = 0;
            let mut id = vec![0; n];
            let mut vs = vec![0; n * 2 - 1];
            let mut depth = vec![0; n];
            Lca::dfs(root, None, 0, &mut k, &mut id, &mut vs, &mut depth, &graph);

            let op = move |i: Option<usize>, j: Option<usize>| -> Option<usize> {
                match (i, j) {
                    (Some(i), Some(j)) => {
                        if depth[i] <= depth[j] {
                            Some(i)
                        } else {
                            Some(j)
                        }
                    }
                    (Some(i), None) => Some(i),
                    (None, Some(j)) => Some(j),
                    (None, None) => None,
                }
            };
            let segtree =
                SegmentTree::new(&vs.iter().map(|x| Some(*x)).collect(), Box::new(op), None);

            let lca = Lca { id, segtree };

            lca
        }

        pub fn lca(&self, u: usize, v: usize) -> usize {
            use std::cmp::{max, min};
            self.segtree
                .query(min(self.id[u], self.id[v]), max(self.id[u], self.id[v]) + 1)
                .unwrap()
        }

        fn dfs(
            v: usize,
            parent: Option<usize>,
            d: usize,
            k: &mut usize,
            id: &mut Vec<usize>,
            vs: &mut Vec<usize>,
            depth: &mut Vec<usize>,
            graph: &Vec<Vec<usize>>,
        ) {
            id[v] = *k;
            vs[*k] = v;
            depth[v] = d;
            *k += 1;

            for &next in graph[v].iter().filter(|x| match parent {
                Some(p) => **x != p,
                None => true,
            }) {
                Lca::dfs(next, Some(v), d + 1, k, id, vs, depth, graph);
                vs[*k] = v;
                *k += 1;
            }
        }
    }

    struct SegmentTree<T> {
        len: usize,
        data: Vec<T>,
        operator: Box<dyn Fn(T, T) -> T>,
        unit: T,
    }

    impl<T> SegmentTree<T>
    where
        T: Clone + Copy,
    {
        fn new(v: &Vec<T>, operator: Box<dyn Fn(T, T) -> T>, unit: T) -> SegmentTree<T> {
            let n = v.len();
            let mut len = 1;
            while len < n {
                len *= 2;
            }

            let mut segtree = SegmentTree {
                len,
                data: vec![unit; 2 * len - 1],
                operator,
                unit,
            };

            for i in 0..n {
                segtree.data[i + segtree.len - 1] = v[i];
            }
            for i in (0..(segtree.len - 1)).rev() {
                segtree.data[i] =
                    (segtree.operator)(segtree.data[2 * i + 1], segtree.data[2 * i + 2]);
            }

            segtree
        }

        #[allow(unused)]
        fn update(&mut self, k: usize, a: T) {
            let data = &mut self.data;
            let mut k = k + self.len - 1;
            data[k] = a;
            while k > 0 {
                k = (k - 1) / 2;
                data[k] = (self.operator)(data[k * 2 + 1], data[k * 2 + 2]);
            }
        }

        #[allow(unused)]
        fn get(&self, k: usize) -> T {
            self.data[k + self.len - 1]
        }

        fn query(&self, a: usize, b: usize) -> T {
            self.process_query(a, b, 0, 0, self.len)
        }

        fn process_query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
            if r <= a || b <= l {
                return self.unit;
            }

            if a <= l && r <= b {
                self.data[k]
            } else {
                let vl = self.process_query(a, b, k * 2 + 1, l, (l + r) / 2);
                let vr = self.process_query(a, b, k * 2 + 2, (l + r) / 2, r);
                (self.operator)(vl, vr)
            }
        }
    }
}
