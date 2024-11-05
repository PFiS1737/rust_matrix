use std::ops::Mul;

use crate::{Error, Matrix, Result};

impl Matrix {
    /// Returns a new matrix that is the product of this matrix and another matrix.
    ///
    /// # Errors
    /// Throws an error if the number of columns of this matrix is different from the number of rows of the other matrix.
    ///
    /// # Examples
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m1 = matrix![
    ///     -1, 2;
    ///     1 / 3, 4;
    /// ];
    ///
    /// let m2 = matrix![
    ///     1, 0.2;
    ///     4, -5;
    /// ];
    ///
    /// // You can also use the `*` operator.
    /// assert!((m1.multiply(&m2).unwrap()).epsilon_equals(&matrix![
    ///     7, -10.2;
    ///     16, -20;
    /// ]));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Matrix multiplication](https://en.wikipedia.org/wiki/Matrix_multiplication)
    /// * [`Matrix::multiply_vector`]
    pub fn multiply(&self, other: &Self) -> Result<Self> {
        if self.cols_number != other.rows_number {
            return Err(Error::InvalidOperation(
                "Matrix multiplication is only available for MxP * PxN",
            ));
        }

        Ok(Matrix::from_cols(
            other
                .as_cols()
                .into_iter()
                .map(|col| self.multiply_vector(&col).unwrap()) // INFO: safe to unwrap
                .collect(),
        ))
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(&rhs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix, MatrixElement};

    #[test]
    fn multiply() {
        let m1 = matrix![
            0.1, 2;
            3, -4;
            5, 6;
        ];
        let m2 = matrix![
            1, 2, 1.3;
            4, -5, 6;
        ];

        assert!((m1.clone() * m2.clone()).epsilon_equals(&matrix![
            8.1, -9.8, 12.13;
            -13, 26, -20.1;
            29, -20, 42.5;
        ]));
        assert!((m2 * m1).epsilon_equals(&matrix![
            12.6, 1.8;
            15.4, 64;
        ]));
    }

    #[test]
    fn mul_to_unit() {
        let m = matrix![ 1, 2; 3, 4; ];
        let i = Matrix::identity(2);

        assert!((m.clone() * i.clone()).epsilon_equals(&(i * m)))
    }

    #[test]
    fn mul_wrong_size() {
        let m1 = matrix![1, 2; 3, 4;];
        let m2 = matrix![1, 2; 3, 4; 5, 6;];

        m1.multiply(&m2).unwrap_err();
    }
}
