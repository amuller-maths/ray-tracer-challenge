use crate::geometry::{Point, Vector};
use crate::transform::{
    inv_rotation_x, inv_rotation_y, inv_rotation_z, inv_scaling, inv_translation, rotation_x,
    rotation_y, rotation_z, scaling, shearing, translation,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    fn position(self, t: f64) -> Point {
        self.origin + self.direction * t
    }
    fn translation(self, x: f64, y: f64, z: f64) -> Self {
        let m = translation(x, y, z);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }

    fn inv_translation(self, x: f64, y: f64, z: f64) -> Self {
        let m = inv_translation(x, y, z);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }

    fn scaling(self, x: f64, y: f64, z: f64) -> Self {
        let m = scaling(x, y, z);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }

    fn inv_scaling(self, x: f64, y: f64, z: f64) -> Self {
        let m = inv_scaling(x, y, z);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
    fn rotation_x(self, angle: f64) -> Self {
        let m = rotation_x(angle);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }

    fn inv_rotation_x(self, angle: f64) -> Self {
        let m = inv_rotation_x(angle);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
    fn rotation_y(self, angle: f64) -> Self {
        let m = rotation_y(angle);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }

    fn inv_rotation_y(self, angle: f64) -> Self {
        let m = inv_rotation_y(angle);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }

    fn rotation_z(self, angle: f64) -> Self {
        let m = rotation_z(angle);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }

    fn inv_rotation_z(self, angle: f64) -> Self {
        let m = inv_rotation_z(angle);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }

    fn shearing(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let m = shearing(xy, xz, yx, yz, zx, zy);
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::{intersection::Intersection, shape::Shape};
    #[test]
    fn point_from_distance() {
        let r = Ray {
            origin: Point(2., 3., 4.),
            direction: Vector(1., 0., 0.),
        };
        assert_eq!(r.position(0.), Point(2., 3., 4.));
        assert_eq!(r.position(1.), Point(3., 3., 4.));
        assert_eq!(r.position(-1.), Point(1., 3., 4.));
        assert_eq!(r.position(2.5), Point(4.5, 3., 4.));
    }
    #[test]
    fn intersect1() {
        let s = Shape::Sphere {
            center: Point(0., 0., 0.),
            radius: 1.,
        };
        let ray = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let xs = s.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], Intersection { t: 4., object: s });
        assert_eq!(xs[1], Intersection { t: 6., object: s });
    }
    #[test]
    fn intersect2() {
        let s = Shape::Sphere {
            center: Point(0., 0., 0.),
            radius: 1.,
        };
        let ray = Ray {
            origin: Point(0., 1., -5.),
            direction: Vector(0., 0., 1.),
        };
        let xs = s.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], Intersection { t: 5., object: s });
        assert_eq!(xs[1], Intersection { t: 5., object: s });
    }
    #[test]
    fn intersect3() {
        let s = Shape::Sphere {
            center: Point(0., 0., 0.),
            radius: 1.,
        };
        let ray = Ray {
            origin: Point(0., 2., -5.),
            direction: Vector(0., 0., 1.),
        };
        let xs = s.intersect(ray);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn intersect4() {
        let s = Shape::Sphere {
            center: Point(0., 0., 0.),
            radius: 1.,
        };
        let ray = Ray {
            origin: Point(0., 0., 0.),
            direction: Vector(0., 0., 1.),
        };
        let xs = s.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], Intersection { t: -1., object: s });
        assert_eq!(xs[1], Intersection { t: 1., object: s });
    }
    #[test]
    fn intersect5() {
        let s = Shape::Sphere {
            center: Point(0., 0., 0.),
            radius: 1.,
        };
        let ray = Ray {
            origin: Point(0., 0., 5.),
            direction: Vector(0., 0., 1.),
        };
        let xs = s.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], Intersection { t: -6., object: s });
        assert_eq!(xs[1], Intersection { t: -4., object: s });
    }
    #[test]
    fn translate_ray() {
        let r = Ray {
            origin: Point(1., 2., 3.),
            direction: Vector(0., 1., 0.),
        };
        let r2 = r.translation(3., 4., 5.);
        assert_eq!(
            r2,
            Ray {
                origin: Point(4., 6., 8.),
                direction: Vector(0., 1., 0.)
            }
        )
    }
    #[test]
    fn scaling_ray() {
        let r = Ray {
            origin: Point(1., 2., 3.),
            direction: Vector(0., 1., 0.),
        };
        let r2 = r.scaling(2., 3., 4.);
        assert_eq!(
            r2,
            Ray {
                origin: Point(2., 6., 12.),
                direction: Vector(0., 3., 0.)
            }
        )
    }
}
