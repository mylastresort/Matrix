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
    + MulAdd<Self, Self>
    + Lerp
{
    type AbsOutput: Sum<Self::AbsOutput>
        + Debug
        + Copy
        + Default
        + std::cmp::PartialOrd
        + std::ops::AddAssign
        + MulAdd<Self::AbsOutput, Self::AbsOutput>
        + Sqrt
        + std::ops::Mul<Output = Self::AbsOutput>;
    type TanOutput;
    type CosOutput;
    type SinOutput;

    fn abs(&self) -> Self::AbsOutput;
    fn one() -> Self;
    fn inv(self) -> Self;
    fn tan(self) -> Self::TanOutput;
    fn sin(self) -> Self::SinOutput;
    fn cos(self) -> Self::CosOutput;
}

pub trait MulAdd<U, V> {
    fn mul_add(self, a: &U, b: &V) -> Self;
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

pub trait Lerp {
    fn lerp(u: Self, v: Self, t: f32) -> Self;
}

pub fn lerp<V: Lerp>(u: V, v: V, t: f32) -> V {
    V::lerp(u, v, t)
}
