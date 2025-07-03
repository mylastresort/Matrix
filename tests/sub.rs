use matrix::{complex::Complex, Matrix, Vector, C, M, V};

#[test]
fn test_vector_sub() {
    let mut u = V!([2., 3.]);
    let v = V!([5., 7.]);
    u.sub(&v);
    assert!(u._d.iter().eq(&[-3., -4.]));
}

#[test]
fn test_matrix_sub() {
    let mut u = M!([[1., 2.], [3., 4.]]);
    let v = M!([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    assert!(u.data.iter().eq(&[-6., -2., 5., 2.]));
}

#[test]
#[should_panic(expected = "vectors must be the same size")]
fn test_vector_sub_fail() {
    let mut u = V!([2., 3.]);
    let v = V!([5., 7., 8.]);
    u.sub(&v);
}

#[test]
#[should_panic(expected = "matrices must be the same size")]
fn test_matrix_sub_fail() {
    let mut u = M!([[1., 2.], [3., 4.]]);
    let v = M!([[7., 4.], [-2., 2.], [1., 1.]]);
    u.sub(&v);
}

#[test]
fn test_vector_sub_2() {
    for i in 0..20 {
        let size = i;
        let mut v = vec![];
        for i in 0..size {
            v.push(i as f32);
        }
        let mut a = V!(v);
        let b = a.clone();
        a.sub(&b);

        for i in 0..size {
            assert_eq!(a[i], 0.0, "a[i] must be zero after subtraction");
        }
    }
}

#[test]
fn test_sub_complex_vector() {
    let mut u = V!([C!(1., 2.), C!(-1., -3.)]);
    let v = V!([C!(1., 3.), C!(2., 3.)]);
    u.sub(&v);
    assert!(u._d.iter().eq(&[C!(0., -1.), C!(-3., -6.)]));
}

#[test]
fn test_sub_complex_matrix() {
    let mut u = M!([[C!(1., 0.), C!(2., 0.)], [C!(3., 0.), C!(5., 0.)],]);
    let v = M!([[C!(7., 0.), C!(3., 0.)], [C!(-2., 0.), C!(2., 0.)],]);
    u.sub(&v);
    assert!(u.data.iter().eq(&[
        C!(-6., 0.),
        C!(-1., 0.),
        C!(5., 0.),
        C!(3., 0.)
    ]));
}

#[test]
fn test_sub_complex() {
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
            assert_eq!(a[i], 0.0, "a[i] must be zero after subtraction");
        }
    }
}
