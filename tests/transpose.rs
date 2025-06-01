use matrix::Matrix;

#[test]
fn test_transpose() {
    let mat = Matrix::from([[1., 2.], [3., 4.]]);
    let a = mat.transpose();
    assert_eq!(a[0][0], 1.);
    assert_eq!(a[0][1], 3.);
    assert_eq!(a[1][0], 2.);
    assert_eq!(a[1][1], 4.);

    let mat = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    let a = mat.transpose();
    assert_eq!(a[0][0], 1.);
    assert_eq!(a[0][1], 4.);
    assert_eq!(a[1][0], 2.);
    assert_eq!(a[1][1], 5.);
    assert_eq!(a[2][0], 3.);
    assert_eq!(a[2][1], 6.);
}
