use matrix::{Complex, Matrix, Transpose, C, M};

#[test]
fn test_transpose() {
    let mat = M!([[1., 2.], [3., 4.]]);
    let a = mat.transpose();
    assert!(a._d.eq(&[1., 3., 2., 4.]));

    let mat = M!([[1., 2., 3.], [4., 5., 6.]]);
    let a = mat.transpose();
    assert!(a._d.eq(&[1., 4., 2., 5., 3., 6.]));

    let mat = M!([[1., 2.], [3., 4.], [5., 6.]]);
    let a = mat.transpose();
    assert!(a._d.eq(&[1., 3., 5., 2., 4., 6.]));

    let mat = M!([[1.], [2.], [3.]]);
    let a = mat.transpose();
    assert!(a._d.eq(&[1., 2., 3.]));

    let mat = M!([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
    let a = mat.transpose();
    assert!(a._d.eq(&[1., 4., 7., 2., 5., 8., 3., 6., 9.]));

    let mat = M!([[1.], [2.], [3.], [4.]]);
    let a = mat.transpose();
    assert!(a._d.eq(&[1., 2., 3., 4.]));

    let mat = M!([[1., 2.], [3., 4.], [5., 6.], [7., 8.]]);
    let a = mat.transpose();
    assert!(a._d.eq(&[1., 3., 5., 7., 2., 4., 6., 8.]));

    let mat = M!([[1., 2.], [3., 4.], [5., 6.], [7., 8.], [9., 10.]]);
    let a = mat.transpose();
    assert!(a._d.eq(&[1., 3., 5., 7., 9., 2., 4., 6., 8., 10.]));
}

#[test]
fn test_transpose_complex() {
    let mat = M!([[C!(1., 2.), C!(3., 4.)], [C!(5., 6.), C!(7., 8.)]]);
    let a = mat.transpose();
    assert!(a
        ._d
        .eq(&[C!(1., -2.), C!(5., -6.), C!(3., -4.), C!(7., -8.)]));

    let mat = M!([
        [C!(1., 2.), C!(3., 4.), C!(5., 6.)],
        [C!(7., 8.), C!(9., 10.), C!(11., 12.)]
    ]);
    let a = mat.transpose();
    assert!(a._d.eq(&[
        C!(1., -2.),
        C!(7., -8.),
        C!(3., -4.),
        C!(9., -10.),
        C!(5., -6.),
        C!(11., -12.)
    ]));

    let mat = M!([[C!(1., 2.)], [C!(3., 4.)], [C!(5., 6.)]]);
    let a = mat.transpose();
    assert!(a._d.eq(&[C!(1., -2.), C!(3., -4.), C!(5., -6.)]));
}
