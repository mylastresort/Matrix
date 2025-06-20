use std::{
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
    },
};

use crate::scalar::Scalar;

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Debug)]
pub struct Complex {
    x: f32,
    y: f32,
}

impl From<[f32; 2]> for Complex {
    fn from(value: [f32; 2]) -> Self {
        Complex {
            x: value[0],
            y: value[1],
        }
    }
}

impl Scalar for Complex {
    type AbsOutput = f32;

    fn abs(self) -> Self::AbsOutput {
        f32::sqrt(self.x.powi(2) + self.y.powi(2))
    }

    fn inv(self) -> Self {
        Complex {
            x: 1. / self.x,
            y: 1. / self.y,
        }
    }

    fn sqrt(v: Self) -> Self {
        Complex {
            x: f32::sqrt(v.x),
            y: f32::sqrt(v.y),
        }
    }

    fn one() -> Self {
        Complex::from([1., 0.])
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Complex {
            x: f32::mul_add(self.x, a.x, b.x),
            y: f32::mul_add(self.y, a.y, b.y),
        }
    }

    type TanOutput = Complex;
    fn tan(self) -> Self::TanOutput {
        Complex {
            x: f32::sin(2. * self.x)
                / (f32::cos(2. * self.x) + f32::cosh(2. * self.y)),
            y: f32::sinh(2. * self.y)
                / (f32::cos(2. * self.x) + f32::cosh(2. * self.y)),
        }
    }

    type SinOutput = Complex;
    fn sin(self) -> Self::SinOutput {
        todo!()
    }

    type CosOutput = Complex;
    fn cos(self) -> Self::CosOutput {
        todo!()
    }
}

impl Sum for Complex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Complex::default(), |a, b| Complex {
            x: a.x + b.x,
            y: a.y + b.y,
        })
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Complex {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<&Complex> for Complex {
    fn add_assign(&mut self, rhs: &Complex) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<&Complex> for Complex {
    fn sub_assign(&mut self, rhs: &Complex) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.x * rhs.y + self.y * rhs.x,
        }
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let x = self.x * rhs.x - self.y * rhs.y;
        self.y = self.x * rhs.y + self.y * rhs.x;
        self.x = x;
    }
}

impl MulAssign<&Complex> for Complex {
    fn mul_assign(&mut self, rhs: &Complex) {
        let x = self.x * rhs.x - self.y * rhs.y;
        self.y = self.x * rhs.y + self.y * rhs.x;
        self.x = x;
    }
}

impl Mul<f32> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f32) -> Self::Output {
        Complex {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Complex {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Complex {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign<&Complex> for Complex {
    fn div_assign(&mut self, rhs: &Complex) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
