use crate::geometry::Point;
use crate::intersection::Intersection;
use crate::ray::Ray;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    Sphere { center: Point, radius: f64 },
}

impl Shape {
    pub fn intersect(self, ray: Ray) -> Vec<Intersection> {
        match self {
            Shape::Sphere { center, radius } => {
                let mut xs: Vec<Intersection> = Vec::with_capacity(2);
                let sphere_to_ray = ray.origin - center;
                let a = ray.direction.dot(ray.direction);
                let b = 2. * ray.direction.dot(sphere_to_ray);
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
        }
    }

    pub fn default_sphere() -> Self {
        Shape::Sphere {
            center: Point(0., 0., 0.),
            radius: 1.,
        }
    }
    pub fn new_sphere(center: Point, radius: f64) -> Self {
        Shape::Sphere { center, radius }
    }
}
