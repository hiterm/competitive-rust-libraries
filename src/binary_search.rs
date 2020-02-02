#![allow(unused)]

fn main() {}

// TODO: binary_searchトレイトに統合する
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

// TODO: トレイトにしてメソッド化する
fn binary_search_vec_max<T, F>(v: &[T], condition: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    if v.is_empty() {
        return None;
    }

    let len = v.len();
    if condition(&v[len - 1]) {
        return Some(len - 1);
    }
    if !condition(&v[0]) {
        return None;
    }

    let mut left = 0;
    let mut right = len - 1;
    while left + 1 < right {
        let mid = (left + right) / 2;
        if condition(&v[mid]) {
            left = mid;
        } else {
            right = mid;
        }
    }

    Some(left)
}

fn binary_search_vec_min<T, F>(v: &[T], condition: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    if v.is_empty() {
        return None;
    }

    let len = v.len();

    if condition(&v[0]) {
        return Some(0);
    }
    if !condition(&v[len - 1]) {
        return None;
    }

    let mut left = 0;
    let mut right = len - 1;
    while left + 1 < right {
        let mid = (left + right) / 2;
        if condition(&v[mid]) {
            right = mid;
        } else {
            left = mid;
        }
    }

    Some(right)
}

// trait BinarySearch {
//     type U;
//
//     fn binary_search_max<F>(&self, f: F) -> usize
//         where F: Fn(U) -> bool;
//     fn binary_search_min<F>(&self, f: F) -> usize;
// }
//
// impl Vec {
//     fn binary_search_max<F>(&self, f: F) -> usize
//     {
//         let len = self.len();
//         let mut left = 0;
//         let mut right = len - 1;
//
//         right
//     }
//
//     fn binary_search_min() -> usize {}
// }

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

    #[test]
    fn binary_search_vec_max_test() {
        let v1 = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(Some(3), binary_search_vec_max(&v1, |x| *x <= 3));
        assert_eq!(Some(0), binary_search_vec_max(&v1, |x| *x <= 0));
        assert_eq!(Some(5), binary_search_vec_max(&v1, |x| *x >= 0));
        assert_eq!(None, binary_search_vec_max(&v1, |x| *x >= 100));

        let v2: Vec<usize> = vec![];
        assert_eq!(None, binary_search_vec_max(&v2, |x| *x <= 3));
    }

    #[test]
    fn binary_search_vec_min_test() {
        let v1 = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(Some(3), binary_search_vec_min(&v1, |x| *x >= 3));
        assert_eq!(Some(5), binary_search_vec_min(&v1, |x| *x >= 5));
        assert_eq!(Some(0), binary_search_vec_min(&v1, |x| *x <= 5));
        assert_eq!(None, binary_search_vec_min(&v1, |x| *x >= 100));

        let v2: Vec<usize> = vec![];
        assert_eq!(None, binary_search_vec_max(&v2, |x| *x >= 3));
    }
}
