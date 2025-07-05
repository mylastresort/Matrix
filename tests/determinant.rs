use matrix::{Complex, Matrix, C, M};

#[test]
fn test_determinant() {
    let u = M!([[1., -1.], [-1., 1.]]);
    let a = u.determinant();
    assert_eq!(a, 0.);

    let u = M!([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    let a = u.determinant();
    assert_eq!(a, 8.);

    let u = M!([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    let a = u.determinant();
    assert_eq!(a, -174.);

    let u = M!([
        [8., 5., -2., 4.],
        [4., 2.5, 20., 4.],
        [8., 5., 1., 4.],
        [28., -4., 17., 1.],
    ]);
    let a = u.determinant();
    assert_eq!(a, 1032.);
}

#[test]
#[should_panic(expected = "matrix must be squared")]
fn test_determinant_non_square() {
    let u = M!([[1., 2.], [3., 4.], [5., 6.]]);
    let _ = u.determinant();
}

#[test]
fn test_determinant_complex() {
    let u = M!([[C!(1., -1.), C!(2., 3.)], [C!(4., 5.), C!(6., 7.)]]);
    let a = u.determinant();
    assert_eq!(a, C!(20., -21.));

    let u = M!([[C!(1., 0.), C!(2., 0.)], [C!(3., 0.), C!(4., 0.)]]);
    let a = u.determinant();
    assert_eq!(a, C!(-2., 0.));

    let u = M!([[C!(1., 1.), C!(2., 2.)], [C!(3., 3.), C!(4., 4.)]]);
    let a = u.determinant();
    assert_eq!(a, C!(0., -4.));

    let u = M!([[C!(1., 2.), C!(3., 4.)], [C!(5., 6.), C!(7., 8.)]]);
    let a = u.determinant();
    assert_eq!(a, C!(0., -16.));
}
