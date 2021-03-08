use std::ops::Mul;

use ac_library_rs::ModInt1000000007;

// Zero trait

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero_int(($($ty:ty),*) => {
    $(
        impl Zero for $ty {
            fn zero() -> Self {
                0
            }
        }
    )*
});

impl_zero_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

// One trait

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one_int(($($ty:ty),*) => {
    $(
        impl One for $ty {
            fn one() -> Self {
                1
            }
        }
    )*
});

impl_one_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

// Pow trait

trait Pow {
    fn pow(&self, n: u64) -> Self;
}

impl<T> Pow for T
where
    T: Clone + Mul<Output = T> + One,
{
    fn pow(&self, n: u64) -> Self {
        let mut n = n;
        let mut x = self.clone();
        let mut r = Self::one();
        while n > 0 {
            if n & 1 == 1 {
                r = r * x.clone();
            }
            x = x.clone() * x;
            n >>= 1;
        }
        r
    }
}

// impl Zero and One for ModInt

impl Zero for ModInt1000000007 {
    fn zero() -> Self {
        0.into()
    }
}

impl One for ModInt1000000007 {
    fn one() -> Self {
        1.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _3pow3() {
        assert_eq!(27, Pow::pow(&3u64, 3));
    }

    #[test]
    fn _2pow0() {
        assert_eq!(1, Pow::pow(&2u64, 0));
    }

    #[test]
    fn _0pow1() {
        assert_eq!(0, Pow::pow(&0u64, 1));
    }
}
