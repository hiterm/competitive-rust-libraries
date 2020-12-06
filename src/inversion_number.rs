// Not well tested
fn inversion_number(v: &[usize], max: usize) -> usize {
    let mut inv = 0;
    let mut bit = FenwickTree::new(max + 1, 0);
    for (i, vi) in v.iter().copied().enumerate() {
        bit.add(vi, 1);
        inv += i + 1 - bit.sum(0, vi + 1);
    }
    inv
}
