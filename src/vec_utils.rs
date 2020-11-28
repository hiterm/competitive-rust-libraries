pub fn eprint_2dim_vec<T: ToString>(v: &[Vec<T>]) {
    eprintln!("[");
    for row in v {
        let s = row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
        eprintln!("    [{}],", s);
    }
    eprintln!("]");
}
