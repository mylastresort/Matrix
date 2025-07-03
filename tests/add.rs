use matrix::{complex::Complex, Matrix, Vector, C, M, V};

#[test]
fn test_vector_add() {
    let mut u = V!([2., 3.]);
    let v = V!([5., 7.]);
    u.add(&v);
    assert!(u._d.iter().eq(&[7., 10.]));
}

#[test]
fn test_matrix_add() {
    let mut u = M!([[1., 2.], [3., 4.]]);
    let v = M!([[7., 4.], [-2., 2.]]);
    u.add(&v);
    assert!(u.data.iter().eq(&[8., 6., 1., 6.]));
}

#[test]
fn test_vector_add_2() {
    for i in 0..20 {
        let size = i;
        let mut v = vec![];
        for i in 0..size {
            v.push(i as f32);
        }
        let mut a = V!(v);
        let b = a.clone();
        a.add(&b);

        for i in 0..size {
            assert_eq!(a[i] / 2., b[i], "a[i] and b[i] must equal")
        }
    }
}

#[test]
fn test_add_complex_vector() {
    let mut u = V!([C!(1., 2.), C!(-1., -3.)]);
    let v = V!([C!(1., 3.), C!(2., 3.)]);
    u.add(&v);
    assert!(u._d.iter().eq(&[C!(2., 5.), C!(1., 0.)]));
}

#[test]
fn test_add_complex_matrix() {
    let mut u = M!([[C!(1., 0.), C!(2., 0.)], [C!(3., 0.), C!(5., 0.)],]);
    let v = M!([[C!(7., 0.), C!(3., 0.)], [C!(-2., 0.), C!(2., 0.)],]);
    u.add(&v);
    assert!(u.data.iter().eq(&[
        C!(8., 0.),
        C!(5., 0.),
        C!(1., 0.),
        C!(7., 0.)
    ]));
}

#[test]
fn test_add_complex() {
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
