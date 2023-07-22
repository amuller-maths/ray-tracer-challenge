use std::f64::consts::PI;

use ray_tracer_challenge::{
    camera::Camera,
    canvas::Color,
    geometry::{Point, Vector},
    light::PointLight,
    object::Object,
    transform::Transform,
    world::World,
};

fn main() {
    let mut floor = Object::sphere();
    floor.set_transform(Transform::scaling(10., 0.01, 10.));
    floor.material.color = Color(1., 0.9, 0.9);
    floor.material.specular = 0.;

    let mut left_wall = Object::sphere();
    left_wall.set_transform(
        Transform::translation(0., 0., 5.)
            * Transform::rotation_y(-PI / 4.)
            * Transform::rotation_x(PI / 2.)
            * Transform::scaling(10., 0.01, 10.),
    );
    left_wall.material = floor.material;

    let mut right_wall = Object::sphere();
    right_wall.set_transform(
        Transform::translation(0., 0., 5.)
            * Transform::rotation_y(PI / 4.)
            * Transform::rotation_x(PI / 2.)
            * Transform::scaling(10., 0.01, 10.),
    );
    left_wall.material = floor.material;

    let mut middle = Object::sphere();
    middle.set_transform(Transform::translation(-0.5, 1., 0.5));
    middle.material.color = Color(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Object::sphere();
    right.set_transform(Transform::translation(1.5, 0.5, -0.5) * Transform::scaling(0.5, 0.5, 0.5));
    right.material.color = Color(0.5, 1., 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Object::sphere();
    left.set_transform(
        Transform::translation(-1.5, 0.33, -0.75) * Transform::scaling(0.33, 0.33, 0.33),
    );
    // left.set_transform(
    //     Transform::translation(-1.5, 2., 0.5) * Transform::scaling(0.33, 0.33, 0.33),
    // );
    left.material.color = Color(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let light_source = PointLight {
        position: Point(-10., 10., -10.),
        intensity: Color(1., 1., 1.),
    };

    let light_source2 = PointLight {
        position: Point(0., 10., -10.),
        intensity: Color(1., 1., 1.),
    };

    let world = World {
        lights: vec![light_source, light_source2],
        objects: vec![floor, left_wall, right_wall, middle, right, left],
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
    canvas.save("first_scene_with_shadows.png").unwrap();

    // let img_size = 500;
    // let origin = Point(0., 0., -5.);
    // let wall_z = 10f64;
    // let wall_size = 7f64;
    // let mut canvas = Canvas::new(img_size, img_size, None);
    // let pixel_size = wall_size / (img_size as f64);
    // let half = wall_size / 2.;
    // // let red = Color(1., 0., 0.);
    // let mut object = Object::sphere();
    // object.set_transform(Transform::rotation_x(PI / 4.));
    // object.material.color = Color(1., 1., 1.);
    // let light = PointLight {
    //     position: Point(5., 10., -10.),
    //     intensity: Color(1., 1., 1.),
    // };
    // for y in 0..img_size {
    //     let world_y = half - pixel_size * (y as f64);
    //     for x in 0..img_size {
    //         let world_x = -half + pixel_size * (x as f64);
    //         let position = Point(world_x, world_y, wall_z);
    //         let r = Ray {
    //             origin,
    //             direction: (position - origin).normalize(),
    //         };
    //         let mut xs = object.intersect(r);
    //         if let Some(hit) = xs.hit() {
    //             let point = r.position(hit.t);
    //             let normal = hit.object.normal_at(point);
    //             let eye = -r.direction;
    //             let color = hit.object.material.lighting(light, point, eye, normal);
    //             canvas.write_pixel(x, y, color);
    //         }
    //     }
    // }
    // canvas.save("sphere_light.png").unwrap();
}
