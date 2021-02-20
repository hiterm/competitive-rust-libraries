// slower than Scanner

use std::{
    io::{BufRead, StdinLock},
    str::{FromStr, SplitWhitespace},
};

pub struct StdinScanner<'a> {
    stdin: StdinLock<'a>,
    iter: SplitWhitespace<'a>,
}

impl<'a> StdinScanner<'a> {
    pub fn new(stdin: StdinLock<'a>) -> StdinScanner<'a> {
        StdinScanner {
            stdin,
            iter: "".split_whitespace(),
        }
    }

    pub fn next<T>(&mut self) -> T
    where
        T: FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let s = self.iter.next();
        match s {
            Some(s) => s.parse().unwrap(),
            None => {
                let mut line = String::new();
                self.stdin.read_line(&mut line).unwrap();
                let line = Box::leak(Box::new(line));
                self.iter = line.split_whitespace();
                self.next()
            }
        }
    }

    pub fn next_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut v = vec![];
        for _ in 0..n {
            v.push(self.next());
        }
        v
    }

    pub fn chars(&mut self) -> Vec<char> {
        let s: &str = self.iter.next().unwrap();
        s.chars().collect()
    }
}
