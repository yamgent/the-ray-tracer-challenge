use the_ray_tracer_challenge::math::Vec4f;

#[derive(Debug)]
struct Projectile {
    position: Vec4f,
    velocity: Vec4f,
}

impl Projectile {
    fn new(position: Vec4f, velocity: Vec4f) -> Self {
        assert!(position.is_point());
        assert!(velocity.is_vector());

        Self { position, velocity }
    }
}

#[derive(Debug)]
struct Environment {
    gravity: Vec4f,
    wind: Vec4f,
}

impl Environment {
    fn new(gravity: Vec4f, wind: Vec4f) -> Self {
        assert!(gravity.is_vector());
        assert!(wind.is_vector());

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
        Vec4f::new_point(0.0, 1.0, 0.0),
        Vec4f::new_vector(1.0, 1.0, 0.0).normalize(),
    );
    let e = Environment::new(
        Vec4f::new_vector(0.0, -0.1, 0.0),
        Vec4f::new_vector(-0.01, 0.0, 0.0),
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
