use the_ray_tracer_challenge::math::Tuple4f;

#[derive(Debug)]
struct Projectile {
    position: Tuple4f,
    velocity: Tuple4f,
}

impl Projectile {
    fn new(position: Tuple4f, velocity: Tuple4f) -> Self {
        assert!(position.is_point());
        assert!(velocity.is_vector());

        Self { position, velocity }
    }
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple4f,
    wind: Tuple4f,
}

impl Environment {
    fn new(gravity: Tuple4f, wind: Tuple4f) -> Self {
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
        Tuple4f::new_point(0.0, 1.0, 0.0),
        Tuple4f::new_vector(1.0, 1.0, 0.0).normalize(),
    );
    let e = Environment::new(
        Tuple4f::new_vector(0.0, -0.1, 0.0),
        Tuple4f::new_vector(-0.01, 0.0, 0.0),
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
