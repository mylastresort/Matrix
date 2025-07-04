use matrix::{cross_product, Complex, Vector, C, V};

#[test]
fn test_cross_product() {
    let u = V!([0., 0., 1.]);
    let v = V!([1., 0., 0.]);
    let a = cross_product(&u, &v);
    assert!(a._d.iter().eq(&[0., 1., 0.]));

    let u = V!([1., 2., 3.]);
    let v = V!([4., 5., 6.]);
    let b = cross_product(&u, &v);
    assert!(b._d.iter().eq(&[-3., 6., -3.]));

    let u = V!([4., 2., -3.]);
    let v = V!([-2., -5., 16.]);
    let c = cross_product(&u, &v);
    assert!(c._d.iter().eq(&[17., -58., -16.]));
}

#[test]
#[should_panic(expected = "vectors must have be of size 3")]
fn test_cross_product_invalid_length() {
    let u = V!([1., 2.]);
    let v = V!([3., 4., 5.]);
    cross_product(&u, &v);
    cross_product(&v, &u);
}

#[test]
fn test_cross_product_complex() {
    let u = V!([C!(1., 2.), C!(3., 4.), C!(5., 6.)]);
    let v = V!([C!(7., 8.), C!(9., 10.), C!(11., 12.)]);
    let a = cross_product(&u, &v);
    assert!(a._d.iter().eq(&[C!(0., -24.), C!(0., 48.), C!(0., -24.)]));
}
