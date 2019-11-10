use std::ops::Add;

fn main() {
    println!("{:?}", Modulo(2))
}

#[derive(Debug)]
struct Modulo(usize);

impl Modulo {
    const MODULO: usize = 3;
}

impl Add for Modulo {
    type Output = Modulo;

    fn add(self, other: Modulo) -> Modulo {
        Modulo ((self.0 + other.0) % Modulo::MODULO)
    }
}
