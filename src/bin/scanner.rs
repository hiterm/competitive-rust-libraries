use competitive_rust_libraries::scanner::{read_stdin, Scanner};

fn main() {
    let s = read_stdin();
    let mut sc = Scanner::new(&s);
    let a: usize = sc.next();
    println!("{}", a);
    // let v: Vec<usize> = sc.next_vec(3);
    // println!("{:?}", v);
}
