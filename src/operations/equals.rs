use crate::Matrix;

impl Matrix {
    /// Checks if the matrix is equal to another matrix within a certain epsilon.
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
    /// let n = matrix![
    ///     1.000000000002, 2, 3;
    ///     4, 4.999999999997, 6;
    ///     7, 8, 8.999999999993;
    /// ];
    ///
    /// assert!(m.epsilon_equals(&n));
    /// ```
    ///
    /// # See also
    ///
    /// * [`Vector::epsilon_equals`](crate::Vector::epsilon_equals)
    /// * [`MatrixElement::epsilon_equals`](crate::MatrixElement::epsilon_equals)
    pub fn epsilon_equals(&self, other: &Self) -> bool {
        self.as_rows()
            .into_iter()
            .zip(other.as_rows())
            .all(|(row1, row2)| row1.epsilon_equals(&row2))
    }

    /// Checks if the matrix is equivalent to another matrix.
    ///
    /// # See also
    ///
    /// * Wikipedia: [Matrix equivalence](https://en.wikipedia.org/wiki/Matrix_equivalence)
    pub fn is_equivalent_to(&self, other: &Self) -> bool {
        self.to_rref().epsilon_equals(&other.to_rref())
    }
}
