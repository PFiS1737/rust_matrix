use crate::{Matrix, Result};

impl Matrix {
    /// Swaps two elements in the matrix.
    pub fn swap(&mut self, pos1: (usize, usize), pos2: (usize, usize)) -> Result<()> {
        let (row1, col1) = pos1;
        let (row2, col2) = pos2;

        self.assert_index(row1, col1)?;
        self.assert_index(row2, col2)?;

        let temp = self.get(row1, col1)?;

        self.set(row1, col1, self.get(row2, col2)?)?;
        self.set(row2, col2, temp)?;

        Ok(())
    }

    /// Swaps two rows in the matrix.
    ///
    /// # Also see
    ///
    /// * Wikipedia: [Row-switching transformations](https://en.wikipedia.org/wiki/Elementary_matrix#Row-switching_transformations)
    pub fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<()> {
        self.assert_index(row1, 0)?;
        self.assert_index(row2, 0)?;

        self.elements.swap(row1, row2);

        Ok(())
    }

    /// Swaps two columns in the matrix.
    pub fn swap_cols(&mut self, col1: usize, col2: usize) -> Result<()> {
        self.assert_index(0, col1)?;
        self.assert_index(0, col2)?;

        for i in 0..self.rows_number {
            self.elements[i].swap(col1, col2);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix, MatrixElement};

    #[test]
    fn swap() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.swap((0, 0), (1, 1)).unwrap();

        assert!(m.epsilon_equals(&matrix![
            5, 2, 3;
            4, 1, 6;
        ]));
    }

    #[test]
    fn swap_out_of_bounds() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.swap((0, 0), (2, 2)).unwrap_err();
    }

    #[test]
    fn swap_rows() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.swap_rows(0, 1).unwrap();

        assert!(m.epsilon_equals(&matrix![
            4, 5, 6;
            1, 2, 3;
        ]));
    }

    #[test]
    fn swap_rows_out_of_bounds() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.swap_rows(0, 2).unwrap_err();
    }

    #[test]
    fn swap_cols() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.swap_cols(0, 1).unwrap();

        assert!(m.epsilon_equals(&matrix![
            2, 1, 3;
            5, 4, 6;
        ]));
    }

    #[test]
    fn swap_cols_out_of_bounds() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.swap_cols(0, 4).unwrap_err();
    }
}
