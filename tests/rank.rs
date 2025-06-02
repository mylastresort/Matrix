use matrix::Matrix;

#[test]
fn test_rank() {
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    let a = u.rank(); // 3
    assert_eq!(a, 3);
    //
    let u =
        Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
    let a = u.rank(); // 2
                      //
    println!("rank {:?}", a);
    assert_eq!(a, 2);

    let u = Matrix::from([
        [8., 5., -2.],
        [4., 7., 20.],
        [7., 6., 1.],
        [21., 18., 7.],
    ]);
    let a = u.rank(); // 3
    assert_eq!(a, 3);
}
