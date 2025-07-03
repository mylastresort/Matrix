use matrix::{linear_combination, Complex, Vector, C, V};

#[test]
fn test_linear_combination() {
    let e1 = V!([1., 0., 0.]);
    let e2 = V!([0., 1., 0.]);
    let e3 = V!([0., 0., 1.]);
    let v1 = V!([1., 2., 3.]);
    let v2 = V!([0., 10., -100.]);

    let a = linear_combination(&[&e1, &e2, &e3], &[10., -2., 0.5]);
    assert!(a._d.iter().eq(&[10., -2., 0.5]));

    let b = linear_combination(&[&v1, &v2], &[10., -2.]);
    assert!(b._d.iter().eq(&[10., 0., 230.]));

    let c = linear_combination(&[&e1, &v1, &v2], &[10., -2., 0.5]);
    assert!(c._d.iter().eq(&[8., 1., -56.,]));
}

#[test]
fn test_linear_combination_empty() {
    let v: Vec<&Vector<f32>> = vec![];
    let coefs: Vec<f32> = vec![];
    let a = linear_combination(&v, &coefs);
    assert!(
        a.is_empty(),
        "linear combination of empty vectors should be empty"
    );
}

#[test]
#[should_panic(expected = "vectors and scalers must be the same size")]
fn test_linear_combination_fail() {
    let e1 = V!([1., 0., 0.]);
    let e2 = V!([0., 1., 0.]);
    let e3 = V!([0., 0., 1.]);
    let v1 = V!([1., 2., 3.]);
    let v2 = V!([0., 10., -100.]);
    let _ = linear_combination(&[&e1, &e2, &e3], &[10., -2.]);
    let _ = linear_combination(&[&v1, &v2], &[10., -2., 0.5]);
}

#[test]
#[should_panic(expected = "vectors must be have the same dimention")]
fn test_linear_combination_fail_different_dimensions() {
    let e1 = V!([1., 0., 0.]);
    let e2 = V!([0., 1., 0.]);
    let e3 = V!([0., 0., 1., 0.]);
    let v1 = V!([1., 2., 3.]);
    let v2 = V!([0., 10., -100., 1.]);
    let _ = linear_combination(&[&e1, &e2, &e3], &[10., -2., 0.5]);
    let _ = linear_combination(&[&v1, &v2], &[10., -2.]);
}

#[test]
fn test_linear_combination_vector_complex_numbers() {
    let e1 = V!([C!(1., 0.), C!(0., 0.)]);
    let e2 = V!([C!(0., 1.), C!(0., 0.)]);
    let e3 = V!([C!(0., 0.), C!(1., 0.)]);
    let v1 = V!([C!(1., 2.), C!(3., 4.)]);
    let v2 = V!([C!(0., 10.), C!(-100., 0.)]);
    let a = linear_combination(
        &[&e1, &e2, &e3],
        &[C!(10., 0.), C!(-2., 0.), C!(0.5, 0.)],
    );
    assert_eq!(
        a._d,
        (e1 * C!(10., 0.) + e2 * C!(-2., 0.) + e3 * C!(0.5, 0.))._d,
        "linear combination of complex vectors failed"
    );
    let b = linear_combination(&[&v1, &v2], &[C!(10., 0.), C!(-2., 0.)]);
    assert_eq!(
        b._d,
        (v1 * C!(10., 0.) + v2 * C!(-2., 0.))._d,
        "linear combination of complex vectors failed"
    );
}
