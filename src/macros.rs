pub const EPSILON: f64 = 1.0e-4;

pub trait AlmostEq {
    fn almost_eq(self, other: Self, eps: f64) -> bool;
}

impl AlmostEq for f64 {
    fn almost_eq(self, other: Self, eps: f64) -> bool {
        (self - other).abs() < eps
    }
}

#[macro_export]

macro_rules! assert_almost_eq {
    ($a:expr, $b:expr) => {
        let (a, b) = (&$a, &$b);
        assert!(
            (*a).almost_eq(*b, crate::macros::EPSILON),
            "assertion failed: `(left !== right)` \n
        (left: `{:?}`, right: `{:?}`)",
            *a,
            *b
        )
    };
    ($a:expr, $b:expr, $eps:expr) => {
        let eps = &$eps;
        let (a, b) = (&$a, &$b);
        assert!(
            (*a).almost_eq(*b, *eps),
            "assertion failed: `(left !== right)` \n
        (left: `{:?}`, right: `{:?}`)",
            *a,
            *b
        )
    };
}
