use matrix::Matrix;

#[test]
fn test_determinant() {
    let u = Matrix::from([[1., -1.], [-1., 1.]]);
    let a = u.determinant(); // 0.0
    assert_eq!(a, 0.);

    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    let a = u.determinant(); // 8.0
    assert_eq!(a, 8.);

    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    let a = u.determinant(); // -174.0
    assert_eq!(a, -174.);

    let u = Matrix::from([
        [8., 5., -2., 4.],
        [4., 2.5, 20., 4.],
        [8., 5., 1., 4.],
        [28., -4., 17., 1.],
    ]);
    let a = u.determinant(); // 1032
    assert_eq!(a, 1032.);
}
