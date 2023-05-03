use std::fs;

use the_ray_tracer_challenge::{
    graphics::{Canvas, Color},
    image,
    math::{Point3f, Vector3f},
};

#[derive(Debug)]
struct Projectile {
    position: Point3f,
    velocity: Vector3f,
}

impl Projectile {
    fn new(position: Point3f, velocity: Vector3f) -> Self {
        Self { position, velocity }
    }
}

#[derive(Debug)]
struct Environment {
    gravity: Vector3f,
    wind: Vector3f,
}

impl Environment {
    fn new(gravity: Vector3f, wind: Vector3f) -> Self {
        Self { gravity, wind }
    }
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    Projectile::new(
        proj.position + proj.velocity,
        proj.velocity + env.gravity + env.wind,
    )
}

fn main() {
    let start = Point3f::new(0.0, 1.0, 0.0);
    let velocity = Vector3f::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut p = Projectile::new(start, velocity);

    let gravity = Vector3f::new(0.0, -0.1, 0.0);
    let wind = Vector3f::new(-0.01, 0.0, 0.0);
    let e = Environment::new(gravity, wind);

    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(1.0, 0.0, 0.0);

    // range: (0 883.2461462246473 1 489.492666280364)
    while p.position.y() > 0.0 {
        canvas.write_px(
            p.position.x().round() as usize,
            canvas.h() - p.position.y().round() as usize,
            color,
        );
        p = tick(&e, &p);
    }

    let ppm = image::canvas_to_ppm(&canvas);
    fs::write("./ch02_cannon_pic.ppm", ppm).expect("Cannot write image to file!");
}
