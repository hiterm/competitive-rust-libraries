use ac_library_rs::modint::ModIntBase;

#[derive(Debug)]
pub struct MintUtil<T> {
    factorial: Vec<T>,
    factorial_inv: Vec<T>,
    inv: Vec<T>,
}

impl<ModInt: ModIntBase> MintUtil<ModInt> {
    #[allow(unused)]
    pub fn new() -> MintUtil<ModInt> {
        MintUtil {
            factorial: vec![ModInt::new(1), ModInt::new(1)],
            factorial_inv: vec![ModInt::new(1), ModInt::new(1)],
            inv: vec![ModInt::new(0), ModInt::new(1)],
        }
    }

    fn calc_cache(&mut self, n: usize) {
        let len = self.factorial.len();
        if len < n + 1 {
            for i in len..(n + 1) {
                let prev = *self.factorial.last().unwrap();
                self.factorial.push(prev * ModInt::from(i));

                let inv_i = -self.inv[ModInt::modulus() as usize % i]
                    * ModInt::from(ModInt::modulus() as usize / i);
                self.inv.push(inv_i);

                let prev = *self.factorial_inv.last().unwrap();
                self.factorial_inv.push(prev * self.inv[i]);
            }
        }
    }

    #[allow(unused)]
    pub fn factorial(&mut self, n: usize) -> ModInt {
        self.calc_cache(n);
        self.factorial[n]
    }

    #[allow(unused)]
    pub fn factorial_inv(&mut self, n: usize) -> ModInt {
        self.calc_cache(n);
        self.factorial_inv[n]
    }

    // when MODULO is prime
    #[allow(unused)]
    pub fn binom_coef(&mut self, n: usize, k: usize) -> ModInt {
        if n < k {
            return ModInt::from(0);
        }

        self.calc_cache(n);
        self.factorial[n] * self.factorial_inv[k] * self.factorial_inv[n - k]
    }

    #[allow(unused)]
    fn perm(&mut self, n: usize, k: usize) -> ModInt {
        if n < k {
            return ModInt::from(0);
        }
        self.factorial(n) * self.factorial_inv(n - k)
    }

    // Not tested!!
    #[allow(unused)]
    pub fn multi_coef(&mut self, v: &[usize]) -> ModInt {
        let n = v.iter().sum();
        self.calc_cache(n);

        let mut ret = ModInt::from(1);
        ret *= self.factorial[n];
        for v_i in v {
            ret *= self.factorial_inv[*v_i];
        }

        ret
    }
}
