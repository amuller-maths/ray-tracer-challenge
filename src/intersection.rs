use crate::{
    geometry::{Point, Vector},
    macros::EPSILON,
    object::Object,
    ray::Ray,
};
use core::cmp::Ordering;

#[derive(Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Object,
}

#[derive(Debug)]
pub struct Computations {
    pub t: f64,
    pub object: Object,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub over_point: Point,
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t.is_nan() {
            Ordering::Greater
        } else if other.t.is_nan() {
            Ordering::Less
        } else if self.t > other.t {
            Ordering::Greater
        } else if self.t < other.t {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl Eq for Intersection {}

pub struct Intersections(pub Vec<Intersection>);

impl Intersection {
    pub fn prepare_computations(&self, r: Ray) -> Computations {
        let t = self.t;
        let object = self.object;
        let point = r.position(t);
        let eyev = -r.direction;
        let mut normalv = object.normal_at(point);
        let inside: bool;
        if normalv.dot(eyev) < 0. {
            inside = true;
            normalv = -normalv
        } else {
            inside = false;
        }
        let over_point = point + normalv * EPSILON;

        Computations {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        }
    }
}

impl Intersections {
    pub fn push(&mut self, element: Intersection) {
        let Intersections(v) = self;
        v.push(element);
        v.sort_unstable();
    }

    pub fn append(&mut self, elements: &mut Intersections) {
        let Intersections(v) = self;
        let Intersections(v2) = elements;
        v.append(v2);
        v.sort_unstable();
    }
    pub fn hit(&self) -> Option<&Intersection> {
        self.0.iter().find(|i| i.t >= 0.)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::{Point, Vector},
        intersection::Intersections,
        macros::EPSILON,
        object::Object,
        ray::Ray,
        transform::Transform,
    };

    use super::Intersection;
    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: 1., object: s };
        let i2 = Intersection { t: 2., object: s };
        intersections.push(i2);
        intersections.push(i1);
        assert_eq!(
            intersections.hit(),
            Some(&Intersection { t: 1., object: s })
        );
    }
    #[test]
    fn the_hit_when_all_intersections_have_positive_t_reversed() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: 1., object: s };
        let i2 = Intersection { t: 2., object: s };
        intersections.push(i1);
        intersections.push(i2);
        assert_eq!(
            intersections.hit(),
            Some(&Intersection { t: 1., object: s })
        );
    }
    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: -1., object: s };
        let i2 = Intersection { t: 1., object: s };
        intersections.push(i1);
        intersections.push(i2);
        assert_eq!(
            intersections.hit(),
            Some(&Intersection { t: 1., object: s })
        );
    }
    #[test]
    fn the_hit_when_some_intersections_have_negative_t_reversed() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: -1., object: s };
        let i2 = Intersection { t: 1., object: s };
        intersections.push(i2);
        intersections.push(i1);
        assert_eq!(
            intersections.hit(),
            Some(&Intersection { t: 1., object: s })
        );
    }
    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: -1., object: s };
        let i2 = Intersection { t: -2., object: s };
        intersections.push(i2);
        intersections.push(i1);
        assert_eq!(intersections.hit(), None);
    }
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: 5., object: s };
        let i2 = Intersection { t: 7., object: s };
        let i3 = Intersection { t: -3., object: s };
        let i4 = Intersection { t: 2., object: s };
        intersections.push(i1);
        intersections.push(i2);
        intersections.push(i3);
        intersections.push(i4);
        assert_eq!(
            intersections.hit(),
            Some(&Intersection { t: 2., object: s })
        );
    }
    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let s = Object::sphere();
        let i = Intersection { t: 4., object: s };
        let comps = (&i).prepare_computations(r);
        assert_eq!(comps.t, (&i).t);
        assert_eq!(comps.object, (&i).object);
        assert_eq!(comps.point, Point(0., 0., -1.));
        assert_eq!(comps.eyev, Vector(0., 0., -1.));
        assert_eq!(comps.normalv, Vector(0., 0., -1.));
    }
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let s = Object::sphere();
        let i = Intersection { t: 4., object: s };
        let comps = (&i).prepare_computations(r);
        assert_eq!(comps.inside, false);
    }
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray {
            origin: Point(0., 0., 0.),
            direction: Vector(0., 0., 1.),
        };
        let s = Object::sphere();
        let i = Intersection { t: 1., object: s };
        let comps = (&i).prepare_computations(r);
        assert_eq!(comps.point, Point(0., 0., 1.));
        assert_eq!(comps.eyev, Vector(0., 0., -1.));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Vector(0., 0., -1.));
    }
    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let mut shape = Object::sphere();
        shape.set_transform(Transform::translation(0., 0., 1.));
        let i = Intersection {
            t: 5.,
            object: shape,
        };
        let comps = i.prepare_computations(r);
        assert!(comps.over_point.2 < -EPSILON / 2.);
        assert!(comps.point.2 > comps.over_point.2);
    }
}
