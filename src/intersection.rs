use uuid::Uuid;

use crate::{
    geometry::{Point, Vector},
    macros::EPSILON,
    object::Object,
    ray::Ray,
};
use core::cmp::Ordering;
use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'inter> {
    pub t: f64,
    pub object: &'inter Object,
}

#[derive(Debug)]
pub struct Computations<'inter> {
    pub t: f64,
    pub object: &'inter Object,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub reflectv: Vector,
    pub inside: bool,
    pub over_point: Point,
    pub under_point: Point,
    pub n1: f64,
    pub n2: f64,
}

impl<'inter> Ord for Intersection<'inter> {
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

impl<'inter> PartialOrd for Intersection<'inter> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'inter> PartialEq for Intersection<'inter> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl<'inter> Eq for Intersection<'inter> {}

pub struct Intersections<'inter>(pub Vec<Intersection<'inter>>);

impl<'inter> Index<usize> for Intersections<'inter> {
    type Output = Intersection<'inter>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'inter> Intersection<'inter> {
    pub fn prepare_computations(
        &self,
        r: Ray,
        hit_index: usize,
        xs: &Intersections,
    ) -> Computations {
        let mut n1: f64 = 1.;
        let mut n2: f64 = 1.;
        let mut containers: Vec<(Uuid, f64)> = Vec::with_capacity(hit_index);

        for (idx, i) in xs.0.iter().enumerate() {
            if idx == hit_index {
                if containers.len() == 0 {
                    n1 = 1.;
                } else {
                    n1 = containers[containers.len() - 1].1;
                }
            }

            if let Some(uuid_idx) = containers.iter().position(|&(x, _)| x == i.object.uuid) {
                containers.remove(uuid_idx);
            } else {
                containers.push((i.object.uuid, i.object.material.refractive_index));
            }

            if idx == hit_index {
                if containers.len() == 0 {
                    n2 = 1.;
                } else {
                    n2 = containers[containers.len() - 1].1;
                }
                break;
            }
        }
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
        let under_point = point - normalv * EPSILON;
        let reflectv = r.direction.reflect(normalv);

        Computations {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
            under_point,
            reflectv,
            n1,
            n2,
        }
    }
}

impl<'a> Intersections<'a> {
    pub fn push(&mut self, element: Intersection<'a>) {
        let Intersections(v) = self;
        v.push(element);
        v.sort_unstable();
    }

    pub fn append(&mut self, elements: &mut Intersections<'a>) {
        let Intersections(v) = self;
        let Intersections(v2) = elements;
        v.append(v2);
        v.sort_unstable();
    }
    pub fn hit(&self) -> Option<(usize, &Intersection)> {
        self.0.iter().enumerate().find(|(_, i)| i.t >= 0.)
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
        let i1 = Intersection { t: 1., object: &s };
        let i2 = Intersection { t: 2., object: &s };
        intersections.push(i2);
        intersections.push(i1);
        assert_eq!(
            intersections.hit(),
            Some((0, &Intersection { t: 1., object: &s }))
        );
    }
    #[test]
    fn the_hit_when_all_intersections_have_positive_t_reversed() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: 1., object: &s };
        let i2 = Intersection { t: 2., object: &s };
        intersections.push(i1);
        intersections.push(i2);
        assert_eq!(
            intersections.hit(),
            Some((0, &Intersection { t: 1., object: &s }))
        );
    }
    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: -1., object: &s };
        let i2 = Intersection { t: 1., object: &s };
        intersections.push(i1);
        intersections.push(i2);
        assert_eq!(
            intersections.hit(),
            Some((1, &Intersection { t: 1., object: &s }))
        );
    }
    #[test]
    fn the_hit_when_some_intersections_have_negative_t_reversed() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: -1., object: &s };
        let i2 = Intersection { t: 1., object: &s };
        intersections.push(i2);
        intersections.push(i1);
        assert_eq!(
            intersections.hit(),
            Some((1, &Intersection { t: 1., object: &s }))
        );
    }
    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: -1., object: &s };
        let i2 = Intersection { t: -2., object: &s };
        intersections.push(i2);
        intersections.push(i1);
        assert_eq!(intersections.hit(), None);
    }
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let mut intersections = Intersections(vec![]);
        let s = Object::sphere();
        let i1 = Intersection { t: 5., object: &s };
        let i2 = Intersection { t: 7., object: &s };
        let i3 = Intersection { t: -3., object: &s };
        let i4 = Intersection { t: 2., object: &s };
        intersections.push(i1);
        intersections.push(i2);
        intersections.push(i3);
        intersections.push(i4);
        assert_eq!(
            intersections.hit(),
            Some((1, &Intersection { t: 2., object: &s }))
        );
    }
    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let s = Object::sphere();
        let i = Intersection { t: 4., object: &s };
        let comps = (&i).prepare_computations(r, 0, &Intersections(vec![i]));
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
        let i = Intersection { t: 4., object: &s };
        let comps = i.prepare_computations(r, 0, &Intersections(vec![i]));
        assert_eq!(comps.inside, false);
    }
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray {
            origin: Point(0., 0., 0.),
            direction: Vector(0., 0., 1.),
        };
        let s = Object::sphere();
        let i = Intersection { t: 1., object: &s };
        let comps = i.prepare_computations(r, 0, &Intersections(vec![i]));
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
            object: &shape,
        };
        let comps = i.prepare_computations(r, 0, &Intersections(vec![i]));
        assert!(comps.over_point.2 < -EPSILON / 2.);
        assert!(comps.point.2 > comps.over_point.2);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let object = Object::plane();
        let r = Ray {
            origin: Point(0., 1., -1.),
            direction: Vector(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        };
        let i = Intersection {
            t: 2f64.sqrt(),
            object: &object,
        };
        let inter = &Intersections(vec![i]);
        let comps = (&i).prepare_computations(r, 0, inter);
        assert_eq!(
            comps.reflectv,
            Vector(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.)
        );
    }
    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = Object::glass_sphere()
            .set_transform(Transform::scaling(2., 2., 2.))
            .set_refractive_index(1.5);
        let b = Object::glass_sphere()
            .set_transform(Transform::translation(0., 0., -0.25))
            .set_refractive_index(2.);
        let c = Object::glass_sphere()
            .set_transform(Transform::translation(0., 0., 0.25))
            .set_refractive_index(2.5);
        let r = Ray {
            origin: Point(0., 0., -4.),
            direction: Vector(0., 0., 1.),
        };
        let i1 = Intersection { t: 2., object: &a };
        let i2 = Intersection {
            t: 2.75,
            object: &b,
        };
        let i3 = Intersection {
            t: 3.25,
            object: &c,
        };
        let i4 = Intersection {
            t: 4.75,
            object: &b,
        };
        let i5 = Intersection {
            t: 5.25,
            object: &c,
        };
        let i6 = Intersection { t: 6., object: &a };
        let xs = Intersections(vec![i1, i2, i3, i4, i5, i6]);
        let tests: Vec<(usize, f64, f64)> = vec![
            (0, 1.0, 1.5),
            (1, 1.5, 2.),
            (2, 2., 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.),
        ];
        tests.into_iter().for_each(|(index, n1, n2)| {
            let comps = xs[index].prepare_computations(r, index, &xs);
            assert_eq!(comps.n1, n1);
            assert_eq!(comps.n2, n2);
        });
    }
    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let shape = Object::glass_sphere().set_transform(Transform::translation(0., 0., 1.));
        let i = Intersection {
            t: 5.,
            object: &shape,
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, 0, &xs);
        assert!(comps.under_point.2 > EPSILON / 2.);
        assert!(comps.point.2 < comps.under_point.2);
    }
}
