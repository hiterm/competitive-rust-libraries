#[allow(unused_macros)]
macro_rules! chmax {
    ($a:expr, $b:expr) => {$a = std::cmp::max($a, $b)}
}

#[allow(unused_macros)]
macro_rules! chmin {
    ($a:expr, $b:expr) => {$a = std::cmp::min($a, $b)}
}
