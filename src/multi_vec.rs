macro_rules! multi_vec {
    ( $elem:expr; $num:expr ) => (vec![$elem; $num]);
    ( $elem:expr; $num:expr, $($rest:expr),* ) => (vec![multi_vec![$elem; $($rest),*]; $num]);
}
