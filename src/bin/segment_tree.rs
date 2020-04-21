fn main() {

}

struct SegmentTree {
    len: usize,
    data: Vec<u64>,
}

impl SegmentTree {
    const INF: u64 = std::u64::MAX >> 2;

    fn new(n: usize) -> SegmentTree {
        let mut len = 1;
        while len < n {
            len *= 2;
        }

        let mut segtree = SegmentTree {
            len: len,
            data: vec![],
        };

        for _ in 0..(2 * len - 1) {
            segtree.data.push(SegmentTree::INF);
        }

        segtree
    }

    fn update(&mut self, k: usize, a: u64) {
        use std::cmp::min;

        let data = &mut self.data;
        let mut k = k + self.len - 1;
        data[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            data[k] = min(data[k * 2 + 1], data[k * 2 + 2]);
        }
    }

    fn query(&self, a: usize, b: usize) -> u64 {
        self.process_query(a, b, 0, 0, self.len)
    }

    fn process_query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> u64 {
        if r <= a || b <= l {
            return SegmentTree::INF;
        }

        if a <= l && r <= b {
            self.data[k]
        } else {
            let vl = self.process_query(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.process_query(a, b, k * 2 + 2, (l + r) / 2, r);
            std::cmp::min(vl, vr)
        }
    }
}
