fn cumulative_sum<T>(v: &Vec<T>) -> Vec<T>
where
    T: Clone + std::ops::Add<Output = T> + From<u8>,
{
    let zero: T = 0.into();
    let mut sum = vec![zero.clone()];
    let mut now = zero;
    for vi in v.iter().cloned() {
        now = now + vi;
        sum.push(now.clone());
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn cumulative_sum_i64_test() {
        let v: Vec<i64> = vec![1, 2, 3];
        let csum = cumulative_sum(&v);
        assert_eq!(vec![0, 1, 3, 6], csum);
    }

    fn cumulative_sum_usize_test() {
        let v: Vec<usize> = vec![1, 2, 3];
        let csum = cumulative_sum(&v);
        assert_eq!(vec![0, 1, 3, 6], csum);
    }
}
