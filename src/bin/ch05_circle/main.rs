use std::fs;

use the_ray_tracer_challenge::{
    geometry::{Ray, Sphere},
    graphics::{Canvas, Color},
    image,
    math::Point3f,
};

const PER_DRAWING_SIZE: usize = 100;

fn draw_scenario(
    canvas: &mut Canvas,
    canvas_start_x: usize,
    canvas_start_y: usize,
    sphere: Sphere,
    color: Color,
) {
    let wall_size = 7.0;
    let wall_z = 10.0;
    let px_size = wall_size / PER_DRAWING_SIZE as f64;
    let half = wall_size / 2.0;
    let camera = Point3f::new(0.0, 0.0, -5.0);

    (0..PER_DRAWING_SIZE).for_each(|y| {
        let d_y = -(-half + px_size * y as f64);

        (0..PER_DRAWING_SIZE).for_each(|x| {
            let d_x = -half + px_size * x as f64;

            let dest = Point3f::new(d_x, d_y, wall_z);

            let ray = Ray::new(camera, dest - camera);
            if ray.intersect_sphere(&sphere).hit().is_some() {
                canvas.write_px(canvas_start_x + x, canvas_start_y + y, color);
            }
        });
    });
}

fn main() {
    let mut canvas = Canvas::new(PER_DRAWING_SIZE, PER_DRAWING_SIZE);

    draw_scenario(
        &mut canvas,
        0,
        0,
        Sphere::default(),
        Color::new(1.0, 0.0, 0.0),
    );

    let ppm = image::canvas_to_ppm(&canvas);
    fs::write("./ch05_circle.ppm", ppm).expect("Cannot write image to file!");
}
