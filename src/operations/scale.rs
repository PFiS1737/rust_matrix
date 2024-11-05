use std::ops::{Div, Mul};

use crate::{Matrix, MatrixElement, Result};

impl Matrix {
    /// Scales the matrix by a scalar.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// let m = matrix![
    ///     1, 2;
    ///     0.3, 8;
    /// ];
    ///
    /// // You can use the `*` operator or just `m / 2`.
    /// assert!(m.scale(0.5).epsilon_equals(&matrix![
    ///     0.5, 1;
    ///     0.15, 4;
    /// ]));
    /// ```
    ///
    /// # Also see
    ///
    /// * Wikipedia: [Scalar multiplication](https://en.wikipedia.org/wiki/Scalar_multiplication)
    /// * Wikipedia: [Row-multiplying transformations](https://en.wikipedia.org/wiki/Elementary_matrix#Row-multiplying_transformations)
    /// * [`Vector::scale`](crate::Vector::scale)
    /// * [`Matrix::multiply`]
    pub fn scale<T: Into<MatrixElement> + Copy>(&self, scalar: T) -> Self {
        self.as_rows().iter().map(|col| col.scale(scalar)).collect()
    }

    /// Scales a row of the matrix by a scalar.
    pub fn scale_row<T: Into<MatrixElement> + Copy>(
        &mut self,
        row: usize,
        scalar: T,
    ) -> Result<()> {
        self.set_row(row, self.get_row(row)?.scale(scalar))?;

        Ok(())
    }

    /// Scales a column of the matrix by a scalar
    pub fn scale_col<T: Into<MatrixElement> + Copy>(
        &mut self,
        col: usize,
        scalar: T,
    ) -> Result<()> {
        self.set_col(col, self.get_col(col)?.scale(scalar))?;

        Ok(())
    }
}

impl Mul<MatrixElement> for Matrix {
    type Output = Self;

    fn mul(self, rhs: MatrixElement) -> Self::Output {
        self.scale(rhs)
    }
}

impl Div<MatrixElement> for Matrix {
    type Output = Self;

    fn div(self, rhs: MatrixElement) -> Self::Output {
        self.scale(rhs.inverse())
    }
}

macro_rules! impl_mul {
    ($( $t:ty ),*) => {
        $(
            impl Mul<Matrix> for $t {
                type Output = Matrix;

                fn mul(self, rhs: Matrix) -> Self::Output {
                     rhs.scale(self)
                }
            }

            impl Mul<$t> for Matrix {
                type Output = Self;

                fn mul(self, rhs: $t) -> Self::Output {
                    self.scale(rhs)
                }
            }

            impl Div<$t> for Matrix {
                type Output = Self;

                fn div(self, rhs: $t) -> Self::Output {
                    let rhs: MatrixElement = rhs.into();

                    self.scale(rhs.inverse())
                }
            }
        )*
    };
}

impl_mul!(i8, i16, i32, i64, f32, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn scale() {
        let m1 = matrix![
            1, 2;
            3, 4;
        ];
        let m2 = matrix![
            2, 4;
            6, 8;
        ];
        assert!(m1.epsilon_equals(&(m2.clone() * 0.5)));
        assert!(m2.epsilon_equals(&(2.0 * m1.clone())));

        // FIXME: have to annotations the type
        let m: Matrix = m2 / 2;
        assert!(m.epsilon_equals(&m1));
    }
}
