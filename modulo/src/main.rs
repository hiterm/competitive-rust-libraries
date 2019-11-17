use std::ops::{Add, Sub, Mul};

fn main() {
    println!("{:?}", Modulo(5) + Modulo(6));
    println!("{:?}", Modulo(1) - Modulo(2));
    println!("{:?}", Modulo(3) - Modulo(1));
    println!("{:?}", Modulo(5) * Modulo(5));
    println!("{:?}", Modulo(3).pow(2));
    println!("{:?}", Modulo(3).pow(1000000000000000));
}

fn positive_rem(a: isize, b: usize) -> isize {
    let b = b as isize;
    let mut value = a % b;
    if value < 0 {
        value += b;
    }
    value
}

#[derive(Debug, Copy, Clone)]
struct Modulo(isize);

const MODULO: isize = 11;

impl Modulo {

    fn new(n: isize) -> Modulo {
        Modulo(n % MODULO)
    }

    fn pow(self, p: usize) -> Modulo {
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

    // when p is prime
    fn inv(self) -> Modulo {
        self.pow((MODULO - 2) as usize)
    }

    // when p is prime
    fn binom_coef(n: isize, k: isize) -> Modulo {
        let mut ret = Modulo::new(1);
        for i in 1..(k + 1) {
            ret = ret * Modulo::new(n - i + 1) * Modulo::new(i).inv();
            // debugln!("{:?}", ret);
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
        Modulo(positive_rem(self.0 - other.0, MODULO as usize))
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
        assert_eq!(Modulo::new(1), Modulo::new(1 + MODULO));
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
    fn binom_coef() {
        assert_eq!(Modulo(6), Modulo::binom_coef(4, 2));
    }
}
