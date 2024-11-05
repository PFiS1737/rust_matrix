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
