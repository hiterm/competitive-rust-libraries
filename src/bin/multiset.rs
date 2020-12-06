use competitive_rust_libraries::multiset::BTreeMultiSet;

fn main() {
    let mut set = BTreeMultiSet::new();
    set.insert(1i64);
    set.insert(1i64);
    dbg!(&set);
}
