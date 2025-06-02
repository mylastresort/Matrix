use matrix::Matrix;

macro_rules! approx_eq {
    ($a: expr, $b: expr) => {
        ($a - $b).abs() < 1e-6
    };
}

#[test]
fn test_inverse() {
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    let a = u.inverse().unwrap();
    assert_eq!(a[0][0], 1.);
    assert_eq!(a[0][1], 0.);
    assert_eq!(a[0][2], 0.);
    assert_eq!(a[1][0], 0.);
    assert_eq!(a[1][1], 1.);
    assert_eq!(a[1][2], 0.);
    assert_eq!(a[2][0], 0.);
    assert_eq!(a[2][1], 0.);
    assert_eq!(a[2][2], 1.);

    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    let a = u.inverse().unwrap();

    assert_eq!(a[0][0], 0.5);
    assert_eq!(a[0][1], 0.);
    assert_eq!(a[0][2], 0.);
    assert_eq!(a[1][0], 0.);
    assert_eq!(a[1][1], 0.5);
    assert_eq!(a[1][2], 0.);
    assert_eq!(a[2][0], 0.);
    assert_eq!(a[2][1], 0.);
    assert_eq!(a[2][2], 0.5);

    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    let a = u.inverse().unwrap();
    assert!(approx_eq!(a[0][0], 0.649425287_f64));
    assert!(approx_eq!(a[0][1], 0.097701149_f64));
    assert!(approx_eq!(a[0][2], -0.655172414_f64));
    assert!(approx_eq!(a[1][0], -0.781609195_f64));
    assert!(approx_eq!(a[1][1], -0.126436782_f64));
    assert!(approx_eq!(a[1][2], 0.965517241_f64));
    assert!(approx_eq!(a[2][0], 0.143678161_f64));
    assert!(approx_eq!(a[2][1], 0.074712644_f64));
    assert!(approx_eq!(a[2][2], -0.206896552_f64));
}
