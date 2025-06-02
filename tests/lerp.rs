use matrix::{lerp, Matrix, Vector};

#[test]
fn test_lerp() {
    assert_eq!(lerp(0., 1., 0.), 0.); // 0.0
    assert_eq!(lerp(0., 1., 1.), 1.); // 1.0
    assert_eq!(lerp(0., 1., 0.5), 0.5); // 0.5
    assert_eq!(lerp(21., 42., 0.3), 27.3); // 27.3

    let a = lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3);

    assert_eq!(a[0], 2.6);
    assert_eq!(a[1], 1.3);

    let b = lerp(
        Matrix::from([[2., 1.], [3., 4.]]),
        Matrix::from([[20., 10.], [30., 40.]]),
        0.5,
    );

    assert_eq!(b[0][0], 11.);
    assert_eq!(b[0][1], 5.5);
    assert_eq!(b[1][0], 16.5);
    assert_eq!(b[1][1], 22.);
}
