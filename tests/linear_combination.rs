use matrix::{linear_combination, Vector};

#[test]
fn test_linear_combination() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    let a = linear_combination(&[&e1, &e2, &e3], &[10., -2., 0.5]);

    assert_eq!(a[0], 10.);
    assert_eq!(a[1], -2.);
    assert_eq!(a[2], 0.5);

    let b = linear_combination(&[&v1, &v2], &[10., -2.]);

    assert_eq!(b[0], 10.);
    assert_eq!(b[1], 0.);
    assert_eq!(b[2], 230.);
}
