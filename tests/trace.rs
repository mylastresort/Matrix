use matrix::Matrix;

#[test]
fn test_trace() {
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let a = u.trace();
    assert_eq!(a.unwrap(), 2.);

    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    let a = u.trace();
    assert_eq!(a.unwrap(), 9.);

    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    let a = u.trace();
    assert_eq!(a.unwrap(), -21.);
}
