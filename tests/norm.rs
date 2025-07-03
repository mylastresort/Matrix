use matrix::{Complex, Vector, C, V};

macro_rules! approx_eq {
    ($a: expr, $b: expr) => {
        ($a - $b).abs() < 1e-6
    };
}

#[test]
fn test_norm() {
    let u = V!([0., 0., 0.]);
    assert_eq!(u.norm_1(), 0.);
    assert_eq!(u.norm(), 0.);
    assert_eq!(u.norm_inf(), 0.);

    let u = V!([1., 2., 3.]);
    assert_eq!(u.norm_1(), 6.);
    assert!(approx_eq!(u.norm() as f64, 3.74165738_f64));
    assert_eq!(u.norm_inf(), 3.);

    let u = V!([-1., -2.]);
    assert_eq!(u.norm_1(), 3.);
    assert!(approx_eq!(u.norm() as f64, 2.236067977_f64));
    assert_eq!(u.norm_inf(), 2.);

    let u = V!([1., 2., 3., 4.]);
    assert_eq!(u.norm_1(), 10.);
    assert!(approx_eq!(u.norm() as f64, 5.477225575_f64));
    assert_eq!(u.norm_inf(), 4.);

    let u = V!([1., 2., 3., 4., 5.]);
    assert_eq!(u.norm_1(), 15.);
    assert!(approx_eq!(u.norm() as f64, 7.416198487_f64));
    assert_eq!(u.norm_inf(), 5.);

    let u = V!([1., 2., 3., 4., 5., 6.]);
    assert_eq!(u.norm_1(), 21.);
    assert!(approx_eq!(u.norm() as f64, 9.539392014_f64));
    assert_eq!(u.norm_inf(), 6.);
}

#[test]
fn test_norm_complex_numbers() {
    let u = V!([C!(1., 2.), C!(3., 4.)]);
    assert!(approx_eq!(u.norm_1() as f64, 7.236068));
    assert!(approx_eq!(u.norm() as f64, 5.477225575));
    assert_eq!(u.norm_inf(), 5.);
}
