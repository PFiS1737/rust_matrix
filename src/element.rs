use std::{
    cmp::Ordering,
    fmt::{self, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

/// A matrix element.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct MatrixElement {
    data: f64,
}

impl MatrixElement {
    /// Creates a new matrix element from [`f64`].
    pub fn new(value: f64) -> Self {
        MatrixElement { data: value }
    }
}

impl Display for MatrixElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

macro_rules! impl_from {
    ($( $type:ty ),*) => {
        $(
            impl From<$type> for MatrixElement {
                fn from(value: $type) -> Self {
                    MatrixElement::new(value as f64)
                }
            }
        )*
    };
}
impl_from!(i8, i16, i32, i64, f32, f64);

impl MatrixElement {
    /// Returns a matrix element with value `0`.
    pub fn zero() -> Self {
        MatrixElement::new(0.0)
    }

    /// Returns a matrix element with value `1`.
    pub fn one() -> Self {
        MatrixElement::new(1.0)
    }

    /// Returns the fractional part of the matrix element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::MatrixElement;
    /// let element = MatrixElement::new(3.5);
    ///
    /// assert_eq!(element.fract(), 0.5);
    /// ```
    pub fn fract(&self) -> f64 {
        self.data.fract()
    }

    /// Returns the integer part of the matrix element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::MatrixElement;
    /// let element = MatrixElement::new(3.5);
    ///
    /// assert_eq!(element.trunc(), 3.0);
    /// ```
    pub fn trunc(&self) -> f64 {
        self.data.trunc()
    }

    /// Checks if the matrix element is zero.
    pub fn is_zero(&self) -> bool {
        self.epsilon_equals(&Self::zero())
    }

    /// Checks if the matrix element is one.
    pub fn is_one(&self) -> bool {
        self.epsilon_equals(&Self::one())
    }

    /// Checks if the matrix element is an integer.
    pub fn is_integer(&self) -> bool {
        MatrixElement::from(self.fract()).is_zero()
    }

    /// Checks if the matrix element is a float.
    pub fn is_float(&self) -> bool {
        !self.is_integer()
    }

    /// Checks if the matrix element is positive.
    pub fn is_positive(&self) -> bool {
        self > &Self::zero()
    }

    /// Checks if the matrix element is negative.
    pub fn is_negative(&self) -> bool {
        self < &Self::zero()
    }

    /// Negates the matrix element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::MatrixElement;
    /// let element = MatrixElement::new(3.0);
    ///
    /// assert!(element.negate().epsilon_equals(&(-3.0)));
    /// ```
    pub fn negate(&self) -> Self {
        Self::zero() - *self
    }

    /// Returns the inverse of the matrix element.
    ///
    /// # Panics
    ///
    /// Panics if the matrix element is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::MatrixElement;
    /// let element = MatrixElement::new(3.0);
    ///
    /// assert!(element.inverse().epsilon_equals(&(1.0 / 3.0)));
    /// ```
    pub fn inverse(&self) -> Self {
        if self.is_zero() {
            panic!("Cannot invert zero");
        }

        Self::one() / *self
    }

    /// Returns the absolute value of the matrix element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::MatrixElement;
    /// let element = MatrixElement::new(-3.0);
    ///
    /// assert!(element.abs().epsilon_equals(&MatrixElement::new(3.0)));
    /// ```
    pub fn abs(&self) -> Self {
        if *self < Self::zero() {
            self.negate()
        } else {
            *self
        }
    }

    /// Checks if the matrix element is equal to another matrix element within a certain epsilon.
    ///
    /// NOTE: The epsilon value is `10e-8`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_matrix::MatrixElement;
    /// let element = MatrixElement::new(3.0);
    /// let other = MatrixElement::new(3.0000000002);
    ///
    /// assert!(element.epsilon_equals(&other));
    /// ```
    pub fn epsilon_equals<T: Into<MatrixElement> + Copy>(&self, other: &T) -> bool {
        let other: MatrixElement = (*other).into();

        (self.data - other.data).abs() < 10e-8
    }

    /// Compares the matrix element to another matrix element within a certain epsilon.
    pub fn epsilon_cmp(&self, other: &Self) -> Ordering {
        if self.epsilon_equals(other) {
            Ordering::Equal
        } else if self < other {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    /// Checks if the matrix element is less than another matrix element within a certain epsilon.
    pub fn epsilon_lt(&self, other: &Self) -> bool {
        matches!(self.epsilon_cmp(other), Ordering::Less)
    }

    /// Checks if the matrix element is less than or equal to another matrix element within a certain epsilon.
    pub fn epsilon_le(&self, other: &Self) -> bool {
        matches!(self.epsilon_cmp(other), Ordering::Less | Ordering::Equal)
    }

    /// Checks if the matrix element is greater than another matrix element within a certain epsilon.
    pub fn epsilon_gt(&self, other: &Self) -> bool {
        matches!(self.epsilon_cmp(other), Ordering::Greater)
    }

    /// Checks if the matrix element is greater than or equal to another matrix element within a certain epsilon.
    pub fn epsilon_ge(&self, other: &Self) -> bool {
        matches!(self.epsilon_cmp(other), Ordering::Greater | Ordering::Equal)
    }
}

impl Add for MatrixElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MatrixElement::new(self.data + rhs.data)
    }
}

impl AddAssign for MatrixElement {
    fn add_assign(&mut self, rhs: Self) {
        self.data += rhs.data;
    }
}

impl Sub for MatrixElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        MatrixElement::new(self.data - rhs.data)
    }
}

impl SubAssign for MatrixElement {
    fn sub_assign(&mut self, rhs: Self) {
        self.data -= rhs.data;
    }
}

impl Mul for MatrixElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        MatrixElement::new(self.data * rhs.data)
    }
}

impl MulAssign for MatrixElement {
    fn mul_assign(&mut self, rhs: Self) {
        self.data *= rhs.data;
    }
}

impl Div for MatrixElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() {
            panic!("Cannot divide by zero");
        }

        MatrixElement::new(self.data / rhs.data)
    }
}

impl DivAssign for MatrixElement {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.is_zero() {
            panic!("Cannot divide by zero");
        }

        self.data /= rhs.data;
    }
}

macro_rules! impl_ops {
    ($( $type:ty ),*) => {
        $(
            impl Add<$type> for MatrixElement {
                type Output = Self;

                fn add(self, rhs: $type) -> Self::Output {
                    self + MatrixElement::from(rhs)
                }
            }
            impl AddAssign<$type> for MatrixElement {
                fn add_assign(&mut self, rhs: $type) {
                    *self += MatrixElement::from(rhs);
                }
            }
            impl Add<MatrixElement> for $type {
                type Output = MatrixElement;

                fn add(self, rhs: MatrixElement) -> Self::Output {
                    MatrixElement::from(self) + rhs
                }
            }

            impl Sub<$type> for MatrixElement {
                type Output = Self;

                fn sub(self, rhs: $type) -> Self::Output {
                    self - MatrixElement::from(rhs)
                }
            }
            impl SubAssign<$type> for MatrixElement {
                fn sub_assign(&mut self, rhs: $type) {
                    *self -= MatrixElement::from(rhs);
                }
            }
            impl Sub<MatrixElement> for $type {
                type Output = MatrixElement;

                fn sub(self, rhs: MatrixElement) -> Self::Output {
                    MatrixElement::from(self) - rhs
                }
            }

            impl Mul<$type> for MatrixElement {
                type Output = Self;

                fn mul(self, rhs: $type) -> Self::Output {
                    self * MatrixElement::from(rhs)
                }
            }
            impl MulAssign<$type> for MatrixElement {
                fn mul_assign(&mut self, rhs: $type) {
                    *self *= MatrixElement::from(rhs);
                }
            }
            impl Mul<MatrixElement> for $type {
                type Output = MatrixElement;

                fn mul(self, rhs: MatrixElement) -> Self::Output {
                    MatrixElement::from(self) * rhs
                }
            }

            impl Div<$type> for MatrixElement {
                type Output = Self;

                fn div(self, rhs: $type) -> Self::Output {
                    self / MatrixElement::from(rhs)
                }
            }
            impl DivAssign<$type> for MatrixElement {
                fn div_assign(&mut self, rhs: $type) {
                    *self /= MatrixElement::from(rhs);
                }
            }
            impl Div<MatrixElement> for $type {
                type Output = MatrixElement;

                fn div(self, rhs: MatrixElement) -> Self::Output {
                    MatrixElement::from(self) / rhs
                }
            }
        )*
    };
}
impl_ops!(i8, i16, i32, i64, f32, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn inverse_zero() {
        MatrixElement::zero().inverse();
    }
}
