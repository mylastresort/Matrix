# Matrix

A linear algebra library written in Rust with basic matrix and vector operations.

## Overview

This is a practical library that implements essential linear algebra operations for matrices and vectors. It provides the fundamental building blocks you need for mathematical computations: dot and cross products for vectors, matrix operations (multiplication, transpose, inverse, determinant, row echelon form, rank), and support for real numbers (`f32`, `f64`) and complex numbers. Perfect for applications requiring basic linear algebra without the overhead of larger mathematical libraries.

## Notes

This is a basic educational library implementing fundamental linear algebra operations. It's designed for learning purposes and simple mathematical computations rather than high-performance applications.

## Features

### Core Data Structures

- **Matrix<K>**: Generic matrix implementation
- **Vector<K>**: Generic vector implementation
- **Complex**: Complex number implementation

The core data structures are implemented as:

**Vector<K>**:

```rust
pub struct Vector<K> {
    pub _d: Vec<K>,  // Internal data storage
}
```

**Matrix<K>**:

```rust
pub struct Matrix<K> {
    pub _d: Vec<K>,     // Flattened data storage
    pub rows: usize,    // Number of rows
    pub cols: usize,    // Number of columns
}
```

Both structures are generic over type `K` that must implement the `Scalar` trait, allowing them to work with `f32`, `f64`, and `Complex` numbers. Matrices use row-major order for internal storage, and both provide convenient macros (`V![]` and `M![]`) for creation.

### Basic Operations

#### Vector Operations

- Addition, subtraction, scalar multiplication
- Dot product, cross product (3D only)
- Linear combination
- Norm calculation, angle between vectors (cosine)
- Linear interpolation (lerp)

#### Matrix Operations

- Addition, subtraction, scalar multiplication
- Matrix multiplication, matrix-vector multiplication
- Transpose, inverse, row echelon form
- Trace, determinant, rank

#### Simple 2D/3D Transformations

- **2D**: scale, rotate, shear, reflect
- **3D**: scale, rotate around axes

### Scalar Types

- `f32` and `f64` floating-point numbers
- Complex numbers (`Complex`)

## Usage

### Vectors

```rust
use matrix::{Vector, Matrix, V, M, linear_combination, cross_product, angle_cos, lerp};

// Creating vectors
let v1 = V!([1.0, 2.0, 3.0]);
let v2 = V!([4.0, 5.0, 6.0]);

// Vector arithmetic
let u = V!([2.0, 3.0]);
let v = V!([5.0, 7.0]);
let scalar = 2.0;

// Addition
let result = &u + &v; // [7.0, 10.0]

// Subtraction
let result = &u - &v; // [-3.0, -4.0]

// Scaling
let result = &u * &scalar; // [4.0, 6.0]

// Vector dot product
let u = V!([0.0, 0.0]);
let v = V!([1.0, 1.0]);
let result = &u * &v; // 0.0

// Vector cross product
let u = V!([0.0, 0.0, 1.0]);
let v = V!([1.0, 0.0, 0.0]);
let result = cross_product(&u, &v); // [0.0, 1.0, 0.0]

// Vector norm
let u = V!([1.0, 2.0, 3.0]);
let magnitude = u.norm(); // 3.7416573867739413

// Vector angle
let u = V!([1.0, 0.0]);
let v = V!([1.0, 1.0]);
let cos_angle = angle_cos(&u, &v); // 0.7071067811865476

// Vector linear interpolation
let u = V!([2.0, 1.0]);
let v = V!([4.0, 2.0]);
let result = lerp(u, v, 0.3); // [2.6, 1.3]

// Vector linear combination
let e1 = V!([1.0, 0.0, 0.0]);
let e2 = V!([0.0, 1.0, 0.0]);
let e3 = V!([0.0, 0.0, 1.0]);
let result = linear_combination(&[&e1, &e2, &e3], &[10.0, -2.0, 0.5]); // [10.0, -2.0, 0.5]
```

### Matrices

```rust
use matrix::{M, V};

// Creating matrices
let m = M!([[1.0, 2.0], [3.0, 4.0]]);

// Matrix arithmetic
let u = M!([[1.0, 2.0], [3.0, 4.0]]);
let v = M!([[7.0, 4.0], [-2.0, 2.0]]);
let scalar = 2.0;

// Addition
let result = &u + &v; // [[8.0, 6.0], [1.0, 6.0]]

// Subtraction
let result = &u - &v; // [[-6.0, -2.0], [5.0, 2.0]]

// Scaling
let result = &u * &scalar; // [[2.0, 4.0], [6.0, 8.0]]

// Matrix multiplication
let A = M!([[1.0, 0.0], [0.0, 1.0]]);
let B = M!([[1.0, 0.0], [0.0, 1.0]]);
let v = V!([4.0, 2.0]);

// Matrix-Matrix multiplication
let result = &A * &B; // [[1.0, 0.0], [0.0, 1.0]]

// Matrix-Vector multiplication
let result = &A * &v; // [4.0, 2.0]

// Matrix transpose
let m = M!([[1.0, 2.0], [3.0, 4.0]]);
let result = m.transpose(); // [[1.0, 3.0], [2.0, 4.0]]

// Matrix trace
let m = M!([[1.0, 2.0], [3.0, 4.0]]);
if let Some(tr) = m.trace() {
    println!("Trace: {}", tr); // 5.0
}

// Matrix determinant
let m = M!([[1.0, -1.0], [1.0, 1.0]]);
if let Some(det) = m.determinant() {
    println!("Determinant: {}", det); // 2.0
}

// Matrix inverse
let m = M!([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
if let Some(inv) = m.inverse() {
    println!("Inverse: {:?}", inv); // Identity matrix
}

// Matrix row echelon form
let mut m = M!([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
m.row_echelon();
println!("Row echelon form: {:?}", m);

// Matrix rank
let m = M!([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
let rank = m.rank(); // 3
```

### Complex Numbers

`Vector<K>` and `Matrix<K>` are generic over scalar types, so you can use complex numbers as the generic parameter. This means all vector and matrix operations mentioned above (arithmetic, dot product, cross product, norm, transpose, determinant, inverse, etc.) are available for complex numbers.

The `Complex` struct is implemented as:

```rust
pub struct Complex {
    pub x: f64,  // Real part
    pub y: f64,  // Imaginary part
}
```

It implements the `Scalar` trait with full arithmetic operations, trigonometric functions (sin, cos, tan), square root, absolute value (magnitude), and complex conjugate. The struct uses `f64` precision for both real and imaginary components and supports all standard mathematical operations you'd expect from complex numbers.

```rust
use matrix::{Complex, Vector, Matrix, C, V, M, linear_combination};

// Creating complex numbers
let c1 = C!(1.0, 2.0); // 1 + 2i
let c2 = C!(3.0, 4.0); // 3 + 4i

// Basic arithmetic
let result = &c1 + &c2; // 4 + 6i
let result = &c1 * &c2; // -5 + 10i
let result = &c1 / &c2; // 0.44 + 0.08i

// Complex vectors
let cv1 = V!([C!(1.0, 0.0), C!(0.0, 1.0)]);
let cv2 = V!([C!(2.0, 1.0), C!(1.0, -1.0)]);
let result = &cv1 * &cv2; // Complex dot product with conjugate

// Complex matrices
let cm1 = M!([[C!(1.0, 0.0), C!(0.0, 1.0)], [C!(1.0, 1.0), C!(2.0, 0.0)]]);
let cm2 = M!([[C!(2.0, 0.0), C!(1.0, 0.0)], [C!(0.0, 1.0), C!(1.0, 1.0)]]);
let result = &cm1 * &cm2; // Complex matrix multiplication

// Complex conjugate transpose
let result = cm1.transpose(); // Conjugate transpose for complex matrices

// Linear combination with complex coefficients
let e1 = V!([C!(1.0, 0.0), C!(0.0, 0.0)]);
let e2 = V!([C!(0.0, 0.0), C!(1.0, 0.0)]);
let result = linear_combination(&[&e1, &e2], &[C!(2.0, 1.0), C!(1.0, -1.0)]);
```

## Project Structure

```
src/
├── lib.rs          # Main library exports
├── matrix.rs       # Matrix implementation
├── vector.rs       # Vector implementation
├── complex.rs      # Complex number implementation
├── scalar.rs       # Scalar trait definitions
├── f32.rs          # f32 implementations
├── f64.rs          # f64 implementations
├── vec2.rs         # 2D transformation functions
├── vec3.rs         # 3D transformation functions
└── utils.rs        # Utilities

tests/              # Test files for each operation
matrix_display/     # 3D projection visualization tool
```

## Testing

Run the test suite:

```bash
cargo test
```

Tests cover basic operations like addition, multiplication, dot products, and edge cases.

## Matrix Display Tool

The `matrix_display/` directory contains a simple 3D visualization tool for testing projection matrices. It uses 4x4 matrices because this is the standard format for 3D graphics transformations in homogeneous coordinates. 4x4 matrices can represent translation, rotation, scaling, and projection operations in 3D space - the fourth dimension allows for perspective projection and proper handling of 3D-to-2D transformations.

### Creating a Projection Matrix File

To test your projection matrix, create a `proj` file in the `matrix_display/` directory with your 4x4 matrix values. For example, using the library's projection function:

```rust
use matrix::projection;

// Create a perspective projection matrix
// fov: 20 degrees, aspect ratio: 1.5, near: 2.0, far: 50.0
let proj_matrix = projection(20.0, 1.5, 2.0, 50.0);
```

This generates a matrix that you can save to the `proj` file:

```
3.7808547, 0, 0, 0
0, 5.671282, 0, 0
0, 0, -1.0416666, -2.0833333
0, 0, -1, 0
```

Run the display program to see if your projection matrix correctly renders the 3D scene.
