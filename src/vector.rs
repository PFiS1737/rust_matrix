use std::{ops::Index, vec::IntoIter};

use crate::MatrixElement;

/// A vector.
#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    /// The raw data of the vector.
    pub data: Vec<MatrixElement>,
}

impl Vector {
    /// Creates a new vector from a [`Vec`] of [`MatrixElement`].
    pub fn new(data: Vec<MatrixElement>) -> Self {
        Vector { data }
    }
}

/// Creates a new vector from a list of elements.
///
/// # Examples
///
/// ```
/// use rust_matrix::vector;
///
/// // necessary to use the macro
/// use rust_matrix::Vector;
/// use rust_matrix::MatrixElement;
///
/// let vector = vector![1, 2, 3];
///
/// assert!(vector.epsilon_equals(&Vector::new(
///     vec![MatrixElement::new(1.0), MatrixElement::new(2.0), MatrixElement::new(3.0)]
/// )));
/// ```
///
/// # See also
/// * [`Vector`]
/// * [`MatrixElement`]
/// * [`matrix!`](crate::matrix!)
#[macro_export]
macro_rules! vector {
    ($( $items:expr ),*) => {
        Vector::new(vec![$(MatrixElement::from($items)),*])
    };
}

impl From<Vec<MatrixElement>> for Vector {
    fn from(value: Vec<MatrixElement>) -> Self {
        Vector::new(value)
    }
}

impl From<Vector> for Vec<MatrixElement> {
    fn from(val: Vector) -> Self {
        val.data
    }
}

impl IntoIterator for Vector {
    type Item = MatrixElement;
    type IntoIter = IntoIter<MatrixElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl FromIterator<MatrixElement> for Vector {
    fn from_iter<T: IntoIterator<Item = MatrixElement>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl Index<usize> for Vector {
    type Output = MatrixElement;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Vector {
    /// Returns a vector with length `len` with all elements set to `0`.
    pub fn zero(len: usize) -> Self {
        Vector::new(vec![MatrixElement::zero(); len])
    }

    /// Returns the length of the vector.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::Vector;
    /// assert!(Vector::new(vec![]).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Checks if the vector is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::Vector;
    /// assert!(Vector::zero(9).is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.data
            .clone()
            .into_iter()
            .all(|element| element.is_zero())
    }

    /// Negates the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{vector, Vector, MatrixElement};
    /// assert!(vector![1, 2, -3].negate().epsilon_equals(&vector![-1, -2, 3]));
    /// ```
    pub fn negate(&self) -> Self {
        self.data
            .clone()
            .into_iter()
            .map(|element| element.negate())
            .collect()
    }

    /// Adds two vectors.
    ///
    /// # Panics
    ///
    /// Panics if the vectors have different lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{vector, Vector, MatrixElement};
    /// assert!(vector![1, 2, -3].add(&vector![4, 5, 6]).epsilon_equals(&vector![5, 7, 3]));
    /// ```
    pub fn add(&self, other: &Self) -> Self {
        if self.len() != other.len() {
            panic!("Vector length must be equal to add");
        }

        self.data
            .clone()
            .into_iter()
            .zip(other.clone())
            .map(|(a, b)| a + b)
            .collect()
    }

    /// Subtracts two vectors.
    ///
    /// # Panics
    ///
    /// Panics if the vectors have different lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{vector, Vector, MatrixElement};
    /// assert!(vector![1, 2, -3].subtract(&vector![4, 5, 6]).epsilon_equals(&vector![-3, -3, -9]));
    /// ```
    pub fn subtract(&self, other: &Self) -> Self {
        if self.len() != other.len() {
            panic!("Vector length must be equal to subtract");
        }

        self.data
            .clone()
            .into_iter()
            .zip(other.clone())
            .map(|(a, b)| a - b)
            .collect()
    }

    /// Scales the vector by a scalar.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{vector, Vector, MatrixElement};
    /// assert!(vector![1, 2, -3].scale(2).epsilon_equals(&vector![2, 4, -6]));
    /// ```
    pub fn scale<T: Into<MatrixElement> + Copy>(&self, scalar: T) -> Self {
        self.data
            .clone()
            .into_iter()
            .map(|element| element * scalar.into())
            .collect()
    }

    /// Returns the dot product of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{vector, Vector, MatrixElement};
    /// assert!(vector![1, 2, -3].dot(&vector![4, 5, 6]).epsilon_equals(&-4));
    /// ```
    pub fn dot(&self, other: &Self) -> MatrixElement {
        self.data
            .clone()
            .into_iter()
            .zip(other.clone())
            .map(|(a, b)| a * b)
            .fold(MatrixElement::zero(), |acc, x| acc + x)
    }

    /// Checks if the vector is equal to another vector within a certain epsilon.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::{vector, Vector, MatrixElement};
    /// let vector = vector![1, 2, -3];
    /// let other = vector![1.0000000001, 2.0000000002, -2.9999999997];
    ///
    /// assert!(vector.epsilon_equals(&other));
    /// ```
    ///
    /// # See also
    ///
    /// * [`MatrixElement::epsilon_equals`]
    pub fn epsilon_equals(&self, other: &Self) -> bool {
        self.data
            .clone()
            .into_iter()
            .zip(other.clone())
            .all(|(a, b)| a.epsilon_equals(&b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vector, Vector};

    #[test]
    #[should_panic]
    fn add_diff_length() {
        let _ = vector![1, 2, -3].add(&vector![4, 5]);
    }

    #[test]
    #[should_panic]
    fn subtract_diff_length() {
        let _ = vector![1, 2, -3].subtract(&vector![4, 5]);
    }
}
