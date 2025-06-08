use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
    slice::{Chunks, ChunksMut},
};

use crate::{scalar::Scalar, vector::Vector};

#[derive(Clone)]
pub struct Matrix<K> {
    pub data: Vec<K>,
    pub rows: usize,
    pub cols: usize,
}

impl<K: Scalar> Debug for Matrix<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for i in 0..self.rows {
            write!(f, "{:?}", &self[i])?;
            if i != self.rows - 1 {
                writeln!(f, ",")?;
            }
        }
        f.write_str("]")?;
        Ok(())
    }
}

impl<K: Clone, const N: usize, const D: usize> From<[[K; D]; N]> for Matrix<K> {
    fn from(value: [[K; D]; N]) -> Self {
        let mut vec = Vec::with_capacity(N * D);
        (0..N).for_each(|i| {
            for j in 0..D {
                vec.push(value[i][j].clone());
            }
        });

        Matrix {
            rows: N,
            cols: D,
            data: vec,
        }
    }
}

impl<'a, K> IntoIterator for &'a Matrix<K> {
    type Item = &'a [K];
    type IntoIter = Chunks<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.chunks(self.cols)
    }
}

impl<'a, K> IntoIterator for &'a mut Matrix<K> {
    type Item = &'a mut [K];
    type IntoIter = ChunksMut<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.chunks_mut(self.cols)
    }
}

impl<K> Index<usize> for Matrix<K> {
    type Output = [K];

    fn index(&self, index: usize) -> &[K] {
        let start = index * self.cols;
        &self.data[start..start + self.cols]
    }
}

impl<K> IndexMut<usize> for Matrix<K> {
    fn index_mut(&mut self, index: usize) -> &mut [K] {
        let start = index * self.cols;
        &mut self.data[start..start + self.cols]
    }
}

impl<K: Scalar> Add for Matrix<K> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(
            self.shape(),
            other.shape(),
            "matrices must be the same size"
        );

        let mut vec = Vec::with_capacity(self.rows * self.cols);
        for i in 0..self.data.len() {
            vec.push(self.data[i] + other.data[i]);
        }

        Matrix {
            data: vec,
            cols: self.cols,
            rows: self.rows,
        }
    }
}

impl<K: Scalar> AddAssign<&Matrix<K>> for Matrix<K> {
    fn add_assign(&mut self, rhs: &Matrix<K>) {
        assert_eq!(self.shape(), rhs.shape(), "matrices must be the same size");

        for i in 0..self.data.len() {
            self.data[i] += rhs.data[i]
        }
    }
}

impl<K: Scalar> Sub for Matrix<K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(
            self.shape(),
            other.shape(),
            "matrices must be the same size"
        );

        let mut vec = Vec::with_capacity(self.rows * self.cols);
        for i in 0..self.data.len() {
            vec.push(self.data[i] - other.data[i]);
        }

        Matrix {
            data: vec,
            cols: self.cols,
            rows: self.rows,
        }
    }
}

impl<K: Scalar> SubAssign<&Matrix<K>> for Matrix<K> {
    fn sub_assign(&mut self, rhs: &Matrix<K>) {
        assert_eq!(self.shape(), rhs.shape(), "matrices must be the same size");

        for i in 0..self.data.len() {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<K: Scalar> Mul<K> for Matrix<K> {
    type Output = Matrix<K>;

    fn mul(self, a: K) -> Self::Output {
        let mut vec = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            vec.push(self.data[i] * a);
        }

        Matrix {
            data: vec,
            cols: self.cols,
            rows: self.rows,
        }
    }
}

impl<K: Scalar> MulAssign<&K> for Matrix<K> {
    fn mul_assign(&mut self, rhs: &K) {
        for i in 0..self.data.len() {
            self.data[i] *= rhs;
        }
    }
}

impl<K: Scalar> Mul<&Vector<K>> for &Matrix<K> {
    type Output = Vector<K>;

    fn mul(self, rhs: &Vector<K>) -> Self::Output {
        assert_eq!(
            self.shape().0,
            rhs.size(),
            "bad input for matrix and vector column multiplication"
        );

        let mut vec = Vec::with_capacity(rhs.size());
        for i in 0..rhs.size() {
            vec.push(&self[i] * rhs);
        }
        Vector::from(vec)
    }
}

impl<K: Scalar> Mul<&Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;

    fn mul(self, rhs: &Matrix<K>) -> Self::Output {
        let cols = rhs.cols;
        let rows = self.rows;
        let mut vec = Vec::with_capacity(rows * cols);

        for i in 0..rows {
            for j in 0..cols {
                let mut val = self[i][0] * rhs[0][j];
                for r in 1..self.cols {
                    val += self[i][r] * rhs[r][j];
                }
                vec.push(val);
            }
        }

        Matrix {
            data: vec,
            rows,
            cols,
        }
    }
}

impl<K: Scalar> Matrix<K> {
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
    pub fn iter_mut(&mut self) -> ChunksMut<'_, K> {
        self.data.chunks_mut(self.cols)
    }

    pub fn iter(&self) -> Chunks<'_, K> {
        self.data.chunks(self.cols)
    }

    pub fn add(&mut self, v: &Matrix<K>) {
        *self += v;
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        *self -= v;
    }

    pub fn scl(&mut self, a: K) {
        *self *= &a;
    }

    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        self * vec
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        self * mat
    }

    pub fn trace(&self) -> Option<K> {
        assert!(self.is_square(), "matrix must be squared");

        match self.rows {
            0 => None,
            _ => Some((0..self.rows).map(|i| self[i][i]).sum()),
        }
    }

    pub fn transpose(&self) -> Matrix<K> {
        let mut vec = Vec::with_capacity(self.rows * self.cols);
        for i in 0..self.cols {
            for j in 0..self.rows {
                vec.push(self[j][i]);
            }
        }

        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: vec,
        }
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        let mut ret = self.clone();
        ret.row_echelon_mut();
        ret
    }

    pub fn row_echelon_mut(&mut self) -> &mut Self {
        let mut col: usize = 0;

        for r in 0..self.rows {
            let mut row = None;

            for j in col..self.cols {
                for i in r..self.rows {
                    if self[i][j] != K::default() {
                        row = Some((i, j));
                        break;
                    }
                }

                if row.is_some() {
                    break;
                }
            }

            if row.is_none() {
                break;
            }
            let cur = row.unwrap();
            col = cur.1;
            if r != cur.0 {
                for c in col..self.cols {
                    let tmp = self[r][c];
                    self[r][c] = self[cur.0][c];
                    self[cur.0][c] = tmp;
                }
            }

            let p = self[r][col];
            for i in 0..self.rows {
                if i == r {
                    continue;
                }
                let cur = self[i][col];
                if cur == K::default() {
                    continue;
                }

                let a = -cur / p;
                for j in col..self.cols {
                    let x = self[r][j];
                    self[i][j] += x * a;
                }
            }

            for i in col..self.cols {
                self[r][i] *= p.inv();
            }
        }

        self
    }

    pub fn determinant(&self) -> K {
        assert!(self.is_square(), "matrix must be squared");

        fn det<K: Scalar>(
            mat: &[K],
            row_size: usize,
            cur: usize,
            skip_cols: &[bool],
        ) -> K {
            let mut cols = Vec::with_capacity(skip_cols.len());

            for (col, skip) in skip_cols.iter().enumerate() {
                if !*skip {
                    cols.push(col);
                }
            }

            match cur {
                0 => K::default(),
                1 => mat[0],
                2 => {
                    mat[cols[0]] * mat[row_size + cols[1]]
                        - mat[cols[1]] * mat[row_size + cols[0]]
                }
                _ => {
                    let mut result = K::default();
                    for (i, col) in cols.iter().enumerate() {
                        let mut m_col = skip_cols.to_vec();
                        m_col[*col] = true;
                        let ret = mat[*col]
                            * det(&mat[row_size..], row_size, cur - 1, &m_col);
                        result += if i % 2 == 1 { -ret } else { ret };
                        m_col[*col] = false;
                    }
                    result
                }
            }
        }

        det(
            self.data.as_slice(),
            self.rows,
            self.rows,
            &vec![false; self.cols],
        )
    }

    pub fn inverse(&self) -> Result<Matrix<K>, &'static str> {
        let mut aug_m = Matrix {
            data: vec![K::default(); self.rows * self.cols * 2],
            cols: self.cols * 2,
            rows: self.rows,
        };

        for row in 0..self.rows {
            for col in 0..self.cols {
                aug_m.data[row * (self.cols * 2) + col] = self[row][col];
            }
            aug_m.data[row * (self.cols * 2) + self.cols + row] = K::one();
        }

        aug_m.row_echelon_mut();

        let mut vec = Vec::with_capacity(self.rows * self.cols);

        for row in aug_m.data.chunks(self.cols * 2) {
            for &v in row.iter().skip(self.cols) {
                vec.push(v);
            }
        }

        Ok(Matrix {
            data: vec,
            cols: self.cols,
            rows: self.rows,
        })
    }

    pub fn rank(&self) -> usize {
        let mat = self.row_echelon();
        let mut rank = 0;

        let mut cur = 0;
        for row in &mat {
            for (j, v) in row.iter().enumerate().skip(cur) {
                if *v != K::default() {
                    cur = j + 1;
                    rank += 1;
                    break;
                }
            }
        }

        rank
    }

    pub fn is_independent(&self) -> bool {
        self.rank() == self.rows
    }

    pub fn row_space(&self) -> Matrix<K> {
        let mat = self.row_echelon();
        let mut vec = vec![];

        let mut cur = 0;
        for row in &mat {
            for (j, v) in row.iter().enumerate().skip(cur) {
                if *v != K::default() {
                    cur = j + 1;
                    vec.copy_from_slice(row);
                    break;
                }
            }
        }

        Matrix {
            cols: self.cols,
            rows: vec.len() / self.cols,
            data: vec,
        }
    }
}

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let tan = f32::tan(fov.to_radians() / 2.);
    let top = near * tan;
    let right = top * ratio;

    Matrix::from([
        [near / right, 0.0, 0.0, 0.0],
        [0.0, near / top, 0.0, 0.0],
        [
            0.0,
            0.0,
            -(far + near) / (far - near),
            -2. * far * near / (far - near),
        ],
        [0.0, 0.0, -1.0, 0.0],
    ])
}
