use matrix::{Complex, Matrix, C, M};

#[test]
fn test_trace() {
    let u = M!([[1., 0.], [0., 1.]]);
    let a = u.trace();
    assert_eq!(a.unwrap(), 2.);

    let u = M!([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    let a = u.trace();
    assert_eq!(a.unwrap(), 9.);

    let u = M!([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    let a = u.trace();
    assert_eq!(a.unwrap(), -21.);
}

#[test]
#[should_panic(expected = "matrix must be squared")]
fn test_trace_fail() {
    let u = M!([[1., 0.], [0., 1.], [1., 0.]]);
    let _a = u.trace(); // This should panic
}

#[test]
fn test_trace_complex() {
    let u = M!([[C!(1., 0.), C!(0., 1.)], [C!(0., 0.), C!(1., 0.)]]);
    let a = u.trace();
    assert_eq!(a.unwrap(), C!(2., 0.));

    let u = M!([[C!(2., -5.), C!(0., 0.)], [C!(4., 3.), C!(7., 0.)],]);
    let a = u.trace();
    assert_eq!(a.unwrap(), C!(9., -5.));

    let u = M!([
        [C!(-2., 0.), C!(-8., 0.), C!(4., 0.)],
        [C!(1., 0.), C!(-23., 0.), C!(4., 0.)],
        [C!(0., 0.), C!(6., 0.), C!(4., 0.)]
    ]);
    let a = u.trace();
    assert_eq!(a.unwrap(), C!(-21., 0.));
}
