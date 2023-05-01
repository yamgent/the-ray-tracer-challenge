use the_ray_tracer_challenge::math::{Point3f, Vector3f};

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
    let mut p = Projectile::new(
        Point3f::new(0.0, 1.0, 0.0),
        Vector3f::new(1.0, 1.0, 0.0).normalize(),
    );
    let e = Environment::new(
        Vector3f::new(0.0, -0.1, 0.0),
        Vector3f::new(-0.01, 0.0, 0.0),
    );
    let mut ticks = 0;

    while p.position.y() > 0.0 {
        println!("{:?}", p);
        p = tick(&e, &p);
        ticks += 1;
    }

    println!("{:?}", p);
    println!("Ticks: {}", ticks);
}
