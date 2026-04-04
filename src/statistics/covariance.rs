use std::ops::Div;

use crate::{scalar::Sqrt, Matrix, Scalar, Transpose, Vector};

impl<K: Scalar> Matrix<K> {
    /// Computes per-feature sample means for multivariate summarization.
    ///
    /// Each output entry is the mean of one matrix column (feature), used to
    /// center data before covariance/correlation analysis.
    pub fn column_means(&self) -> Vector<K> {
        self.col_iter()
            .map(|col| col.sum() / K::from_usize(self.rows))
            .collect::<Vec<K>>()
            .into()
    }

    /// Returns the mean-centered data matrix `X̃`.
    ///
    /// Every value is transformed as `x_ij - μ_j`, where `μ_j` is the mean of
    /// feature `j`. Mean-centering makes covariance a scaled dot-product of
    /// centered feature vectors.
    pub fn center(&self) -> Matrix<K> {
        let _means = self.column_means();

        let _data = self
            .row_iter()
            .flat_map(|row| {
                row.iter().enumerate().map(|(i, &value)| value - _means[i])
            })
            .collect::<Vec<K>>();

        Matrix {
            _d: _data,
            cols: self.cols,
            rows: self.rows,
        }
    }
}

impl<K: Scalar + Sqrt> Matrix<K>
where
    Matrix<K>: Transpose<K> + Div<K, Output = Matrix<K>>,
{
    /// Computes the sample covariance matrix between all feature pairs.
    ///
    /// Uses the Bessel-corrected form
    /// `C = (1 / (m - 1)) * X̃ᵀX̃`, where `m` is the number of rows
    /// (observations) and `X̃` is mean-centered data.
    pub fn covariance_matrix(&self) -> Matrix<K> {
        assert!(self.rows > 1, "covariance_matrix requires at least 2 rows");

        let c = self.center();
        let cov = &c.transpose() * &c;
        cov / K::from_float((self.rows - 1) as f64)
    }

    /// Computes the Pearson correlation matrix from covariance.
    ///
    /// Each entry is `R_ij = C_ij / (σ_i σ_j)` with
    /// `σ_i = sqrt(C_ii)`. The result is unit-less and normalizes away feature
    /// scale, making association strength comparable across attributes.
    pub fn correlation_matrix(&self) -> Matrix<K> {
        let cov = self.covariance_matrix();
        let stddev =
            (0..cov.rows).map(|i| cov[i][i].sqrt()).collect::<Vec<K>>();
        let stddev_ref = &stddev;

        let data = cov
            .row_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                let sigma_i = stddev_ref[i];
                row.iter().enumerate().map(move |(j, &value)| {
                    let denom = sigma_i * stddev_ref[j];
                    if denom.is_non_zero() {
                        value / denom
                    } else {
                        K::default()
                    }
                })
            })
            .collect::<Vec<K>>();

        Matrix {
            _d: data,
            rows: cov.rows,
            cols: cov.cols,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{approx_eq, M};

    #[test]
    fn column_means_returns_expected_values() {
        let m = M!([[1.0_f32, 2.0], [3.0, 4.0]]);

        let means = m.column_means();

        assert_eq!(means._d, vec![2.0, 3.0]);
    }

    #[test]
    fn center_returns_expected_values() {
        let m = M!([[1.0_f32, 2.0], [3.0, 4.0]]);
        let centered = m.center();

        assert_eq!(centered[0], [-1.0, -1.0]);
        assert_eq!(centered[1], [1.0, 1.0]);
    }

    #[test]
    fn covariance_matrix_returns_expected_values() {
        let m = M!([[1.0_f32, 2.0], [3.0, 4.0]]);
        let cov = m.covariance_matrix();

        assert_eq!(cov[0], [2.0, 2.0]);
        assert_eq!(cov[1], [2.0, 2.0]);
    }

    #[test]
    fn correlation_matrix_returns_expected_values() {
        let m = M!([[1.0_f32, 2.0], [3.0, 4.0]]);
        let corr = m.correlation_matrix();

        assert!(approx_eq!(corr[0][0], 1.0));
        assert!(approx_eq!(corr[0][1], 1.0));
        assert!(approx_eq!(corr[1][0], 1.0));
        assert!(approx_eq!(corr[1][1], 1.0));
    }
}
