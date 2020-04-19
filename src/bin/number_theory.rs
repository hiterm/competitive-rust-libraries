#![allow(unused)]
use std::collections::{HashSet, HashMap};
#[allow(unused_imports)]
use std::time::Instant;

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

    // for newer rust version
    //
    // println!("{:?}", divisors_set(12));
    // let start = Instant::now();
    // println!("{:?}", divisors_set(10_000_000_000_000));
    // let end = start.elapsed();
    // println!("{}ms", end.as_millis());
    //
    // println!("{:?}", divisors_vec(12));
    // let start = Instant::now();
    // println!("{:?}", divisors_vec(10_000_000_000_000));
    // let end = start.elapsed();
    // println!("{}ms", end.as_millis());
}

fn gcd(a: usize, b: usize) -> usize {
    let c = a % b;
    if c == 0 {
        b
    } else {
        gcd(b, c)
    }
}

fn prime_sieve(n: usize) -> Vec<u64> {
    let mut table: Vec<u64> = vec![0; n + 1];
    let mut primes: Vec<u64> = Vec::new();

    for i in 2..=n {
        if table[i] == 0 {
            primes.push(i as u64);
            for j in 2..n {
                if i * j > n {
                    break
                }
                table[i * j] = 1
            }
        }
    }

    primes
}

fn prime_factor(n: u64) -> HashMap<u64, u64> {
    let sqrt = (n as f64).sqrt() as u64;
    let mut rest = n;
    let mut factors = HashMap::new();
    for d in 2..=sqrt {
        while rest % d == 0 {
            let count = factors.entry(d).or_insert(0);
            *count += 1;
            rest /= d;
        }
    }
    if rest != 1 {
        factors.insert(rest, 1);
    }

    factors
}

fn divisors_vec(n: u64) -> Vec<u64> {
    prime_factor(n).iter().fold(vec![1], |acc, (&p, &pow)| {
        (0..=pow).flat_map(|i| acc.iter().map(move |a| a * p.pow(i as u32))).collect()
    })
}

fn divisors_set(n: u64) -> HashSet<u64> {
    let mut set = HashSet::new();
    set.insert(1);

    prime_factor(n).iter().fold(set, |acc, (&p, &pow)| {
        (0..=pow).flat_map(|i| acc.iter().map(move |a| a * p.pow(i as u32))).collect()
    })
}

fn binom_coef(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }

    if n - k < k {
        return binom_coef(n, n - k);
    }

    let mut ret = 1;
    for i in 1..=k {
        ret = ret * (n - k + i) / i
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binom_coef_test(){
        assert_eq!(6, binom_coef(4, 2));
        assert_eq!(3, binom_coef(3, 2));
        assert_eq!(20, binom_coef(6, 3));
        assert_eq!(1, binom_coef(10, 0));
        assert_eq!(10, binom_coef(10, 1));
        assert_eq!(1, binom_coef(10, 10));
        assert_eq!(0, binom_coef(2, 3));
    }
}
