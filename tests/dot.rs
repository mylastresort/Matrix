use matrix::Vector;

#[test]
fn test_dot() {
    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    assert_eq!(u.dot(&v), 0.);

    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    assert_eq!(u.dot(&v), 2.);

    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    assert_eq!(u.dot(&v), 9.);
}
