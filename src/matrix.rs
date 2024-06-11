/// Dirt-simple matrix algebra library.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix<const ROWS: usize, const COLS: usize> {
    pub data: [[f64; COLS]; ROWS],
}

fn dot_product<const N: usize>(xs: &[f64; N], ys: &[f64; N]) -> f64 {
    xs.iter().zip(ys).map(|(x, y)| x * y).sum()
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    /// Creates a zero matrix.
    /// ```
    /// use flux_problem::matrix::Matrix;
    ///
    /// assert_eq!(Matrix::<2, 2>::zero(), Matrix {data: [[0f64, 0f64], [0f64, 0f64]]});
    /// ```
    pub fn zero() -> Self {
        Matrix {
            data: [[0.0; COLS]; ROWS],
        }
    }
    /// Transposes the matrix.
    ///
    /// ```
    /// use flux_problem::matrix::Matrix;
    ///
    /// assert_eq!(
    ///     (Matrix {data: [[1f64, 2f64], [3f64, 4f64]]}).transpose(),
    ///     Matrix {data: [[1f64, 3f64], [2f64, 4f64]]});
    /// ```
    pub fn transpose(&self) -> Matrix<COLS, ROWS> {
        let mut result = Matrix::<COLS, ROWS>::zero();
        for row in 0..ROWS {
            for col in 0..COLS {
                result.data[col][row] = self.data[row][col];
            }
        }
        result
    }
    /// Applies a function to every element of the matrix.
    ///
    /// ```
    /// use flux_problem::matrix::Matrix;
    ///
    /// assert_eq!((Matrix {data: [[1f64, 2f64], [3f64, 4f64]]}).map(|x| x + 1f64),
    ///            Matrix {data: [[2f64, 3f64], [4f64, 5f64]]});
    /// ```
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let mut result = self.clone();
        for row in 0..ROWS {
            for col in 0..COLS {
                result.data[row][col] = f(result.data[row][col])
            }
        }
        result
    }
}

impl<const ROWS: usize, const COLS: usize> std::ops::Add for Matrix<ROWS, COLS> {
    type Output = Self;
    fn add(self: Self, other: Self) -> Self::Output {
        let mut result = Matrix::<ROWS, COLS>::zero();
        for row in 0..ROWS {
            for col in 0..COLS {
                result.data[row][col] = self.data[row][col] + other.data[row][col];
            }
        }
        result
    }
}

impl<const L: usize, const M: usize, const N: usize> std::ops::Mul<Matrix<M, N>> for Matrix<L, M> {
    type Output = Matrix<L, N>;
    /// Matrix multiplication.
    /// ```
    /// use flux_problem::matrix::Matrix;
    ///
    /// let m1 = Matrix {
    ///     data: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
    /// };
    /// let m2 = Matrix {
    ///     data: [[10.0, 11.0], [20.0, 21.0], [30.0, 31.0]],
    /// };
    /// assert_eq!(
    ///     Matrix {
    ///         data: [[140.0, 146.0], [320.0, 335.0]]
    ///     },
    ///     m1 * m2
    /// );
    /// ```
    fn mul(self: Self, other: Matrix<M, N>) -> Self::Output {
        let mut result = Self::Output::zero();
        let transpose = other.transpose();
        for row in 0..L {
            for col in 0..N {
                result.data[row][col] = dot_product(&self.data[row], &transpose.data[col]);
            }
        }
        result
    }
}

impl<const ROWS: usize, const COLS: usize> std::ops::Mul<Matrix<ROWS, COLS>> for f64 {
    type Output = Matrix<ROWS, COLS>;
    /// Scalar multiplication of a matrix.
    ///
    /// ```
    /// use flux_problem::matrix::Matrix;
    /// assert_eq!(2f64 * Matrix {data: [[1f64, 2f64], [3f64, 4f64]]},
    ///            Matrix {data: [[2f64, 4f64], [6f64, 8f64]]});
    /// ```
    fn mul(self: Self, other: Matrix<ROWS, COLS>) -> Self::Output {
        other.map(|x| x * self)
    }
}

/// Generates a one-row `Matrix` containing `ELEMENTS` columns, each evenly spaced
/// from `start` to `end`, inclusive. The 0th element is `start`, and the last element
/// is `end`.
///
/// ```
/// use flux_problem::matrix::{linspace, Matrix};
/// assert_eq!(linspace::<6>(0f64, 5f64), Matrix {data: [[0f64, 1f64, 2f64, 3f64, 4f64, 5f64]]});
/// ```
pub fn linspace<const ELEMENTS: usize>(start: f64, end: f64) -> Matrix<1, ELEMENTS> {
    let mut result = Matrix::<1, ELEMENTS>::zero();
    let step: f64 = (end - start) / ((ELEMENTS - 1) as f64);
    for i in 0..ELEMENTS {
        result.data[0][i] = start + (step * (i as f64));
    }
    result
}

mod test {

    #[cfg(test)]
    use super::{linspace, Matrix};

    #[test]
    fn test_multiplication() {
        let m1 = Matrix {
            data: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
        };
        let m2 = Matrix {
            data: [[10.0, 11.0], [20.0, 21.0], [30.0, 31.0]],
        };
        assert_eq!(
            Matrix {
                data: [[140.0, 146.0], [320.0, 335.0]]
            },
            m1 * m2
        );
    }

    #[test]
    fn test_linspace() {
        assert_eq!(
            linspace::<11>(0.0, 10.0),
            Matrix {
                data: [[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]]
            }
        )
    }
}
