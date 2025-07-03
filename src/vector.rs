use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

use crate::scalar::{Lerp, MulAdd, Scalar};

#[derive(Clone, Default)]
pub struct Vector<K> {
    pub _d: Vec<K>,
}

#[macro_export]
macro_rules! V {
    () => {
        Vector::default()
    };

    ($values:expr) => {
        Vector::from($values)
    };
}

impl<K: Scalar> Debug for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_list().entries(self._d.iter()).finish()
    }
}

impl<K: Clone, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Vector {
            _d: Vec::from(data),
        }
    }
}

impl<K> From<Vec<K>> for Vector<K> {
    fn from(data: Vec<K>) -> Self {
        Vector { _d: data }
    }
}

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self._d[index]
    }
}

impl<K: Scalar> Add for Vector<K> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.size(), other.size(), "vectors must be the same size");

        let mut vec = Vec::with_capacity(self.size());
        for i in 0..self.size() {
            vec.push(self[i] + other[i]);
        }

        V!(vec)
    }
}

impl<K: Scalar> AddAssign<&Vector<K>> for Vector<K> {
    fn add_assign(&mut self, rhs: &Vector<K>) {
        assert_eq!(self.size(), rhs.size(), "vectors must be the same size");

        for i in 0..self.size() {
            self._d[i] += rhs[i];
        }
    }
}

impl<K: Scalar> Sub for Vector<K> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(), rhs.size(), "vectors must be the same size");

        let mut vec = Vec::with_capacity(self.size());
        for i in 0..self.size() {
            vec.push(self[i] - rhs[i]);
        }

        V!(vec)
    }
}

impl<K: Scalar> SubAssign<&Vector<K>> for Vector<K> {
    fn sub_assign(&mut self, rhs: &Vector<K>) {
        assert_eq!(self.size(), rhs.size(), "vectors must be the same size");

        for i in 0..self.size() {
            self._d[i] -= rhs[i];
        }
    }
}

impl<K: Scalar + Mul<U, Output = K>, U: Scalar> Mul<U> for Vector<K> {
    type Output = Self;

    fn mul(self, a: U) -> Self::Output {
        &self * &a
    }
}

impl<K: Scalar + Mul<U, Output = K>, U: Scalar> Mul<&U> for &Vector<K> {
    type Output = Vector<K>;

    fn mul(self, &a: &U) -> Self::Output {
        let mut vec = Vec::with_capacity(self.size());

        for i in 0..self.size() {
            vec.push(self[i] * a);
        }

        V!(vec)
    }
}

impl<K: Scalar + MulAssign<U>, U: Scalar> MulAssign<&U> for Vector<K> {
    fn mul_assign(&mut self, a: &U) {
        for i in &mut self._d {
            *i *= *a;
        }
    }
}

impl<K: Scalar + Mul<U, Output = K> + MulAdd<U, K>, U: Scalar>
    MulAdd<U, Vector<K>> for Vector<K>
{
    fn mul_add(self, a: U, b: &Vector<K>) -> Self {
        assert!(self.size() == b.size(), "vectors must be the same size");

        let mut vec = Vec::with_capacity(self.size());

        for i in 0..self.size() {
            vec.push(self[i].mul_add(a, &b[i]))
        }

        V!(vec)
    }
}

impl<K: Scalar> Mul<&[K]> for &Vector<K> {
    type Output = K;

    fn mul(self, rhs: &[K]) -> Self::Output {
        let mut sum = K::default();

        for (a, b) in self._d.iter().zip(rhs) {
            sum = a.mul_add(*b, &sum);
        }

        sum
    }
}

impl<K: Scalar> Mul<&Vector<K>> for &[K] {
    type Output = K;

    fn mul(self, rhs: &Vector<K>) -> Self::Output {
        rhs * self
    }
}

impl<K: Scalar> Mul<&Vec<K>> for &Vector<K> {
    type Output = K;

    fn mul(self, rhs: &Vec<K>) -> Self::Output {
        self * &rhs[..]
    }
}

impl<K: Scalar> Mul<&Vector<K>> for &Vec<K> {
    type Output = K;

    fn mul(self, rhs: &Vector<K>) -> Self::Output {
        rhs * self
    }
}

impl<K: Scalar> Mul<&Vector<K>> for &Vector<K> {
    type Output = K;

    fn mul(self, rhs: &Vector<K>) -> Self::Output {
        self * &rhs._d
    }
}

pub fn linear_combination<K: Scalar>(
    u: &[&Vector<K>],
    coefs: &[K],
) -> Vector<K> {
    assert_eq!(
        u.len(),
        coefs.len(),
        "vectors and scalers must be the same size"
    );

    assert!(
        u.iter().all(|v| v.size() == u[0].size()),
        "vectors must be have the same dimention"
    );

    let mut iter = u.iter().zip(coefs);

    if let Some(mut sum) = iter.next().map(|(&v, &k)| v.clone() * k) {
        for (v, &k) in iter {
            for i in 0..sum.size() {
                sum._d[i] = v[i].mul_add(k, &sum[i]);
            }
        }
        sum
    } else {
        V!()
    }
}

impl<K: Scalar + MulAdd<f32, K>> Lerp for Vector<K> {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        match t {
            0. => u,
            1. => v,
            p => {
                let mut vec = Vec::with_capacity(u.size());

                for i in 0..u.size() {
                    vec.push((v[i] - u[i]).mul_add(p, &u[i]))
                }

                V!(vec)
            }
        }
    }
}

pub fn angle_cos<K: Scalar>(u: &Vector<K>, v: &Vector<K>) -> K {
    assert_eq!(u.size(), v.size(), "vectors must be the same size");
    assert!(!u.is_empty(), "vectors must be the same size");

    u.dot(v) / (u.norm() * v.norm())
}

pub fn cross_product<K: Scalar>(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
    assert!(
        u.size() == 3 && v.size() == u.size(),
        "vectors must have be of size 3"
    );

    V!([
        (u[1] * v[2]) - (u[2] * v[1]),
        (u[2] * v[0]) - (u[0] * v[2]),
        (u[0] * v[1]) - (u[1] * v[0]),
    ])
}

impl<K: Scalar> Vector<K> {
    pub fn zero(size: usize) -> Self {
        V!(vec![K::default(); size])
    }

    pub fn size(&self) -> usize {
        self._d.len()
    }

    pub fn first(&self) -> Option<&K> {
        self._d.first()
    }

    pub fn is_empty(&self) -> bool {
        self._d.is_empty()
    }

    pub fn add(&mut self, v: &Vector<K>) {
        *self += v;
    }

    pub fn sub(&mut self, vec: &Vector<K>) {
        *self -= vec;
    }

    pub fn scl(&mut self, a: K) {
        *self *= &a;
    }

    pub fn dot(&self, v: &Vector<K>) -> K {
        assert_eq!(v.size(), self.size(), "vectors must be the same size");

        self * v
    }

    pub fn norm_1(&self) -> K::AbsOutput {
        self._d.iter().map(|&x| x.abs()).sum::<K::AbsOutput>()
    }

    pub fn norm(&self) -> K {
        K::sqrt(self._d.iter().map(|&x| x * x).sum())
    }

    pub fn norm_inf(&self) -> K::AbsOutput {
        self._d.iter().map(|&x| x.abs()).fold(
            K::AbsOutput::default(),
            |acc, cur| if acc > cur { acc } else { cur },
        )
    }
}
