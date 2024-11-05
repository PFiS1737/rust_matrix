use crate::{Error, Matrix, Result};

impl Matrix {
    /// Returns the row echelon form of the matrix.
    ///
    ///  NOTE: The row echelon form of the matrix is not unique, and it may be different from the
    ///        result you calculate by hand.
    ///
    /// # Returns
    ///
    /// A tuple containing the row echelon form of the matrix and the number of row swaps during the process.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m = matrix![
    ///     0, 3, -6, 6, 4, -5;
    ///     3, -7, 8, -5, 8, 9;
    ///     3, -9, 12, -9, 6, 15;
    /// ];
    ///
    /// let (result, swap_count) = m.row_echelon().unwrap();
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Row echelon form](https://en.wikipedia.org/wiki/Row_echelon_form#(General)_row_echelon_form)
    /// * [`Matrix::to_rref`]
    pub fn row_echelon(&self) -> Result<(Self, usize)> {
        let (output, _, swap_count) = self._row_echelon(None)?;

        Ok((output, swap_count))
    }

    /// Returns the reduced row echelon form of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m = matrix![
    ///     0, 3, -6, 6, 4, -5;
    ///     3, -7, 8, -5, 8, 9;
    ///     3, -9, 12, -9, 6, 15;
    /// ];
    ///
    /// assert!(m.to_rref().epsilon_equals(&matrix![
    ///     1, 0, -2, 3, 0, -24;
    ///     0, 1, -2, 2, 0, -7;
    ///     0, 0, 0, 0, 1, 4;
    /// ]));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Reduced row echelon form](https://en.wikipedia.org/wiki/Row_echelon_form#Reduced_row_echelon_form)
    /// * [`Matrix::to_rref_apply_to`]
    pub fn to_rref(&self) -> Self {
        self._to_rref(None).unwrap().0
    }

    /// Returns the reduced row echelon form of the matrix and applies the same steps to another matrix.
    ///
    /// # Errors
    ///
    /// Throws an error if the matrix to apply steps to has a different number of rows than the original matrix.
    ///
    /// # Examples
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m = matrix![
    ///     1, 3, -6;
    ///     3, -7, 8;
    ///     3, -9, 12;
    /// ];
    ///
    /// let (result, applied) = m.to_rref_apply_to(Matrix::identity(3)).unwrap();
    ///
    /// assert!(result.epsilon_equals(&m.to_rref()));
    /// assert!(applied.epsilon_equals(&m.inverse().unwrap()));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Reduced row echelon form](https://en.wikipedia.org/wiki/Row_echelon_form#Reduced_row_echelon_form)
    /// * [`Matrix::to_rref`]
    /// * [`Matrix::inverse`]
    pub fn to_rref_apply_to(&self, other: Self) -> Result<(Self, Self)> {
        self._to_rref(Some(other))
    }

    fn _row_echelon(&self, apply_to: Option<Self>) -> Result<(Self, Self, usize)> {
        let m = self.rows_number;
        let n = self.cols_number;

        let mut i = 0;
        let mut j = 0;
        let mut origin = self.clone();
        let mut output = if let Some(apply_to) = apply_to {
            if apply_to.rows_number != m {
                return Err(Error::InvalidOperation(
                    "Matrix to apply steps to must have the same number of rows as the original matrix",
                ));
            }

            apply_to
        } else {
            self.clone()
        };

        let mut swap_count = 0;

        while i < m && j < n {
            // find the element with the largest absolute value in the current column
            let mut max_index = i;
            for k in i..m {
                if origin
                    .get(k, j)?
                    .abs()
                    .epsilon_gt(&origin.get(max_index, j)?.abs())
                {
                    max_index = k;
                }
            }

            // if it is not zero, swap the row to the pivot
            if !origin.get(max_index, j)?.is_zero() {
                if max_index != i {
                    swap_count += 1;

                    origin.swap_rows(i, max_index)?;
                    output.swap_rows(i, max_index)?;
                }

                // eliminate all entries below the pivot
                for k in (i + 1)..m {
                    let factor = (origin.get(k, j)? / origin.get(i, j)?).negate();

                    origin.add_scaled_row_from_to(i, k, factor)?;
                    output.add_scaled_row_from_to(i, k, factor)?;
                }

                i += 1;
            }

            j += 1;
        }

        Ok((origin, output, swap_count))
    }

    fn _to_rref(&self, apply_to: Option<Self>) -> Result<(Self, Self)> {
        let m = self.rows_number;
        let n = self.cols_number;

        let (mut origin, mut output, _) = self._row_echelon(apply_to)?;

        for i in (0..m).rev() {
            // find pivot in row_i
            let mut pivot = None;
            for j in 0..n {
                if !origin.get(i, j)?.is_zero() {
                    pivot = Some(j);
                    break;
                }
            }

            if let Some(pivot) = pivot {
                // normalize the leading entry of row_i
                let leading = origin.get(i, pivot)?;
                if !leading.is_one() {
                    origin.scale_row(i, leading.inverse())?;
                    output.scale_row(i, leading.inverse())?;
                }

                // eliminate all entries above the leading entry
                for k in 0..i {
                    let factor = origin.get(k, pivot)?.negate();
                    if !factor.is_zero() {
                        origin.add_scaled_row_from_to(i, k, factor)?;
                        output.add_scaled_row_from_to(i, k, factor)?;
                    }
                }
            }
        }

        Ok((origin, output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix, MatrixElement};

    #[test]
    fn to_rref() {
        let m = matrix![
            0, -3, -6, 4, 9;
            -1, -2, -1, 3, 1;
            -2, -3, 0, 3, -1;
            1, 4, 5, -9, -7;
        ];

        assert!(m.to_rref().epsilon_equals(&matrix![
            1, 0, -3, 0, 5;
            0, 1, 2, 0, -3;
            0, 0, 0, 1, 0;
            0, 0, 0, 0, 0;
        ]));
    }
}
