use crate::{Error, Matrix, Result};

impl Matrix {
    /// Returns the inverse of the matrix.
    ///
    /// # Errors
    ///
    /// Throws an error if the matrix cannot be inverted.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m = matrix![
    ///     0, 1, 2;
    ///     1, 0, 3;
    ///     4, -3, 8;
    /// ];
    ///
    /// assert!(m.inverse().unwrap().epsilon_equals(&matrix![
    ///     -4.5, 7, -1.5;
    ///     -2, 4, -1;
    ///     1.5, -2, 0.5;
    /// ]));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Invertible matrix](https://en.wikipedia.org/wiki/Invertible_matrix)
    /// * [`Matrix::can_be_inverted`]
    /// * [`Matrix::to_rref_apply_to`]
    pub fn inverse(&self) -> Result<Self> {
        // [ A | I ] -> [ I | A^(-1) ]
        let (origin, applied) = self.to_rref_apply_to(Self::identity(self.rows_number))?;

        if !origin.epsilon_equals(&Self::identity(self.rows_number)) {
            return Err(Error::InvalidOperation("The matrix cannot be inverted"));
        }

        Ok(applied)
    }

    /// Checks if the matrix can be inverted.
    pub fn can_be_inverted(&self) -> bool {
        self.to_rref()
            .epsilon_equals(&Self::identity(self.rows_number))
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use super::*;
    use crate::element::MatrixElement;
    use crate::matrix;

    #[test]
    fn inverse() {
        let m1 = matrix![
            0, 1, 2;
            1, 0, 3;
            4, -3, 8;
        ];

        assert!(m1.can_be_inverted());
        assert!(m1.inverse().unwrap().epsilon_equals(&matrix![
            -4.5, 7, -1.5;
            -2, 4, -1;
            1.5, -2, 0.5;
        ]));
        assert!(m1
            .scale(3.0)
            .inverse()
            .unwrap()
            .epsilon_equals(&(m1.inverse().unwrap().scale(1.0 / 3.0))));
        assert!(m1.inverse().unwrap().inverse().unwrap().epsilon_equals(&m1));
        assert!(m1
            .transpose()
            .inverse()
            .unwrap()
            .epsilon_equals(&m1.inverse().unwrap().transpose()));
        assert!(m1
            .inverse()
            .unwrap()
            .mul(m1.clone())
            .epsilon_equals(&Matrix::identity(m1.rows_number)));

        let m2 = matrix![
            1, 3, 4;
            2, 5, 6;
            3, 7, 9;
        ];

        assert!((m1.inverse().unwrap() * m2.inverse().unwrap())
            .epsilon_equals(&(m2 * m1).inverse().unwrap()));
    }

    #[test]
    fn inverse_unable() {
        let m = matrix![
            1, 3, 4;
            2, 5, 6;
            3, 7, 8;
        ];

        m.inverse().unwrap_err();
    }
}
