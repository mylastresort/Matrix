use crate::{Matrix, Scalar, Vector};

impl<K: Scalar> Matrix<K> {
    pub fn dominant_eigenpair(
        &self,
        _max_iter: usize,
        _tol: K,
    ) -> Option<(K, Vector<K>)> {
        todo!()
    }

    pub fn eigenvalues_power(
        &self,
        _k: usize,
        _max_iter: usize,
        _tol: K,
    ) -> Vec<(K, Vector<K>)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::M;

    // #[test]
    // fn dominant_eigenpair() {
    //     let a = M!([[4., 1.], [2., 3.]]);

    //     let (lambda, v) = a.dominant_eigenpair(1000, 1e-6).unwrap();

    //     assert!(f64::abs(lambda - 5.) < 1e-6);
    //     assert!(f64::abs(v[0] / v[1] - 2.) < 1e-6);
    // }
}
