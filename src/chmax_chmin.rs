#[allow(unused_macros)]
macro_rules! chmax {
    ($a:expr, $b:expr) => {
        $a = std::cmp::max($a, $b)
    };
}

#[allow(unused_macros)]
macro_rules! chmin {
    ($a:expr, $b:expr) => {
        $a = std::cmp::min($a, $b)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn chmax_test() {
        let mut a = 5;
        chmax!(a, 1);
        assert_eq!(5, a);
        chmax!(a, 10);
        assert_eq!(10, a);
    }
}
