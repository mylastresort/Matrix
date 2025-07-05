use matrix::{approx_eq, Complex, Matrix, Scalar, C, M};

#[test]
fn test_inverse() {
    let u = M!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    let a = u.inverse().unwrap();
    assert!(a._d.eq(&[1., 0., 0., 0., 1., 0., 0., 0., 1.,]));

    let u = M!([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    let a = u.inverse().unwrap();

    assert!(a._d.eq(&[0.5, 0., 0., 0., 0.5, 0., 0., 0., 0.5]));

    let u = M!([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    let a = u.inverse().unwrap();

    assert!(a
        ._d
        .iter()
        .zip(&[
            0.649425287_f64,
            0.097701149_f64,
            -0.655172414_f64,
            -0.781609195_f64,
            -0.126436782_f64,
            0.965517241_f64,
            0.143678161_f64,
            0.074712644_f64,
            -0.206896552_f64,
        ])
        .all(|(x, y)| approx_eq!(*x, *y)));
}

#[test]
fn test_inverse_complex() {
    let u = M!([[C!(1., -1.), C!(2., 3.)], [C!(4., 5.), C!(6., 7.)]]);
    let a = u.inverse().unwrap();
    assert!(a
        ._d
        .iter()
        .zip(&[
            C!(-0.03210463733650415, 0.3162901307966706),
            C!(0.027348394768133177, -0.12128418549346016),
            C!(0.029726516052318668, -0.21878715814506539),
            C!(0.04875148632580262, 0.0011890606420927466),
        ])
        .all(|(x, y)| approx_eq!(*x, *y)));

    let u = M!([[C!(1., 0.), C!(2., 0.)], [C!(3., 0.), C!(4., 0.)]]);
    let a = u.inverse().unwrap();
    assert!(a
        ._d
        .iter()
        .zip(&[C!(-2., 0.), C!(1., 0.), C!(1.5, 0.), C!(-0.5, -0.),])
        .all(|(x, y)| approx_eq!(*x, *y)));

    let u = M!([[C!(1., 1.), C!(2., 2.)], [C!(3., 3.), C!(4., 4.)]]);
    let a = u.inverse().unwrap();
    assert!(a
        ._d
        .iter()
        .zip(&[C!(-1., 1.), C!(0.5, -0.5), C!(0.75, -0.75), C!(-0.25, 0.25),])
        .all(|(x, y)| approx_eq!(*x, *y)));

    let u = M!([[C!(1., 2.), C!(3., 4.)], [C!(5., 6.), C!(7., 8.)]]);
    let a = u.inverse().unwrap();
    assert!(a
        ._d
        .iter()
        .zip(&[
            C!(-0.5, 0.4375),
            C!(0.25, -0.1875),
            C!(0.375, -0.3125),
            C!(-0.125, 0.0625),
        ])
        .all(|(x, y)| approx_eq!(*x, *y)));
}

#[test]
#[should_panic(expected = "matrix must be squared")]
fn test_inverse_non_square() {
    let u = M!([[1., 2.], [3., 4.], [5., 6.]]);
    let _ = u.inverse();
}

#[test]
#[should_panic(expected = "matrix must be singular")]
fn test_inverse_singular() {
    let u = M!([[1., -1.], [-1., 1.]]);
    let _ = u.inverse();
}
