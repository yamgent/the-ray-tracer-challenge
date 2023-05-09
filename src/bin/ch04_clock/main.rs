use std::{f64::consts::PI, fs};
use the_ray_tracer_challenge::{
    graphics::{Canvas, Color},
    image,
    math::{Matrix4x4f, Point3f, Vector3f},
};

fn main() {
    let mut canvas = Canvas::new(512, 512);
    let color = Color::new(1.0, 1.0, 1.0);

    (0..12).for_each(|i| {
        let m = Matrix4x4f::identity()
            .translate(Vector3f::new(0.0, 1.0, 0.0))
            .rotate_z((-PI / 6.0) * i as f64)
            .scale(Vector3f::new(128.0, 128.0, 1.0))
            .translate(Vector3f::new(256.0, 256.0, 0.0));
        let p = m * Point3f::new(0.0, 0.0, 0.0);

        canvas.write_px(p.x().round() as usize, p.y().round() as usize, color);
    });

    let ppm = image::canvas_to_ppm(&canvas);
    fs::write("./ch04_clock.ppm", ppm).expect("Cannot write image to file!");
}
