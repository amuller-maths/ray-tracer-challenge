use crate::geometry::{Point, Vector};
use crate::transform::{Transform, Transformable, Transformed};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

impl Transformable for Ray {
    fn transform(self, t: Transform) -> Self {
        Self {
            origin: t.m * self.origin,
            direction: t.m * self.direction,
        }
    }
}
impl Transformed for Ray {}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::{
        intersection::{Intersection, Intersections},
        object::Object,
    };
    #[test]
    fn computing_a_point_from_a_distance() {
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
    fn a_ray_intersects_a_sphere_at_two_points() {
        let s = Object::sphere();
        let ray = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let Intersections(xs) = s.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], Intersection { t: 4., object: s });
        assert_eq!(xs[1], Intersection { t: 6., object: s });
    }
    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let s = Object::sphere();
        let ray = Ray {
            origin: Point(0., 1., -5.),
            direction: Vector(0., 0., 1.),
        };
        let Intersections(xs) = s.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], Intersection { t: 5., object: s });
        assert_eq!(xs[1], Intersection { t: 5., object: s });
    }
    #[test]
    fn a_ray_misses_a_sphere() {
        let s = Object::sphere();
        let ray = Ray {
            origin: Point(0., 2., -5.),
            direction: Vector(0., 0., 1.),
        };
        let Intersections(xs) = s.intersect(ray);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let s = Object::sphere();
        let ray = Ray {
            origin: Point(0., 0., 0.),
            direction: Vector(0., 0., 1.),
        };
        let Intersections(xs) = s.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], Intersection { t: -1., object: s });
        assert_eq!(xs[1], Intersection { t: 1., object: s });
    }
    #[test]
    fn a_sphere_is_behind_a_ray() {
        let s = Object::sphere();
        let ray = Ray {
            origin: Point(0., 0., 5.),
            direction: Vector(0., 0., 1.),
        };
        let Intersections(xs) = s.intersect(ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], Intersection { t: -6., object: s });
        assert_eq!(xs[1], Intersection { t: -4., object: s });
    }
    #[test]
    fn translating_a_ray() {
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
    fn scaling_a_ray() {
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
