use competitive_rust_libraries::number_theory::*;

fn main() {
    let a = 12;
    let b = 18;

    let gcd = gcd(a, b);
    println!("gcd: {}", gcd);
    println!("sqrt: {}", (gcd as f64).sqrt() as usize);

    let primes = prime_sieve(100);
    println!("primes: {:?}", primes);

    for i in 1..101 {
        println!("prime factor of {}: {:?}", i, prime_factor(i));
    }

    println!("{:?}", divisors(12));
}
