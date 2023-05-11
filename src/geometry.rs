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

pub struct Sphere {
    origin: Point3f,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            origin: Point3f::new(0.0, 0.0, 0.0),
        }
    }
}

pub fn intersect_ray_sphere(ray: &Ray, sphere: &Sphere) -> Vec<f64> {
    let sphere_to_ray = ray.origin - sphere.origin;

    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = (b * b) - (4.0 * a * c);

    if discriminant < 0.0 {
        vec![]
    } else {
        vec![
            (-b - discriminant.sqrt()) / (2.0 * a),
            (-b + discriminant.sqrt()) / (2.0 * a),
        ]
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

    #[test]
    fn test_sphere_intersect() {
        [
            ((0.0, 0.0, -5.0), vec![4.0, 6.0]),
            ((0.0, 1.0, -5.0), vec![5.0, 5.0]),
            ((0.0, 2.0, -5.0), vec![]),
            ((0.0, 0.0, 0.0), vec![-1.0, 1.0]),
            ((0.0, 0.0, 5.0), vec![-6.0, -4.0]),
        ]
        .into_iter()
        .for_each(|(starting_point, expected)| {
            let r = Ray::new(
                Point3f::new(starting_point.0, starting_point.1, starting_point.2),
                Vector3f::new(0.0, 0.0, 1.0),
            );
            let s = Sphere::new();

            let result = intersect_ray_sphere(&r, &s);
            assert_eq!(result, expected);
        });
    }
}
