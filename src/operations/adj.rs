use crate::{Matrix, Result};

impl Matrix {
    /// Returns the adjugate matrix which is the transpose of the cofactor matrix.
    ///
    /// # Examples
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m = matrix![
    ///     1, 2, 3;
    ///     4, 5, 6;
    ///     7, 8, 9;
    /// ];
    ///
    /// assert!(m.adj().unwrap().epsilon_equals(&matrix![
    ///     -3, 6, -3;
    ///     6, -12, 6;
    ///     -3, 6, -3;
    /// ]));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Adjugate matrix](https://en.wikipedia.org/wiki/Adjugate_matrix)
    /// * [`Matrix::get_cofactor_matrix`]
    pub fn adj(&self) -> Result<Self> {
        Ok(self.get_cofactor_matrix()?.transpose())
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix, Matrix, MatrixElement};

    #[test]
    fn adj() {
        let m1 = matrix![
            2, 5, 7;
            -3, 6, -2;
            -8, 1.5, 9;
        ];
        let m2 = matrix![
            1, 9, -2.5;
            -15, 7, -3;
            -3.5, 7.5, 10;
        ];

        assert!((m1.clone() * m2.clone())
            .adj()
            .unwrap()
            .epsilon_equals(&(m2.adj().unwrap() * m1.adj().unwrap())))
    }
}
