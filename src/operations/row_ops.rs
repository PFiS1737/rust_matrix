use crate::{Matrix, MatrixElement, Result};

impl Matrix {
    /// Adds the row at index `from` to the row at index `to`.
    ///
    /// # Errors
    ///
    /// Throws an error if the row index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let mut m = matrix![
    ///     1, 2, 3;
    ///     4, 5, 6;
    ///     7, 8, 9;
    /// ];
    ///
    /// m.add_row_from_to(2, 0).unwrap();
    ///
    /// assert!(m.epsilon_equals(&matrix![
    ///     8, 10, 12;
    ///     4, 5, 6;
    ///     7, 8, 9;
    /// ]));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Row-addition transformations](https://en.wikipedia.org/wiki/Elementary_matrix#Row-addition_transformations)
    /// * [`Matrix::add_scaled_row_from_to`]
    pub fn add_row_from_to(&mut self, from: usize, to: usize) -> Result<()> {
        let row_from = self.get_row(from)?;
        let row_to = self.get_row(to)?;

        self.set_row(to, row_to.add(&row_from))?;

        Ok(())
    }

    /// Adds the row at index `from` scaled by `scalar` to the row at index `to`.
    ///
    /// # Errors
    ///
    /// Throws an error if the row index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let mut m = matrix![
    ///     1, 2, 3;
    ///     4, 5, 6;
    ///     7, 8, 9;
    /// ];
    ///
    /// m.add_scaled_row_from_to(0, 1, 0.1).unwrap();
    ///
    /// assert!(m.epsilon_equals(&matrix![
    ///     1, 2, 3;
    ///     4.1, 5.2, 6.3;
    ///     7, 8, 9;
    /// ]));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Row-addition transformations](https://en.wikipedia.org/wiki/Elementary_matrix#Row-addition_transformations)
    /// * [`Matrix::add_row_from_to`]
    /// * [`Matrix::scale_row`]
    pub fn add_scaled_row_from_to<T: Into<MatrixElement> + Copy>(
        &mut self,
        from: usize,
        to: usize,
        scalar: T,
    ) -> Result<()> {
        let row_from = self.get_row(from)?;
        let row_to = self.get_row(to)?;

        self.set_row(to, row_to.add(&row_from.scale(scalar)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn add_row_from_to() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
            7, 8, 9;
        ];

        m.add_row_from_to(2, 0).unwrap();

        assert!(m.epsilon_equals(&matrix![
            8, 10, 12;
            4, 5, 6;
            7, 8, 9;
        ]))
    }

    #[test]
    fn add_scaled_row_from_to() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
            7, 8, 9;
        ];

        m.add_scaled_row_from_to(0, 1, 0.1).unwrap();

        assert!(m.epsilon_equals(&matrix![
            1, 2, 3;
            4.1, 5.2, 6.3;
            7, 8, 9;
        ]))
    }
}
