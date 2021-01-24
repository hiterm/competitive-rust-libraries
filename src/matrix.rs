use std::ops::{Add, Mul};

#[derive(Clone)]
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

impl<T> Mul for Matrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Clone,
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
                // Defaultが加法単位元であることを仮定
                let mut sum = T::default();
                for i in 0..mul_num {
                    sum = sum + self.get(r, i) * rhs.get(i, c);
                }
                ret[r][c] = sum;
            }
        }
        Matrix::new(ret)
    }
}
