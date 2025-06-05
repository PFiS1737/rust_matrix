use crate::{Matrix, Result};

impl Matrix {
    /// Returns the LU decomposition of the matrix.
    ///
    /// # Returns
    ///
    /// A tuple containing the lower triangular matrix `L`, the upper triangular matrix `U`, and the permutation matrix `P`.
    /// Thus, P * self == L * U.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m = matrix![
    ///     2, 4, -1, 5, -2;
    ///    -4, -5, 3, -8, 1;
    ///     2, -5, -4, 1, 8;
    ///     -6, 0, 7, -3, 1;
    /// ];
    ///
    /// let (l, u, p) = m.lup_decomposition().unwrap();
    ///
    /// assert!((p * m).epsilon_equals(&(l * u)));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [LU decomposition](https://en.wikipedia.org/wiki/LU_decomposition)
    /// * [`Matrix::row_echelon`]
    pub fn lup_decomposition(&self) -> Result<(Self, Self, Self)> {
        let m = self.rows_number;
        let n = self.cols_number;

        let p = self._pivot()?;

        let mut l = Matrix::identity(m);
        let mut u = p.clone() * self.clone();

        let mut i = 0;
        let mut j = 0;

        // Here is an example to understand it.
        //
        // NOTE: The result of this function may be different from what you calculate by hand.
        //
        //     [  {2}  4 -1  5 -2  ]   [  2    4   -1   5  -2  ]   [  2  4 -1   5  -2  ]   [  2  4 -1  5  -2  ]
        // A = [ {-4} -5  3 -8  1  ] ~ [  0   {3}   1  -2   3  ] ~ [  0  3  1  -2   3  ] ~ [  0  3  1 -2   3  ] = U
        //     [  {2} -5 -4  1  8  ]   [  0  {-9}  -3  -4  10  ]   [  0  0  0  {2}  1  ]   [  0  0  0  2   1  ]
        //     [ {-6}  0  7 -3  1  ]   [  0  {12}   4  12  -5  ]   [  0  0  0  {4}  7  ]   [  0  0  0  0  {5} ]
        //        |                           |                                 |                          |
        //        | div 2                     | div 3                           | div 2                    | div 5
        //        |                           |                                 |                          |
        //     [  1  ]
        //     [ -2  ]                     [  1  ]
        //     [  1  ]                     [ -3  ]                           [  1  ]
        //     [ -3  ]                     [  4  ]                           [  2  ]                    [  1  ]
        //
        //            [  1  0  0  0  ]
        //  Thus, L = [ -2  1  0  0  ]
        //            [  1 -3  1  0  ]
        //            [ -3  4  2  1  ]
        while i < m && j < n {
            let pivot = u.get(i, j)?;

            if !pivot.is_zero() {
                for k in (i + 1)..m {
                    let factor = u.get(k, j)? / pivot;

                    // This line does the second step
                    l.set(k, i, factor)?;

                    u.add_scaled_row_from_to(i, k, factor.negate())?;
                }

                i += 1;
            }

            j += 1;
        }

        Ok((l, u, p))
    }

    /// Returns the permutation matrix `P`.
    fn _pivot(&self) -> Result<Self> {
        let m = self.rows_number;
        let n = self.cols_number;

        let mut p = Matrix::identity(m);

        let mut i = 0;
        let mut j = 0;

        while i < m && j < n {
            let mut max_index = i;
            for k in i..m {
                if self
                    .get(k, j)?
                    .abs()
                    .epsilon_gt(&self.get(max_index, j)?.abs())
                {
                    max_index = k;
                }
            }

            if max_index != i {
                p.swap_rows(i, max_index)?;
            }

            i += 1;
            j += 1;
        }

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix, MatrixElement};

    #[test]
    fn lu() {
        let m = matrix![
            11, 9, 24, 2;
            1, 5, 2, 6;
            3, 17, 18, 1;
            2, 5, 7, 1;
        ];
        let (l, u, p) = m.lup_decomposition().unwrap();
        assert!((p * m).epsilon_equals(&(l * u)));

        let m = matrix![
            2, -4, -2, 3;
            6, -9, -5, 8;
            2, -7, -3, 9;
            4, -2, -2, -1;
            -6, 3, 3, 4;
        ];
        let (l, u, p) = m.lup_decomposition().unwrap();

        // println!("l:\n{}", l);
        // println!("u:\n{}", u);
        // println!("p:\n{}", p);
        // panic!();

        assert!((p * m).epsilon_equals(&(l * u)));
    }
}
