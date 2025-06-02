use matrix::{Matrix, Vector};

#[test]
fn test_sub() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(&v);
    assert_eq!(u[0], -3.);
    assert_eq!(u[1], -4.);

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    assert_eq!(u[0][0], -6.);
    assert_eq!(u[0][1], -2.);
    assert_eq!(u[1][0], 5.);
    assert_eq!(u[1][1], 2.);

    for i in 0..20 {
        let size = i;
        let mut v = vec![];
        for i in 0..size {
            v.push(i as f32);
        }
        let mut a: Vector<f32> = Vector::from(v);
        let b = a.clone();
        a.sub(&b);

        for i in 0..size {
            assert_eq!(a[i] + b[i], b[i], "a[i] and b[i] must equal")
        }
    }
}
