fn main() {

}

struct SegmentTree<T, F> {
    len: usize,
    data: Vec<T>,
    operator: F,
    unit: T,
}

impl<T, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T + Copy,
    T: Clone + Copy,
{
    // const INF: u64 = std::u64::MAX >> 2;

    fn new(v: &Vec<T>, operator: F, unit: T) -> SegmentTree<T, F> {
        let n = v.len();
        let mut len = 1;
        while len < n {
            len *= 2;
        }

        let mut segtree = SegmentTree {
            len: len,
            data: vec![unit; 2 * len - 1],
            operator,
            unit,
        };

        for i in 0..n {
            segtree.data[i + segtree.len - 1] = v[i];
        }
        for i in (0..(segtree.len - 1)).rev() {
            segtree.data[i] = operator(segtree.data[2 * i + 1], segtree.data[2 * i + 2]);
        }

        segtree
    }

    fn update(&mut self, k: usize, a: T) {
        let data = &mut self.data;
        let mut k = k + self.len - 1;
        data[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            data[k] = (self.operator)(data[k * 2 + 1], data[k * 2 + 2]);
        }
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
