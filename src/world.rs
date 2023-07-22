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
    pub fn intersect(&self, r: Ray) -> Intersections {
        let mut xs = Intersections(vec![]);
        (self.objects)
            .iter()
            .for_each(|o| xs.append(&mut o.intersect(r)));
        xs.0.sort_unstable();
        xs
    }

    pub fn shade_hit(&self, comps: Computations) -> Color {
        (self.lights).iter().fold(Color::black(), |acc, light| {
            let shadowed = self.is_shadowed(light.position, comps.over_point);
            acc + comps.object.material.lighting(
                *light,
                comps.point,
                comps.eyev,
                comps.normalv,
                shadowed,
            )
        })
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect(r);
        if let Some(hit) = xs.hit() {
            let comps = hit.prepare_computations(r);
            self.shade_hit(comps)
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
        h.is_some() && h.unwrap().t < distance
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
        let i = Intersection { t: 4., object: s };
        let comps = (&i).prepare_computations(r);
        let c = w.shade_hit(comps);
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
        let i = Intersection { t: 0.5, object: s };
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert!(almost_eq(c, Color(0.90498, 0.90498, 0.90498)));
    }
    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point(0., 0., -5.), Vector(0., 1., 0.));
        assert_eq!(w.color_at(r), Color(0., 0., 0.));
    }
    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point(0., 0., -5.), Vector(0., 0., 1.));
        assert!(almost_eq(w.color_at(r), Color(0.38066, 0.47583, 0.2855)));
    }
    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.;
        w.objects[1].material.ambient = 1.;
        let r = Ray::new(Point(0., 0., 0.75), Vector(0., 0., -1.));
        assert_eq!(w.color_at(r), w.objects[1].material.color);
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
        let mut s2 = Object::sphere();
        s2.set_transform(Transform::translation(0., 0., 10.));
        let w = World {
            lights: vec![light],
            objects: vec![s1, s2],
        };
        let r = Ray {
            origin: Point(0., 0., 5.),
            direction: Vector(0., 0., 1.),
        };
        let i = Intersection { t: 4., object: s2 };
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_almost_eq!(c, Color(0.1, 0.1, 0.1));
    }
}
