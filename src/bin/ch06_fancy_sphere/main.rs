use std::fs;

use the_ray_tracer_challenge::{
    geometry::{Ray, Sphere},
    graphics::{Canvas, Color},
    image,
    math::{Matrix4x4f, Point3f, Vector3f},
    shading::{lighting, LightingArgs, Material, PointLight},
};

const PER_DRAWING_SIZE: usize = 100;

fn draw_scenario(
    canvas: &mut Canvas,
    canvas_start_x: usize,
    canvas_start_y: usize,
    wall_color: Color,
    light: PointLight,
    sphere: Sphere,
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

            let ray = Ray::new(camera, (dest - camera).normalize());
            canvas.write_px(
                canvas_start_x + x,
                canvas_start_y + y,
                ray.intersect_sphere(&sphere)
                    .hit()
                    .map(|hit| {
                        let point = ray.position(hit.t());
                        let normal = sphere.normal_at(&point);
                        let eye = -ray.get_direction();
                        lighting(LightingArgs {
                            material: sphere.get_material(),
                            light,
                            point,
                            eyev: eye,
                            normalv: normal,
                        })
                    })
                    .unwrap_or(wall_color),
            );
        });
    });
}

fn main() {
    let mut canvas = Canvas::new(PER_DRAWING_SIZE * 3, PER_DRAWING_SIZE * 2);

    let wall_colors = [Color::new(0.7, 0.7, 0.7), Color::new(0.8, 0.8, 0.8)];

    fn material_with_color(color: Color) -> Material {
        let mut material = Material::default();
        material.color = color;
        material
    }

    draw_scenario(
        &mut canvas,
        0,
        0,
        wall_colors[0],
        PointLight::new(Point3f::new(-10.0, 10.0, -10.0), Color::WHITE),
        Sphere::new(
            Matrix4x4f::identity(),
            material_with_color(Color::new(1.0, 0.2, 1.0)),
        ),
    );

    draw_scenario(
        &mut canvas,
        PER_DRAWING_SIZE,
        0,
        wall_colors[1],
        PointLight::new(Point3f::new(-10.0, 10.0, -10.0), Color::WHITE),
        Sphere::new(
            Matrix4x4f::scaling(Vector3f::new(1.0, 0.5, 1.0)),
            material_with_color(Color::new(0.2, 0.2, 1.0)),
        ),
    );

    draw_scenario(
        &mut canvas,
        PER_DRAWING_SIZE * 2,
        0,
        wall_colors[0],
        PointLight::new(Point3f::new(-10.0, 10.0, -10.0), Color::new(1.0, 0.2, 0.4)),
        Sphere::new(
            Matrix4x4f::scaling(Vector3f::new(0.5, 1.0, 1.0)),
            material_with_color(Color::new(0.2, 1.0, 0.2)),
        ),
    );

    draw_scenario(
        &mut canvas,
        0,
        PER_DRAWING_SIZE,
        wall_colors[1],
        PointLight::new(Point3f::new(-10.0, 10.0, -10.0), Color::WHITE),
        Sphere::new(
            Matrix4x4f::identity()
                .scale(Vector3f::new(0.5, 1.0, 1.0))
                .rotate_z(std::f64::consts::PI / 4.0),
            Material {
                color: Color::new(1.0, 1.0, 0.2),
                specular: 0.3,
                shininess: 50.0,
                ..Default::default()
            },
        ),
    );

    draw_scenario(
        &mut canvas,
        PER_DRAWING_SIZE,
        PER_DRAWING_SIZE,
        wall_colors[1],
        PointLight::new(Point3f::new(-10.0, 10.0, -10.0), Color::WHITE),
        Sphere::new(
            Matrix4x4f::identity()
                .scale(Vector3f::new(0.5, 1.0, 1.0))
                .rotate_z(std::f64::consts::PI / 4.0),
            Material {
                color: Color::new(1.0, 1.0, 0.2),
                shininess: 50.0,
                ..Default::default()
            },
        ),
    );

    draw_scenario(
        &mut canvas,
        PER_DRAWING_SIZE * 2,
        PER_DRAWING_SIZE,
        wall_colors[0],
        PointLight::new(Point3f::new(-10.0, 10.0, -10.0), Color::WHITE),
        Sphere::new(
            Matrix4x4f::identity()
                .scale(Vector3f::new(0.5, 1.0, 1.0))
                .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            Material {
                color: Color::new(0.2, 1.0, 1.0),
                ambient: 0.6,
                diffuse: 0.4,
                ..Default::default()
            },
        ),
    );

    let ppm = image::canvas_to_ppm(&canvas);
    fs::write("./ch06_fancy_sphere.ppm", ppm).expect("Cannot write image to file!");
}
