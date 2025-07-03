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
    type AbsOutput: Sum<Self::AbsOutput> + Default + std::cmp::PartialOrd;
    type TanOutput;
    type CosOutput;
    type SinOutput;

    fn abs(self) -> Self::AbsOutput;
    fn sqrt(v: Self) -> Self;
    fn one() -> Self;
    fn inv(self) -> Self;
    fn tan(self) -> Self::TanOutput;
    fn sin(self) -> Self::SinOutput;
    fn cos(self) -> Self::CosOutput;
}

pub trait MulAdd<U, V> {
    fn mul_add(self, a: U, b: &V) -> Self;
}

pub trait Lerp {
    fn lerp(u: Self, v: Self, t: f32) -> Self;
}

pub fn lerp<V: Lerp>(u: V, v: V, t: f32) -> V {
    V::lerp(u, v, t)
}

impl Scalar for f32 {
    type AbsOutput = f32;
    type SinOutput = f32;
    type CosOutput = f32;

    fn abs(self) -> Self::AbsOutput {
        f32::abs(self)
    }

    fn sqrt(v: Self) -> Self {
        v.sqrt()
    }

    fn one() -> Self {
        1.
    }

    fn inv(self) -> Self {
        1. / self
    }

    type TanOutput = f32;
    fn tan(self) -> Self::TanOutput {
        f32::tan(self)
    }

    fn cos(self) -> Self::CosOutput {
        f32::cos(self)
    }

    fn sin(self) -> Self::SinOutput {
        f32::sin(self)
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

    fn inv(self) -> Self {
        1. / self
    }

    type TanOutput = f64;
    fn tan(self) -> Self::TanOutput {
        f64::tan(self)
    }

    type CosOutput = f64;
    fn cos(self) -> Self::CosOutput {
        f64::sin(self)
    }

    type SinOutput = f64;
    fn sin(self) -> Self::SinOutput {
        f64::sin(self)
    }
}

impl MulAdd<f32, f32> for f32 {
    fn mul_add(self, a: f32, b: &f32) -> Self {
        self.mul_add(a, *b)
    }
}

impl MulAdd<f64, f64> for f64 {
    fn mul_add(self, a: f64, b: &f64) -> Self {
        self.mul_add(a, *b)
    }
}

impl Lerp for f32 {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        match t {
            0. => u,
            1. => v,
            p => u + (v - u) * p,
        }
    }
}

impl Lerp for f64 {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        match t {
            0. => u,
            1. => v,
            p => u + (v - u) * p as f64,
        }
    }
}
