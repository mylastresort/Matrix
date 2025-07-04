use matrix::{angle_cos, Complex, Vector, C, V};

macro_rules! approx_eq {
    ($a: expr, $b: expr) => {
        ($a - $b).abs() < 1e-6
    };
}

#[test]
fn test_angle_cos() {
    let u = V!([1., 0.]);
    let v = V!([1., 0.]);
    assert_eq!(angle_cos(&u, &v), 1.);

    let u = V!([1., 0.]);
    let v = V!([0., 1.]);
    assert_eq!(angle_cos(&u, &v), 0.); // 0.0

    let u = V!([-1., 1.]);
    let v = V!([1., -1.]);
    assert!(approx_eq!(angle_cos(&u, &v) as f64, -1.0_f64)); // -1.0

    let u = V!([2., 1.]);
    let v = V!([4., 2.]);
    assert!(approx_eq!(angle_cos(&u, &v) as f64, 1.0f64)); // 1.0

    let u = V!([1., 2., 3.]);
    let v = V!([4., 5., 6.]);
    assert!(approx_eq!(angle_cos(&u, &v) as f64, 0.974631846_f64));
}

#[test]
fn test_angle_cos_complex_vectors() {
    let u = V!([C!(1., 0.), C!(0., 1.)]);
    let v = V!([C!(1., 0.), C!(0., 1.)]);
    assert!(approx_eq!(angle_cos(&u, &v) as f32, 1.0_f32));
}
