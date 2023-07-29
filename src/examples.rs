use std::f64::consts::PI;

use crate::{
    camera::Camera,
    canvas::Color,
    geometry::{Point, Vector},
    light::PointLight,
    object::Object,
    pattern::Pattern,
    transform::Transform,
    world::World,
};

pub fn floor_with_3_spheres() {
    let floor = Object::plane()
        .set_color(Color(1., 0.9, 0.9))
        .set_specular(0.)
        .set_reflective(1.);

    let light_source = PointLight {
        position: Point(-10., 10., -10.),
        intensity: Color::white(),
    };

    let middle = Object::sphere()
        .set_transform(Transform::translation(-0.5, 1., 0.5))
        .set_color(Color(0.1, 1., 0.5))
        .set_diffuse(0.7)
        .set_reflective(0.5)
        .set_specular(0.3);

    let right = Object::sphere()
        .set_transform(Transform::translation(1.5, 0.5, -0.5) * Transform::scaling(0.5, 0.5, 0.5))
        .set_color(Color(0.5, 1., 0.1))
        .set_diffuse(0.7)
        .set_reflective(0.1)
        .set_specular(0.3);

    let left = Object::sphere()
        .set_transform(
            Transform::translation(-1.5, 0.33, -0.75) * Transform::scaling(0.33, 0.33, 0.33),
        )
        .set_color(Color(1., 0.8, 0.1))
        .set_diffuse(0.7)
        .set_reflective(0.3)
        .set_specular(0.3);

    let world = World {
        lights: vec![light_source],
        objects: vec![floor, middle, left, right],
    };

    let camera = Camera::new(
        1000,
        500,
        PI / 3.,
        Some(Transform::view_transform(
            Point(0., 1.5, -5.),
            Point(0., 1., 0.),
            Vector(0., 1., 0.),
        )),
    );

    let canvas = camera.render(&world);
    canvas.save("planes_and_three_sheres.png").unwrap();
}

pub fn floor_with_3_spheres_and_wall() {
    let floor = Object::plane()
        .set_color(Color(1., 0.9, 0.9))
        .set_specular(0.);

    let wall = Object::plane()
        .set_color(Color::blue())
        .set_specular(0.)
        .set_transform(Transform::translation(0., 0., 10.) * Transform::rotation_x(PI / 2.));

    let light_source = PointLight {
        position: Point(-10., 10., -10.),
        intensity: Color::white(),
    };

    let middle = Object::sphere()
        .set_transform(Transform::translation(-0.5, 1., 0.5))
        .set_color(Color(0.1, 1., 0.5))
        .set_diffuse(0.7)
        .set_specular(0.3);

    let right = Object::sphere()
        .set_transform(Transform::translation(1.5, 0.5, -0.5) * Transform::scaling(0.5, 0.5, 0.5))
        .set_color(Color(0.5, 1., 0.1))
        .set_diffuse(0.7)
        .set_specular(0.3);

    let left = Object::sphere()
        .set_transform(
            Transform::translation(-1.5, 0.33, -0.75) * Transform::scaling(0.33, 0.33, 0.33),
        )
        .set_color(Color(1., 0.8, 0.1))
        .set_diffuse(0.7)
        .set_specular(0.3);

    let world = World {
        lights: vec![light_source],
        objects: vec![floor, wall, middle, left, right],
    };

    let camera = Camera::new(
        1000,
        500,
        PI / 3.,
        Some(Transform::view_transform(
            Point(0., 1.5, -5.),
            Point(0., 1., 0.),
            Vector(0., 1., 0.),
        )),
    );

    let canvas = camera.render(&world);
    canvas.save("planes_and_three_sheres_and_wall.png").unwrap();
}

pub fn floor_with_pattern() {
    let floor = Object::plane()
        .set_specular(0.)
        .set_pattern(Pattern::stripe_pattern(Color::white(), Color::black()));

    let light_source = PointLight {
        position: Point(-10., 10., -10.),
        intensity: Color::white(),
    };

    let middle = Object::sphere()
        .set_transform(Transform::translation(-0.5, 1., 0.5))
        .set_pattern(Pattern::stripe_pattern(
            Color(0.1, 1., 0.5),
            Color(1., 0.1, 0.5),
        ))
        .set_diffuse(0.7)
        .set_specular(0.3);

    let right = Object::sphere()
        .set_transform(Transform::translation(1.5, 0.5, -0.5) * Transform::scaling(0.5, 0.5, 0.5))
        .set_color(Color(0.5, 1., 0.1))
        .set_diffuse(0.7)
        .set_specular(0.3);

    let left = Object::sphere()
        .set_transform(
            Transform::translation(-1.5, 0.33, -0.75) * Transform::scaling(0.33, 0.33, 0.33),
        )
        .set_color(Color(1., 0.8, 0.1))
        .set_diffuse(0.7)
        .set_specular(0.3);

    let world = World {
        lights: vec![light_source],
        objects: vec![floor, middle, left, right],
    };

    let camera = Camera::new(
        1000,
        500,
        PI / 3.,
        Some(Transform::view_transform(
            Point(0., 1.5, -5.),
            Point(0., 1., 0.),
            Vector(0., 1., 0.),
        )),
    );

    let canvas = camera.render(&world);
    canvas.save("floor_with_pattern.png").unwrap();
}

pub fn checkered_plane() {
    let floor = Object::plane()
        .set_specular(0.)
        .set_pattern(Pattern::checkers_pattern(Color::white(), Color::black()));

    let light_source = PointLight {
        position: Point(-10., 10., -10.),
        intensity: Color::white(),
    };
    let world = World {
        lights: vec![light_source],
        objects: vec![floor],
    };

    let camera = Camera::new(
        1000,
        500,
        PI / 3.,
        Some(Transform::view_transform(
            Point(0., 1.5, -5.),
            Point(0., 1., 0.),
            Vector(0., 1., 0.),
        )),
    );

    let canvas = camera.render(&world);
    canvas.save("checkered_plane.png").unwrap();
}
pub fn checkered_sphere() {
    let pattern = Pattern::checkers_pattern(Color::white(), Color::black())
        .set_transform(Transform::scaling(0.25, 0.25, 0.25));
    let sphere = Object::sphere()
        .set_transform(Transform::translation(-0.5, 1., 0.5) * Transform::rotation_y(PI / 4.))
        .set_pattern(pattern);

    let light_source = PointLight {
        position: Point(-10., 10., -10.),
        intensity: Color::white(),
    };
    let world = World {
        lights: vec![light_source],
        objects: vec![sphere],
    };

    let camera = Camera::new(
        1000,
        500,
        PI / 3.,
        Some(Transform::view_transform(
            Point(0., 1.5, -5.),
            Point(0., 1., 0.),
            Vector(0., 1., 0.),
        )),
    );

    let canvas = camera.render(&world);
    canvas.save("checkered_sphere.png").unwrap();
}

pub fn gradient_plane() {
    let floor = Object::plane()
        .set_specular(0.)
        .set_pattern(Pattern::gradient_pattern(Color::white(), Color::black()));

    let light_source = PointLight {
        position: Point(-10., 10., -10.),
        intensity: Color::white(),
    };
    let world = World {
        lights: vec![light_source],
        objects: vec![floor],
    };

    let camera = Camera::new(
        1000,
        500,
        PI / 3.,
        Some(Transform::view_transform(
            Point(0., 1.5, -5.),
            Point(0., 1., 0.),
            Vector(0., 1., 0.),
        )),
    );

    let canvas = camera.render(&world);
    canvas.save("gradient_plane.png").unwrap();
}
pub fn gradient_sphere() {
    let pattern = Pattern::gradient_pattern(Color::white(), Color::black());
    let sphere = Object::sphere()
        .set_transform(Transform::translation(-0.5, 1., 0.5))
        .set_pattern(pattern);

    let light_source = PointLight {
        position: Point(-10., 10., -10.),
        intensity: Color::white(),
    };
    let world = World {
        lights: vec![light_source],
        objects: vec![sphere],
    };

    let camera = Camera::new(
        1000,
        500,
        PI / 3.,
        Some(Transform::view_transform(
            Point(0., 1.5, -5.),
            Point(0., 1., 0.),
            Vector(0., 1., 0.),
        )),
    );

    let canvas = camera.render(&world);
    canvas.save("gradient_sphere.png").unwrap();
}
pub fn ring_plane() {
    let floor = Object::plane()
        .set_specular(0.)
        .set_pattern(Pattern::ring_pattern(Color::white(), Color::black()));

    let light_source = PointLight {
        position: Point(-10., 10., -10.),
        intensity: Color::white(),
    };
    let world = World {
        lights: vec![light_source],
        objects: vec![floor],
    };

    let camera = Camera::new(
        1000,
        500,
        PI / 3.,
        Some(Transform::view_transform(
            Point(0., 1.5, -5.),
            Point(0., 1., 0.),
            Vector(0., 1., 0.),
        )),
    );

    let canvas = camera.render(&world);
    canvas.save("ring_plane.png").unwrap();
}
pub fn ring_sphere() {
    let pattern = Pattern::ring_pattern(Color::white(), Color::black());
    let sphere = Object::sphere()
        .set_transform(Transform::translation(-0.5, 1., 0.5))
        .set_pattern(pattern);

    let light_source = PointLight {
        position: Point(-10., 10., -10.),
        intensity: Color::white(),
    };
    let world = World {
        lights: vec![light_source],
        objects: vec![sphere],
    };

    let camera = Camera::new(
        1000,
        500,
        PI / 3.,
        Some(Transform::view_transform(
            Point(0., 1.5, -5.),
            Point(0., 1., 0.),
            Vector(0., 1., 0.),
        )),
    );

    let canvas = camera.render(&world);
    canvas.save("ring_sphere.png").unwrap();
}
