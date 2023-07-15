use crate::geometry::Tuple;
use crate::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    Matrix([
        [1., 0., 0., x],
        [0., 1., 0., y],
        [0., 0., 1., z],
        [0., 0., 0., 1.],
    ])
}

pub fn inv_translation(x: f64, y: f64, z: f64) -> Matrix {
    Matrix([
        [1., 0., 0., -x],
        [0., 1., 0., -y],
        [0., 0., 1., -z],
        [0., 0., 0., 1.],
    ])
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    Matrix([
        [x, 0., 0., 0.],
        [0., y, 0., 0.],
        [0., 0., z, 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn inv_scaling(x: f64, y: f64, z: f64) -> Matrix {
    Matrix([
        [1. / x, 0., 0., 0.],
        [0., 1. / y, 0., 0.],
        [0., 0., 1. / z, 0.],
        [0., 0., 0., 1.],
    ])
}
pub fn rotation_x(angle: f64) -> Matrix {
    Matrix([
        [1., 0., 0., 0.],
        [0., angle.cos(), -angle.sin(), 0.],
        [0., angle.sin(), angle.cos(), 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn inv_rotation_x(angle: f64) -> Matrix {
    Matrix([
        [1., 0., 0., 0.],
        [0., -angle.cos(), angle.sin(), 0.],
        [0., -angle.sin(), -angle.cos(), 0.],
        [0., 0., 0., 1.],
    ])
}
pub fn rotation_y(angle: f64) -> Matrix {
    Matrix([
        [angle.cos(), 0., angle.sin(), 0.],
        [0., 1., 0., 0.],
        [-angle.sin(), 0., angle.cos(), 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn inv_rotation_y(angle: f64) -> Matrix {
    Matrix([
        [-angle.cos(), 0., -angle.sin(), 0.],
        [0., 1., 0., 0.],
        [angle.sin(), 0., -angle.cos(), 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn rotation_z(angle: f64) -> Matrix {
    Matrix([
        [angle.cos(), -angle.sin(), 0., 0.],
        [angle.sin(), angle.cos(), 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn inv_rotation_z(angle: f64) -> Matrix {
    Matrix([
        [-angle.cos(), angle.sin(), 0., 0.],
        [-angle.sin(), -angle.cos(), 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    Matrix([
        [1., xy, xz, 0.],
        [yx, 1., yz, 0.],
        [zx, zy, 1., 0.],
        [0., 0., 0., 1.],
    ])
}

pub trait Transform: Tuple {
    fn translation(self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let m = translation(x, y, z);
        m * self
    }

    fn inv_translation(self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let m = inv_translation(x, y, z);
        m * self
    }

    fn scaling(self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let m = scaling(x, y, z);
        m * self
    }

    fn inv_scaling(self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let m = inv_scaling(x, y, z);
        m * self
    }
    fn rotation_x(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let m = rotation_x(angle);
        m * self
    }

    fn inv_rotation_x(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let m = inv_rotation_x(angle);
        m * self
    }
    fn rotation_y(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let m = rotation_y(angle);
        m * self
    }

    fn inv_rotation_y(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let m = inv_rotation_y(angle);
        m * self
    }

    fn rotation_z(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let m = rotation_z(angle);
        m * self
    }

    fn inv_rotation_z(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let m = inv_rotation_z(angle);
        m * self
    }

    fn shearing(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self
    where
        Self: Sized,
    {
        let m = shearing(xy, xz, yx, yz, zx, zy);
        m * self
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::geometry::{Point, Vector};
    use crate::matrix::Matrix;

    fn almost_eq(p1: Point, p2: Point) -> bool {
        (p1.x() - p2.x()).abs() < 1e6
            && (p1.y() - p2.y()).abs() < 1e6
            && (p1.z() - p2.z()).abs() < 1e6
    }
    #[test]
    fn mul_tuple() {
        let a = Matrix([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let v = Vector(1., 2., 3.);

        assert_eq!(a * v, Vector(14., 38., 46.));
    }

    #[test]
    fn translate_point() {
        let p = Point(1., 2., 3.);
        assert_eq!(Point(2., 3., 4.), p.translation(1., 1., 1.))
    }

    #[test]
    fn translate_vector() {
        let v = Vector(1., 2., 3.);
        let w = v.translation(1., 1., 1.);
        assert_eq!(w, v);
    }

    #[test]
    fn scale_point() {
        let p = Point(-4., 6., 8.);
        assert_eq!(p.scaling(2., 3., 4.), Point(-8., 18., 32.));
    }

    #[test]
    fn scale_vector() {
        let v = Vector(-4., 6., 8.);
        assert_eq!(v.scaling(2., 3., 4.), Vector(-8., 18., 32.));
    }
    #[test]
    fn rotate_x_point() {
        let p = Point(0., 1., 0.);
        assert!(almost_eq(
            p.rotation_x(PI / 4.),
            Point(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.)
        ));
        assert!(almost_eq(p.rotation_x(PI / 2.), Point(0., 0., 1.)));
        assert!(almost_eq(
            p.inv_rotation_x(PI / 4.),
            Point(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.)
        ));
    }
    #[test]
    fn rotate_y_point() {
        let p = Point(0., 0., 1.);
        assert!(almost_eq(
            p.rotation_x(PI / 4.),
            Point(2f64.sqrt() / 2., 0., 2f64.sqrt() / 2.)
        ));
        assert!(almost_eq(p.rotation_x(PI / 2.), Point(1., 0., 0.)));
        assert!(almost_eq(
            p.inv_rotation_x(PI / 4.),
            Point(2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.)
        ));
    }
    #[test]
    fn rotate_z_point() {
        let p = Point(0., 1., 0.);
        assert!(almost_eq(
            p.rotation_x(PI / 4.),
            Point(-2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.)
        ));
        assert!(almost_eq(p.rotation_x(PI / 2.), Point(-1., 0., 0.)));
        assert!(almost_eq(
            p.inv_rotation_x(PI / 4.),
            Point(2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.)
        ));
    }
    #[test]
    fn shearing() {
        let p = Point(2., 3., 4.);
        assert_eq!(p.shearing(1., 0., 0., 0., 0., 0.), Point(5., 3., 4.));
        assert_eq!(p.shearing(0., 1., 0., 0., 0., 0.), Point(6., 3., 4.));
        assert_eq!(p.shearing(0., 0., 1., 0., 0., 0.), Point(2., 5., 4.));
        assert_eq!(p.shearing(0., 0., 0., 1., 0., 0.), Point(2., 7., 4.));
        assert_eq!(p.shearing(0., 0., 0., 0., 1., 0.), Point(2., 3., 6.));
        assert_eq!(p.shearing(0., 0., 0., 0., 0., 1.), Point(2., 3., 7.));
    }
    #[test]
    fn chaining() {
        let p = Point(1., 0., 1.)
            .rotation_x(PI / 2.)
            .scaling(5., 5., 5.)
            .translation(10., 5., 7.);
        assert_eq!(p, Point(15., 0., 7.));
    }
}
