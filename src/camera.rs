use crate::{
    canvas::Canvas,
    geometry::Point,
    ray::Ray,
    transform::{Transform, Transformable},
    world::World,
};

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Transform,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64, t: Option<Transform>) -> Self {
        let half_view = (field_of_view / 2.).tan();
        let aspect = (hsize as f64) / (vsize as f64);
        let half_width: f64;
        let half_height: f64;
        if aspect >= 1. {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.) / (hsize as f64);
        let transform: Transform;

        match t {
            Some(t) => {
                transform = t;
            }
            None => {
                transform = Transform::default();
            }
        }
        Self {
            hsize,
            vsize,
            field_of_view,
            pixel_size,
            half_width,
            half_height,
            transform,
        }
    }
}

impl Camera {
    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = Point(world_x, world_y, -1.).transform(self.transform.inverse());
        let origin = Point(0., 0., 0.).transform(self.transform.inverse());
        let direction = (pixel - origin).normalize();
        Ray { origin, direction }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize, None);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray, 5);
                image.write_pixel(x, y, color);
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        canvas::Color,
        geometry::{Point, Vector},
        macros::AlmostEq,
    };
    use std::f64::consts::PI;

    use crate::assert_almost_eq;

    use super::*;

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2., None);
        assert_almost_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2., None);
        assert_almost_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructiong_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2., None);
        let r = c.ray_for_pixel(100, 50);
        assert_almost_eq!(r.origin, Point(0., 0., 0.));
        assert_almost_eq!(r.direction, Vector(0., 0., -1.));
    }

    #[test]
    fn constructiong_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2., None);
        let r = c.ray_for_pixel(0, 0);
        assert_almost_eq!(r.origin, Point(0., 0., 0.));
        assert_almost_eq!(r.direction, Vector(0.66519, 0.33259, -0.66851), 1e-5);
    }

    #[test]
    fn constructiong_a_ray_when_the_camera_is_transformed() {
        let c = Camera::new(
            201,
            101,
            PI / 2.,
            Some(Transform::rotation_y(PI / 4.) * Transform::translation(0., -2., 5.)),
        );
        let r = c.ray_for_pixel(100, 50);
        assert_almost_eq!(r.origin, Point(0., 2., -5.));
        assert_almost_eq!(r.direction, Vector(2f64.sqrt() / 2., 0., -2f64.sqrt() / 2.));
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let from = Point(0., 0., -5.);
        let to = Point(0., 0., 0.);
        let up = Vector(0., 1., 0.);
        let t = Transform::view_transform(from, to, up);
        let c = Camera::new(11, 11, PI / 2., Some(t));
        let image = c.render(&w);
        assert_almost_eq!(image.pixel_at(5, 5), Color(0.38066, 0.47583, 0.2855));
    }
}
