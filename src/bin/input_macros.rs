use std::io;

use competitive_rust_libraries::{getl, getl_vec};

fn main() {
    // let (a, b) = getl!(usize, usize);
    // let v = getl_vec!(usize);
    // let s = getl!(String);
    // println!("{} {}", a, b);
    // println!("{:?}", v);

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    // let (a, b) = getl!(reader; usize, usize);
    // println!("{} {}", a, b);
    let v = getl!(reader; [usize; 2]);
    println!("{:?}", v);
}
