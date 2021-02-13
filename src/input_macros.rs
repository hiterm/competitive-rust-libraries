#[allow(unused_macros)]
#[macro_export]
macro_rules! getl {
    ( $r:expr; $( $t:tt ),* ) => {
        {
            use std::io::BufRead;
            let mut s = String::new();
            $r.read_line(&mut s).unwrap();
            let s = s.trim_end();
            let mut ws = s.split_whitespace();
            ( $( $crate::getl_parse_one!( ws, $t ) ),* )
        }
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! getl_parse_one {
    // ($it:expr, [$t:ty; $n:expr]) => {
    //     let mut v = vec![];
    //     for _ in 0..$n {
    //         v.push($it.next().unwrap().parse::<$t>().unwrap());
    //     }
    //     v
    // };
    ($it:expr, $t:ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! getl_vec {
    ( $t:ty ) => {{
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let s = s.trim_end();
        s.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<$t>>()
    }};
}
