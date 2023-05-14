use crate::{
    graphics::Color,
    math::{Point3f, Vector3f},
};

#[derive(Copy, Clone)]
pub struct PointLight {
    pub position: Point3f,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point3f, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

// phong shading material
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    // best range = 10 (large) to 200 (small)
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

pub struct LightingArgs {
    pub material: Material,
    pub light: PointLight,
    pub point: Point3f,
    pub eyev: Vector3f,
    pub normalv: Vector3f,
}

pub fn lighting(args: LightingArgs) -> Color {
    let effective_color = args.material.color * args.light.intensity;
    let lightv = (args.light.position - args.point).normalize();
    let ambient = effective_color * args.material.ambient;

    let light_dot_normal = lightv.dot(&args.normalv);

    let (diffuse, specular) = if light_dot_normal < 0.0 {
        // light is on the other side
        (Color::BLACK, Color::BLACK)
    } else {
        let diffuse = effective_color * args.material.diffuse * light_dot_normal;
        let reflectv = (-lightv).reflect(&args.normalv);
        let reflect_dot_eye = reflectv.dot(&args.eyev);

        let specular = if reflect_dot_eye <= 0.0 {
            Color::BLACK
        } else {
            let factor = reflect_dot_eye.powf(args.material.shininess);
            args.light.intensity * args.material.specular * factor
        };

        (diffuse, specular)
    };

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pointlight_new() {
        let light = PointLight::new(Point3f::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));
        assert_eq!(light.position, Point3f::new(0.0, 0.0, 0.0));
        assert_eq!(light.intensity, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_material_default() {
        assert_eq!(
            Material::default(),
            Material::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0)
        );
    }

    #[test]
    fn test_lighting() {
        let material = Material::default();
        let position = Point3f::new(0.0, 0.0, 0.0);
        let normalv = Vector3f::new(0.0, 0.0, -1.0);

        assert_eq!(
            lighting(LightingArgs {
                eyev: Vector3f::new(0.0, 0.0, -1.0),
                normalv,
                light: PointLight::new(Point3f::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0)),
                material,
                point: position,
            }),
            Color::new(1.9, 1.9, 1.9)
        );

        assert_eq!(
            lighting(LightingArgs {
                eyev: Vector3f::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0),
                normalv,
                light: PointLight::new(Point3f::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0)),
                material,
                point: position,
            }),
            Color::new(1.0, 1.0, 1.0)
        );

        assert_eq!(
            lighting(LightingArgs {
                eyev: Vector3f::new(0.0, 0.0, -1.0),
                normalv,
                light: PointLight::new(Point3f::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0)),
                material,
                point: position,
            }),
            Color::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
        );

        assert_eq!(
            lighting(LightingArgs {
                eyev: Vector3f::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0),
                normalv,
                light: PointLight::new(Point3f::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0)),
                material,
                point: position,
            }),
            Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
        );

        assert_eq!(
            lighting(LightingArgs {
                eyev: Vector3f::new(0.0, 0.0, -1.0),
                normalv,
                light: PointLight::new(Point3f::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0)),
                material,
                point: position,
            }),
            Color::new(0.1, 0.1, 0.1)
        );
    }
}
