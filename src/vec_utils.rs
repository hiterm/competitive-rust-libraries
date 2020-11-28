pub fn eprint_2dim_vec<T: ToString>(v: &[Vec<T>]) {
    println!("[");
    for row in v {
        let s = row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
        println!("    [{}],", s);
    }
    println!("]");
}
