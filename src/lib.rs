pub mod complex;

pub mod scalar;

pub mod matrix;
pub mod vector;

use std::ops::{Add, Mul, Sub};

pub use matrix::Matrix;
pub use vector::Vector;

pub use matrix::projection;
pub use vector::angle_cos;
pub use vector::cross_product;
pub use vector::linear_combination;

pub fn lerp<
    V: Clone + Sub<Output = V> + Mul<f32, Output = V> + Add<Output = V>,
>(
    u: V,
    v: V,
    t: f32,
) -> V {
    match t {
        0. => u.clone(),
        1. => v.clone(),
        p => u.clone() + (v - u) * p,
    }
}
