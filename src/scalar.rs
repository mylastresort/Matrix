use std::{fmt::Debug, iter::Sum};

pub trait Scalar:
    Debug
    + Copy
    + Default
    + std::cmp::PartialOrd
    + std::ops::Neg<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + std::iter::Sum
{
    type AbsOutput: Sum<Self::AbsOutput> + Default + std::cmp::PartialOrd;
    type TanOutput;

    fn abs(self) -> Self::AbsOutput;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn sqrt(v: Self) -> Self;
    fn one() -> Self;
    fn inv(self) -> Self;
    fn tan(self) -> Self::TanOutput;
}

impl Scalar for f32 {
    type AbsOutput = f32;

    fn abs(self) -> Self::AbsOutput {
        f32::abs(self)
    }

    fn sqrt(v: Self) -> Self {
        v.sqrt()
    }

    fn one() -> Self {
        1.
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }

    fn inv(self) -> Self {
        1. / self
    }

    type TanOutput = f32;
    fn tan(self) -> Self::TanOutput {
        f32::tan(self)
    }
}

impl Scalar for f64 {
    type AbsOutput = f64;

    fn abs(self) -> Self::AbsOutput {
        f64::abs(self)
    }

    fn sqrt(v: Self) -> Self {
        v.sqrt()
    }

    fn one() -> Self {
        1.
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }

    fn inv(self) -> Self {
        1. / self
    }

    type TanOutput = f64;
    fn tan(self) -> Self::TanOutput {
        f64::tan(self)
    }
}
