use crate::{graphics::Color, math::Point3f};

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
}
