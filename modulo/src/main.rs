use std::ops::{Add, Sub, Mul};

fn main() {
    println!("{:?}", Modulo(5) + Modulo(6));
    println!("{:?}", Modulo(1) - Modulo(2));
    println!("{:?}", Modulo(3) - Modulo(1));
    println!("{:?}", Modulo(5) * Modulo(5));
    println!("{:?}", Modulo(3).pow(2));
    println!("{:?}", Modulo(3).pow(1000000000000000));
}

#[derive(Debug, Copy, Clone)]
struct Modulo(isize);

impl Modulo {
    const MODULO: isize = 10;

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
}

impl Add for Modulo {
    type Output = Modulo;

    fn add(self, other: Modulo) -> Modulo {
        Modulo((self.0 + other.0) % Modulo::MODULO)
    }
}

impl Sub for Modulo {
    type Output = Modulo;

    fn sub(self, other: Modulo) -> Modulo {
        let mut value = (self.0 - other.0) % Modulo::MODULO;
        if value < 0 {
            value += Modulo::MODULO;
        }
        Modulo(value)
    }
}

impl Mul for Modulo {
    type Output = Modulo;

    fn mul(self, other: Modulo) -> Modulo {
        Modulo((self.0 * other.0) % Modulo::MODULO)
    }
}

impl PartialEq for Modulo {
    fn eq(&self, &other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Modulo {}
