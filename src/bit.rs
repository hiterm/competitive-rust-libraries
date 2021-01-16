pub struct Bit {
    pub n: usize,
    pub v: Vec<i64>,
}

impl Bit {
    pub fn new(n: usize) -> Bit {
        let mut m = 1;
        while m < n {
            m *= 2;
        }
        Bit { n, v: vec![0; m] }
    }

    pub fn sum(&self, i: usize) -> i64 {
        let mut i = i + 1;
        let mut ret = 0;
        while i > 0 {
            ret += self.v[i - 1];
            i -= (i as i64 & -(i as i64)) as usize;
        }
        ret
    }

    pub fn add(&mut self, i: usize, x: i64) {
        let mut i = i + 1;
        while i <= self.n {
            self.v[i - 1] += x;
            i += (i as i64 & -(i as i64)) as usize;
        }
    }
}
