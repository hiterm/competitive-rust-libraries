//{{{
#[allow(unused_macros)]
macro_rules! getl {
    ( $( $t:ty ),* ) => {
        {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let s = s.trim_end();
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
        let s = s.trim_end();
        s.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<$t>>()
    }};
}
//}}}

const INF: u64 = (1 << 31) - 1;

fn main() {
    const UPDATE: u64 = 0;
    const FIND: u64 = 1;

    let (n, q) = getl!(usize, usize);
    let mut queries = vec![];
    for _ in 0..q {
        queries.push(getl!(u64, u64, u64));
    }

    let mut segtree = SegmentTree::new(&vec![INF; n], std::cmp::min, INF);
    for (ty, a, b) in queries {
        if ty == UPDATE {
            segtree.update(a as usize, b);
        } else {
            let ans = segtree.query(a as usize, b as usize + 1);
            println!("{}", ans);
        }
    }
}

// TODO: indexの範囲チェック

struct SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
    T: Clone + Copy,
{
    len: usize,
    data: Vec<T>,
    operator: F,
    unit: T,
}

impl<T, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
    T: Clone + Copy,
{
    fn build(v: &Vec<T>, operator: F, unit: T) -> SegmentTree<T, F> {
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
            segtree.data[i] = (segtree.operator)(segtree.data[2 * i + 1], segtree.data[2 * i + 2]);
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
