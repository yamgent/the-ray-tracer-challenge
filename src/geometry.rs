use crate::math::{Point3f, Vector3f};

pub struct Ray {
    origin: Point3f,
    direction: Vector3f,
}

impl Ray {
    pub fn new(origin: Point3f, direction: Vector3f) -> Self {
        Self { origin, direction }
    }

    pub fn get_origin(&self) -> Point3f {
        self.origin
    }

    pub fn get_direction(&self) -> Vector3f {
        self.direction
    }

    pub fn position(&self, t: f64) -> Point3f {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_new() {
        let r = Ray::new(Point3f::new(1.0, 2.0, 3.0), Vector3f::new(4.0, 5.0, 6.0));

        assert_eq!(r.get_origin(), Point3f::new(1.0, 2.0, 3.0));
        assert_eq!(r.get_direction(), Vector3f::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_ray_position() {
        let r = Ray::new(Point3f::new(2.0, 3.0, 4.0), Vector3f::new(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Point3f::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point3f::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point3f::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point3f::new(4.5, 3.0, 4.0));
    }
}
