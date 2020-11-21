#[allow(unused)]
fn rotate<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    let h = grid.len();
    let w = grid[0].len();

    let mut rotated = vec![];
    for c in 0..w {
        let mut rotated_row = vec![];
        for r in (0..h).rev() {
            rotated_row.push(grid[r][c].clone());
        }
        rotated.push(rotated_row);
    }

    rotated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_test() {
        let v = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(vec![vec![5, 3, 1], vec![6, 4, 2]], rotate(&v));
    }
}
