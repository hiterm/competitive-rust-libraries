use modint::ModInt;

fn main() {
    println!("{:?}", ModInt::from(5) + ModInt::from(6));
    println!("{:?}", ModInt::from(1) - ModInt::from(2));
    println!("{:?}", ModInt::from(3) - ModInt::from(1));
    println!("{:?}", ModInt::from(5) * ModInt::from(5));
    println!("{:?}", ModInt::from(3).pow(2));
    println!("{:?}", ModInt::from(3).pow(1000000000000000));
    println!("{:?}", ModInt::from(2).inv());
    println!("{:?}", ModInt::from(3).inv());

    let mut modulo_utils = modint::ModIntUtil::new();
    println!("{:?}", modulo_utils.binom_coef(4, 2));
    println!("{:?}", modulo_utils);

    let mut a = ModInt::from(1);
    a += ModInt::from(1);
    println!("{:?}", a);
    let mut a = ModInt::from(2);
    a *= ModInt::from(2);
    println!("{:?}", a);
    let mut a = ModInt::from(3);
    a -= ModInt::from(2);
    println!("{:?}", a);
}

mod modint {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    pub const MODULO: usize = 1_000_000_007;
    // pub const MODULO: usize = 11;

    fn positive_rem(a: isize, b: usize) -> usize {
        let b = b as isize;
        let mut value = a % b;
        if value < 0 {
            value += b;
        }
        // TODO: TryFrom
        value as usize
    }

    /// Return (x, y) s.t. ax + by = d where d = gcd(a, b)
    #[allow(unused)]
    pub fn ext_gcd(a: usize, b: usize) -> (isize, isize) {
        if b == 0 {
            return (1, 0);
        }

        let q = (a / b) as isize;
        let r = a % b;
        let (x1, y1) = ext_gcd(b, r);
        (y1, x1 - q * y1)
    }

    #[derive(Debug, Copy, Clone)]
    pub struct ModInt(pub usize);

    impl From<usize> for ModInt {
        fn from(n: usize) -> ModInt {
            ModInt(n % MODULO)
        }
    }

    impl From<isize> for ModInt {
        fn from(n: isize) -> ModInt {
            // TODO: use TryFrom
            ModInt(positive_rem(n, MODULO as usize))
        }
    }

    impl From<i32> for ModInt {
        fn from(n: i32) -> ModInt {
            // TODO: use TryFrom
            ModInt(positive_rem(n as isize, MODULO as usize))
        }
    }

    impl ModInt {
        #[allow(unused)]
        pub fn pow(self, p: usize) -> ModInt {
            if self == ModInt::from(0) {
                return ModInt::from(0);
            }
            if p == 0 {
                return ModInt::from(1);
            }

            if p % 2 == 0 {
                let half = self.pow(p / 2);
                half * half
            } else {
                self.pow(p - 1) * self
            }
        }

        // when MODULO is prime
        #[allow(unused)]
        pub fn inv(self) -> ModInt {
            let (x, _) = ext_gcd(self.0 as usize, MODULO as usize);
            ModInt::from(x)
        }
    }

    impl Add for ModInt {
        type Output = ModInt;

        fn add(self, other: ModInt) -> ModInt {
            ModInt::from(self.0 + other.0)
        }
    }

    impl Sub for ModInt {
        type Output = ModInt;

        fn sub(self, other: ModInt) -> ModInt {
            ModInt::from(self.0 as isize - other.0 as isize)
        }
    }

    impl Mul for ModInt {
        type Output = ModInt;

        fn mul(self, other: ModInt) -> ModInt {
            ModInt::from(self.0 * other.0)
        }
    }

    impl Neg for ModInt {
        type Output = ModInt;

        fn neg(self) -> Self::Output {
            ModInt::from(0) - self
        }
    }

    impl AddAssign for ModInt {
        fn add_assign(&mut self, other: Self) {
            *self = *self + other;
        }
    }

    impl MulAssign for ModInt {
        fn mul_assign(&mut self, other: Self) {
            *self = *self * other;
        }
    }

    impl SubAssign for ModInt {
        fn sub_assign(&mut self, other: Self) {
            *self = *self - other;
        }
    }

    impl PartialEq for ModInt {
        fn eq(&self, &other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for ModInt {}

    #[derive(Debug)]
    pub struct ModIntUtil {
        factorial: Vec<ModInt>,
        factorial_inv: Vec<ModInt>,
        inv: Vec<ModInt>,
    }

    impl ModIntUtil {
        #[allow(unused)]
        pub fn new() -> ModIntUtil {
            ModIntUtil {
                factorial: vec![ModInt::from(1), ModInt::from(1)],
                factorial_inv: vec![ModInt::from(1), ModInt::from(1)],
                inv: vec![ModInt::from(0), ModInt::from(1)],
            }
        }

        fn calc_cache(&mut self, n: usize) {
            let len = self.factorial.len();
            if len < n + 1 {
                for i in len..(n + 1) {
                    let prev = *self.factorial.last().unwrap();
                    self.factorial.push(prev * ModInt::from(i));

                    let inv_i = -self.inv[MODULO % i] * ModInt::from(MODULO / i);
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
}

// Tests for MODULO = 11
//
// #[cfg(test)]
// mod tests {
//     use modint::*;
//
//     #[test]
//     fn eq() {
//         assert_eq!(ModInt::from(1), ModInt::from((1 + MODULO) as isize));
//     }
//
//     #[test]
//     fn add1() {
//         assert_eq!(ModInt::from(1), ModInt::from(6) + ModInt::from(6));
//     }
//
//     #[test]
//     fn add2() {
//         assert_eq!(ModInt::from(0), ModInt::from(5) + ModInt::from(6));
//     }
//
//     #[test]
//     fn sub1() {
//         assert_eq!(ModInt::from(2), ModInt::from(3) - ModInt::from(1));
//     }
//
//     #[test]
//     fn sub2() {
//         assert_eq!(ModInt::from(10), ModInt::from(1) - ModInt::from(2));
//     }
//
//     #[test]
//     fn mul() {
//         assert_eq!(ModInt::from(3), ModInt::from(5) * ModInt::from(5));
//     }
//
//     #[test]
//     fn pow() {
//         assert_eq!(ModInt::from(5), ModInt::from(7).pow(2));
//     }
//
//     #[test]
//     fn inv() {
//         assert_eq!(ModInt::from(1), ModInt::from(2) * ModInt::from(2).inv());
//         assert_eq!(ModInt::from(1), ModInt::from(3) * ModInt::from(3).inv());
//     }
//
//     #[test]
//     fn ext_gcd_test() {
//         let a = 12isize;
//         let b = 18isize;
//         let (x, y) = ext_gcd(a as usize, b as usize);
//         assert_eq!(6, a * x + b * y);
//
//         let a = 4isize;
//         let b = 12isize;
//         let (x, y) = ext_gcd(a as usize, b as usize);
//         assert_eq!(4, a * x + b * y);
//     }
//
//     #[test]
//     fn binom_coef() {
//         let mut modint_util = ModIntUtil::new();
//         assert_eq!(ModInt::from(6), modint_util.binom_coef(4, 2));
//         assert_eq!(ModInt::from(3), modint_util.binom_coef(3, 1));
//         assert_eq!(ModInt::from(1), modint_util.binom_coef(3, 0));
//         assert_eq!(ModInt::from(10), modint_util.binom_coef(5, 2));
//     }
// }
