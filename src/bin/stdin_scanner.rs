use std::io::stdin;

use competitive_rust_libraries::stdin_scanner::StdinScanner;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut sc = StdinScanner::new(stdin);
    let n: usize = sc.next();
    println!("{}", n);
    let v: Vec<u64> = sc.next_vec(n);
    println!("{:?}", v);
}
