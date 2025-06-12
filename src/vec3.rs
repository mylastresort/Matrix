use crate::{Matrix, Vector};

macro_rules! M {
    ($values:expr) => {
        &Matrix::from($values)
    };
}

macro_rules! size {
    ($vec:expr) => {
        assert!($vec.size() == 3, "vector must be of size 3");
    };
}

pub fn scale3(v: &Vector<f32>, dx: f32, dy: f32, dz: f32) -> Vector<f32> {
    size!(v);
    M!([[dx, 0., 0.], [0., dy, 0.], [0., 0., dz]]) * v
}

pub fn rotate3x(v: &Vector<f32>, deg: f32) -> Vector<f32> {
    size!(v);
    M!([
        [1., 0., 0.],
        [0., deg.cos(), -deg.sin()],
        [0., deg.sin(), deg.cos()]
    ]) * v
}

pub fn rotate3y(v: &Vector<f32>, deg: f32) -> Vector<f32> {
    size!(v);
    M!([
        [deg.cos(), 0., deg.sin()],
        [0., 1., 0.],
        [-deg.sin(), 0., deg.cos()]
    ]) * v
}

pub fn rotate3z(v: &Vector<f32>, deg: f32) -> Vector<f32> {
    size!(v);
    M!([
        [deg.cos(), -deg.sin(), 0.],
        [deg.sin(), deg.cos(), 0.],
        [0., 0., 1.]
    ]) * v
}
