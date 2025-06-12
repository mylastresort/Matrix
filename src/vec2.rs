use crate::{Matrix, Vector};

macro_rules! M {
    ($values:expr) => {
        &Matrix::from($values)
    };
}

macro_rules! size {
    ($vec:expr) => {
        assert!($vec.size() == 2, "vector must be of size 2");
    };
}

pub fn scale2(v: &Vector<f32>, dx: f32, dy: f32) -> Vector<f32> {
    size!(v);
    M!([[dx, 0.], [0., dy]]) * v
}

pub fn shear2x(v: &Vector<f32>, s: f32) -> Vector<f32> {
    size!(v);
    M!([[1., s], [0., 1.]]) * v
}

pub fn shear2y(v: &Vector<f32>, s: f32) -> Vector<f32> {
    size!(v);
    M!([[1., 0.], [s, 1.]]) * v
}

pub fn rotate2(v: &Vector<f32>, deg: f32) -> Vector<f32> {
    size!(v);
    M!([[deg.cos(), -deg.sin()], [deg.sin(), deg.cos()]]) * v
}

pub fn reflect2x(v: &Vector<f32>) -> Vector<f32> {
    size!(v);
    M!([[1., 0.], [0., -1.]]) * v
}

pub fn reflect2y(v: &Vector<f32>) -> Vector<f32> {
    size!(v);
    M!([[-1., 0.], [0., 1.]]) * v
}
