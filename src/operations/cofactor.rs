use crate::{Matrix, MatrixElement, Result};

impl Matrix {
    /// Returns the cofactor matrix.
    ///
    /// # Examples
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m = matrix![
    ///     1, 4, 7;
    ///     3, 0, 5;
    ///     -1, 9, 11;
    /// ];
    ///
    /// assert!(m.get_cofactor_matrix().unwrap().epsilon_equals(&matrix![
    ///     -45, -38, 27;
    ///     19, 18, -13;
    ///     20, 16, -12;
    /// ]));
    ///```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Minor](https://en.wikipedia.org/wiki/Minor_(linear_algebra))
    /// * [`Matrix::get_cofactor`]
    /// * [`Matrix::get_minor`]
    pub fn get_cofactor_matrix(&self) -> Result<Self> {
        let mut elements = Vec::new();

        for i in 0..self.rows_number {
            let mut row = Vec::new();

            for j in 0..self.cols_number {
                row.push(self.get_cofactor(i, j)?);
            }

            elements.push(row);
        }

        Ok(Self::new(elements))
    }

    /// Returns the cofactor of the matrix element at the given row and column.
    pub fn get_cofactor(&self, row: usize, col: usize) -> Result<MatrixElement> {
        let sign = if (row + col) % 2 == 0 { 1 } else { -1 };

        Ok(sign * self.get_minor(row, col)?)
    }

    /// Returns the minor of the matrix element at the given row and column.
    pub fn get_minor(&self, row: usize, col: usize) -> Result<MatrixElement> {
        self.assert_index(row, col)?;

        let mut elements = Vec::new();

        for i in 0..self.rows_number {
            if i == row {
                continue;
            }

            let mut row_elements = Vec::new();

            for j in 0..self.cols_number {
                if j == col {
                    continue;
                }

                row_elements.push(self.get(i, j)?);
            }

            elements.push(row_elements);
        }

        Self::new(elements).det()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn get_cofactor_matrix() {
        let m = matrix![
            1, -3, 7;
            -0.5, 2, -1;
            3, -1, 0;
        ];

        assert!(m.get_cofactor_matrix().unwrap().epsilon_equals(&matrix![
            -1, -3, -5.5;
            -7, -21, -8;
            -11, -2.5, 0.5;
        ]))
    }
}
