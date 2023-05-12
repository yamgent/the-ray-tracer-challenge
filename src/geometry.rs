use crate::math::{Matrix4x4f, Point3f, Vector3f};

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
        let sphere_to_ray = self.origin - Point3f::new(0.0, 0.0, 0.0);

        let a = self.direction.dot(&self.direction);
        let b = 2.0 * self.direction.dot(&sphere_to_ray);
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

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Sphere;

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

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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
            let s = Sphere;

            let result = r.intersect_sphere(&s);
            assert_eq!(result.iter().map(|x| x.t).collect::<Vec<_>>(), expected);
            assert!(result.iter().all(|x| match x.get_object() {
                IntersectionObject::Sphere(sphere) => std::ptr::eq(*sphere, &s),
            }));
        });
    }

    #[test]
    fn test_intersection_new() {
        let s = Sphere;
        let i = Intersection::new(3.5, IntersectionObject::Sphere(&s));

        assert_eq!(i.t(), 3.5);
        match i.get_object() {
            IntersectionObject::Sphere(sphere) => assert!(std::ptr::eq(*sphere, &s)),
        }
    }

    #[test]
    fn test_intersections_new() {
        let s = Sphere;
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
            let s = Sphere;
            let i1 = Intersection::new(1.0, IntersectionObject::Sphere(&s));
            let i2 = Intersection::new(2.0, IntersectionObject::Sphere(&s));
            let xs = Intersections::new(vec![i1, i2]);
            assert_eq!(xs.hit(), Some(&i1));
        }
        {
            let s = Sphere;
            let i1 = Intersection::new(-1.0, IntersectionObject::Sphere(&s));
            let i2 = Intersection::new(1.0, IntersectionObject::Sphere(&s));
            let xs = Intersections::new(vec![i1, i2]);
            assert_eq!(xs.hit(), Some(&i2));
        }
        {
            let s = Sphere;
            let i1 = Intersection::new(-2.0, IntersectionObject::Sphere(&s));
            let i2 = Intersection::new(-1.0, IntersectionObject::Sphere(&s));
            let xs = Intersections::new(vec![i1, i2]);
            assert_eq!(xs.hit(), None);
        }
        {
            let s = Sphere;
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
}
