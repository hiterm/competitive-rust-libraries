use std::collections::{HashSet, HashMap};
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

fn divisors_vec(n: usize) -> Vec<usize> {
    prime_factor(n).iter().fold(vec![1], |acc, (&p, &pow)| {
        (0..(pow + 1)).flat_map(|i| acc.iter().map(move |a| a * p.pow(i as u32))).collect()
    })
}

fn divisors_set(n: usize) -> HashSet<usize> {
    let mut set = HashSet::new();
    set.insert(1);

    // 素朴な実装
    // for (p, pow) in prime_factor(n) {
    //     let set_prev = set.clone();
    //     for i in 1..=pow {
    //         set.extend(set_prev.iter().map(|a| a * p.pow(i as u32)));
    //     }
    // }

    // iterator使用
    prime_factor(n).iter().fold(set, |acc, (&p, &pow)| {
        (0..(pow + 1)).flat_map(|i| acc.iter().map(move |a| a * p.pow(i as u32))).collect()
    })
}

fn binom_coef(n: usize, k: usize) -> usize {
    let mut ret = 1;
    for i in 1..(k + 1) {
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
    }
}
