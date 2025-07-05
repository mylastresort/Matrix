use matrix::{approx_eq, Complex, Matrix, Scalar, C, M};

#[test]
fn test_row_echelon() {
    let u = M!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    let a = u.row_echelon();

    assert!(a._d.eq(&[1., 0., 0., 0., 1., 0., 0., 0., 1.]));

    let u = M!([[1., 2.], [3., 4.]]);
    let a = u.row_echelon();
    assert!(a._d.eq(&[1., 0., 0., 1.]));

    let u = M!([[1., 2.], [2., 4.]]);
    let a = u.row_echelon();

    assert!(a._d.eq(&[1., 2., 0., 0.]));

    let u = M!([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    let a = u.row_echelon();

    assert!(a
        ._d
        .iter()
        .zip([
            1.0_f64,
            0.625_f64,
            0.0_f64,
            0.0_f64,
            -12.1666667_f64,
            0.0_f64,
            0.0_f64,
            1.0_f64,
            0.0_f64,
            -3.6666667_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            1.0_f64,
            29.5_f64,
        ])
        .all(|(x, y)| approx_eq!(*x, y)));
}

#[test]
fn test_row_echelon_complex() {
    let u = M!([[C!(1., -1.), C!(2., 3.)], [C!(4., 5.), C!(6., 7.)]]);
    let a = u.row_echelon();
    assert!(a
        ._d
        .iter()
        .zip(&[C!(1., 0.), C!(0., 0.), C!(0., 0.), C!(1., 0.)])
        .all(|(x, y)| approx_eq!(*x, *y)));

    let u = M!([[C!(1., 2.), C!(3., -12.)], [C!(2., 4.), C!(6., -24.)]]);
    let a = u.row_echelon();
    assert!(a
        ._d
        .iter()
        .zip(&[C!(1., 0.), C!(-4.2, -3.6), C!(0., 0.), C!(0., 0.)])
        .all(|(x, y)| approx_eq!(*x, *y)));

    let u = M!([
        [C!(1., 2.), C!(3., -12.), C!(5., 6.)],
        [C!(2., 4.), C!(6., -24.), C!(8., 9.)],
        [C!(3., 6.), C!(9., -36.), C!(11., 12.)],
    ]);
    let a = u.row_echelon();
    assert!(a
        ._d
        .iter()
        .zip(&[
            C!(1., 0.),
            C!(-4.2, -3.6),
            C!(0., 0.),
            C!(0., 0.),
            C!(0., 0.),
            C!(1., 0.),
            C!(0., 0.),
            C!(0., 0.),
            C!(0., 0.)
        ])
        .all(|(x, y)| approx_eq!(*x, *y)));
}
