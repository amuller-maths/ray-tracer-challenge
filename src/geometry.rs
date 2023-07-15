use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::transform::Transform;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector(pub f64, pub f64, pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(pub f64, pub f64, pub f64);

pub trait Tuple {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
    fn new(x: f64, y: f64, z: f64) -> Self;
}

impl Tuple for Vector {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
    fn z(&self) -> f64 {
        self.2
    }
    fn w(&self) -> f64 {
        0.
    }
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
}

impl Tuple for Point {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
    fn z(&self) -> f64 {
        self.2
    }
    fn w(&self) -> f64 {
        1.
    }
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
}
// impl From<Vector> for Tuple {
//     fn from(value: Vector) -> Self {
//         Tuple(value.0, value.1, value.2, 0.)
//     }
// }
//
// impl From<Point> for Tuple {
//     fn from(value: Point) -> Self {
//         Tuple(value.0, value.1, value.2, 1.)
//     }
// }
//
// impl TryFrom<Tuple> for Vector {
//     type Error = String;
//
//     fn try_from(value: Tuple) -> Result<Self, Self::Error> {
//         if value.3 != 0. {
//             Err(format!(
//                 "w component of tuple should be 0 instead of {}",
//                 value.3
//             ))
//         } else {
//             Ok(Vector(value.0, value.1, value.2))
//         }
//     }
// }
//
// impl TryFrom<Tuple> for Point {
//     type Error = String;
//
//     fn try_from(value: Tuple) -> Result<Self, Self::Error> {
//         if value.3 != 1. {
//             Err(format!(
//                 "w component of tuple should be 1 instead of {}",
//                 value.3
//             ))
//         } else {
//             Ok(Point(value.0, value.1, value.2))
//         }
//     }
// }
//
// impl Add for Tuple {
//     type Output = Self;
//
//     fn add(self, other: Self) -> Self {
//         Self(
//             self.0 + other.0,
//             self.1 + other.1,
//             self.2 + other.2,
//             self.3 + other.3,
//         )
//     }
// }

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

// impl Sub for Tuple {
//     type Output = Self;
//
//     fn sub(self, other: Self) -> Self {
//         Self(
//             self.0 - other.0,
//             self.1 - other.1,
//             self.2 - other.2,
//             self.3 - other.3,
//         )
//     }
// }

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, other: Vector) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

// impl Neg for Tuple {
//     type Output = Self;
//
//     fn neg(self) -> Self::Output {
//         Tuple(-self.0, -self.1, -self.2, -self.3)
//     }
// }
//

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let a = 1. / rhs;
        self * a
    }
}
// impl Mul<f64> for Tuple {
//     type Output = Self;
//
//     fn mul(self, rhs: f64) -> Self::Output {
//         Tuple(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
//     }
// }
//
// impl Div<f64> for Tuple {
//     type Output = Self;
//
//     fn div(self, rhs: f64) -> Self::Output {
//         let a = 1. / rhs;
//         self * a
//     }
// }

// impl Tuple {
//     fn magnitude(self) -> f64 {
//         (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
//     }
//
//     fn normalize(self) -> Self {
//         let m = self.magnitude();
//         self / m
//     }
//
//     fn dot(self, other: Self) -> f64 {
//         self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3
//     }
// }

impl Vector {
    fn magnitude(self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let n = 1. / self.magnitude();
        self * n
    }

    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl Transform for Vector {}
impl Transform for Point {}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn tuple_to_point() {
    //     let a = Tuple(4.3, -4.2, 3.2, 1.0);
    //     let p = Point(4.3, -4.2, 3.2);
    //     match a.try_into() {
    //         Ok(Point(x, y, z)) => assert_eq!(p, Point(x, y, z)),
    //         Err(msg) => panic!("{}", msg),
    //     }
    // }
    // #[test]
    // fn tuple_to_vector() {
    //     let a = Tuple(4.3, -4.2, 3.2, 0.0);
    //     let v = Vector(4.3, -4.2, 3.2);
    //     match a.try_into() {
    //         Ok(Vector(x, y, z)) => assert_eq!(v, Vector(x, y, z)),
    //         Err(msg) => panic!("{}", msg),
    //     }
    // }
    // #[test]
    // fn point_to_tuple() {
    //     let a = Tuple(4.3, -4.2, 3.2, 1.0);
    //     let p = Point(4.3, -4.2, 3.2);
    //     assert_eq!(Tuple::from(p), a);
    // }
    // #[test]
    // fn vector_to_tuple() {
    //     let a = Tuple(4.3, -4.2, 3.2, 0.0);
    //     let v = Vector(4.3, -4.2, 3.2);
    //     assert_eq!(Tuple::from(v), a);
    // }
    #[test]
    fn add_vectors() {
        let a = Vector(3., -2., 5.);
        let b = Vector(-2., 3., 1.);
        assert_eq!(a + b, Vector(1., 1., 6.));
    }

    #[test]
    fn sub_points() {
        let a = Point(3., 2., 1.);
        let b = Point(5., 6., 7.);

        assert_eq!(a - b, Vector(-2., -4., -6.))
    }

    #[test]
    fn sub_point_vector() {
        let a = Point(3., 2., 1.);
        let b = Vector(5., 6., 7.);

        assert_eq!(a - b, Point(-2., -4., -6.));
    }
    #[test]
    fn sub_vectors() {
        let a = Vector(3., 2., 1.);
        let b = Vector(5., 6., 7.);
        assert_eq!(a - b, Vector(-2., -4., -6.));
    }

    #[test]
    fn neg_vector() {
        let a = Vector(1., -2., 3.);
        assert_eq!(-a, Vector(-1., 2., -3.))
    }

    #[test]
    fn mul_vector() {
        let a = Vector(1., -2., 3.);
        assert_eq!(a * 3.5, Vector(3.5, -7., 10.5))
    }

    #[test]
    fn div_vector() {
        let a = Vector(1., -2., 3.);
        assert_eq!(a / 2., Vector(0.5, -1., 1.5))
    }

    #[test]
    fn magnitudes() {
        assert_eq!(Vector(1., 0., 0.).magnitude(), 1.);
        assert_eq!(Vector(0., 1., 0.).magnitude(), 1.);
        assert_eq!(Vector(0., 0., 1.).magnitude(), 1.);
        assert_eq!(Vector(1., 2., 3.).magnitude(), 14f64.sqrt());
        assert_eq!(Vector(-1., -2., -3.).magnitude(), 14f64.sqrt());
    }

    #[test]
    fn normalizes() {
        assert_eq!(Vector(4., 0., 0.).normalize(), Vector(1., 0., 0.));
        assert_eq!(
            Vector(1., 2., 3.).normalize(),
            Vector(1. / 14f64.sqrt(), 2. / 14f64.sqrt(), 3. / 14f64.sqrt())
        );
        assert_eq!(Vector(1., 2., 3.).normalize().magnitude(), 1.);
    }

    #[test]
    fn dot() {
        assert_eq!(Vector(1., 2., 3.).dot(Vector(2., 3., 4.)), 20.)
    }

    #[test]
    fn cross() {
        assert_eq!(
            Vector(1., 2., 3.).cross(Vector(2., 3., 4.)),
            Vector(-1., 2., -1.)
        )
    }
}
