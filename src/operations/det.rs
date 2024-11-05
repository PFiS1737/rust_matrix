use crate::{Matrix, MatrixElement, Result};

impl Matrix {
    /// Returns the determinant of the matrix.
    ///
    /// # Errors
    ///
    /// Throws an error if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m = matrix![
    ///     3, -7, 8, 9, -6;
    ///     0, 2, -5, 7, 3;
    ///     0, 0, 1, 5, 0;
    ///     0, 0, 2, 4, -1;
    ///     0, 0, 0, -2, 0;
    /// ];
    ///
    /// assert!(m.det().unwrap().epsilon_equals(&(-12)));
    ///```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Determinant](https://en.wikipedia.org/wiki/Determinant)
    /// * [`Matrix::row_echelon`]
    pub fn det(&self) -> Result<MatrixElement> {
        self.assert_square("Only square matrices have determinants")?;

        let n = self.rows_number;

        if n == 1 {
            return self.get(0, 0);
        }
        if n == 2 {
            // |a b| = a * d - b * c
            // |c d|
            return Ok(self.get(0, 0)? * self.get(1, 1)? - self.get(0, 1)? * self.get(1, 0)?);
        }

        // INFO: There is no row scaling when calculating the row echelon form.
        let (matrix, swap_count) = self.row_echelon()?;

        // The determinant of a matrix is the product of the diagonal elements of its row echelon form, ...
        let mut det = MatrixElement::one();
        for i in 0..n {
            det *= matrix.get(i, i)?;
        }

        // the sign of it is (-1)^r where r is the times of row swaps
        let sign = if swap_count % 2 == 0 { 1 } else { -1 };

        Ok(sign * det)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn det() {
        let mut m1 = matrix![
            3, -7, 8, 9, -6;
            0, 2, -5, 7, 3;
            0, 0, 1, 5, 0;
            0, 0, 2, 4, -1;
            0, 0, 0, -2, 0;
        ];
        let m2 = matrix![
            1, 2, 3, -1, 0.3;
            0, 1, 4, 9, -3.5;
            5, -6, 0, 0, 7;
            5, 9, 0, 0, -13;
            1, 6, 10, 0.5, -1;
        ];

        let det_m1 = MatrixElement::from(-12);
        let det_m2 = MatrixElement::from(-2291.25);

        assert!(m1.det().unwrap().epsilon_equals(&det_m1));
        assert!(m1.transpose().det().unwrap().epsilon_equals(&det_m1));
        assert!(m1
            .inverse()
            .unwrap()
            .det()
            .unwrap()
            .epsilon_equals(&det_m1.inverse()));

        assert!(m2.det().unwrap().epsilon_equals(&det_m2));
        assert!((m1.clone() * m2.clone())
            .det()
            .unwrap()
            .epsilon_equals(&(det_m1 * det_m2)));

        m1.swap_rows(0, 1).unwrap();
        assert!(m1.det().unwrap().epsilon_equals(&det_m1.negate()));

        m1.swap_rows(2, 1).unwrap();
        assert!(m1.det().unwrap().epsilon_equals(&det_m1.negate().negate()));

        m1.scale_row(1, 0.5).unwrap();
        assert!(m1.det().unwrap().epsilon_equals(&(det_m1 * 0.5)));

        m1.scale_row(3, 2.0).unwrap();
        assert!(m1.det().unwrap().epsilon_equals(&(det_m1 * 0.5 * 2.0)));

        m1.add_scaled_row_from_to(0, 1, 2.0).unwrap();
        assert!(m1.det().unwrap().epsilon_equals(&det_m1));

        m1.add_scaled_row_from_to(1, 2, 3.0).unwrap();
        assert!(m1.det().unwrap().epsilon_equals(&det_m1));
    }

    #[test]
    fn det_not_square() {
        matrix![
            1, 2, 3;
            4, 5, 6;
        ]
        .det()
        .unwrap_err();
    }
}
