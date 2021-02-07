// https://en.wikipedia.org/wiki/Integer_square_root#Digit-by-digit_algorithm
pub fn floor_sqrt(n: i64) -> i64 {
    assert!(n >= 0);

    if n < 2 {
        return n;
    }

    let mut shift = 2;
    while n >> shift != 0 {
        shift += 2;
    }

    let mut result = 0;
    while shift >= 0 {
        result = result << 1;
        let large_cand = result + 1;
        if large_cand * large_cand <= n >> shift {
            result = large_cand
        }
        shift -= 2;
    }

    result
}

pub fn floor_sqrt_binary_search(x: i64) -> i64 {
    assert!(x >= 0);

    if x == 1 {
        return 1;
    }

    let mut left = 0;
    let mut right = x as i128;
    while left + 1 < right {
        let mid = (left + right) / 2;
        if mid * mid <= x as i128 {
            left = mid;
        } else {
            right = mid;
        }
    }
    left as i64
}

// https://en.wikipedia.org/wiki/Integer_square_root#Algorithm_using_Newton's_method
pub fn floor_sqrt_newton(n: i64) -> i64 {
    assert!(n >= 0);

    let mut x0 = n / 2;
    if x0 == 0 {
        return n;
    }
    let mut x1 = (x0 + n / x0) / 2;
    while x1 < x0 {
        x0 = x1;
        x1 = (x0 + n / x0) / 2;
    }
    x0
}

#[cfg(test)]
mod tests {
    use super::*;

    mod digit_by_digit {
        use super::*;

        #[test]
        fn _0() {
            assert_eq!(0, floor_sqrt(0));
        }

        #[test]
        fn _1() {
            assert_eq!(1, floor_sqrt(1));
        }

        #[test]
        fn _2() {
            assert_eq!(1, floor_sqrt(2));
        }

        #[test]
        fn _3() {
            assert_eq!(1, floor_sqrt(3));
        }

        #[test]
        fn _4() {
            assert_eq!(2, floor_sqrt(4));
        }

        #[test]
        fn _99_99_99_99_99_99_99() {
            assert_eq!(9999999, floor_sqrt(99_99_99_99_99_99_99));
        }

        #[test]
        fn _1_00_00_00_00_00_00_00() {
            assert_eq!(10000000, floor_sqrt(1_00_00_00_00_00_00_00));
        }
    }

    mod binary_search {
        use super::*;

        #[test]
        fn _0() {
            assert_eq!(0, floor_sqrt_binary_search(0));
        }

        #[test]
        fn _1() {
            assert_eq!(1, floor_sqrt_binary_search(1));
        }

        #[test]
        fn _2() {
            assert_eq!(1, floor_sqrt_binary_search(2));
        }

        #[test]
        fn _3() {
            assert_eq!(1, floor_sqrt_binary_search(3));
        }

        #[test]
        fn _4() {
            assert_eq!(2, floor_sqrt_binary_search(4));
        }
    }

    mod newton {
        use super::*;

        #[test]
        fn _0() {
            assert_eq!(0, floor_sqrt_newton(0));
        }

        #[test]
        fn _1() {
            assert_eq!(1, floor_sqrt_newton(1));
        }

        #[test]
        fn _2() {
            assert_eq!(1, floor_sqrt_newton(2));
        }

        #[test]
        fn _3() {
            assert_eq!(1, floor_sqrt_newton(3));
        }

        #[test]
        fn _4() {
            assert_eq!(2, floor_sqrt_newton(4));
        }
    }
}
