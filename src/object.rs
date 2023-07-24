use crate::{
    canvas::Color,
    geometry::{Point, Vector},
    intersection::{Intersection, Intersections},
    macros::EPSILON,
    material::Material,
    pattern::Pattern,
    ray::Ray,
    transform::{Transform, Transformable},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    Sphere,
    Plane,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Object {
    pub shape: Shape,
    pub transform: Transform,
    pub material: Material,
}

impl Object {
    pub fn sphere() -> Self {
        Self {
            shape: Shape::Sphere,
            transform: Transform::default(),
            material: Material::default(),
        }
    }

    pub fn plane() -> Self {
        Self {
            shape: Shape::Plane,
            transform: Transform::default(),
            material: Material::default(),
        }
    }

    pub fn set_transform(&mut self, t: Transform) -> Self {
        self.transform = t;
        *self
    }

    pub fn set_material(&mut self, m: Material) -> Self {
        self.material = m;
        *self
    }

    pub fn set_color(&mut self, c: Color) -> Self {
        self.material.color = c;
        *self
    }

    pub fn set_ambient(&mut self, a: f64) -> Self {
        self.material.ambient = a;
        *self
    }

    pub fn set_diffuse(&mut self, d: f64) -> Self {
        self.material.diffuse = d;
        *self
    }

    pub fn set_specular(&mut self, s: f64) -> Self {
        self.material.specular = s;
        *self
    }

    pub fn set_shininess(&mut self, s: f64) -> Self {
        self.material.shininess = s;
        *self
    }

    pub fn set_pattern(&mut self, p: Pattern) -> Self {
        self.material.pattern = Some(p);
        *self
    }

    pub fn intersect(self, ray: Ray) -> Intersections {
        let local_ray = ray.transform(self.transform.inverse());
        match self.shape {
            Shape::Sphere => {
                let mut xs: Intersections = Intersections(Vec::with_capacity(2));
                let sphere_to_ray = local_ray.origin - Point(0., 0., 0.);
                let a = local_ray.direction.dot(local_ray.direction);
                let b = 2f64 * local_ray.direction.dot(sphere_to_ray);
                let c = sphere_to_ray.dot(sphere_to_ray) - 1.;
                let discriminant = b.powi(2) - 4. * a * c;
                if discriminant < 0. {
                    xs
                } else {
                    xs.push(Intersection {
                        t: (-b - discriminant.sqrt()) / (2. * a),
                        object: self,
                    });
                    xs.push(Intersection {
                        t: (-b + discriminant.sqrt()) / (2. * a),
                        object: self,
                    });
                    xs
                }
            }
            Shape::Plane => {
                let mut xs: Intersections = Intersections(Vec::with_capacity(1));
                if local_ray.direction.1.abs() >= EPSILON {
                    xs.push(Intersection {
                        t: -local_ray.origin.1 / local_ray.direction.1,
                        object: self,
                    })
                }
                xs
            }
        }
    }

    pub fn normal_at(self, p: Point) -> Vector {
        let local_point = self.transform.minv * p;
        let local_normal: Vector;
        match self.shape {
            Shape::Sphere => {
                local_normal = local_point - Point(0., 0., 0.);
            }
            Shape::Plane => {
                local_normal = Vector(0., 1., 0.);
            }
        }
        let world_normal = self.transform.minv.transpose() * local_normal;
        world_normal.normalize()
    }
}
#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::{Object, Shape};
    use crate::geometry::{Point, Vector};
    use crate::intersection::Intersections;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::transform::Transform;

    fn almost_eq(v1: Vector, v2: Vector) -> bool {
        (v1.0 - v2.0).abs() < 1e6 && (v1.1 - v2.1).abs() < 1e6 && (v1.2 - v2.2).abs() < 1e6
    }
    #[test]
    fn a_default_sphere() {
        let s = Object::sphere();
        assert_eq!(
            s,
            Object {
                shape: Shape::Sphere,
                transform: Transform {
                    m: Matrix::id(),
                    minv: Matrix::id()
                },
                material: Material::default()
            }
        );
    }
    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Object::sphere();
        let t = Transform::translation(2., 3., 4.);
        s.set_transform(t);
        assert_eq!(
            s,
            Object {
                shape: Shape::Sphere,
                transform: t,
                material: Material::default()
            }
        )
    }
    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let mut s = Object::sphere();
        s.set_transform(Transform::translation(5., 0., 0.));
        let Intersections(xs) = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray {
            origin: Point(0., 0., -5.),
            direction: Vector(0., 0., 1.),
        };
        let mut s = Object::sphere();
        s.set_transform(Transform::scaling(2., 2., 2.));
        let Intersections(xs) = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let o = Object::sphere();
        let n = o.normal_at(Point(1., 0., 0.));
        assert_eq!(n, Vector(1., 0., 0.));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let o = Object::sphere();
        let n = o.normal_at(Point(0., 1., 0.));
        assert_eq!(n, Vector(0., 1., 0.));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let o = Object::sphere();
        let n = o.normal_at(Point(0., 0., 1.));
        assert_eq!(n, Vector(0., 0., 1.));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let o = Object::sphere();
        let n = o.normal_at(Point(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.));
        assert_eq!(
            n,
            Vector(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.)
        );
    }
    #[test]
    fn the_normal_is_a_normalized_vector() {
        let o = Object::sphere();
        let n = o.normal_at(Point(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.));

        assert_eq!(n, n.normalize());
    }
    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Object::sphere();
        s.set_transform(Transform::translation(0., 1., 0.));
        assert!(almost_eq(
            s.normal_at(Point(0., 1.70711, -0.70711)),
            Vector(0., 0.70711, -0.70711)
        ));
    }
    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Object::sphere();
        let t = Transform::scaling(1., 0.5, 1.) * Transform::rotation_z(PI / 5.);
        s.set_transform(t);
        assert!(almost_eq(
            s.normal_at(Point(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.)),
            Vector(0., 0.97014, -0.24254)
        ));
    }
    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Object::plane();
        let n1 = p.normal_at(Point(0., 0., 0.));
        let n2 = p.normal_at(Point(10., 0., -10.));
        let n3 = p.normal_at(Point(-5., 0., 150.));
        assert_eq!(n1, Vector(0., 1., 0.));
        assert_eq!(n2, Vector(0., 1., 0.));
        assert_eq!(n3, Vector(0., 1., 0.));
    }

    #[test]
    fn intersect_a_ray_parallel_to_the_plane() {
        let p = Object::plane();
        let r = Ray {
            origin: Point(0., 10., 0.),
            direction: Vector(0., 0., 1.),
        };
        let Intersections(xs) = p.intersect(r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Object::plane();
        let r = Ray {
            origin: Point(0., 0., 0.),
            direction: Vector(0., 0., 1.),
        };
        let Intersections(xs) = p.intersect(r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Object::plane();
        let r = Ray {
            origin: Point(0., 1., 0.),
            direction: Vector(0., -1., 0.),
        };
        let Intersections(xs) = p.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object, p);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Object::plane();
        let r = Ray {
            origin: Point(0., -1., 0.),
            direction: Vector(0., 1., 0.),
        };
        let Intersections(xs) = p.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object, p);
    }
}
