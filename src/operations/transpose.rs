use crate::Matrix;

impl Matrix {
    /// Transpose a matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m = matrix![
    ///     1, 2, 3;
    ///     4, 5, 6;
    /// ];
    ///
    /// assert!(m.transpose().epsilon_equals(&matrix![
    ///    1, 4;
    ///    2, 5;
    ///    3, 6;
    /// ]));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Transpose](https://en.wikipedia.org/wiki/Transpose)
    pub fn transpose(&self) -> Self {
        self.as_cols().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix, MatrixElement};

    #[test]
    fn transpose() {
        let m1 = matrix![
            1, 2.7;
            2, -5;
            3.6, 6;
        ];
        let m2 = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        assert!(m1.transpose().epsilon_equals(&matrix![
            1, 2, 3.6;
            2.7, -5, 6;
        ]));
        assert!(m1.epsilon_equals(&m1.transpose().transpose()));
        assert!((m1.transpose() * m2.transpose()).epsilon_equals(&(m2 * m1).transpose()))
    }
}
