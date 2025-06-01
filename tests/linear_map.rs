use matrix::{Matrix, Vector};

#[test]
fn test_linear_map() {
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from([4., 2.]);
    let a = u.mul_vec(&v);
    assert_eq!(a[0], 4.);
    assert_eq!(a[1], 2.);

    let u = Matrix::from([[2., 0.], [0., 2.]]);
    let v = Vector::from([4., 2.]);
    let a = u.mul_vec(&v); // [8.] // [4.]
    assert_eq!(a[0], 8.);
    assert_eq!(a[1], 4.);

    let u = Matrix::from([[2., -2.], [-2., 2.]]);
    let v = Vector::from([4., 2.]);
    let a = u.mul_vec(&v); // [4.] // [-4.]
    assert_eq!(a[0], 4.);
    assert_eq!(a[1], -4.);

    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[1., 0.], [0., 1.]]);
    let a = u.mul_mat(&v); // [1., 0.] // [0., 1.]
    assert_eq!(a[0][0], 1.);
    assert_eq!(a[0][1], 0.);
    assert_eq!(a[1][0], 0.);
    assert_eq!(a[1][1], 1.);

    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    let a = u.mul_mat(&v); // [2., 1.] // [4., 2.]
    assert_eq!(a[0][0], 2.);
    assert_eq!(a[0][1], 1.);
    assert_eq!(a[1][0], 4.);
    assert_eq!(a[1][1], 2.);

    let u = Matrix::from([[3., -5.], [6., 8.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    let a = u.mul_mat(&v); // [-14., -7.] // [44., 22.]
    assert_eq!(a[0][0], -14.);
    assert_eq!(a[0][1], -7.);
    assert_eq!(a[1][0], 44.);
    assert_eq!(a[1][1], 22.);
}
