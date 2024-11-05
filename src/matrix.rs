use core::panic;

use crate::{Error, MatrixElement, Result, Vector};

/// A matrix.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    /// The number of columns.
    pub cols_number: usize,

    /// The number of rows.
    pub rows_number: usize,

    /// The elements of the matrix.
    pub elements: Vec<Vec<MatrixElement>>,
}

impl From<Vec<Vec<MatrixElement>>> for Matrix {
    fn from(value: Vec<Vec<MatrixElement>>) -> Self {
        Matrix::new(value)
    }
}

impl From<Vec<Vector>> for Matrix {
    fn from(value: Vec<Vector>) -> Self {
        Matrix::new(value.into_iter().map(|v| v.into()).collect())
    }
}

impl FromIterator<Vec<MatrixElement>> for Matrix {
    fn from_iter<T: IntoIterator<Item = Vec<MatrixElement>>>(iter: T) -> Self {
        Matrix::new(iter.into_iter().collect())
    }
}

impl FromIterator<Vector> for Matrix {
    fn from_iter<T: IntoIterator<Item = Vector>>(iter: T) -> Self {
        Matrix::new(iter.into_iter().map(|v| v.into()).collect())
    }
}

impl Matrix {
    /// Creates a new matrix from a list of elements.
    ///
    /// # Panics
    ///
    /// Panics if the list is empty or if the rows have different lengths.
    pub fn new(elements: Vec<Vec<MatrixElement>>) -> Self {
        if elements.is_empty() {
            panic!("Matrix must have at least one row");
        }

        if elements.iter().any(|row| row.len() != elements[0].len()) {
            panic!("All rows must have the same length");
        }

        Self {
            cols_number: elements[0].len(),
            rows_number: elements.len(),
            elements,
        }
    }

    /// Creates a new matrix from a list of columns.
    ///
    /// # Panics
    ///
    /// Panics if the list is empty or if the columns have different lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, vector, Matrix, MatrixElement, Vector};
    /// let matrix = Matrix::from_cols(vec![
    ///     vector![1, 2, 3],
    ///     vector![4, 5, 6],
    ///     vector![7, 8, 9],
    /// ]);
    ///
    /// assert!(matrix.epsilon_equals(&matrix![
    ///     1, 4, 7;
    ///     2, 5, 8;
    ///     3, 6, 9;
    /// ]));
    /// ```
    pub fn from_cols(cols: Vec<Vector>) -> Self {
        if cols.is_empty() {
            panic!("Matrix must have at least one column");
        }

        if cols.iter().any(|col| col.data.len() != cols[0].data.len()) {
            panic!("All columns must have the same length");
        }

        let rows_number = cols[0].len();
        let cols_number = cols.len();

        let mut elements = Vec::new();

        for i in 0..rows_number {
            let mut row = Vec::new();

            for col in cols.iter() {
                row.push(col[i]);
            }

            elements.push(row);
        }

        Self {
            cols_number,
            rows_number,
            elements,
        }
    }

    /// Creates a new matrix with the given size of which all elements is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// assert!((Matrix::zero(3, 5)).epsilon_equals(&matrix![
    ///     0, 0, 0, 0, 0;
    ///     0, 0, 0, 0, 0;
    ///     0, 0, 0, 0, 0;
    /// ]));
    /// ```
    pub fn zero(rows: usize, cols: usize) -> Self {
        Self::new(vec![vec![MatrixElement::zero(); cols]; rows])
    }

    /// Creates the identity matrix of the given size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// assert!((Matrix::identity(1)).epsilon_equals(&matrix![1]));
    /// assert!((Matrix::identity(4)).epsilon_equals(&matrix![
    ///     1, 0, 0, 0;
    ///     0, 1, 0, 0;
    ///     0, 0, 1, 0;
    ///     0, 0, 0, 1;
    /// ]));
    /// ```
    ///
    /// # See also
    ///
    /// * Wikipedia: [Identity matrix](https://en.wikipedia.org/wiki/Identity_matrix)
    /// * [`Matrix::zero`]
    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::zero(size, size);

        for i in 0..size {
            matrix.elements[i][i] = MatrixElement::one();
        }

        matrix
    }
}

impl Matrix {
    /// Asserts that the matrix is square.
    ///
    /// Returns an error if the matrix is not square.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// matrix![
    ///     1, 2;
    ///     3, 4;
    ///     5, 6;
    /// ].assert_square("Matrix must be square").unwrap_err();
    /// ```
    pub fn assert_square(&self, msg: &'static str) -> Result<()> {
        if self.rows_number != self.cols_number {
            return Err(Error::ShouldBeSquare(msg));
        }

        Ok(())
    }

    /// Checks if the given row and column indices are within the bounds of the matrix.
    ///
    /// Returns an error if the indices are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{matrix, Matrix, MatrixElement};
    /// matrix![
    ///     1, 2;
    ///     3, 4;
    ///     5, 6;
    /// ].assert_index(3, 0).unwrap_err();
    /// ```
    pub fn assert_index(&self, row: usize, col: usize) -> Result<()> {
        if row >= self.rows_number {
            return Err(Error::IndexOutOfBounds("Row index out of bounds"));
        }
        if col >= self.cols_number {
            return Err(Error::IndexOutOfBounds("Column index out of bounds"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn new_empty() {
        Matrix::new(Vec::<Vec<MatrixElement>>::new());
    }

    #[test]
    #[should_panic]
    fn new_diff_length() {
        Matrix::new(vec![
            vec![MatrixElement::new(1.0), MatrixElement::new(2.0)],
            vec![MatrixElement::new(1.0)],
        ]);
    }
}
