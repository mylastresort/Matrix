pub const EPSILON: f64 = 1e-10;

#[macro_export]
macro_rules! approx_eq {
    ($a: expr, $b: expr) => {
        ($a - $b).abs() < 1e-6
    };

    ($a: expr, $b: expr, $c: expr) => {
        ($a - $b).abs() < $c
    }
}
