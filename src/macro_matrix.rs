/// Create a matrix in a more readable way.
///
/// # Examples
///
/// ```
/// use rust_matrix::matrix;
///
/// // necessary to use the macro
/// use rust_matrix::Matrix;
/// use rust_matrix::MatrixElement;
///
/// let m = matrix![
///     1, 2, 3;
///     4, 5, 6;
///     7, 8, 9;
/// ];
///
/// assert_eq!(
///     m,
///     Matrix::new(vec![
///         vec![MatrixElement::from(1), MatrixElement::from(2), MatrixElement::from(3)],
///         vec![MatrixElement::from(4), MatrixElement::from(5), MatrixElement::from(6)],
///         vec![MatrixElement::from(7), MatrixElement::from(8), MatrixElement::from(9)],
///     ])
/// );
/// ```
///
/// # See also
/// * [`Matrix`](crate::Matrix)
/// * [`MatrixElement`](crate::MatrixElement)
/// * [`vector!`](crate::vector!)
#[macro_export]
macro_rules! matrix {
    ( $( $( $row:expr ),+ );+ ; ) => {
        Matrix::new(vec![
            $(
                vec![$(MatrixElement::from($row)),+],
            )+
        ])
    };
    ( $( $( $row:expr ),+ );+ ) => {
        matrix![ $( $( $row ),+ );+ ; ]
    };
}
