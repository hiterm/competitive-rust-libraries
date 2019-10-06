use std::collections::HashMap;

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
}

fn gcd(a: usize, b: usize) -> usize {
    let c = a % b;
    if c == 0 {
        b
    } else {
        gcd(b, c)
    }
}

fn prime_sieve(n: usize) -> Vec<usize> {
    let mut table: Vec<usize> = vec![0; (n + 1) as usize];
    let mut primes: Vec<usize> = Vec::new();

    for i in 2..(n+1) {
        if table[i as usize] == 0 {
            primes.push(i);
            for j in 2..n {
                if i * j > n {
                    break
                }
                table[(i * j) as usize] = 1
            }
        }
    }

    primes
}

fn prime_factor(n: usize) -> HashMap<usize, usize> {
    let mut factors = HashMap::new();

    let primes = prime_sieve((n as f64).sqrt() as usize);
    let mut rest = n;
    for p in primes {
        while rest % p == 0 {
            let count = factors.entry(p).or_insert(0);
            *count += 1;
            rest /= p;
        }
    }
    if rest != 1 {
        factors.insert(rest, 1);
    }

    factors
}
