use matrix::{Complex, Vector, C, V};

#[test]
fn test_dot() {
    let u = V!([0., 0.]);
    let v = V!([1., 1.]);
    assert_eq!(u.dot(&v), 0.);

    let u = V!([1., 1.]);
    let v = V!([1., 1.]);
    assert_eq!(u.dot(&v), 2.);

    let u = V!([-1., 6.]);
    let v = V!([3., 2.]);
    assert_eq!(u.dot(&v), 9.);

    let u = V!([1., 2., 3.]);
    let v = V!([4., 5., 6.]);
    assert_eq!(u.dot(&v), 32.);

    let u = V!([1., 2.]);
    let v = V!([3., 4.]);
    assert_eq!(u.dot(&v), 11.);
}

#[test]
#[should_panic(expected = "vectors must be the same size")]
fn test_dot_panic() {
    let u = V!([1., 2.]);
    let v = V!([1., 2., 3.]);
    u.dot(&v);
}

#[test]
fn test_dot_complex() {
    let u = V!([C!(1., 2.), C!(3., 4.)]);
    let v = V!([C!(5., 6.), C!(7., 8.)]);
    assert_eq!(u.dot(&v), C!(-18., 68.));

    let u = V!([C!(1., 2.)]);
    let v = V!([C!(3., 4.)]);
    assert_eq!(u.dot(&v), C!(-5., 10.));
}
