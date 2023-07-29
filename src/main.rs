use std::f64::consts::PI;

use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::examples::*;
use ray_tracer_challenge::{
    canvas::Color, geometry::Point, geometry::Vector, light::PointLight, object::Object,
    pattern::Pattern, ray::Ray, transform::Transform, world::World,
};

fn main() {
    let mut w = World::empty();
    let wall = Object::plane()
        .set_transform(Transform::translation(0., 0., 10.) * Transform::rotation_x(1.5708))
        .set_pattern(Pattern::checkers_pattern(
            Color(0.15, 0.15, 0.15),
            Color(0.85, 0.85, 0.85),
        ))
        .set_ambient(0.8)
        .set_diffuse(0.2)
        .set_specular(0.);
    w.add_object(wall);
    let ball = Object::sphere()
        .set_color(Color(1., 1., 1.))
        .set_ambient(0.)
        .set_diffuse(0.)
        .set_specular(0.9)
        .set_shininess(300.)
        .set_reflective(0.9)
        .set_transparency(0.9)
        .set_refractive_index(1.5);
    w.add_object(ball);

    let center = Object::sphere()
        .set_transform(Transform::scaling(0.5, 0.5, 0.5))
        .set_color(Color(1., 1., 1.))
        .set_ambient(0.)
        .set_diffuse(0.)
        .set_specular(0.9)
        .set_shininess(300.)
        .set_reflective(0.9)
        .set_transparency(0.9)
        .set_refractive_index(1.0000034);
    w.add_object(center);
    let camera = Camera::new(
        600,
        600,
        0.45,
        Some(Transform::view_transform(
            Point(0., 0., -5.),
            Point(0., 0., 0.),
            Vector(0., 1., 0.),
        )),
    );

    let light = PointLight {
        position: Point(2., 10., -5.),
        intensity: Color(0.9, 0.9, 0.9),
    };
    w.add_light(light);

    let canvas = camera.render(&w);
    canvas.save("fresnel.png").unwrap();
}
