use std::{
    f32::consts::PI,
    fmt::Debug,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

use crate::{
    scalar::{Lerp, MulAdd, Scalar},
    vector::Vector,
};

#[derive(Clone)]
pub struct Matrix<K> {
    pub _d: Vec<K>,
    pub rows: usize,
    pub cols: usize,
}

#[macro_export]
macro_rules! M {
    ($values:expr) => {
        Matrix::from($values)
    };
}

impl<K: Scalar> Debug for Matrix<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
            _d: vec,
        }
    }
}

impl<K> Index<usize> for Matrix<K> {
    type Output = [K];

    fn index(&self, index: usize) -> &[K] {
        let start = index * self.cols;
        &self._d[start..start + self.cols]
    }
}

impl<K> IndexMut<usize> for Matrix<K> {
    fn index_mut(&mut self, index: usize) -> &mut [K] {
        let start = index * self.cols;
        &mut self._d[start..start + self.cols]
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
        for i in 0..self._d.len() {
            vec.push(self._d[i] + other._d[i]);
        }

        Matrix {
            _d: vec,
            cols: self.cols,
            rows: self.rows,
        }
    }
}

impl<K: Scalar> AddAssign<&Matrix<K>> for Matrix<K> {
    fn add_assign(&mut self, rhs: &Matrix<K>) {
        assert_eq!(self.shape(), rhs.shape(), "matrices must be the same size");

        for i in 0..self._d.len() {
            self._d[i] += rhs._d[i]
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
        for i in 0..self._d.len() {
            vec.push(self._d[i] - other._d[i]);
        }

        Matrix {
            _d: vec,
            cols: self.cols,
            rows: self.rows,
        }
    }
}

impl<K: Scalar> SubAssign<&Matrix<K>> for Matrix<K> {
    fn sub_assign(&mut self, rhs: &Matrix<K>) {
        assert_eq!(self.shape(), rhs.shape(), "matrices must be the same size");

        for i in 0..self._d.len() {
            self._d[i] -= rhs._d[i];
        }
    }
}

impl<K: Scalar + Mul<U, Output = K>, U: Scalar> Mul<U> for Matrix<K> {
    type Output = Matrix<K>;

    fn mul(self, a: U) -> Self::Output {
        let mut vec = Vec::with_capacity(self._d.len());
        for i in 0..self._d.len() {
            vec.push(self._d[i] * a);
        }

        Matrix {
            _d: vec,
            cols: self.cols,
            rows: self.rows,
        }
    }
}

impl<K: Scalar + MulAssign<U>, U: Scalar> MulAssign<&U> for Matrix<K> {
    fn mul_assign(&mut self, rhs: &U) {
        for i in 0..self._d.len() {
            self._d[i] *= *rhs;
        }
    }
}

impl<K: Scalar + Mul<U, Output = K> + MulAdd<U, K>, U: Scalar>
    MulAdd<U, Matrix<K>> for Matrix<K>
{
    fn mul_add(self, a: U, b: &Matrix<K>) -> Self {
        assert!(self.shape() == b.shape(), "matrices must be the same size");

        let mut vec = Vec::with_capacity(self.rows * self.cols);

        for i in 0..self._d.len() {
            vec.push(self._d[i].mul_add(a, &b._d[i]));
        }

        Matrix {
            _d: vec,
            rows: self.rows,
            cols: self.cols,
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
            _d: vec,
            rows,
            cols,
        }
    }
}

impl<K: Scalar + MulAdd<f32, K>> Lerp for Matrix<K> {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        match t {
            0. => u,
            1. => v,
            p => {
                let mut vec = Vec::with_capacity(u._d.len());

                for i in 0..u._d.len() {
                    vec.push((v._d[i] - u._d[i]).mul_add(p, &u._d[i]))
                }

                Matrix {
                    _d: vec,
                    cols: u.cols,
                    rows: u.rows,
                }
            }
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
            _d: vec,
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
            self._d.as_slice(),
            self.rows,
            self.rows,
            &vec![false; self.cols],
        )
    }

    pub fn inverse(&self) -> Result<Matrix<K>, &'static str> {
        let mut aug_m = Matrix {
            _d: vec![K::default(); self.rows * self.cols * 2],
            cols: self.cols * 2,
            rows: self.rows,
        };

        for row in 0..self.rows {
            for col in 0..self.cols {
                aug_m._d[row * (self.cols * 2) + col] = self[row][col];
            }
            aug_m._d[row * (self.cols * 2) + self.cols + row] = K::one();
        }

        aug_m.row_echelon_mut();

        let mut vec = Vec::with_capacity(self.rows * self.cols);

        for row in aug_m._d.chunks(self.cols * 2) {
            for &v in row.iter().skip(self.cols) {
                vec.push(v);
            }
        }

        Ok(Matrix {
            _d: vec,
            cols: self.cols,
            rows: self.rows,
        })
    }

    pub fn rank(&self) -> usize {
        let mat = self.row_echelon();
        let mut rank = 0;

        let mut cur = 0;
        for row in mat._d.chunks(self.cols) {
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
        let mut rank = 0;
        for row in mat._d.chunks(self.cols) {
            for (j, v) in row.iter().enumerate().skip(cur) {
                if *v != K::default() {
                    vec.copy_from_slice(row);
                    cur = j + 1;
                    rank += 1;
                    break;
                }
            }
        }

        Matrix {
            cols: self.cols,
            rows: rank,
            _d: vec,
        }
    }

    pub fn col_space(&self) -> Matrix<K> {
        let mat = self.row_echelon();
        let mut vec = vec![];

        let mut cur = 0;
        let mut rank = 0;
        for row in mat._d.chunks(self.cols) {
            for (j, v) in row.iter().enumerate().skip(cur) {
                if *v != K::default() {
                    for r in self._d.chunks(self.cols) {
                        vec.push(r[cur])
                    }
                    cur = j + 1;
                    rank += 1;
                    break;
                }
            }
        }

        Matrix {
            cols: self.rows,
            rows: rank,
            _d: vec,
        }
    }
}

pub fn projection(fov: f32, ratio: f32, n: f32, f: f32) -> Matrix<f32> {
    // let Ps the projection of P on the image plane in 3D space.
    // using the similar triangle rule:
    //  (Ps)x = near * Px / -Pz
    //  (Ps)y = near * Py / -Pz
    // (-Pz is negative because camera is in opposite side)

    // (Ps)w
    // we start by deriving the last row of the matrix,
    // to help us convert later the homogeneous point [x, y, z, w]
    // to cartesian point [x/w, y/w, z/w]
    // since camera is in the opposite side, (Ps)w = -Pz
    // therefore, m32 -1

    // (Ps)x
    // left <= (Ps)x <= right
    // we derive the value where the range is [-1, 1] in NDC space
    // -1 <= 2 * n * Px / (-Pz * (r - l)) - (r + l) / (r - l) <= 1
    // since (Ps)x will be divided at the end of the process by -Pz
    // when we convert Ps from homogeneous to cartesian coordinates
    // to have an identical formula we must assign these factors
    // m00 (2*n/(r-l) and m02 ((r+l)/(r-l)) to the 1st and 3rd of (Ps)x

    // (Ps)y
    // similarly, we drive the value (Ps)y to be in [-1, 1] in NDC space
    // we subtitute m11 (2 * n / (top - bottom)) and m12 ((top + right) / (top - bottom))

    // (Ps)z
    // since x and y of P don't depend on getting the z coordinate of Ps
    // we are left with (Ps)z = (A * Pz + B * Pw) / -Pz (Pw is always 1)
    // we derive A and B when (Ps)z is in range [0, 1]
    // and subtitute a system of two eq -n*A+B=0 and -f*A+B=f
    // we have A = m22 = -f/(f-n) and B = m23 = -(f*n)/(f-n)

    // using trigonometry, we can express:
    // tan(fov / 2) = opposite / adjacent = BC / AB = top / near
    // top = tan(fov / 2) * near
    // right = top * aspect_ratio
    let tan = ((fov / 2.) * (PI / 180.)).tan();
    let t = n * tan;
    let r = t * ratio;
    let (l, b) = (-r, -t);

    M!([
        [2. * n / (r - l), 0., (r + l) / (r - l), 0.], // (Ps)x
        [0., 2. * n / (t - b), (t + b) / (t - b), 0.], // (Ps)y
        [0., 0., f / (n - f), (n * f) / (n - f)],      // (Ps)z
        [0., 0., -1., 0.],                             // (Ps)w
    ])
}
