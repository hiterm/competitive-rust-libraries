pub fn string_2dim_vec<T: ToString>(v: &[Vec<T>]) -> String {
    let mut ret = String::new();
    ret.push_str("[\n");
    for row in v {
        let s = row
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        ret.push_str(&format!("    [{}],\n", s));
    }
    ret.push_str("]");

    ret
}
