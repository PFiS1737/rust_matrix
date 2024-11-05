use std::ops::Mul;

use crate::{Error, Matrix, Result, Vector};

impl Matrix {
    /// Multiplies the matrix by a vector.
    ///
    /// # Errors
    ///
    /// Throws an error if the number of columns of the matrix is different from the length of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, vector, Matrix, Vector, MatrixElement};
    /// let vector = vector![1, 2];
    /// let matrix = matrix![
    ///     1, 2;
    ///     3, 4;
    ///     5, 6;
    /// ];
    ///
    /// assert!((matrix.multiply_vector(&vector).unwrap()).epsilon_equals(&vector![5, 11, 17]));
    /// ```
    ///
    /// # See also
    ///
    /// * [`Matrix::multiply`]
    pub fn multiply_vector(&self, vector: &Vector) -> Result<Vector> {
        let cols = self.as_cols();

        if cols.len() != vector.len() {
            return Err(Error::InvalidOperation(
                "Matrix columns number must be equal to vector length for multiplication",
            ));
        };

        Ok(cols
            .iter()
            .zip(vector.clone())
            .map(|(col, scalar)| col.scale(scalar))
            .fold(Vector::zero(self.rows_number), |acc, x| acc.add(&x)))
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.multiply_vector(&rhs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix, vector, MatrixElement};

    #[test]
    fn multiply_vector() {
        let matrix = matrix![ 1, 2; 3, 4; 5, 6 ];
        let vector = vector![1, 2];
        assert!((matrix * vector).epsilon_equals(&vector![5, 11, 17]));

        let matrix = matrix![ 1, 2, 3; 4, 5, 6 ];
        let vector = vector![1, 2, 3];
        assert!((matrix * vector).epsilon_equals(&vector![14, 32]));

        let matrix = Matrix::identity(3);
        let vector = vector![1, 0, 3.4];
        assert!((matrix * vector.clone()).epsilon_equals(&vector));
    }

    #[test]
    fn multiply_vector_with_wrong_length() {
        matrix![ 1, 2; 3, 4; 5, 6 ]
            .multiply_vector(&vector![1, 2, 3])
            .unwrap_err();
    }
}
