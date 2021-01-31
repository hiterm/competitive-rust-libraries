#![allow(unused)]
use std::collections::{HashMap, HashSet};

pub fn gcd(a: usize, b: usize) -> usize {
    let c = a % b;
    if c == 0 {
        b
    } else {
        gcd(b, c)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub fn mod_inv(x: i64, m: i64) -> i64 {
    let (x, _) = ext_gcd(x, m);
    ((x % m) + m) % m
}

pub fn ext_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }

    let q = a / b;
    let r = a % b;
    let (x1, y1) = ext_gcd(b, r);
    (y1, x1 - q * y1)
}

pub fn prime_sieve(n: usize) -> Vec<u64> {
    let mut table: Vec<u64> = vec![0; n + 1];
    let mut primes: Vec<u64> = Vec::new();

    for i in 2..=n {
        if table[i] == 0 {
            primes.push(i as u64);
            for j in 2..n {
                if i * j > n {
                    break;
                }
                table[i * j] = 1
            }
        }
    }

    primes
}

pub fn prime_factor(n: u64) -> HashMap<u64, u64> {
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

pub fn divisors(n: u64) -> Vec<u64> {
    let mut ret = vec![];

    for k in 1..=n {
        if k * k > n {
            break;
        }

        if n % k == 0 {
            ret.push(k);
            if n / k != k {
                ret.push(n / k);
            }
        }
    }

    ret
}

pub fn divisors_using_prime_factor(n: u64) -> Vec<u64> {
    prime_factor(n).iter().fold(vec![1], |acc, (&p, &pow)| {
        (0..=pow)
            .flat_map(|i| acc.iter().map(move |a| a * p.pow(i as u32)))
            .collect()
    })
}

pub fn binom_coef(n: usize, k: usize) -> usize {
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

pub struct Factorize {
    min_factor: Vec<usize>,
}

impl Factorize {
    pub fn new(max: usize) -> Factorize {
        let min_factor = Factorize::prime_min_factor(max);
        Factorize { min_factor }
    }

    fn prime_min_factor(max: usize) -> Vec<usize> {
        let mut min_factor = vec![0usize; max + 1];
        min_factor[0] = 0;
        min_factor[1] = 1;

        for i in 2..=max {
            if min_factor[i] == 0 {
                min_factor[i] = i;
                let mut j = i * 2;
                while j <= max {
                    if min_factor[j] == 0 {
                        min_factor[j] = i;
                    }
                    j += i;
                }
            }
        }

        min_factor
    }

    pub fn factorize(&mut self, n: usize) -> Vec<(usize, usize)> {
        let mut factors = vec![];
        let mut n = n;
        while n != 1 {
            let prime = self.min_factor[n];
            let mut exp = 0;
            while self.min_factor[n] == prime {
                exp += 1;
                n /= prime;
            }
            factors.push((prime, exp));
        }

        factors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binom_coef_test() {
        assert_eq!(6, binom_coef(4, 2));
        assert_eq!(3, binom_coef(3, 2));
        assert_eq!(20, binom_coef(6, 3));
        assert_eq!(1, binom_coef(10, 0));
        assert_eq!(10, binom_coef(10, 1));
        assert_eq!(1, binom_coef(10, 10));
        assert_eq!(0, binom_coef(2, 3));
    }

    #[test]
    fn ext_gcd_test_5_7() {
        let (a, b) = ext_gcd(5, 7);
        let actual = a * 5 + b * 7;
        assert_eq!(1, actual);
    }

    #[test]
    fn ext_gcd_test_4_6() {
        let (a, b) = ext_gcd(4, 6);
        let actual = a * 4 + b * 6;
        assert_eq!(2, actual);
    }

    #[test]
    fn mod_inv_test() {
        assert_eq!(1, (3 * mod_inv(3, 7)) % 7);
    }
}
