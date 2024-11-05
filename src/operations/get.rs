use std::ops::Index;

use crate::{Matrix, MatrixElement, Result, Vector};

impl Matrix {
    #[doc(hidden)]
    pub fn get(&self, row: usize, col: usize) -> Result<MatrixElement> {
        self.assert_index(row, col)?;

        Ok(self.elements[row][col])
    }

    #[doc(hidden)]
    pub fn get_row(&self, row: usize) -> Result<Vector> {
        self.assert_index(row, 0)?;

        Ok(self.elements[row].clone().into())
    }

    #[doc(hidden)]
    pub fn get_col(&self, col: usize) -> Result<Vector> {
        self.assert_index(0, col)?;

        let mut column = Vec::new();

        for i in 0..self.rows_number {
            column.push(self.elements[i][col]);
        }

        Ok(column.into())
    }
}

impl Matrix {
    /// Returns the matrix as a vector of rows.
    pub fn as_rows(&self) -> Vec<Vector> {
        let mut rows = Vec::new();

        for i in 0..self.rows_number {
            rows.push(self.get_row(i).unwrap()); // INFO: safe to unwrap
        }

        rows
    }

    /// Returns the matrix as a vector of columns.
    pub fn as_cols(&self) -> Vec<Vector> {
        let mut cols = Vec::new();

        for i in 0..self.cols_number {
            cols.push(self.get_col(i).unwrap()); // INFO: safe to unwrap
        }

        cols
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = MatrixElement;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;

        &self.elements[row][col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector;
    use crate::{matrix, vector};

    #[test]
    fn get() {
        let m = matrix![
            1, 2, 3;
            4, 5, 6;
            7, 8, 9;
        ];

        assert!(m.get(0, 0).unwrap().epsilon_equals(&1.0));
        assert!(m.get(0, 2).unwrap().epsilon_equals(&3.0));
        assert!(m.get(1, 0).unwrap().epsilon_equals(&4.0));
        assert!(m.get(1, 2).unwrap().epsilon_equals(&6.0));
        assert!(m.get(2, 0).unwrap().epsilon_equals(&7.0));
    }

    #[test]
    fn get_row() {
        let m = matrix![
            1, 2, 3;
            4, 5, 6;
            7, 8, 9;
        ];

        assert!(m.get_row(0).unwrap().epsilon_equals(&vector![1, 2, 3]));
        assert!(m.get_row(1).unwrap().epsilon_equals(&vector![4, 5, 6]));
        assert!(m.get_row(2).unwrap().epsilon_equals(&vector![7, 8, 9]));
    }

    #[test]
    fn get_col() {
        let m = matrix![
            1, 2, 3;
            4, 5, 6;
            7, 8, 9;
        ];

        assert!(m.get_col(0).unwrap().epsilon_equals(&vector![1, 4, 7]));
        assert!(m.get_col(1).unwrap().epsilon_equals(&vector![2, 5, 8]));
        assert!(m.get_col(2).unwrap().epsilon_equals(&vector![3, 6, 9]));
    }
}
