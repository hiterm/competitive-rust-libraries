use std::ops::{Add, Mul};

use crate::num_traits::Zero;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Matrix<T> {
    entries: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Clone,
{
    fn new(entries: Vec<Vec<T>>) -> Matrix<T> {
        Matrix { entries }
    }

    fn get(&self, r: usize, c: usize) -> T {
        self.entries[r][c].clone()
    }

    fn height(&self) -> usize {
        self.entries.len()
    }

    fn width(&self) -> usize {
        self.entries[0].len()
    }
}

impl<T> Add for Matrix<T>
where
    T: Clone + Default + Add<Output = T> + Zero,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width(), rhs.width());
        assert_eq!(self.height(), rhs.height());

        let width = self.width();
        let height = self.height();
        let mut ret = vec![vec![T::default(); width]; height];
        for r in 0..height {
            for c in 0..width {
                ret[r][c] = self.get(r, c) + rhs.get(r, c);
            }
        }
        Matrix::new(ret)
    }
}

impl<T> Mul for Matrix<T>
where
    T: Clone + Default + Add<Output = T> + Mul<Output = T> + Zero,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width(), rhs.height());
        let height = self.height();
        let width = rhs.width();
        let mul_num = self.width();
        let mut ret = vec![vec![T::default(); width]; height];
        for r in 0..height {
            for c in 0..width {
                let mut sum = T::zero();
                for i in 0..mul_num {
                    sum = sum + self.get(r, i) * rhs.get(i, c);
                }
                ret[r][c] = sum;
            }
        }
        Matrix::new(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_case1() {
        let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
        let m2 = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(Matrix::new(vec![vec![2, 2], vec![3, 5]]), m1 + m2);
    }

    #[test]
    fn mul_case1() {
        let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
        let m2 = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(Matrix::new(vec![vec![1, 2], vec![3, 4]]), m1 * m2);
    }

    #[test]
    fn mul_case2() {
        let m1 = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
        let m2 = Matrix::new(vec![vec![4, 3], vec![2, 1]]);
        assert_eq!(Matrix::new(vec![vec![8, 5], vec![20, 13]]), m1 * m2);
    }
}
