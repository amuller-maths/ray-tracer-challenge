use std::ops::Mul;

use crate::geometry::{Point, Vector};
use crate::macros::AlmostEq;
use crate::matrix::Matrix;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Transform {
    pub m: Matrix,
    pub minv: Matrix,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            m: Matrix::id(),
            minv: Matrix::id(),
        }
    }
}

impl Transform {
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self {
            m: Matrix([
                [1., 0., 0., x],
                [0., 1., 0., y],
                [0., 0., 1., z],
                [0., 0., 0., 1.],
            ]),
            minv: Matrix([
                [1., 0., 0., -x],
                [0., 1., 0., -y],
                [0., 0., 1., -z],
                [0., 0., 0., 1.],
            ]),
        }
    }
    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self {
            m: Matrix([
                [x, 0., 0., 0.],
                [0., y, 0., 0.],
                [0., 0., z, 0.],
                [0., 0., 0., 1.],
            ]),
            minv: Matrix([
                [1. / x, 0., 0., 0.],
                [0., 1. / y, 0., 0.],
                [0., 0., 1. / z, 0.],
                [0., 0., 0., 1.],
            ]),
        }
    }
    pub fn rotation_x(angle: f64) -> Self {
        Self {
            m: Matrix([
                [1., 0., 0., 0.],
                [0., angle.cos(), -angle.sin(), 0.],
                [0., angle.sin(), angle.cos(), 0.],
                [0., 0., 0., 1.],
            ]),
            minv: Matrix([
                [1., 0., 0., 0.],
                [0., angle.cos(), angle.sin(), 0.],
                [0., -angle.sin(), angle.cos(), 0.],
                [0., 0., 0., 1.],
            ]),
        }
    }
    pub fn rotation_y(angle: f64) -> Self {
        Self {
            m: Matrix([
                [angle.cos(), 0., angle.sin(), 0.],
                [0., 1., 0., 0.],
                [-angle.sin(), 0., angle.cos(), 0.],
                [0., 0., 0., 1.],
            ]),
            minv: Matrix([
                [angle.cos(), 0., -angle.sin(), 0.],
                [0., 1., 0., 0.],
                [angle.sin(), 0., angle.cos(), 0.],
                [0., 0., 0., 1.],
            ]),
        }
    }
    pub fn rotation_z(angle: f64) -> Self {
        Self {
            m: Matrix([
                [angle.cos(), -angle.sin(), 0., 0.],
                [angle.sin(), angle.cos(), 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]),
            minv: Matrix([
                [angle.cos(), angle.sin(), 0., 0.],
                [-angle.sin(), angle.cos(), 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]),
        }
    }
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let m = Matrix([
            [1., xy, xz, 0.],
            [yx, 1., yz, 0.],
            [zx, zy, 1., 0.],
            [0., 0., 0., 1.],
        ]);
        let minv = m.inverse();
        Self { m, minv }
    }

    pub fn view_transform(from: Point, to: Point, up: Vector) -> Self {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = forward.cross(upn);
        let true_up = left.cross(forward);
        let left2 = left.normalize();
        let up2 = true_up.normalize();
        let m = Matrix([
            [left.0, left.1, left.2, 0.],
            [true_up.0, true_up.1, true_up.2, 0.],
            [-forward.0, -forward.1, -forward.2, 0.],
            [0., 0., 0., 1.],
        ]);
        let minv = m.inverse();
        let orientation = Self {
            m,
            minv, // m: Matrix([
                  //     [left.0, left.1, left.2, 0.],
                  //     [true_up.0, true_up.1, true_up.2, 0.],
                  //     [-forward.0, -forward.1, -forward.2, 0.],
                  //     [0., 0., 0., 1.],
                  // ]),
                  // minv: Matrix([
                  //     [left2.0, up2.0, -forward.0, 0.],
                  //     [left2.1, up2.1, -forward.1, 0.],
                  //     [left2.2, up2.2, -forward.2, 0.],
                  //     [0., 0., 0., 1.],
                  // ]),
                  // m
        };
        // println!(
        //     "***********\n{:?}\n***********",
        //     orientation.m * orientation.minv
        // );
        orientation * Transform::translation(-from.0, -from.1, -from.2)
    }

    pub fn inverse(self) -> Self {
        Self {
            m: self.minv,
            minv: self.m,
        }
    }
}

impl Mul for Transform {
    type Output = Transform;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m: self.m * rhs.m,
            minv: rhs.minv * self.minv,
        }
    }
}

impl AlmostEq for Transform {
    fn almost_eq(self, other: Self, eps: f64) -> bool {
        self.m.almost_eq(other.m, eps) && self.minv.almost_eq(other.minv, eps)
    }
}

pub trait Transformable {
    fn transform(self, t: Transform) -> Self;
}

pub trait Transformed: Transformable {
    fn translation(self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::translation(x, y, z);
        self.transform(t)
    }
    fn inv_translation(self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::translation(x, y, z);
        self.transform(t.inverse())
    }
    fn scaling(self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::scaling(x, y, z);
        self.transform(t)
    }
    fn inv_scaling(self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::scaling(x, y, z);
        self.transform(t.inverse())
    }
    fn rotation_x(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::rotation_x(angle);
        self.transform(t)
    }
    fn inv_rotation_x(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::rotation_x(angle);
        self.transform(t.inverse())
    }

    fn rotation_y(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::rotation_y(angle);
        self.transform(t)
    }
    fn inv_rotation_y(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::rotation_y(angle);
        self.transform(t.inverse())
    }
    fn rotation_z(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::rotation_z(angle);
        self.transform(t)
    }
    fn inv_rotation_z(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::rotation_z(angle);
        self.transform(t.inverse())
    }
    fn shearing(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self
    where
        Self: Sized,
    {
        let t = Transform::shearing(xy, xz, yx, yz, zx, zy);
        self.transform(t)
    }
}

#[cfg(test)]
mod tests {
    use crate::macros::AlmostEq;
    use std::f64::consts::PI;

    use super::*;
    use crate::assert_almost_eq;
    use crate::geometry::{Point, Vector};
    use crate::matrix::Matrix;

    #[test]
    fn translating_a_point() {
        let p = Point(1., 2., 3.);
        assert_eq!(Point(2., 3., 4.), p.translation(1., 1., 1.));
    }

    #[test]
    fn translating_a_vector() {
        let v = Vector(1., 2., 3.);
        let w = v.translation(1., 1., 1.);
        assert_eq!(w, v);
    }

    #[test]
    fn scaling_a_point() {
        let p = Point(-4., 6., 8.);
        assert_eq!(p.scaling(2., 3., 4.), Point(-8., 18., 32.));
    }

    #[test]
    fn scaling_a_vector() {
        let v = Vector(-4., 6., 8.);
        assert_eq!(v.scaling(2., 3., 4.), Vector(-8., 18., 32.));
    }
    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Point(0., 1., 0.);
        assert_almost_eq!(
            p.rotation_x(PI / 4.),
            Point(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.)
        );
        assert_almost_eq!(p.rotation_x(PI / 2.), Point(0., 0., 1.));
        assert_almost_eq!(
            p.inv_rotation_x(PI / 4.),
            Point(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.)
        );
    }
    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Point(0., 0., 1.);
        assert_almost_eq!(
            p.rotation_y(PI / 4.),
            Point(2f64.sqrt() / 2., 0., 2f64.sqrt() / 2.)
        );
        assert_almost_eq!(p.rotation_y(PI / 2.), Point(1., 0., 0.));
        assert_almost_eq!(
            p.inv_rotation_y(PI / 4.),
            Point(-2f64.sqrt() / 2., 0., 2f64.sqrt() / 2.)
        );
    }
    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Point(0., 1., 0.);
        assert_almost_eq!(
            p.rotation_z(PI / 4.),
            Point(-2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.)
        );
        assert_almost_eq!(p.rotation_z(PI / 2.), Point(-1., 0., 0.));
        assert_almost_eq!(
            p.inv_rotation_z(PI / 4.),
            Point(2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.)
        );
    }
    #[test]
    fn applying_a_shearing_transform_to_a_point() {
        let p = Point(2., 3., 4.);
        assert_eq!(p.shearing(1., 0., 0., 0., 0., 0.), Point(5., 3., 4.));
        assert_eq!(p.shearing(0., 1., 0., 0., 0., 0.), Point(6., 3., 4.));
        assert_eq!(p.shearing(0., 0., 1., 0., 0., 0.), Point(2., 5., 4.));
        assert_eq!(p.shearing(0., 0., 0., 1., 0., 0.), Point(2., 7., 4.));
        assert_eq!(p.shearing(0., 0., 0., 0., 1., 0.), Point(2., 3., 6.));
        assert_eq!(p.shearing(0., 0., 0., 0., 0., 1.), Point(2., 3., 7.));
    }
    #[test]
    fn chaining_transforms() {
        let p = Point(1., 0., 1.)
            .rotation_x(PI / 2.)
            .scaling(5., 5., 5.)
            .translation(10., 5., 7.);
        assert_eq!(p, Point(15., 0., 7.));
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = Point(0., 0., 0.);
        let to = Point(0., 0., -1.);
        let up = Vector(0., 1., 0.);
        assert_eq!(
            Transform::view_transform(from, to, up),
            Transform::default()
        );
    }
    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Point(0., 0., 0.);
        let to = Point(0., 0., 1.);
        let up = Vector(0., 1., 0.);
        assert_eq!(
            Transform::view_transform(from, to, up),
            Transform::scaling(-1., 1., -1.)
        );
    }
    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Point(1., 3., 2.);
        let to = Point(4., -2., 8.);
        let up = Vector(1., 1., 0.);
        let m = Matrix([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.],
            [0., 0., 0., 1.],
        ]);
        let minv = m.inverse();
        assert_almost_eq!(
            Transform::view_transform(from, to, up),
            Transform { m, minv },
            1e-4f64
        );
    }
}
