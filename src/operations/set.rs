use crate::{Error, Matrix, MatrixElement, Result, Vector};

impl Matrix {
    #[doc(hidden)]
    pub fn set<T: Into<MatrixElement>>(&mut self, row: usize, col: usize, value: T) -> Result<()> {
        self.assert_index(row, col)?;

        let value: MatrixElement = value.into();

        // To avoid negative zero
        self.elements[row][col] = if value.is_zero() {
            MatrixElement::zero()
        } else {
            value
        };

        Ok(())
    }

    #[doc(hidden)]
    pub fn set_row(&mut self, row: usize, values: Vector) -> Result<()> {
        self.assert_index(row, 0)?;

        if values.len() != self.cols_number {
            return Err(Error::InvalidOperation(
                "Values length must be equal to columns to set row",
            ));
        }

        for (i, item) in values.into_iter().enumerate() {
            self.set(row, i, item)?;
        }

        Ok(())
    }

    #[doc(hidden)]
    pub fn set_col(&mut self, col: usize, values: Vector) -> Result<()> {
        self.assert_index(0, col)?;

        if values.len() != self.rows_number {
            return Err(Error::InvalidOperation(
                "Values length must be equal to rows to set column",
            ));
        }

        for (i, item) in values.into_iter().enumerate() {
            self.set(i, col, item)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix, vector};

    #[test]
    fn set() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.set(1, 1, 10).unwrap();
        m.set(0, 2, -12).unwrap();

        assert!(m.epsilon_equals(&matrix![
            1, 2, -12;
            4, 10, 6;
        ]));
    }

    #[test]
    fn set_out_of_bounds() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.set(2, 0, 10).unwrap_err();
    }

    #[test]
    fn set_row() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.set_row(1, vector![10, 20, 30]).unwrap();

        assert!(m.epsilon_equals(&matrix![
            1, 2, 3;
            10, 20, 30;
        ]))
    }

    #[test]
    fn set_row_diff_length() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.set_row(1, vector![10, 20]).unwrap_err();
    }

    #[test]
    fn set_col() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.set_col(1, vector![10, 20]).unwrap();

        assert!(m.epsilon_equals(&matrix![
            1, 10, 3;
            4, 20, 6;
        ]));
    }

    #[test]
    fn set_col_diff_length() {
        let mut m = matrix![
            1, 2, 3;
            4, 5, 6;
        ];

        m.set_col(1, vector![10, 20, 30]).unwrap_err();
    }
}
