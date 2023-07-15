use ray_tracer_challenge::canvas::{Canvas, Color};
use ray_tracer_challenge::geometry::{Point, Vector};
struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Projectile {
    fn draw(&self, canvas: &mut Canvas) {
        let Point(x, y, _) = self.position;
        let x = (x * 100.).round() as u32;
        let y = (y * 100.).round() as u32;
        canvas.write_pixel(y, x, Color(1., 1., 1.))
    }
}

fn tick(environment: &Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;
    Projectile { position, velocity }
}

fn main() {
    let position = Point(0., 1., 0.);
    let velocity = (Vector(1., 1.8, 0.) * 11.25).normalize();
    let mut p = Projectile { position, velocity };

    let gravity = Vector(0., -0.1, 0.);
    let wind = Vector(-0.01, 0., 0.);
    let e = Environment { gravity, wind };

    let mut c = Canvas::new(900, 550, None);

    while p.position.1 > 0. {
        p = tick(&e, p);
        p.draw(&mut c);
        println!("{:?}", p.position);
    }
    c.save("img.png").unwrap();
}
