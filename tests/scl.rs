use matrix::{Matrix, Vector};

#[test]
fn test_scl_main() {
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    assert_eq!(u[0], 4.);
    assert_eq!(u[1], 6.);

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    assert_eq!(u[0][0], 2.);
    assert_eq!(u[0][1], 4.);
    assert_eq!(u[1][0], 6.);
    assert_eq!(u[1][1], 8.);
}
