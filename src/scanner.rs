use std::str::{FromStr, SplitWhitespace};

pub struct Scanner<'a> {
    iter: SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(s: &'a str) -> Scanner<'a> {
        Scanner {
            iter: s.split_whitespace(),
        }
    }

    pub fn next<T>(&mut self) -> T
    where
        T: FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        {
            self.iter.next().unwrap().parse().unwrap()
        }
    }
}

pub fn read_stdin() -> String {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s
}
