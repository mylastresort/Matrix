use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Index, Mul, MulAssign, Sub, SubAssign},
    slice::{Iter, IterMut},
};

use crate::scalar::Scalar;

#[derive(Clone, Default)]
pub struct Vector<K> {
    pub data: Vec<K>,
}

#[derive(Clone, Debug)]
pub struct ScaledVector<'a, K: Scalar> {
    pub v: &'a Vector<K>,
    pub k: &'a K,
}

impl<K: Scalar> Debug for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K: Clone, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Vector {
            data: Vec::from(data),
        }
    }
}

impl<K> From<Vec<K>> for Vector<K> {
    fn from(data: Vec<K>) -> Self {
        Vector { data }
    }
}

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
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

        Vector::from(vec)
    }
}

impl<K: Scalar> AddAssign<&Vector<K>> for Vector<K> {
    fn add_assign(&mut self, rhs: &Vector<K>) {
        assert_eq!(self.size(), rhs.size(), "vectors must be the same size");

        for i in 0..self.size() {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<'a, K: Scalar> AddAssign<ScaledVector<'a, K>> for Vector<K> {
    fn add_assign(&mut self, rhs: ScaledVector<'a, K>) {
        assert_eq!(self.size(), rhs.v.size(), "vectors must be the same size");

        for i in 0..self.size() {
            self.data[i] = rhs.v.data[i].mul_add(*rhs.k, self.data[i]);
        }
    }
}

impl<K: Scalar> Sub for Vector<K> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(), rhs.size(), "vectors must be the same size");

        let mut vec = Vec::with_capacity(self.size());
        for i in 0..self.size() {
            vec.push(self.data[i] - rhs.data[i]);
        }

        Vector::from(vec)
    }
}

impl<K: Scalar> SubAssign<&Vector<K>> for Vector<K> {
    fn sub_assign(&mut self, rhs: &Vector<K>) {
        assert_eq!(self.size(), rhs.size(), "vectors must be the same size");

        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(x1, x2)| x1.sub_assign(x2));
    }
}

impl<K: Scalar> Mul<K> for Vector<K> {
    type Output = Self;

    fn mul(self, a: K) -> Self::Output {
        &self * &a
    }
}

impl<K: Scalar> Mul<&K> for &Vector<K> {
    type Output = Vector<K>;

    fn mul(self, &a: &K) -> Self::Output {
        let mut vec = Vec::with_capacity(self.size());

        for i in 0..self.size() {
            vec.push(self.data[i] * a);
        }
        Vector::from(vec)
    }
}

impl<K: Scalar> MulAssign<&K> for Vector<K> {
    fn mul_assign(&mut self, a: &K) {
        for i in self.iter_mut() {
            *i *= a;
        }
    }
}

impl<K: Scalar> Mul<&Vector<K>> for &Vector<K> {
    type Output = K;

    fn mul(self, other: &Vector<K>) -> Self::Output {
        self.iter()
            .zip(other.iter())
            .map(|(&x1, &x2)| x1.mul(x2))
            .sum()
    }
}

impl<K: Scalar> Mul<&Vector<K>> for &[K] {
    type Output = K;

    fn mul(self, other: &Vector<K>) -> Self::Output {
        self.iter()
            .zip(other.iter())
            .map(|(&x1, &x2)| x1.mul(x2))
            .sum()
    }
}

impl<'a, K: Scalar> Sum<ScaledVector<'a, K>> for Vector<K> {
    fn sum<I: Iterator<Item = ScaledVector<'a, K>>>(mut iter: I) -> Self {
        match iter.next().map(|vec| vec.v * vec.k) {
            None => Vector::default(),
            Some(first) => iter.fold(first, |mut acc, cur| {
                acc += cur;
                acc
            }),
        }
    }
}

#[macro_export]
macro_rules! linear_combination {
    ($u:expr, $coefs:expr) => {
        $u.into_iter()
            .zip($coefs)
            .map(|(v, k)| $crate::ScaledVector { v, k })
            .sum()
    };
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
    assert!(!u.is_empty(), "list must not be empty");

    assert!(
        u.iter().all(|v| v.size() == u[0].size()),
        "vectors must be have the same dimention"
    );

    linear_combination!(u.iter().copied(), coefs)
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

    Vector::from([
        (u[1] * v[2]) - (u[2] * v[1]),
        (u[2] * v[0]) - (u[0] * v[2]),
        (u[0] * v[1]) - (u[1] * v[0]),
    ])
}

impl<K: Scalar> Vector<K> {
    pub fn zero(size: usize) -> Self {
        Vector::from(vec![K::default(); size])
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> Iter<'_, K> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K> {
        self.data.iter_mut()
    }

    pub fn first(&self) -> Option<&K> {
        self.data.first()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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
        self.iter().map(|&x| x.abs()).sum::<K::AbsOutput>()
    }

    pub fn norm(&self) -> K {
        K::sqrt(self.iter().map(|&x| x * x).sum())
    }

    pub fn norm_inf(&self) -> K::AbsOutput {
        self.iter().map(|&x| x.abs()).fold(
            K::AbsOutput::default(),
            |acc, cur| if acc > cur { acc } else { cur },
        )
    }
}
