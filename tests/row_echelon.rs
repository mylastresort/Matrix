use matrix::Matrix;

macro_rules! approx_eq {
    ($a: expr, $b: expr) => {
        ($a - $b).abs() < 1e-6
    };
}

#[test]
fn test_row_echelon() {
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    let a = u.row_echelon();

    assert_eq!(a[0][0], 1.);
    assert_eq!(a[0][1], 0.);
    assert_eq!(a[0][2], 0.);
    assert_eq!(a[1][0], 0.);
    assert_eq!(a[1][1], 1.);
    assert_eq!(a[1][2], 0.);
    assert_eq!(a[2][0], 0.);
    assert_eq!(a[2][1], 0.);
    assert_eq!(a[2][2], 1.);

    let u = Matrix::from([[1., 2.], [3., 4.]]);
    let a = u.row_echelon();
    assert_eq!(a[0][0], 1.);
    assert_eq!(a[0][1], 0.);
    assert_eq!(a[1][0], 0.);
    assert_eq!(a[1][1], 1.);

    let u = Matrix::from([[1., 2.], [2., 4.]]);
    let a = u.row_echelon();

    assert_eq!(a[0][0], 1.);
    assert_eq!(a[0][1], 2.);
    assert_eq!(a[1][0], 0.);
    assert_eq!(a[1][1], 0.);

    let u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    let a = u.row_echelon();
    assert!(approx_eq!(a[0][0], 1.0_f64));
    assert!(approx_eq!(a[0][1], 0.625_f64));
    assert!(approx_eq!(a[0][2], 0.0_f64));
    assert!(approx_eq!(a[0][3], 0.0_f64));
    assert!(approx_eq!(a[0][4], -12.1666667_f64));
    assert!(approx_eq!(a[1][0], 0.0_f64));
    assert!(approx_eq!(a[1][1], 0.0_f64));
    assert!(approx_eq!(a[1][2], 1.0_f64));
    assert!(approx_eq!(a[1][3], 0.0_f64));
    assert!(approx_eq!(a[1][4], -3.6666667_f64));
    assert!(approx_eq!(a[2][0], 0.0_f64));
    assert!(approx_eq!(a[2][1], 0.0_f64));
    assert!(approx_eq!(a[2][2], 0.0_f64));
    assert!(approx_eq!(a[2][3], 1.0_f64));
    assert!(approx_eq!(a[2][4], 29.5_f64));
}
