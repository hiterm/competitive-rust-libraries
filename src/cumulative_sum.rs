fn cumulative_sum<T>(v: &Vec<T>) -> Vec<T>
where
    T: Clone + std::ops::Add<Output = T> + From<u32>,
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
