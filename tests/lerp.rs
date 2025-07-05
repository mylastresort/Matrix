use matrix::{approx_eq, lerp, Complex, Matrix, Scalar, Vector, C, M, V};

#[test]
fn test_lerp() {
    assert_eq!(lerp(0., 1., 0.), 0.);
    assert_eq!(lerp(0., 1., 1.), 1.);
    assert_eq!(lerp(0., 1., 0.5), 0.5);
    assert!(approx_eq!(lerp(21., 42., 0.3), 27.3_f32));
}

#[test]
fn test_lerp_vector() {
    let a = lerp(V!([2., 1.]), V!([4., 2.]), 0.3);
    assert!(a
        ._d
        .iter()
        .zip(&[2.6, 1.3])
        .all(|(x, y)| approx_eq!(*x, *y)));
    let b = lerp(V!([2., 1.]), V!([4., 2.]), 0.5);
    assert!(b._d.iter().zip(&[3., 1.5]).all(|(x, y)| approx_eq!(*x, *y)));
    let c = lerp(V!([2., 1.]), V!([4., 2.]), 0.7);
    assert!(c
        ._d
        .iter()
        .zip(&[3.4, 1.7])
        .all(|(x, y)| approx_eq!(*x, *y)));
}

#[test]
fn test_lerp_matrix() {
    let b = lerp(M!([[2., 1.], [3., 4.]]), M!([[20., 10.], [30., 40.]]), 0.5);
    assert!(b
        ._d
        .iter()
        .zip(&[11., 5.5, 16.5, 22.])
        .all(|(x, y)| approx_eq!(*x, *y)));
    let c = lerp(M!([[2., 1.], [3., 4.]]), M!([[20., 10.], [30., 40.]]), 0.3);
    assert!(c
        ._d
        .iter()
        .zip(&[7.4, 3.7, 11.1, 14.8])
        .all(|(x, y)| approx_eq!(*x, *y)));
    let d = lerp(M!([[2., 1.], [3., 4.]]), M!([[20., 10.], [30., 40.]]), 0.);
    assert!(d
        ._d
        .iter()
        .zip(&[2., 1., 3., 4.])
        .all(|(x, y)| approx_eq!(*x, *y)));
    let e = lerp(M!([[2., 1.], [3., 4.]]), M!([[20., 10.], [30., 40.]]), 1.);
    assert!(e
        ._d
        .iter()
        .zip(&[20., 10., 30., 40.])
        .all(|(x, y)| approx_eq!(*x, *y)));
}

#[test]
fn test_lerp_complex_numbers() {
    let a = lerp(C!(2., 1.), C!(4., 2.), 0.3);
    assert!(approx_eq!(a, C!(2.6, 1.3)));
}

#[test]
fn test_lerp_complex_vector() {
    let a = lerp(
        V!([C!(2., 1.), C!(3., 4.)]),
        V!([C!(4., 2.), C!(5., 6.)]),
        0.3,
    );
    assert!(a
        ._d
        .iter()
        .zip(&[C!(2.6, 1.3), C!(3.6, 4.6)])
        .all(|(x, y)| approx_eq!(*x, *y)));

    let b = lerp(
        V!([C!(2., 1.), C!(3., 4.)]),
        V!([C!(4., 2.), C!(5., 6.)]),
        0.5,
    );
    assert!(b
        ._d
        .iter()
        .zip(&[C!(3., 1.5), C!(4., 5.)])
        .all(|(x, y)| approx_eq!(*x, *y)));
}

#[test]
fn test_lerp_complex_matrix() {
    let a = lerp(
        M!([[C!(2., 1.), C!(3., 4.)], [C!(5., 6.), C!(7., 8.)]]),
        M!([[C!(4., 2.), C!(5., 6.)], [C!(8., 9.), C!(10., 11.)]]),
        0.3,
    );

    assert!(a
        ._d
        .iter()
        .zip(&[C!(2.6, 1.3), C!(3.6, 4.6), C!(5.9, 6.9), C!(7.9, 8.9)])
        .all(|(x, y)| approx_eq!(*x, *y)));

    let b = lerp(
        M!([[C!(2., 1.), C!(3., 4.)], [C!(5., 6.), C!(7., 8.)]]),
        M!([[C!(4., 2.), C!(5., 6.)], [C!(8., 9.), C!(10., 11.)]]),
        0.5,
    );

    assert!(b
        ._d
        .iter()
        .zip(&[C!(3., 1.5), C!(4., 5.), C!(6.5, 7.5), C!(8.5, 9.5)])
        .all(|(x, y)| approx_eq!(*x, *y)));
}
