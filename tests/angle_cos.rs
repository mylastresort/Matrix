use matrix::{angle_cos, Vector};

macro_rules! approx_eq {
    ($a: expr, $b: expr) => {
        ($a - $b).abs() < 1e-6
    };
}

#[test]
fn test_angle_cos() {
    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    assert_eq!(angle_cos(&u, &v), 1.);

    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    assert_eq!(angle_cos(&u, &v), 0.); // 0.0

    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);
    assert!(approx_eq!(angle_cos(&u, &v), -1.0_f64)); // -1.0

    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    assert!(approx_eq!(angle_cos(&u, &v), 1.0f64)); // 1.0

    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    assert!(approx_eq!(angle_cos(&u, &v), 0.974631846_f64));
}
