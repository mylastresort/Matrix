use std::ops::Mul;

use crate::{
    matrix::Transpose,
    scalar::{Lerp, MulAdd, Sqrt},
    vector::Angle,
    Dot, Matrix, Scalar, Vector, V,
};

impl Scalar for f32 {
    type AbsOutput = f32;
    type SinOutput = f32;
    type CosOutput = f32;

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

impl MulAdd<f32, f32> for f32 {
    fn mul_add(self, a: &f32, b: &f32) -> Self {
        self.mul_add(*a, *b)
    }
}

impl Sqrt for f32 {
    fn sqrt(self: Self) -> Self {
        self.powf(0.5)
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

impl Dot<f32> for [f32] {
    fn dot(&self, v: &Vector<f32>) -> f32 {
        assert_eq!(v.size(), self.len(), "vectors must be the same size");

        self * v
    }
}

impl Dot<f32> for Vector<f32> {
    fn dot(&self, v: &Vector<f32>) -> f32 {
        assert_eq!(v.size(), self.size(), "vectors must be the same size");

        self * v
    }
}

impl Mul<&Vector<f32>> for &Vector<f32> {
    type Output = f32;

    fn mul(self, rhs: &Vector<f32>) -> Self::Output {
        self * &rhs._d
    }
}

impl Mul<&Vec<f32>> for &Vector<f32> {
    type Output = f32;

    fn mul(self, rhs: &Vec<f32>) -> Self::Output {
        self * &rhs[..]
    }
}

impl Mul<&Vector<f32>> for &Vec<f32> {
    type Output = f32;

    fn mul(self, rhs: &Vector<f32>) -> Self::Output {
        rhs * self
    }
}

impl Mul<&[f32]> for &Vector<f32> {
    type Output = f32;

    fn mul(self, rhs: &[f32]) -> Self::Output {
        let mut sum = f32::default();

        for (a, &b) in self._d.iter().zip(rhs) {
            sum = a.mul_add(b, sum);
        }

        sum
    }
}

impl Mul<&Vector<f32>> for &[f32] {
    type Output = f32;

    fn mul(self, rhs: &Vector<f32>) -> Self::Output {
        rhs * self
    }
}

impl MulAdd<f32, Vector<f32>> for Vector<f32> {
    fn mul_add(self, a: &f32, b: &Vector<f32>) -> Self {
        assert!(self.size() == b.size(), "vectors must be the same size");

        let mut vec = Vec::with_capacity(self.size());

        for i in 0..self.size() {
            vec.push(self[i].mul_add(*a, b[i]))
        }

        V!(vec)
    }
}

impl Angle for Vector<f32> {
    type Output = f32;
    fn angle_cos(u: &Vector<f32>, v: &Vector<f32>) -> Self::Output {
        u.dot(v) / (u.norm() * v.norm())
    }
}

impl Transpose<f32> for Matrix<f32> {
    fn transpose(&self) -> Matrix<f32> {
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
