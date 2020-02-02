use modulo::Modulo;

fn main() {
    println!("{:?}", Modulo::from(5) + Modulo::from(6));
    println!("{:?}", Modulo::from(1) - Modulo::from(2));
    println!("{:?}", Modulo::from(3) - Modulo::from(1));
    println!("{:?}", Modulo::from(5) * Modulo::from(5));
    println!("{:?}", Modulo::from(3).pow(2));
    println!("{:?}", Modulo::from(3).pow(1000000000000000));
    println!("{:?}", Modulo::from(2).inv());
    println!("{:?}", Modulo::from(3).inv());

    let mut modulo_utils = modulo::ModuloUtils::new();
    println!("{:?}", modulo_utils.binom_coef(4, 2));
    println!("{:?}", modulo_utils);
}

mod modulo {
    use std::ops::{Add, Mul, Sub, Neg};

    const MODULO: usize = 1_000_000_007;
    // const MODULO: usize = 11;

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
    fn ext_gcd(a: usize, b: usize) -> (isize, isize) {
        if b == 0 {
            return (1, 0);
        }

        let q = (a / b) as isize;
        let r = a % b;
        let (x1, y1) = ext_gcd(b, r);
        (y1, x1 - q * y1)
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Modulo(pub usize);

    impl From<usize> for Modulo {
        fn from(n: usize) -> Modulo {
            Modulo(n % MODULO)
        }
    }

    impl From<isize> for Modulo {
        fn from(n: isize) -> Modulo {
            // TODO: use TryFrom
            Modulo(positive_rem(n, MODULO as usize))
        }
    }

    impl From<i32> for Modulo {
        fn from(n: i32) -> Modulo {
            // TODO: use TryFrom
            Modulo(positive_rem(n as isize, MODULO as usize))
        }
    }

    impl Modulo {
        pub fn pow(self, p: usize) -> Modulo {
            if self == Modulo::from(0) {
                return Modulo::from(0);
            }
            if p == 0 {
                return Modulo::from(1);
            }

            if p % 2 == 0 {
                let half = self.pow(p / 2);
                half * half
            } else {
                self.pow(p - 1) * self
            }
        }

        // when MODULO is prime
        pub fn inv(self) -> Modulo {
            let (x, _) = ext_gcd(self.0 as usize, MODULO as usize);
            Modulo::from(x)
        }

    }

    impl Add for Modulo {
        type Output = Modulo;

        fn add(self, other: Modulo) -> Modulo {
            Modulo::from(self.0 + other.0)
        }
    }

    impl Sub for Modulo {
        type Output = Modulo;

        fn sub(self, other: Modulo) -> Modulo {
            Modulo::from(self.0 as isize - other.0 as isize)
        }
    }

    impl Mul for Modulo {
        type Output = Modulo;

        fn mul(self, other: Modulo) -> Modulo {
            Modulo::from(self.0 * other.0)
        }
    }

    impl Neg for Modulo {
        type Output = Modulo;

        fn neg(self) -> Self::Output {
            Modulo::from(0) - self
        }
    }

    impl PartialEq for Modulo {
        fn eq(&self, &other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for Modulo {}

    #[derive(Debug)]
    pub struct ModuloUtils {
        factorial: Vec<Modulo>,
        factorial_inv: Vec<Modulo>,
        inv: Vec<Modulo>,
    }

    impl ModuloUtils {
        pub fn new() -> ModuloUtils {
            ModuloUtils {
                factorial: vec![Modulo::from(1), Modulo::from(1)],
                factorial_inv: vec![Modulo::from(1), Modulo::from(1)],
                inv: vec![Modulo::from(0), Modulo::from(1)],
            }
        }

        // when MODULO is prime
        pub fn binom_coef(&mut self, n: usize, k: usize) -> Modulo {
            if n < k {
                return Modulo::from(0);
            }

            let len = self.factorial.len();
            if len < n + 1 {
                for i in len..(n + 1) {
                    let prev = *self.factorial.last().unwrap();
                    self.factorial.push(prev * Modulo::from(i));

                    let inv_i = - self.inv[MODULO % i] * Modulo::from(MODULO / i);
                    self.inv.push(inv_i);

                    let prev = *self.factorial_inv.last().unwrap();
                    self.factorial_inv.push(prev * self.inv[i]);
                }
            }
            self.factorial[n] * self.factorial_inv[k] * self.factorial_inv[n - k]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn eq() {
            assert_eq!(
                Modulo::from(1),
                Modulo::from((1 + MODULO) as isize)
            );
        }

        #[test]
        fn add1() {
            assert_eq!(Modulo::from(1), Modulo::from(6) + Modulo::from(6));
        }

        #[test]
        fn add2() {
            assert_eq!(Modulo::from(0), Modulo::from(5) + Modulo::from(6));
        }

        #[test]
        fn sub1() {
            assert_eq!(Modulo::from(2), Modulo::from(3) - Modulo::from(1));
        }

        #[test]
        fn sub2() {
            assert_eq!(Modulo::from(10), Modulo::from(1) - Modulo::from(2));
        }

        #[test]
        fn mul() {
            assert_eq!(Modulo::from(3), Modulo::from(5) * Modulo::from(5));
        }

        #[test]
        fn pow() {
            assert_eq!(Modulo::from(5), Modulo::from(7).pow(2));
        }

        #[test]
        fn inv() {
            assert_eq!(Modulo::from(1), Modulo::from(2) * Modulo::from(2).inv());
            assert_eq!(Modulo::from(1), Modulo::from(3) * Modulo::from(3).inv());
        }

        #[test]
        fn ext_gcd_test() {
            let a = 12isize;
            let b = 18isize;
            let (x, y) = ext_gcd(a as usize, b as usize);
            assert_eq!(6, a * x + b * y);

            let a = 4isize;
            let b = 12isize;
            let (x, y) = ext_gcd(a as usize, b as usize);
            assert_eq!(4, a * x + b * y);
        }

        #[test]
        fn binom_coef() {
            let mut modulo_utils = ModuloUtils::new();
            assert_eq!(Modulo::from(6), modulo_utils.binom_coef(4, 2));
            assert_eq!(Modulo::from(3), modulo_utils.binom_coef(3, 1));
            assert_eq!(Modulo::from(1), modulo_utils.binom_coef(3, 0));
            assert_eq!(Modulo::from(10), modulo_utils.binom_coef(5, 2));
        }
    }
}
