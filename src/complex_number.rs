use std::{
    cmp::PartialOrd,
    ops::{Add, Div, Mul, Sub},
};
use std::cmp::Ordering;
use std::ops::{Deref, MulAssign};

use num_traits::{Float, Num};

#[derive(Debug, Clone, Copy)]
/// Represents a Complex Number
pub struct ComplexNumber<T: Float + Send + Sync> {
    /// The real part
    pub r: T,
    /// The imaginary part
    pub i: T,
}

impl<T: Float + Send + Sync > ComplexNumber<T> {
    pub fn new(r: T, i: T) -> ComplexNumber<T> {
        ComplexNumber { r, i }
    }

    pub fn abs(self) -> T {
        ((self.r * self.r) + (self.i * self.i)).sqrt()
    }

    pub fn norm_sqr(&self) -> T {
        (self.r * self.r) + (self.i * self.i)
    }

    pub fn pow(&self, e: u32) -> Self {
        let mut r = *self;
        for _ in 1..e {
            r *= *self;
        }

        r
    }
}

impl<T: Add<Output=T> + Float + Send + Sync> Add<ComplexNumber<T>> for ComplexNumber<T> {
    type Output = ComplexNumber<T>;

    /// Adds our `ComplexNumber` to another `ComplexNumber`
    fn add(self, other: ComplexNumber<T>) -> ComplexNumber<T> {
        ComplexNumber {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl<T: Add<Output=T> + Float, R: Num + Into<T> + Copy> Add<R> for ComplexNumber<T>
    where T: Send + Sync,
{
    type Output = ComplexNumber<T>;

    /// Adds our `ComplexNumber` to something that _isn't_ a `ComplexNumber`
    fn add(self, other: R) -> ComplexNumber<T> {
        ComplexNumber {
            r: self.r + (other.into()),
            i: self.i,
        }
    }
}

impl<T: Div<Output=T> + Float, R: Num + Into<T> + Copy> Div<R> for ComplexNumber<T>
    where T: Send + Sync,
{
    type Output = ComplexNumber<T>;

    /// Divides our `ComplexNumber` by something that _isn't_ a `ComplexNumber`
    fn div(self, other: R) -> ComplexNumber<T> {
        ComplexNumber {
            r: self.r / (other.into()),
            i: self.i / (other.into()),
        }
    }
}

impl<T: Mul<Output=T> + Sub<Output=T> + Add<Output=T> + Float> Mul<ComplexNumber<T>>
for ComplexNumber<T>
    where T: Send + Sync,
{
    type Output = ComplexNumber<T>;

    fn mul(self, other: ComplexNumber<T>) -> ComplexNumber<T> {
        ComplexNumber {
            r: (self.r * other.r) - (self.i * other.i),
            i: (self.r * other.i) + (self.i * other.r),
        }
    }
}

impl<T: Mul<Output=T> + Float, R: Num + Into<T> + Copy> Mul<R> for ComplexNumber<T>
    where T: Send + Sync,
{
    type Output = ComplexNumber<T>;

    fn mul(self, other: R) -> ComplexNumber<T> {
        ComplexNumber {
            r: self.r * other.into(),
            i: self.i * other.into(),
        }
    }
}

impl<T: Float + Send + Sync + Mul> MulAssign for ComplexNumber<T> {
    fn mul_assign(&mut self, rhs: Self) {
        let r = self.r;
        let i = self.i;
        self.r = (r * rhs.r) - (i * rhs.i);
        self.i = (r * rhs.i) + (i * rhs.r);
    }
}

impl<T: PartialEq<T> + Float, J: Into<T> + Float> PartialEq<ComplexNumber<J>> for ComplexNumber<T>
    where
        T: Send + Sync,
        J: Send + Sync,
{
    fn eq(&self, other: &ComplexNumber<J>) -> bool {
        (self.r == other.r.into()) && (self.i == other.i.into())
    }
}

impl<T: PartialOrd<T> + Float, J: Into<T> + Float> PartialOrd<ComplexNumber<J>>
for ComplexNumber<T>
    where
        T: Send + Sync,
        J: Send + Sync,
{
    fn partial_cmp(&self, other: &ComplexNumber<J>) -> Option<Ordering> {
        self.abs().partial_cmp(&other.abs().into())
    }
}

#[cfg(test)]
mod tests {
    use super::ComplexNumber;

    #[test]
    fn complex_addition() {
        let a = ComplexNumber::new(4.0, 5.0);
        let b = ComplexNumber::new(5.6, 9.0);

        let c = a + b;

        assert_eq!(c.r, 9.6);
        assert_eq!(c.i, 14.0);

        assert_eq!(a.r, 4.0);
        assert_eq!(b.r, 5.6);
    }

    #[test]
    fn complex_addition2() {
        let a = ComplexNumber::new(4.0, 5.0);
        let b = ComplexNumber::new(5.5, 9.0);

        let c = a + 5;
        assert_eq!(c.r, 9.0);

        let c = b + -5.0;
        assert_eq!(c.r, 0.5);
    }

    #[test]
    fn complex_multiplication() {
        let a = ComplexNumber::new(1.0, 1.0);
        let b = ComplexNumber::new(5.0, 3.0);

        let c = a * b;
        assert_eq!(c.r, 2.0);
        assert_eq!(c.i, 8.0);
    }

    #[test]
    fn complex_multiplication2() {
        let a = ComplexNumber::new(1.0, 3.0);

        let c = a * 2;

        assert_eq!(c.r, 2.0);
        assert_eq!(c.i, 6.0);
    }

    #[test]
    fn complex_multiplication_assign() {
        let mut a = ComplexNumber::new(1.0, 1.0);
        let b = ComplexNumber::new(5.0, 3.0);

        let c = a * b;
        a *= b;

        assert_eq!(c, a);
    }

    #[test]
    fn complex_division() {
        let a = ComplexNumber::new(2.0, 2.0);

        let c = a / 2.0;
        assert_eq!(c.r, 1.0);
        assert_eq!(c.i, 1.0);
    }

    #[test]
    fn complex_abs() {
        assert_eq!(ComplexNumber::new(3.0, 4.0).abs(), 5.0);
        assert_eq!(ComplexNumber::new(-3.0, 4.0).abs(), 5.0);
        assert_eq!(ComplexNumber::new(3.0, -4.0).abs(), 5.0);
        assert_eq!(ComplexNumber::new(-3.0, -4.0).abs(), 5.0);

        assert_eq!(ComplexNumber::new(5.0, 0.0).abs(), 5.0);
        assert_eq!(ComplexNumber::new(0.0, 5.0).abs(), 5.0);
    }

    #[test]
    fn complex_eq() {
        let a = ComplexNumber::new(2.0, 2.0);
        let b = ComplexNumber::new(2.0, 2.0);
        assert_eq!(a, b);

        let a = ComplexNumber::new(3.0, 2.0);
        assert_ne!(a, b);

        let a = ComplexNumber::new(-2.0, 2.0);
        assert_ne!(a, b);
    }

    #[test]
    fn complex_cmp() {
        let a = ComplexNumber::new(2.0, 2.0);

        assert!(a > ComplexNumber::new(2.0, 0.0));
        assert_eq!(a, ComplexNumber::new(2.0, 2.0));
    }
}
