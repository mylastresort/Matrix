use matrix::{cross_product, Vector};

#[test]
fn test_cross_product() {
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    let a = cross_product(&u, &v);
    assert_eq!(a[0], 0.);
    assert_eq!(a[1], 1.);
    assert_eq!(a[2], 0.);

    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    let b = cross_product(&u, &v);
    assert_eq!(b[0], -3.);
    assert_eq!(b[1], 6.);
    assert_eq!(b[2], -3.);

    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    let c = cross_product(&u, &v);
    assert_eq!(c[0], 17.);
    assert_eq!(c[1], -58.);
    assert_eq!(c[2], -16.);
}
