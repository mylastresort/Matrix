use matrix::{Complex, Matrix, Vector, C, M, V};

#[test]
fn test_linear_map_mul_vec() {
    let u = M!([[1., 0.], [0., 1.]]);
    let v = V!([4., 2.]);
    let a = u.mul_vec(&v);
    assert!(a._d.eq(&[4., 2.]));

    let u = M!([[2., 0.], [0., 2.]]);
    let v = V!([4., 2.]);
    let a = u.mul_vec(&v);
    assert!(a._d.eq(&[8., 4.]));

    let u = M!([[2., -2.], [-2., 2.]]);
    let v = V!([4., 2.]);
    let a = u.mul_vec(&v);
    assert!(a._d.eq(&[4., -4.]));
}

#[test]
fn test_linear_map_mul_mat() {
    let u = M!([[1., 0.], [0., 1.]]);
    let v = M!([[1., 0.], [0., 1.]]);
    let a = u.mul_mat(&v);
    assert!(a._d.eq(&[1., 0., 0., 1.]));

    let u = M!([[1., 0.], [0., 1.]]);
    let v = M!([[2., 1.], [4., 2.]]);
    let a = u.mul_mat(&v);
    assert!(a._d.eq(&[2., 1., 4., 2.]));

    let u = M!([[3., -5.], [6., 8.]]);
    let v = M!([[2., 1.], [4., 2.]]);
    let a = u.mul_mat(&v);
    assert!(a._d.eq(&[-14., -7., 44., 22.]));
}

#[test]
#[should_panic(
    expected = "bad input for matrix and vector column multiplication"
)]
fn test_linear_map_mul_vec_fail() {
    let u = M!([[1., 0.], [0., 1.]]);
    let v = V!([4., 2., 3.]); // Incorrect size
    let _a = u.mul_vec(&v); // This should panic
}

#[test]
#[should_panic(expected = "bad input for matrix and matrix multiplication")]
fn test_linear_map_mul_mat_fail() {
    let u = M!([[1., 0.], [0., 1.]]);
    let v = M!([[1., 0.], [0., 1.], [1., 0.]]); // Incorrect size
    let _a = u.mul_mat(&v); // This should panic
}

#[test]
fn test_linear_map_mul_vec_complex() {
    let u = M!([[C!(1., 0.), C!(0., 1.)], [C!(0., 0.), C!(1., 0.)]]);
    let v = V!([C!(4., 0.), C!(2., 0.)]);
    let a = u.mul_vec(&v);
    assert!(a._d.eq(&[C!(4., 2.), C!(2., 0.)]));

    let u = M!([[C!(2., 0.), C!(0., 0.)], [C!(0., 0.), C!(2., 0.)]]);
    let v = V!([C!(4., 0.), C!(2., 0.)]);
    let a = u.mul_vec(&v);
    assert!(a._d.eq(&[C!(8., 0.), C!(4., 0.)]));

    let u = M!([[C!(2., 0.), C!(-2., 0.)], [C!(-2., 0.), C!(2., 0.)]]);
    let v = V!([C!(4., 0.), C!(2., 0.)]);
    let a = u.mul_vec(&v);
    assert!(a._d.eq(&[C!(4., 0.), C!(-4., 0.)]));
}

#[test]
fn test_linear_map_mul_mat_complex() {
    let u = M!([[C!(1., 0.), C!(0., 1.)], [C!(0., 0.), C!(1., 0.)]]);
    let v = M!([[C!(1., 0.), C!(0., 1.)], [C!(0., 0.), C!(1., 0.)]]);
    let a = u.mul_mat(&v);
    assert!(a._d.eq(&[C!(1., 0.), C!(0., 2.), C!(0., 0.), C!(1., 0.)]));

    let u = M!([[C!(1., 0.), C!(0., 1.)], [C!(0., 0.), C!(1., 0.)]]);
    let v = M!([[C!(2., 0.), C!(1., 0.)], [C!(4., 0.), C!(2., 0.)]]);
    let a = u.mul_mat(&v);
    assert!(a._d.eq(&[C!(2., 4.), C!(1., 2.), C!(4., 0.), C!(2., 0.)]));

    let u = M!([[C!(3., 0.), C!(-5., 0.)], [C!(6., 0.), C!(8., 0.)]]);
    let v = M!([[C!(2., 0.), C!(1., 0.)], [C!(4., 0.), C!(2., 0.)]]);
    let a = u.mul_mat(&v);
    assert!(a._d.eq(&[C!(-14., 0.), C!(-7., 0.), C!(44., 0.), C!(22., 0.)]));

    let u = M!([[C!(1., 0.), C!(0., 1.)], [C!(0., 0.), C!(1., 0.)]]);
    let v = M!([[C!(1., 0.), C!(0., 1.)], [C!(0., 0.), C!(1., 0.)]]);
    let a = u.mul_mat(&v);
    assert!(a._d.eq(&[C!(1., 0.), C!(0., 2.), C!(0., 0.), C!(1., 0.)]));
}
