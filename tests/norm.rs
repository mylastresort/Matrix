use matrix::Vector;

macro_rules! approx_eq {
    ($a: expr, $b: expr) => {
        ($a - $b).abs() < 1e-6
    };
}

#[test]
fn test_norm() {
    let u = Vector::from([0., 0., 0.]);
    assert_eq!(u.norm_1(), 0.);

    assert_eq!(u.norm(), 0.);

    assert_eq!(u.norm_inf(), 0.);

    let u = Vector::from([1., 2., 3.]);

    assert_eq!(u.norm_1(), 6.);

    assert!(approx_eq!(u.norm(), 3.74165738_f64));

    assert_eq!(u.norm_inf(), 3.);

    let u = Vector::from([-1., -2.]);

    assert_eq!(u.norm_1(), 3.);

    assert!(approx_eq!(u.norm(), 2.236067977_f64));

    assert_eq!(u.norm_inf(), 2.);
}
