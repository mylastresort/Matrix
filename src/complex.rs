use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
    },
};

use crate::{
    matrix::Transpose,
    scalar::{Lerp, MulAdd, Scalar, Sqrt},
    utils::EPSILON,
    vector::Angle,
    Dot, Matrix, Vector, V,
};

#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Complex {
    pub x: f64,
    pub y: f64,
}

pub trait Conj {
    fn conj(&self) -> Complex;
}

#[macro_export]
macro_rules! C {
    ($r:expr, $i:expr) => {
        Complex::from([$r, $i])
    };
}

impl Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {}i)", self.x, self.y)
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {}i)", self.x, self.y)
    }
}

impl From<[f64; 2]> for Complex {
    fn from(value: [f64; 2]) -> Self {
        Complex {
            x: value[0],
            y: value[1],
        }
    }
}

impl Scalar for Complex {
    type AbsOutput = f64;

    fn abs(&self) -> Self::AbsOutput {
        (self.x.powi(2) + self.y.powi(2)).powf(0.5)
    }

    fn inv(self) -> Self {
        Complex {
            x: self.x / (self.x.powi(2) + self.y.powi(2)),
            y: -self.y / (self.x.powi(2) + self.y.powi(2)),
        }
    }

    fn one() -> Self {
        C!(1., 0.)
    }

    type TanOutput = Complex;
    fn tan(self) -> Self::TanOutput {
        Complex {
            x: f64::sin(2. * self.x)
                / (f64::cos(2. * self.x) + f64::cosh(2. * self.y)),
            y: f64::sinh(2. * self.y)
                / (f64::cos(2. * self.x) + f64::cosh(2. * self.y)),
        }
    }

    type SinOutput = Complex;
    fn sin(self) -> Self::SinOutput {
        Complex {
            x: f64::sin(self.x) * f64::cosh(self.y),
            y: f64::cos(self.x) * f64::sinh(self.y),
        }
    }

    type CosOutput = Complex;
    fn cos(self) -> Self::CosOutput {
        Complex {
            x: f64::cos(self.x) * f64::cosh(self.y),
            y: f64::sin(self.x) * f64::sinh(self.y),
        }
    }

    fn is_non_zero(&self) -> bool {
        self.x.abs() > EPSILON || self.y.abs() > EPSILON
    }
}

impl Sqrt for Complex {
    fn sqrt(self: Self) -> Self {
        Complex {
            x: ((self.abs() + self.x) / 2.).sqrt(),
            y: self.y / self.y.abs() * ((self.abs() - self.x) / 2.).sqrt(),
        }
    }
}

impl Lerp for Complex {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        match t {
            0. => u,
            1. => v,
            p => (v - u) * p as f64 + u,
        }
    }
}

impl Sum for Complex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Complex::default(), |a, b| a + b)
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

impl MulAdd<Complex, Complex> for Complex {
    fn mul_add(self, a: &Self, b: &Self) -> Self {
        Complex {
            x: self.x.mul_add(a.x, self.y.mul_add(-a.y, b.x)),
            y: self.x.mul_add(a.y, self.y.mul_add(a.x, b.y)),
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f64) -> Self::Output {
        Complex {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f64> for Complex {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl MulAdd<f64, Complex> for Complex {
    fn mul_add(self, a: &f64, b: &Self) -> Self {
        Complex {
            x: self.x.mul_add(*a, b.x),
            y: self.y.mul_add(*a, b.y),
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Complex {
            x: (self.x * rhs.x + self.y * rhs.y)
                / (rhs.x.powi(2) + rhs.y.powi(2)),
            y: (self.y * rhs.x - self.x * rhs.y)
                / (rhs.x.powi(2) + rhs.y.powi(2)),
        }
    }
}

impl DivAssign<&Complex> for Complex {
    fn div_assign(&mut self, rhs: &Complex) {
        let x =
            (self.x * rhs.x + self.y * rhs.y) / (rhs.x.powi(2) + rhs.y.powi(2));
        self.y =
            (self.y * rhs.x - self.x * rhs.y) / (rhs.x.powi(2) + rhs.y.powi(2));
        self.x = x;
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        let x =
            (self.x * rhs.x + self.y * rhs.y) / (rhs.x.powi(2) + rhs.y.powi(2));
        self.y =
            (self.y * rhs.x - self.x * rhs.y) / (rhs.x.powi(2) + rhs.y.powi(2));
        self.x = x;
    }
}

impl Dot<Complex> for Vector<Complex> {
    fn dot(&self, v: &Vector<Complex>) -> Complex {
        assert_eq!(v.size(), self.size(), "vectors must be the same size");

        self * v
    }
}

impl Mul<&Vector<Complex>> for &Vector<Complex> {
    type Output = Complex;

    fn mul(self, rhs: &Vector<Complex>) -> Self::Output {
        self * &rhs._d
    }
}

impl Mul<&Vec<Complex>> for &Vector<Complex> {
    type Output = Complex;

    fn mul(self, rhs: &Vec<Complex>) -> Self::Output {
        self * &rhs[..]
    }
}

impl Conj for Complex {
    fn conj(&self) -> Complex {
        Complex {
            x: self.x,
            y: -self.y,
        }
    }
}

impl Mul<&[Complex]> for &Vector<Complex> {
    type Output = Complex;

    fn mul(self, rhs: &[Complex]) -> Self::Output {
        let mut sum = Complex::default();

        for (a, b) in self._d.iter().zip(rhs) {
            sum = a.mul_add(&b.conj(), &sum);
        }

        sum
    }
}

impl Dot<Complex> for [Complex] {
    fn dot(&self, v: &Vector<Complex>) -> Complex {
        assert_eq!(v.size(), self.len(), "vectors must be the same size");

        self * v
    }
}

impl Mul<&Vector<Complex>> for &[Complex] {
    type Output = Complex;

    fn mul(self, rhs: &Vector<Complex>) -> Self::Output {
        let mut sum = Complex::default();

        for (a, b) in self.iter().zip(rhs._d.iter()) {
            sum = a.mul_add(&b.conj(), &sum);
        }

        sum
    }
}
impl MulAdd<Complex, Vector<Complex>> for Vector<Complex> {
    fn mul_add(self, a: &Complex, b: &Vector<Complex>) -> Self {
        assert!(self.size() == b.size(), "vectors must be the same size");

        let mut vec = Vec::with_capacity(self.size());

        for i in 0..self.size() {
            vec.push(self[i].mul_add(a, &b[i]))
        }

        V!(vec)
    }
}

impl MulAdd<f64, Vector<Complex>> for Vector<Complex> {
    fn mul_add(self, a: &f64, b: &Vector<Complex>) -> Self {
        assert!(self.size() == b.size(), "vectors must be the same size");

        let mut vec = Vec::with_capacity(self.size());

        for i in 0..self.size() {
            vec.push(self[i].mul_add(a, &b[i]))
        }

        V!(vec)
    }
}

impl Angle for Vector<Complex> {
    type Output = f64;
    fn angle_cos(u: &Vector<Complex>, v: &Vector<Complex>) -> Self::Output {
        u.dot(v).x / (u.norm() * v.norm())
    }
}

impl Transpose<Complex> for Matrix<Complex> {
    fn transpose(&self) -> Matrix<Complex> {
        let mut vec = Vec::with_capacity(self.rows * self.cols);
        for i in 0..self.cols {
            for j in 0..self.rows {
                vec.push(self[j][i].conj());
            }
        }

        Matrix {
            rows: self.cols,
            cols: self.rows,
            _d: vec,
        }
    }
}

impl Lerp for Vector<Complex> {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        match t {
            0. => u,
            1. => v,
            p => {
                let mut vec = Vec::with_capacity(u.size());

                for i in 0..u.size() {
                    vec.push((v[i] - u[i]).mul_add(&(p as f64), &u[i]))
                }

                V!(vec)
            }
        }
    }
}

impl Lerp for Matrix<Complex> {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        match t {
            0. => u,
            1. => v,
            p => {
                let mut vec = Vec::with_capacity(u._d.len());

                for i in 0..u._d.len() {
                    vec.push((v._d[i] - u._d[i]).mul_add(&(p as f64), &u._d[i]))
                }

                Matrix {
                    _d: vec,
                    cols: u.cols,
                    rows: u.rows,
                }
            }
        }
    }
}
