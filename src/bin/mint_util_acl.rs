use ac_library_rs::modint::ModInt1000000007 as Mint;
use competitive_rust_libraries::mint_util_acl::MintUtil;

fn main() {
    let mut mint_util = MintUtil::<Mint>::new();
    let a = mint_util.binom_coef(4, 2);
    println!("{}", a);
}
