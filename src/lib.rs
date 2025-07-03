pub mod complex;

pub mod scalar;

pub mod matrix;
pub mod vector;

pub use complex::Complex;
pub use matrix::Matrix;
pub use scalar::{lerp, Scalar};
pub use vector::Vector;

pub use matrix::projection;
pub use vector::angle_cos;
pub use vector::cross_product;
pub use vector::linear_combination;

pub mod vec2;
pub mod vec3;
