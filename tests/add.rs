use matrix::{complex::Complex, Matrix, Vector};

#[test]
fn test_add() {
    let mut u: Vector<f32> = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    assert_eq!(u[0], 7.);
    assert_eq!(u[1], 10.);

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(&v);
    assert_eq!(u[0][0], 8.);
    assert_eq!(u[0][1], 6.);
    assert_eq!(u[1][0], 1.);
    assert_eq!(u[1][1], 6.);

    for i in 0..20 {
        let size = i;
        let mut v = vec![];
        for i in 0..size {
            v.push(i as f32);
        }
        let mut a: Vector<f32> = Vector::from(v);
        let b = a.clone();
        a.add(&b);

        for i in 0..size {
            assert_eq!(a[i] / 2., b[i], "a[i] and b[i] must equal")
        }
    }
}

#[test]
fn test_add_complex() {
    let mut u =
        Vector::from([Complex::from([1., 2.]), Complex::from([-1., -3.])]);
    let v = Vector::from([Complex::from([1., 3.]), Complex::from([2., 3.])]);
    u.add(&v);
    assert_eq!(u[0], Complex::from([2., 5.]));
    assert_eq!(u[1], Complex::from([1., 0.]));

    let mut u = Matrix::from([
        [Complex::from([1., 0.]), Complex::from([2., 0.])],
        [Complex::from([3., 0.]), Complex::from([5., 0.])],
    ]);
    let v = Matrix::from([
        [Complex::from([7., 0.]), Complex::from([3., 0.])],
        [Complex::from([-2., 0.]), Complex::from([2., 0.])],
    ]);
    u.add(&v);
    // assert_eq!(u[0][0], 8.);
    // assert_eq!(u[0][1], 6.);
    // assert_eq!(u[1][0], 1.);
    // assert_eq!(u[1][1], 6.);

    for i in 0..20 {
        let size = i;
        let mut v = vec![];
        for i in 0..size {
            v.push(i as f32);
        }
        let mut a: Vector<f32> = Vector::from(v);
        let b = a.clone();
        a.add(&b);

        for i in 0..size {
            assert_eq!(a[i] / 2., b[i], "a[i] and b[i] must equal")
        }
    }
}
