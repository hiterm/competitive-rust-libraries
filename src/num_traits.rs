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
