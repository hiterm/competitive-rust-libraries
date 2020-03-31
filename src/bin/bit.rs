struct Bit {
    n: usize,
    v: Vec<i64>,
}

impl Bit {
    fn new(n: usize) -> Bit {
        let mut m = 1;
        while m < n {
            m *= 2;
        }
        Bit {
            n: n,
            v: vec![0; m],
        }
    }

    fn sum(&self, i: usize) -> i64 {
        let mut i = i + 1;
        let mut ret = 0;
        while i > 0 {
            ret += self.v[i - 1];
            i -= (i as i64 & -(i as i64)) as usize;
        }
        ret
    }

    fn add(&mut self, i: usize, x: i64) {
        let mut i = i + 1;
        while i <= self.n {
            self.v[i - 1] += x;
            i += (i as i64 & -(i as i64)) as usize;
        }
    }
}

