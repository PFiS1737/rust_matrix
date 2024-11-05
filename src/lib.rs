#![allow(dead_code, unused_macros)]
#![warn(clippy::float_cmp)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod decomposition;
mod display;
mod element;
mod macro_matrix;
mod matrix;
mod operations;
mod vector;

pub use element::MatrixElement;
pub use matrix::Matrix;
pub use vector::Vector;

/// Error types
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This error is returned when an operation is not valid for the given matrix.
    /// For example, trying to invert a matrix that cannot be inverted.
    /// Or trying to multiply two matrices with incompatible dimensions.
    ///
    /// The message should describe the operation that was attempted.
    #[error("InvalidOperation: {0}")]
    InvalidOperation(&'static str),

    /// This error is returned when an index is out of bounds.
    ///
    /// You should not use this error directly, use the [`Matrix::assert_index`] method instead.
    #[error("IndexOutOfBounds: {0}")]
    IndexOutOfBounds(&'static str),

    /// This error is returned when a square matrix is required.
    ///
    /// You should not use this error directly, use the [`Matrix::assert_square`] method instead.
    #[error("ShouldBeSquare: {0}")]
    ShouldBeSquare(&'static str),
}

#[doc(hidden)]
pub type Result<T> = std::result::Result<T, Error>;
