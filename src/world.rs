use crate::{
    canvas::Color,
    geometry::Point,
    intersection::{Computations, Intersections},
    light::PointLight,
    material::Material,
    object::Object,
    ray::Ray,
    transform::Transform,
};

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        let lights = vec![PointLight {
            intensity: Color::white(),
            position: Point(-10., 10., -10.),
        }];
        let objects = vec![
            Object {
                material: Material {
                    color: Color(0.8, 1.0, 0.6),
                    diffuse: 0.7,
                    specular: 0.2,
                    ..Material::default()
                },
                ..Object::sphere()
            },
            Object {
                transform: Transform::scaling(0.5, 0.5, 0.5),
                ..Object::sphere()
            },
        ];
        Self { objects, lights }
    }
}

impl World {
    pub fn empty() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn intersect(&self, r: Ray) -> Intersections {
        let mut xs = Intersections(vec![]);
        (self.objects)
            .iter()
            .for_each(|o| xs.append(&mut o.intersect(r)));
        xs.0.sort_unstable();
        xs
    }

    pub fn shade_hit(&self, comps: &Computations, remaining: usize) -> Color {
        (self.lights).iter().fold(Color::black(), |acc, light| {
            let shadowed = self.is_shadowed(light.position, comps.over_point);
            let surface = acc
                + comps.object.material.lighting(
                    &comps.object,
                    *light,
                    comps.over_point,
                    comps.eyev,
                    comps.normalv,
                    shadowed,
                );
            let reflected = self.reflected_color(comps, remaining);
            let refracted = self.refracted_color(comps, remaining);
            surface + reflected + refracted
        })
    }

    pub fn color_at(&self, r: Ray, remaining: usize) -> Color {
        let xs = self.intersect(r);
        if let Some((idx, hit)) = xs.hit() {
            let comps = hit.prepare_computations(r, idx, &xs);
            self.shade_hit(&comps, remaining)
        } else {
            Color::black()
        }
    }

    pub fn is_shadowed(&self, source: Point, point: Point) -> bool {
        let v = source - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray {
            origin: point,
            direction,
        };
        let intersections = self.intersect(r);
        let h = intersections.hit();
        h.is_some() && h.unwrap().1.t < distance
    }

    pub fn reflected_color(&self, comps: &Computations, remaining: usize) -> Color {
        if comps.object.material.reflective == 0. || remaining == 0 {
            Color::black()
        } else {
            let reflect_ray = Ray {
                origin: comps.over_point,
                direction: comps.reflectv,
            };
            let color = self.color_at(reflect_ray, remaining - 1);

            color * comps.object.material.reflective
        }
    }

    pub fn refracted_color(&self, comps: &Computations, remaining: usize) -> Color {
        if comps.object.material.transparency == 0. || remaining == 0 {
            Color::black()
        } else {
            let n_ratio = comps.n1 / comps.n2;
            let cos_i = comps.eyev.dot(comps.normalv);
            let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));
            if sin2_t > 1. {
                Color::black()
            } else {
                let cos_t = (1. - sin2_t).sqrt();

                let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;

                let refract_ray = Ray {
                    origin: comps.under_point,
                    direction,
                };

                self.color_at(refract_ray, remaining - 1) * comps.object.material.transparency
            }
        }
    }

    pub fn add_object(&mut self, o: Object) {
        self.objects.push(o);
    }

    pub fn add_light(&mut self, l: PointLight) {
        self.lights.push(l);
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        assert_almost_eq,
        canvas::Color,
        geometry::{Point, Vector},
        intersection::{Intersection, Intersections},
        light::PointLight,
        macros::AlmostEq,
        object::Object,
        pattern::Pattern,
        ray::Ray,
        transform::Transform,
    };

    use super::World;
    fn almost_eq(c1: Color, c2: Color) -> bool {
        (c1.0 - c2.0).abs() < 1e6 && (c1.1 - c2.1).abs() < 1e6 && (c1.2 - c2.2).abs() < 1e6
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let Intersections(xs) = w.intersect(r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.);
    }
    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point(0., 0., -5.), Vector(0., 0., 1.));
        let s = w.objects[0];
        let i = Intersection { t: 4., object: &s };
        let comps = i.prepare_computations(r, 0, &Intersections(vec![i]));
        let c = w.shade_hit(&comps, 5);
        assert!(almost_eq(c, Color(0.38066, 0.47583, 0.2855)));
    }
    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.lights[0] = PointLight {
            position: Point(0., 0.25, 0.),
            intensity: Color::white(),
        };
        let r = Ray::new(Point(0., 0., 0.), Vector(0., 0., 1.));
        let s = w.objects[1];
        let i = Intersection { t: 0.5, object: &s };
        let comps = i.prepare_computations(r, 0, &Intersections(vec![i]));
        let c = w.shade_hit(&comps, 5);
        assert!(almost_eq(c, Color(0.90498, 0.90498, 0.90498)));
    }
    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point(0., 0., -5.), Vector(0., 1., 0.));
        assert_eq!(w.color_at(r, 5), Color(0., 0., 0.));
    }
    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point(0., 0., -5.), Vector(0., 0., 1.));
        assert!(almost_eq(w.color_at(r, 5), Color(0.38066, 0.47583, 0.2855)));
    }
    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.;
        w.objects[1].material.ambient = 1.;
        let r = Ray::new(Point(0., 0., 0.75), Vector(0., 0., -1.));
        assert_eq!(w.color_at(r, 5), w.objects[1].material.color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Point(0., 10., 0.);
        assert!(!w.is_shadowed(w.lights[0].position, p));
    }
    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Point(10., -10., 10.);
        assert!(w.is_shadowed(w.lights[0].position, p));
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Point(-20., 20., -20.);
        assert!(!w.is_shadowed(w.lights[0].position, p));
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Point(-2., 2., -2.);
        assert!(!w.is_shadowed(w.lights[0].position, p));
    }
    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let light = PointLight {
            position: Point(0., 0., -10.),
            intensity: Color::white(),
        };
        let s1 = Object::sphere();
        let s2 = Object::sphere().set_transform(Transform::translation(0., 0., 10.));

        let w = World {
            lights: vec![light],
            objects: vec![s1, s2],
        };
        let r = Ray {
            origin: Point(0., 0., 5.),
            direction: Vector(0., 0., 1.),
        };
        let i = Intersection { t: 4., object: &s2 };
        let comps = i.prepare_computations(r, 0, &Intersections(vec![i]));
        let c = w.shade_hit(&comps, 5);
        assert_almost_eq!(c, Color(0.1, 0.1, 0.1));
    }
    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let w = World::default();
        let r = Ray {
            origin: Point(0., 0., 0.),
            direction: Vector(0., 0., 1.),
        };
        let mut s = w.objects[1];
        s.material.set_ambient(1.);
        let i = Intersection { t: 1., object: &s };
        let comps = i.prepare_computations(r, 0, &Intersections(vec![i]));
        assert_eq!(w.reflected_color(&comps, 5), Color(0., 0., 0.));
    }
    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = World::default();
        let shape = Object::plane()
            .set_reflective(0.5)
            .set_transform(Transform::translation(0., -1., 0.));
        w.add_object(shape);
        let r = Ray {
            origin: Point(0., 0., -3.),
            direction: Vector(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        };
        let i = Intersection {
            t: 2f64.sqrt(),
            object: &shape,
        };
        let comps = i.prepare_computations(r, 0, &Intersections(vec![i]));
        assert_almost_eq!(
            w.reflected_color(&comps, 5),
            Color(0.19032, 0.2379, 0.14274)
        );
    }
    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = World::default();
        let shape = Object::plane()
            .set_reflective(0.5)
            .set_transform(Transform::translation(0., -1., 0.));
        w.add_object(shape);
        let r = Ray {
            origin: Point(0., 0., -3.),
            direction: Vector(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        };
        let i = Intersection {
            t: 2f64.sqrt(),
            object: &shape,
        };
        let comps = i.prepare_computations(r, 0, &Intersections(vec![i]));
        assert_almost_eq!(w.shade_hit(&comps, 5), Color(0.87677, 0.92436, 0.82918));
    }
    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::empty();

        w.add_light(PointLight {
            position: Point(0., 0., 0.),
            intensity: Color::white(),
        });
        let lower = Object::plane()
            .set_reflective(1.)
            .set_transform(Transform::translation(0., -1., 0.));
        let upper = Object::plane()
            .set_reflective(1.)
            .set_transform(Transform::translation(0., 1., 0.));
        w.add_object(lower);
        w.add_object(upper);
        let r = Ray {
            origin: Point(0., 0., 0.),
            direction: Vector(0., 1., 0.),
        };
        w.color_at(r, 5);
        assert!(true);
    }
    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::default();
        let shape = Object::plane()
            .set_reflective(0.5)
            .set_transform(Transform::translation(0., -1., 0.));
        w.add_object(shape);
        let r = Ray {
            origin: Point(0., 0., -3.),
            direction: Vector(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        };
        let i = Intersection {
            t: 2f64.sqrt(),
            object: &shape,
        };
        let comps = i.prepare_computations(r, 0, &Intersections(vec![i]));
        assert_eq!(w.reflected_color(&comps, 0), Color::black());
    }
    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.objects[0];
        let r = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let xs = Intersections(vec![
            Intersection {
                t: 4.,
                object: &shape,
            },
            Intersection {
                t: 6.,
                object: &shape,
            },
        ]);
        let comps = xs[0].prepare_computations(r, 0, &xs);
        assert_eq!(w.refracted_color(&comps, 5), Color::black());
    }
    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let w = World::default();
        let mut shape = w.objects[0];
        shape.set_transparency(1.0).set_refractive_index(1.5);
        let r = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let xs = Intersections(vec![
            Intersection {
                t: 4.,
                object: &shape,
            },
            Intersection {
                t: 6.,
                object: &shape,
            },
        ]);
        let comps = xs[0].prepare_computations(r, 0, &xs);
        assert_eq!(w.refracted_color(&comps, 0), Color::black());
    }
    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut w = World::default();
        let shape = w.objects[0].set_transparency(1.).set_refractive_index(1.5);
        let r = Ray {
            origin: Point(0., 0., 2f64.sqrt() / 2.),
            direction: Vector(0., 1., 0.),
        };
        let xs = Intersections(vec![
            Intersection {
                t: -2f64.sqrt() / 2.,
                object: &shape,
            },
            Intersection {
                t: 2f64.sqrt() / 2.,
                object: &shape,
            },
        ]);
        let comps = xs[1].prepare_computations(r, 1, &xs);
        assert_eq!(w.refracted_color(&comps, 5), Color::black());
    }
    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = World::default();

        let a = w.objects[0]
            .set_pattern(Pattern::test_pattern())
            .set_ambient(1.0);
        w.objects[0] = a;

        let b = w.objects[1].set_transparency(1.).set_refractive_index(1.5);
        w.objects[1] = b;
        let r = Ray {
            origin: Point(0., 0., 0.1),
            direction: Vector(0., 1., 0.),
        };
        let xs = Intersections(vec![
            Intersection {
                t: -0.9899,
                object: &a,
            },
            Intersection {
                t: -0.4899,
                object: &b,
            },
            Intersection {
                t: 0.4899,
                object: &b,
            },
            Intersection {
                t: 0.9899,
                object: &a,
            },
        ]);
        let comps = xs[2].prepare_computations(r, 2, &xs);
        assert_almost_eq!(w.refracted_color(&comps, 5), Color(0., 0.99888, 0.04725));
    }
    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = World::default();
        let floor = Object::plane()
            .set_transform(Transform::translation(0., -1., 0.))
            .set_transparency(0.5)
            .set_refractive_index(1.5);
        w.add_object(floor);
        let ball = Object::sphere()
            .set_color(Color(1., 0., 0.))
            .set_ambient(0.5)
            .set_transform(Transform::translation(0., -3.5, -0.5));
        w.add_object(ball);
        let r = Ray {
            origin: Point(0., 0., -3.),
            direction: Vector(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        };
        let xs = Intersections(vec![Intersection {
            t: 2f64.sqrt(),
            object: &floor,
        }]);
        let comps = xs[0].prepare_computations(r, 0, &xs);
        assert_almost_eq!(w.shade_hit(&comps, 5), Color(0.93642, 0.68642, 0.68642));
    }
}
