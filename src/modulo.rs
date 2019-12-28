use modulo::Modulo;

fn main() {
    println!("{:?}", Modulo(5) + Modulo(6));
    println!("{:?}", Modulo(1) - Modulo(2));
    println!("{:?}", Modulo(3) - Modulo(1));
    println!("{:?}", Modulo(5) * Modulo(5));
    println!("{:?}", Modulo(3).pow(2));
    println!("{:?}", Modulo(3).pow(1000000000000000));
    println!("{:?}", Modulo(2).inv());
    println!("{:?}", Modulo(3).inv());
}

mod modulo {
    use std::ops::{Add, Mul, Sub};

    const MODULO: usize = 11;

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

    impl Modulo {
        fn from_isize(n: isize) -> Modulo {
            // TODO: use TryFrom
            Modulo(positive_rem(n, MODULO as usize))
        }

        pub fn pow(self, p: usize) -> Modulo {
            if self == Modulo(0) {
                return Modulo(0);
            }
            if p == 0 {
                return Modulo(1);
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
            Modulo::from_isize(x)
        }

        // when MODULO is prime
        pub fn binom_coef(n: isize, k: isize) -> Modulo {
            let mut ret = Modulo::from_isize(1);
            for i in 1..(k + 1) {
                ret = ret * Modulo::from_isize(n - i + 1) * Modulo::from_isize(i).inv();
            }
            ret
        }
    }

    impl Add for Modulo {
        type Output = Modulo;

        fn add(self, other: Modulo) -> Modulo {
            Modulo((self.0 + other.0) % MODULO)
        }
    }

    impl Sub for Modulo {
        type Output = Modulo;

        fn sub(self, other: Modulo) -> Modulo {
            Modulo(positive_rem(
                self.0 as isize - other.0 as isize,
                MODULO as usize,
            ))
        }
    }

    impl Mul for Modulo {
        type Output = Modulo;

        fn mul(self, other: Modulo) -> Modulo {
            Modulo((self.0 * other.0) % MODULO)
        }
    }

    impl PartialEq for Modulo {
        fn eq(&self, &other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for Modulo {}

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn eq() {
            assert_eq!(
                Modulo::from_isize(1),
                Modulo::from_isize((1 + MODULO) as isize)
            );
        }

        #[test]
        fn add1() {
            assert_eq!(Modulo(1), Modulo(6) + Modulo(6));
        }

        #[test]
        fn add2() {
            assert_eq!(Modulo(0), Modulo(5) + Modulo(6));
        }

        #[test]
        fn sub1() {
            assert_eq!(Modulo(2), Modulo(3) - Modulo(1));
        }

        #[test]
        fn sub2() {
            assert_eq!(Modulo(10), Modulo(1) - Modulo(2));
        }

        #[test]
        fn mul() {
            assert_eq!(Modulo(3), Modulo(5) * Modulo(5));
        }

        #[test]
        fn pow() {
            assert_eq!(Modulo(5), Modulo(7).pow(2));
        }

        #[test]
        fn inv() {
            assert_eq!(Modulo(1), Modulo(2) * Modulo(2).inv());
            assert_eq!(Modulo(1), Modulo(3) * Modulo(3).inv());
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
            assert_eq!(Modulo(6), Modulo::binom_coef(4, 2));
        }
    }
}
