use std::ops::Div;

use crate::{rule, Dot, Matrix, Scalar, Transpose, Vector};

// computes xᵀ P x in O(n²): we compute Px first then xᵀ(Px)
pub fn quadratic_form<K: Scalar>(_x: &Vector<K>, _p: &Matrix<K>) -> K
where
    [K]: Dot<K>,
    Vector<K>: Dot<K>,
{
    _x.dot(&(_p * _x))
}

impl<K: Scalar> Matrix<K> {
    // `symmetric (P = Pᵀ)`
    // must be squared first
    fn is_symmetric(&self) -> bool
    where
        Matrix<K>: Transpose<K> + PartialEq<Matrix<K>>,
    {
        rule!(self.is_square());
        self == &self.transpose()
    }

    // SPD:
    // 1. `symmetric (P = Pᵀ)`
    // 2. `xᵀPx > 0 for all nonzero x ∈ ℝⁿ (positive definite)`
    // checks symmetry and that all eighenvalues exceed `tol` > 0
    pub fn is_spd(&self, _tol: K) -> bool
    where
        Matrix<K>: Transpose<K> + PartialEq<Matrix<K>>,
        [K]: Dot<K>,
        Vector<K>: Dot<K>,
    {
        rule!(self.is_symmetric());
        self.eigenvalues_power(self.rows, 1000, _tol)
            .iter()
            .all(|(lambda, _)| *lambda > _tol)
    }

    // force exact symmetry after floating-point drift
    // `(P + Pᵀ) / 2`
    pub fn symmetrize(self) -> Option<Matrix<K>>
    where
        Matrix<K>: Transpose<K> + Div<K, Output = Matrix<K>>,
    {
        rule!(self.is_square(), None);
        Some((self.transpose() + &self) / K::from_float(2.0))
    }

    // adds a small diagonal term to make a near singular SPD matrix safely invertible
    // `P + εI`
    pub fn spd_regularize(&self, eps: K) -> Option<Matrix<K>>
    where
        Matrix<K>: Transpose<K> + PartialEq<Matrix<K>>,
        [K]: Dot<K>,
        Vector<K>: Dot<K>,
    {
        // check if not SPD
        rule!(!self.is_spd(eps), Some(self.clone()));

        let mut data = Vec::with_capacity(self._d.len());

        for i in 0..self.rows {
            for j in 0..self.cols {
                data.push(if i == j { self[i][j] + eps } else { self[i][j] });
            }
        }

        Some(Matrix {
            _d: data,
            rows: self.rows,
            cols: self.cols,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::M;

    #[test]
    fn is_spd_todo_panics() {
        let m = M!([[1.0_f32, 0.0], [0.0, 1.0]]);
        let result = std::panic::catch_unwind(|| {
            let _ = m.is_spd(1e-6);
        });
        assert!(result.is_err());
    }

    #[test]
    fn symmetrize_returns_expected_values() {
        let m = M!([[1.0_f32, 2.0], [3.0, 4.0]]);
        let s = m.symmetrize().unwrap();

        assert_eq!(s[0], [1.0, 2.5]);
        assert_eq!(s[1], [2.5, 4.0]);
    }

    #[test]
    fn spd_regularize_adds_diagonal_shift() {
        let m = M!([[1.0_f32, 2.0], [3.0, 4.0]]);
        let regularized = m.spd_regularize(0.5).unwrap();

        println!("Regularized matrix: {:?}", regularized);

        assert_eq!(regularized[0], [1.5, 2.0]);
        assert_eq!(regularized[1], [3.0, 4.5]);
    }
}
