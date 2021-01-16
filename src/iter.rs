macro_rules! iter_prod {
    ($i1:expr, $i2:expr) => {
        $i1.flat_map(|a| $i2.map(move |b| (a, b)))
    };
}

#[allow(unused)]
fn main() {
    println!("{:?}", iter_prod!(0..3, 0..3).collect::<Vec<_>>())
}
