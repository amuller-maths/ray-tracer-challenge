use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{
    macros::AlmostEq,
    transform::{Transformable, Transformed},
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector(pub f64, pub f64, pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(pub f64, pub f64, pub f64);

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

impl Vector {
    pub fn magnitude(self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let n = 1. / self.magnitude();
        self * n
    }

    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn reflect(self, n: Self) -> Self {
        self - n * 2. * self.dot(n)
    }
}

impl Transformable for Vector {
    fn transform(self, t: crate::transform::Transform) -> Self {
        t.m * self
    }
}
impl Transformable for Point {
    fn transform(self, t: crate::transform::Transform) -> Self {
        t.m * self
    }
}

impl Transformed for Vector {}
impl Transformed for Point {}

impl AlmostEq for Vector {
    fn almost_eq(self, other: Self, eps: f64) -> bool {
        (self.0 - other.0).abs() < eps
            && (self.1 - other.1).abs() < eps
            && (self.2 - other.2).abs() < eps
    }
}

impl AlmostEq for Point {
    fn almost_eq(self, other: Self, eps: f64) -> bool {
        (self.0 - other.0).abs() < eps
            && (self.1 - other.1).abs() < eps
            && (self.2 - other.2).abs() < eps
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_almost_eq;

    use super::*;
    // fn almost_eq(v1: Vector, v2: Vector) -> bool {
    //     (v1.0 - v2.0).abs() < 1e6 && (v1.1 - v2.1).abs() < 1e6 && (v1.2 - v2.2).abs() < 1e6
    // }

    #[test]
    fn adding_two_vectors() {
        let a = Vector(3., -2., 5.);
        let b = Vector(-2., 3., 1.);
        assert_eq!(a + b, Vector(1., 1., 6.));
    }

    #[test]
    fn substracting_two_points() {
        let a = Point(3., 2., 1.);
        let b = Point(5., 6., 7.);

        assert_eq!(a - b, Vector(-2., -4., -6.))
    }

    #[test]
    fn substracting_a_vector_from_a_point() {
        let a = Point(3., 2., 1.);
        let b = Vector(5., 6., 7.);

        assert_eq!(a - b, Point(-2., -4., -6.));
    }
    #[test]
    fn substracting_two_vectors() {
        let a = Vector(3., 2., 1.);
        let b = Vector(5., 6., 7.);
        assert_eq!(a - b, Vector(-2., -4., -6.));
    }

    #[test]
    fn negating_a_vector() {
        let a = Vector(1., -2., 3.);
        assert_eq!(-a, Vector(-1., 2., -3.))
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let a = Vector(1., -2., 3.);
        assert_eq!(a * 3.5, Vector(3.5, -7., 10.5))
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let a = Vector(1., -2., 3.);
        assert_eq!(a / 2., Vector(0.5, -1., 1.5))
    }

    #[test]
    fn computing_several_magnitudes() {
        assert_eq!(Vector(1., 0., 0.).magnitude(), 1.);
        assert_eq!(Vector(0., 1., 0.).magnitude(), 1.);
        assert_eq!(Vector(0., 0., 1.).magnitude(), 1.);
        assert_eq!(Vector(1., 2., 3.).magnitude(), 14f64.sqrt());
        assert_eq!(Vector(-1., -2., -3.).magnitude(), 14f64.sqrt());
    }

    #[test]
    fn normalizing_several_vectors() {
        assert_eq!(Vector(4., 0., 0.).normalize(), Vector(1., 0., 0.));
        assert_eq!(
            Vector(1., 2., 3.).normalize(),
            Vector(1. / 14f64.sqrt(), 2. / 14f64.sqrt(), 3. / 14f64.sqrt())
        );
        assert_eq!(Vector(1., 2., 3.).normalize().magnitude(), 1.);
    }

    #[test]
    fn the_dot_product_of_two_vectors() {
        assert_eq!(Vector(1., 2., 3.).dot(Vector(2., 3., 4.)), 20.)
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        assert_eq!(
            Vector(1., 2., 3.).cross(Vector(2., 3., 4.)),
            Vector(-1., 2., -1.)
        )
    }
    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = Vector(1., -1., 0.);
        let n = Vector(0., 1., 0.);
        assert_eq!(v.reflect(n), Vector(1., 1., 0.));
    }
    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Vector(0., -1., 0.);
        let n = Vector(2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.);
        assert_almost_eq!(v.reflect(n), Vector(1., 0., 0.));
    }
}
