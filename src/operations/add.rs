use std::ops::Add;

use crate::{Error, Matrix, Result};

impl Matrix {
    /// Returns a new matrix that is the sum of this matrix and another matrix.
    ///
    /// # Errors
    ///
    /// Throws an error if the dimensions of the two matrices do not match.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m1 = matrix![
    ///     1, 2, 3;
    ///     4, 5, 6;
    ///     7, 8, 9;
    /// ];
    ///
    /// let m2 = matrix![
    ///     9, 8, 7;
    ///     6, 5, 4;
    ///     3, 2, 1;
    /// ];
    ///
    /// assert!((m1.add_s(&m2).unwrap()).epsilon_equals(&matrix![
    ///     10, 10, 10;
    ///     10, 10, 10;
    ///     10, 10, 10;
    /// ]));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Matrix addition](https://en.wikipedia.org/wiki/Matrix_addition)
    /// * [`Vector::add`](crate::Vector::add)
    pub fn add_s(&self, other: &Self) -> Result<Self> {
        if self.rows_number != other.rows_number || self.cols_number != other.cols_number {
            return Err(Error::InvalidOperation(
                "Matrix dimensions must match for addition",
            ));
        }

        let mut result = Matrix::zero(self.rows_number, self.cols_number);

        for (i, row) in self.as_rows().iter().enumerate() {
            result.set_row(i, row.add(&other.get_row(i)?))?;
        }

        Ok(result)
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_s(&rhs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix, MatrixElement};

    #[test]
    fn add() {
        let m1 = matrix![
            1, 2, 3;
            4, 5, 6;
        ];
        let m2 = matrix![
            7, 8, 9;
            10, 11, 12;
        ];

        assert!((m1 + m2).epsilon_equals(&matrix![
            8, 10, 12;
            14, 16, 18;
        ]));
    }
}
