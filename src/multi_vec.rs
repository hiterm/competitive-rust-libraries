#[allow(unused_macros)]
macro_rules! multi_vec {
    ( $elem:expr; $num:expr ) => (vec![$elem; $num]);
    ( $elem:expr; $num:expr, $($rest:expr),* ) => (vec![multi_vec![$elem; $($rest),*]; $num]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_vec_test() {
        let v = multi_vec![0; 2, 2];
        assert_eq!(vec![vec![0, 0], vec![0, 0]], v);
    }
}
