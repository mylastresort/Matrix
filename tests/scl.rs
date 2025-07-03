use matrix::{complex::Complex, Matrix, Vector, C, M, V};

#[test]
fn test_vector_scl() {
    let mut u = V!([2., 3.]);
    let v = 2.;
    u.scl(v);
    assert!(u.data.iter().eq(&[4., 6.]));
}

#[test]
fn test_matrix_scl() {
    let mut u = M!([[1., 2.], [3., 4.]]);
    let v = 2.;
    u.scl(v);
    assert!(u.data.iter().eq(&[2., 4., 6., 8.]));
}

#[test]
fn test_vector_scl_2() {
    for i in 0..20 {
        let size = i;
        let mut v = vec![];
        for i in 0..size {
            v.push(i as f32);
        }
        let mut a = V!(v);
        let b = a.clone();
        a.scl(2.0);
        for i in 0..size {
            assert_eq!(a[i] / 2., b[i], "a[i] and b[i] must equal");
        }
    }
}

#[test]
fn test_scl_complex_vector() {
    let mut u = V!([C!(1., 2.), C!(-1., -3.)]);
    let v = C!(2., 3.);
    u.scl(v);
    assert!(u.data.iter().eq(&[
        C!(-4., 7.),
        C!(7., -9.)
    ]));

    u = V!([C!(1., 0.), C!(2., 0.)]);
    let v = C!(2., 3.);
    u.scl(v);
    assert!(u.data.iter().eq(&[
        C!(2., 3.),
        C!(4., 6.)
    ]));

    u = V!([C!(1., 0.), C!(2., 0.), C!(3., 0.)]);
    let v = C!(2., 3.);
    u.scl(v);
    assert!(u.data.iter().eq(&[
        C!(2., 3.),
        C!(4., 6.),
        C!(6., 9.)
    ]));
}

#[test]
fn test_scl_complex_matrix() {
    let mut u = M!([[C!(1., 2.), C!(-1., -3.)], [C!(2., 0.), C!(3., 0.)]]);
    let v = C!(2., 3.);
    u.scl(v);
    assert!(u.data.iter().eq(&[
        C!(-4., 7.),
        C!(7., -9.),
        C!(4., 6.),
        C!(6., 9.)
    ]));

    u = M!([[C!(1., 0.), C!(2., 0.)], [C!(3., 0.), C!(4., 0.)]]);
    let v = C!(2., 3.);
    u.scl(v);
    assert!(u.data.iter().eq(&[
        C!(2., 3.),
        C!(4., 6.),
        C!(6., 9.),
        C!(8., 12.)
    ]));

    u = M!([[C!(1., 0.), C!(2., 0.), C!(3., 0.)], [C!(4., 0.), C!(5., 0.), C!(6., 0.)]]);
    let v = C!(9., 4.);
    u.scl(v);
    assert!(u.data.iter().eq(&[
        C!(9., 4.),
        C!(18., 8.),
        C!(27., 12.),
        C!(36., 16.),
        C!(45., 20.),
        C!(54., 24.)
    ]));
}

#[test]
fn test_scl_complex() {
    for i in 0..20 {
        let size = i;
        let mut v = vec![];
        for i in 0..size {
            v.push(C!(i as f32, (i + 1) as f32));
        }
        let mut a = V!(v);
        let b = a.clone();
        a.scl(C!(2., 3.));
        for i in 0..size {
            assert_eq!(b[i], a[i] / C!(2., 3.),
                       "a[i] and b[i] must equal for i = {}", i);
        }
    }
}
