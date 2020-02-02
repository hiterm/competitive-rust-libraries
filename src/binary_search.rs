#![allow(unused)]

fn main() {

}

fn binary_search_int_max<F>(min: isize, max: isize, condition: F) -> Option<isize>
where
    F: Fn(isize) -> bool,
{
    if condition(max) {
        return Some(max);
    }
    if !condition(min) {
        return None;
    }

    let mut left = min;
    let mut right = max;
    while left + 1 < right {
        let mid = (left + right) / 2;

        if condition(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    Some(left)
}

fn binary_search_int_min<F>(min: isize, max: isize, condition: F) -> Option<isize>
where
    F: Fn(isize) -> bool,
{
    if condition(min) {
        return Some(min);
    }
    if !condition(max) {
        return None;
    }

    let mut left = min;
    let mut right = max;
    while left + 1 < right {
        let mid = (left + right) / 2;

        if condition(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }

    Some(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_int_max_test() {
        assert_eq!(Some(5), binary_search_int_max(1, 10, |a| a <= 5));
        assert_eq!(Some(1), binary_search_int_max(1, 10, |a| a <= 1));
        assert_eq!(Some(10), binary_search_int_max(1, 10, |a| a > 0));
        assert_eq!(None, binary_search_int_max(5, 10, |a| a <= 1));
    }

    #[test]
    fn binary_search_int_min_test() {
        assert_eq!(Some(5), binary_search_int_min(1, 10, |a| a >= 5));
        assert_eq!(Some(1), binary_search_int_min(1, 10, |a| a < 50));
        assert_eq!(Some(10), binary_search_int_min(1, 10, |a| a >= 10));
        assert_eq!(None, binary_search_int_min(1, 5, |a| a >= 10));
    }
}
