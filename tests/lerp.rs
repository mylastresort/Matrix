use matrix::{lerp, Complex, Matrix, Vector, C, M, V};

macro_rules! approx_eq {
    ($a: expr, $b: expr) => {
        ($a - $b).abs() < 1e-6
    };
}

#[test]
fn test_lerp() {
    assert_eq!(lerp(0., 1., 0.), 0.);
    assert_eq!(lerp(0., 1., 1.), 1.);
    assert_eq!(lerp(0., 1., 0.5), 0.5);
    // assert_eq!(lerp(21., 42., 0.3), 27.3); // 27.3
    assert!(approx_eq!(lerp(21., 42., 0.3), 27.3_f32));
}

#[test]
fn test_lerp_vector() {
    let a = lerp(V!([2., 1.]), V!([4., 2.]), 0.3);
    assert!(a._d.iter().eq(&[2.6, 1.3]));
    let b = lerp(V!([2., 1.]), V!([4., 2.]), 0.5);
    assert!(b._d.iter().eq(&[3., 1.5]));
    let c = lerp(V!([2., 1.]), V!([4., 2.]), 0.7);
    assert!(c._d.iter().eq(&[3.4, 1.7]));
}

#[test]
fn test_lerp_matrix() {
    let b = lerp(M!([[2., 1.], [3., 4.]]), M!([[20., 10.], [30., 40.]]), 0.5);
    assert!(b._d.iter().eq(&[11., 5.5, 16.5, 22.]));
    let c = lerp(M!([[2., 1.], [3., 4.]]), M!([[20., 10.], [30., 40.]]), 0.3);
    assert!(c._d.iter().eq(&[7.4, 3.7, 11.1, 14.8]));
    let d = lerp(M!([[2., 1.], [3., 4.]]), M!([[20., 10.], [30., 40.]]), 0.);
    assert!(d._d.iter().eq(&[2., 1., 3., 4.]));
    let e = lerp(M!([[2., 1.], [3., 4.]]), M!([[20., 10.], [30., 40.]]), 1.);
    assert!(e._d.iter().eq(&[20., 10., 30., 40.]));
}

#[test]
fn test_lerp_complex_numbers() {
    let a = lerp(C!(2., 1.), C!(4., 2.), 0.3);
    assert_eq!(a, C!(2.6, 1.3)); // Complex number lerp
}

#[test]
fn test_lerp_complex_vector() {
    let a = lerp(
        V!([C!(2., 1.), C!(3., 4.)]),
        V!([C!(4., 2.), C!(5., 6.)]),
        0.3,
    );
    assert!(a._d.iter().eq(&[C!(2.6, 1.3), C!(3.6, 4.6)]));

    let b = lerp(
        V!([C!(2., 1.), C!(3., 4.)]),
        V!([C!(4., 2.), C!(5., 6.)]),
        0.5,
    );
    assert!(b._d.iter().eq(&[C!(3., 1.5), C!(4., 5.)]));
}

#[test]
fn test_lerp_complex_matrix() {
    let a = lerp(
        M!([[C!(2., 1.), C!(3., 4.)], [C!(5., 6.), C!(7., 8.)]]),
        M!([[C!(4., 2.), C!(5., 6.)], [C!(8., 9.), C!(10., 11.)]]),
        0.3,
    );
    assert!(a._d.iter().eq(&[
        C!(2.6, 1.3),
        C!(3.6, 4.6),
        C!(5.9, 6.9),
        C!(7.9, 8.9)
    ]));

    let b = lerp(
        M!([[C!(2., 1.), C!(3., 4.)], [C!(5., 6.), C!(7., 8.)]]),
        M!([[C!(4., 2.), C!(5., 6.)], [C!(8., 9.), C!(10., 11.)]]),
        0.5,
    );
    assert!(b._d.iter().eq(&[
        C!(3., 1.5),
        C!(4., 5.),
        C!(6.5, 7.5),
        C!(8.5, 9.5)
    ]));
}
