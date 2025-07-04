use std::ops::Mul;

use crate::{
    matrix::Transpose,
    scalar::{Lerp, MulAdd, Sqrt},
    vector::Angle,
    Dot, Matrix, Scalar, Vector, V,
};

impl Scalar for f64 {
    type AbsOutput = f64;

    fn abs(&self) -> Self::AbsOutput {
        if *self < 0. {
            -self
        } else {
            self.clone()
        }
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

impl MulAdd<f64, f64> for f64 {
    fn mul_add(self, a: &f64, b: &f64) -> Self {
        self.mul_add(*a, *b)
    }
}

impl Sqrt for f64 {
    fn sqrt(self: Self) -> Self {
        self.powf(0.5)
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

impl Dot<f64> for [f64] {
    fn dot(&self, v: &Vector<f64>) -> f64 {
        assert_eq!(v.size(), self.len(), "vectors must be the same size");

        self * v
    }
}

impl Dot<f64> for Vector<f64> {
    fn dot(&self, v: &Vector<f64>) -> f64 {
        assert_eq!(v.size(), self.size(), "vectors must be the same size");

        self * v
    }
}

impl Mul<&Vector<f64>> for &Vector<f64> {
    type Output = f64;

    fn mul(self, rhs: &Vector<f64>) -> Self::Output {
        self * &rhs._d
    }
}

impl Mul<&Vec<f64>> for &Vector<f64> {
    type Output = f64;

    fn mul(self, rhs: &Vec<f64>) -> Self::Output {
        self * &rhs[..]
    }
}

impl Mul<&Vector<f64>> for &Vec<f64> {
    type Output = f64;

    fn mul(self, rhs: &Vector<f64>) -> Self::Output {
        rhs * self
    }
}

impl Mul<&[f64]> for &Vector<f64> {
    type Output = f64;

    fn mul(self, rhs: &[f64]) -> Self::Output {
        let mut sum = f64::default();

        for (a, &b) in self._d.iter().zip(rhs) {
            sum = a.mul_add(b, sum);
        }

        sum
    }
}

impl Mul<&Vector<f64>> for &[f64] {
    type Output = f64;

    fn mul(self, rhs: &Vector<f64>) -> Self::Output {
        rhs * self
    }
}

impl MulAdd<f64, Vector<f64>> for Vector<f64> {
    fn mul_add(self, a: &f64, b: &Vector<f64>) -> Self {
        assert!(self.size() == b.size(), "vectors must be the same size");

        let mut vec = Vec::with_capacity(self.size());

        for i in 0..self.size() {
            vec.push(self[i].mul_add(*a, b[i]))
        }

        V!(vec)
    }
}

impl Angle for Vector<f64> {
    type Output = f64;
    fn angle_cos(u: &Vector<f64>, v: &Vector<f64>) -> Self::Output {
        u.dot(v) / (u.norm() * v.norm())
    }
}

impl Transpose<f64> for Matrix<f64> {
    fn transpose(&self) -> Matrix<f64> {
        let mut vec = Vec::with_capacity(self.rows * self.cols);
        for i in 0..self.cols {
            for j in 0..self.rows {
                vec.push(self[j][i]);
            }
        }

        Matrix {
            rows: self.cols,
            cols: self.rows,
            _d: vec,
        }
    }
}
