use crate::{
    math::{Matrix4x4f, Point3f, Vector3f, Vector4f},
    shading::Material,
};

#[derive(PartialEq, Debug)]
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

    pub fn intersect_sphere<'a>(&self, sphere: &'a Sphere) -> Intersections<'a> {
        let transformed_ray = self.transform(&sphere.transform.inverse().unwrap());
        let sphere_to_ray = transformed_ray.origin - Point3f::new(0.0, 0.0, 0.0);

        let a = transformed_ray.direction.dot(&transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            Intersections::new_empty()
        } else {
            let first = (-b - discriminant.sqrt()) / (2.0 * a);
            let second = (-b + discriminant.sqrt()) / (2.0 * a);

            Intersections::new(vec![
                Intersection::new(first, IntersectionObject::Sphere(sphere)),
                Intersection::new(second, IntersectionObject::Sphere(sphere)),
            ])
        }
    }

    pub fn transform(&self, matrix: &Matrix4x4f) -> Self {
        Self {
            origin: *matrix * self.origin,
            direction: *matrix * self.direction,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere {
    transform: Matrix4x4f,
    material: Material,
}

impl Sphere {
    pub fn new(transform: Matrix4x4f, material: Material) -> Self {
        Self {
            transform,
            material,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4x4f) {
        self.transform = transform;
    }

    pub fn normal_at(&self, world_point: &Point3f) -> Vector3f {
        let world_point: Vector4f = (*world_point).into();
        let object_origin: Vector4f = (Point3f::new(0.0, 0.0, 0.0)).into();

        let object_point = self.transform.inverse().unwrap() * world_point;
        let object_normal = object_point - object_origin;
        let world_normal = self.transform.inverse().unwrap().transpose() * object_normal;
        // hack, see page 82. Techincally we should remove all manipulation of w in the transposed
        // inversed matrix, but we can also just reset w to 0 (i.e. make it a vector)
        let world_normal = Vector3f::new(world_normal.x(), world_normal.y(), world_normal.z());
        world_normal.normalize()
    }

    pub fn get_material(&self) -> Material {
        self.material
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transform: Matrix4x4f::identity(),
            material: Material::default(),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Intersection<'a> {
    t: f64,
    object: IntersectionObject<'a>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: IntersectionObject<'a>) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn get_object(&self) -> &IntersectionObject<'a> {
        &self.object
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum IntersectionObject<'a> {
    Sphere(&'a Sphere),
}

pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

fn sort_intersections<'a>(intersections: &mut Vec<Intersection<'a>>) {
    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
}

// TODO: We don't know how this data structure will be used in the future. Right now,
// the assumption is that the list will be small and not modified many times, hence we
// just keep a sorted list at all times, and re-sort if the list is modified. However,
// in the future, it may make sense to only sort on demand instead if list can be big, or is
// frequently modified!
impl<'a> Intersections<'a> {
    pub fn new(mut intersections: Vec<Intersection<'a>>) -> Self {
        sort_intersections(&mut intersections);
        Self { intersections }
    }

    pub fn iter(&self) -> std::slice::Iter<Intersection<'a>> {
        self.intersections.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn new_empty() -> Self {
        Self {
            intersections: vec![],
        }
    }

    pub fn hit(&self) -> Option<&Intersection<'a>> {
        // assumption is that list is already sorted
        self.intersections.iter().find(|x| x.t >= 0.0)
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
            let s = Sphere::default();

            let result = r.intersect_sphere(&s);
            assert_eq!(result.iter().map(|x| x.t).collect::<Vec<_>>(), expected);
            assert!(result.iter().all(|x| match x.get_object() {
                IntersectionObject::Sphere(sphere) => std::ptr::eq(*sphere, &s),
            }));
        });
    }

    #[test]
    fn test_intersection_new() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, IntersectionObject::Sphere(&s));

        assert_eq!(i.t(), 3.5);
        match i.get_object() {
            IntersectionObject::Sphere(sphere) => assert!(std::ptr::eq(*sphere, &s)),
        }
    }

    #[test]
    fn test_intersections_new() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, IntersectionObject::Sphere(&s));
        let i2 = Intersection::new(2.0, IntersectionObject::Sphere(&s));

        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<_>>(), vec![1.0, 2.0]);
        assert!(xs.iter().all(|x| match x.get_object() {
            IntersectionObject::Sphere(sphere) => std::ptr::eq(*sphere, &s),
        }));
    }

    #[test]
    fn test_intersections_hit() {
        {
            let s = Sphere::default();
            let i1 = Intersection::new(1.0, IntersectionObject::Sphere(&s));
            let i2 = Intersection::new(2.0, IntersectionObject::Sphere(&s));
            let xs = Intersections::new(vec![i1, i2]);
            assert_eq!(xs.hit(), Some(&i1));
        }
        {
            let s = Sphere::default();
            let i1 = Intersection::new(-1.0, IntersectionObject::Sphere(&s));
            let i2 = Intersection::new(1.0, IntersectionObject::Sphere(&s));
            let xs = Intersections::new(vec![i1, i2]);
            assert_eq!(xs.hit(), Some(&i2));
        }
        {
            let s = Sphere::default();
            let i1 = Intersection::new(-2.0, IntersectionObject::Sphere(&s));
            let i2 = Intersection::new(-1.0, IntersectionObject::Sphere(&s));
            let xs = Intersections::new(vec![i1, i2]);
            assert_eq!(xs.hit(), None);
        }
        {
            let s = Sphere::default();
            let i1 = Intersection::new(5.0, IntersectionObject::Sphere(&s));
            let i2 = Intersection::new(7.0, IntersectionObject::Sphere(&s));
            let i3 = Intersection::new(-3.0, IntersectionObject::Sphere(&s));
            let i4 = Intersection::new(2.0, IntersectionObject::Sphere(&s));
            let xs = Intersections::new(vec![i1, i2, i3, i4]);
            assert_eq!(xs.hit(), Some(&i4));
        }
    }

    #[test]
    fn test_ray_transform() {
        let r = Ray::new(Point3f::new(1.0, 2.0, 3.0), Vector3f::new(0.0, 1.0, 0.0));
        let m = Matrix4x4f::translation(Vector3f::new(3.0, 4.0, 5.0));
        assert_eq!(
            r.transform(&m),
            Ray::new(Point3f::new(4.0, 6.0, 8.0), Vector3f::new(0.0, 1.0, 0.0))
        );
    }

    #[test]
    fn test_sphere_default() {
        assert_eq!(
            Sphere::default(),
            Sphere {
                transform: Matrix4x4f::identity(),
                material: Material::default()
            }
        );
    }

    #[test]
    fn test_sphere_transform() {
        let mut s = Sphere::default();
        let t = Matrix4x4f::translation(Vector3f::new(2.0, 3.0, 4.0));

        s.set_transform(t);
        assert_eq!(s.transform, t);
    }

    #[test]
    fn test_sphere_transformed_intersect() {
        {
            let r = Ray::new(Point3f::new(0.0, 0.0, -5.0), Vector3f::new(0.0, 0.0, 1.0));
            let mut s = Sphere::default();

            s.set_transform(Matrix4x4f::scaling(Vector3f::new(2.0, 2.0, 2.0)));
            let xs = r.intersect_sphere(&s);

            assert_eq!(xs.iter().map(|x| x.t).collect::<Vec<_>>(), vec![3.0, 7.0]);
        }

        {
            let r = Ray::new(Point3f::new(0.0, 0.0, -5.0), Vector3f::new(0.0, 0.0, 1.0));
            let mut s = Sphere::default();

            s.set_transform(Matrix4x4f::translation(Vector3f::new(5.0, 0.0, 0.0)));
            let xs = r.intersect_sphere(&s);

            assert!(xs.is_empty());
        }
    }

    #[test]
    fn test_sphere_basic_normal_at() {
        let s = Sphere::default();

        assert_eq!(
            s.normal_at(&Point3f::new(1.0, 0.0, 0.0)),
            Vector3f::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            s.normal_at(&Point3f::new(0.0, 1.0, 0.0)),
            Vector3f::new(0.0, 1.0, 0.0)
        );
        assert_eq!(
            s.normal_at(&Point3f::new(0.0, 0.0, 1.0)),
            Vector3f::new(0.0, 0.0, 1.0)
        );

        let val = 3_f64.sqrt() / 3.0;
        let n = s.normal_at(&Point3f::new(val, val, val));
        assert_eq!(n, Vector3f::new(val, val, val));
        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn test_sphere_advanced_normal_at() {
        {
            let s = Sphere::new(
                Matrix4x4f::translation(Vector3f::new(0.0, 1.0, 0.0)),
                Material::default(),
            );
            assert_eq!(
                s.normal_at(&Point3f::new(0.0, 1.70711, -0.70711)),
                Vector3f::new(0.0, 0.7071067811865475, -0.7071067811865476),
            );
        }
        {
            let s = Sphere::new(
                Matrix4x4f::identity()
                    .rotate_z(std::f64::consts::PI / 5.0)
                    .scale(Vector3f::new(1.0, 0.5, 1.0)),
                Material::default(),
            );
            assert_eq!(
                s.normal_at(&Point3f::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0)),
                Vector3f::new(0.0, 0.9701425001453319, -0.24253562503633294)
            );
        }
    }

    #[test]
    fn test_sphere_new_material() {
        let mut m = Material::default();
        m.ambient = 1.0;

        let s = Sphere::new(Matrix4x4f::identity(), m);
        assert_eq!(s.material, m);
    }
}
