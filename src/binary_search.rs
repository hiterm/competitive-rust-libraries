#![allow(unused)]

fn main() {}

// TODO: Rangeのメソッドにする

fn binary_search_int_max<F>(min: isize, max: isize, condition: F) -> Option<isize>
where
    F: Fn(isize) -> bool,
{
    if max < min {
        return None;
    }

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
    if max < min {
        return None;
    }

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

trait BinarySearch<T, F> {
    fn binary_search_right(&self, condition: F) -> Option<usize>;
    fn binary_search_left(&self, condition: F) -> Option<usize>;
}

impl<T, F> BinarySearch<T, F> for Vec<T>
where
    F: Fn(&T) -> bool,
{
    fn binary_search_right(&self, condition: F) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let len = self.len();
        if condition(&self[len - 1]) {
            return Some(len - 1);
        }
        if !condition(&self[0]) {
            return None;
        }

        let mut left = 0;
        let mut right = len - 1;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if condition(&self[mid]) {
                left = mid;
            } else {
                right = mid;
            }
        }

        Some(left)
    }

    fn binary_search_left(&self, condition: F) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let len = self.len();

        if condition(&self[0]) {
            return Some(0);
        }
        if !condition(&self[len - 1]) {
            return None;
        }

        let mut left = 0;
        let mut right = len - 1;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if condition(&self[mid]) {
                right = mid;
            } else {
                left = mid;
            }
        }

        Some(right)
    }
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

    #[test]
    fn binary_search_right_test() {
        let v1 = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(Some(3), v1.binary_search_right(|x| *x <= 3));
        assert_eq!(Some(0), v1.binary_search_right(|x| *x <= 0));
        assert_eq!(Some(5), v1.binary_search_right(|x| *x >= 0));
        assert_eq!(None, v1.binary_search_right(|x| *x >= 100));

        let v2: Vec<usize> = vec![];
        assert_eq!(None, v2.binary_search_right(|x| *x <= 3));

        let v3: Vec<usize> = vec![3];
        assert_eq!(Some(0), v3.binary_search_right(|x| *x <= 3));
        assert_eq!(None, v3.binary_search_right(|x| *x <= 2));
    }

    #[test]
    fn binary_search_left_test() {
        let v1 = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(Some(3), v1.binary_search_left(|x| *x >= 3));
        assert_eq!(Some(5), v1.binary_search_left(|x| *x >= 5));
        assert_eq!(Some(0), v1.binary_search_left(|x| *x <= 5));
        assert_eq!(None, v1.binary_search_left(|x| *x >= 100));

        let v2: Vec<usize> = vec![];
        assert_eq!(None, v2.binary_search_left(|x| *x >= 3));

        let v3: Vec<usize> = vec![3];
        assert_eq!(Some(0), v3.binary_search_left(|x| *x >= 3));
        assert_eq!(None, v3.binary_search_left(|x| *x >= 4));
    }
}
