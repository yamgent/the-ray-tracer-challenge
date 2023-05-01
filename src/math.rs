use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait FloatEq {
    fn float_eq(&self, other: &Self) -> bool;
}

impl FloatEq for f64 {
    fn float_eq(&self, other: &Self) -> bool {
        (self - other).abs() < std::f64::EPSILON
    }
}

pub fn assert_float_eq<T>(left: T, right: T)
where
    T: FloatEq + std::fmt::Debug,
{
    assert!(
        left.float_eq(&right),
        "left = {:?}, right = {:?}",
        left,
        right
    );
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector4f {
    vals: [f64; 4],
}

impl Vector4f {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { vals: [x, y, z, w] }
    }

    pub fn new_vector3_tuple(x: f64, y: f64, z: f64) -> Self {
        Self {
            vals: [x, y, z, 0.0],
        }
    }

    pub fn new_point3_tuple(x: f64, y: f64, z: f64) -> Self {
        Self {
            vals: [x, y, z, 1.0],
        }
    }

    pub fn is_vector3_tuple(&self) -> bool {
        self.vals[3].float_eq(&0.0)
    }

    pub fn is_point3_tuple(&self) -> bool {
        self.vals[3].float_eq(&1.0)
    }

    pub fn magnitude(&self) -> f64 {
        self.vals.iter().map(|a| a * a).sum::<f64>().sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.binary_scalar_op(|a| a / self.magnitude())
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.vals
            .iter()
            .zip(other.vals.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    pub fn x(&self) -> f64 {
        self.vals[0]
    }

    pub fn y(&self) -> f64 {
        self.vals[1]
    }

    pub fn z(&self) -> f64 {
        self.vals[2]
    }

    pub fn w(&self) -> f64 {
        self.vals[3]
    }

    fn unary_op<F>(&self, op: F) -> Self
    where
        F: Fn(&f64) -> f64,
    {
        Self {
            vals: [
                op(&self.vals[0]),
                op(&self.vals[1]),
                op(&self.vals[2]),
                op(&self.vals[3]),
            ],
        }
    }

    fn binary_op<F>(&self, other: &Self, op: F) -> Self
    where
        F: Fn(&f64, &f64) -> f64,
    {
        Self {
            vals: [
                op(&self.vals[0], &other.vals[0]),
                op(&self.vals[1], &other.vals[1]),
                op(&self.vals[2], &other.vals[2]),
                op(&self.vals[3], &other.vals[3]),
            ],
        }
    }

    fn binary_scalar_op<F>(&self, op: F) -> Self
    where
        F: Fn(&f64) -> f64,
    {
        Self {
            vals: [
                op(&self.vals[0]),
                op(&self.vals[1]),
                op(&self.vals[2]),
                op(&self.vals[3]),
            ],
        }
    }
}

impl FloatEq for Vector4f {
    fn float_eq(&self, other: &Self) -> bool {
        self.vals
            .iter()
            .zip(other.vals.iter())
            .all(|(a, b)| a.float_eq(b))
    }
}

impl Add for Vector4f {
    type Output = Vector4f;

    fn add(self, rhs: Self) -> Self::Output {
        self.binary_op(&rhs, |a, b| a + b)
    }
}

impl Sub for Vector4f {
    type Output = Vector4f;

    fn sub(self, rhs: Self) -> Self::Output {
        self.binary_op(&rhs, |a, b| a - b)
    }
}

impl Neg for Vector4f {
    type Output = Vector4f;

    fn neg(self) -> Self::Output {
        self.unary_op(|a| -a)
    }
}

impl Mul<f64> for Vector4f {
    type Output = Vector4f;

    fn mul(self, rhs: f64) -> Self::Output {
        self.binary_scalar_op(|a| a * rhs)
    }
}

impl Div<f64> for Vector4f {
    type Output = Vector4f;

    fn div(self, rhs: f64) -> Self::Output {
        self.binary_scalar_op(|a| a / rhs)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point3f(Vector4f);

impl Point3f {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector4f::new_point3_tuple(x, y, z).into()
    }

    pub fn x(&self) -> f64 {
        self.0.vals[0]
    }

    pub fn y(&self) -> f64 {
        self.0.vals[1]
    }

    pub fn z(&self) -> f64 {
        self.0.vals[2]
    }
}

impl From<Vector4f> for Point3f {
    fn from(value: Vector4f) -> Self {
        assert!(value.is_point3_tuple());
        Point3f(value)
    }
}

impl Add<Vector3f> for Point3f {
    type Output = Point3f;

    fn add(self, rhs: Vector3f) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl Sub<Point3f> for Point3f {
    type Output = Vector3f;

    fn sub(self, rhs: Point3f) -> Self::Output {
        (self.0 - rhs.0).into()
    }
}

impl Sub<Vector3f> for Point3f {
    type Output = Point3f;

    fn sub(self, rhs: Vector3f) -> Self::Output {
        (self.0 - rhs.0).into()
    }
}

impl FloatEq for Point3f {
    fn float_eq(&self, other: &Self) -> bool {
        self.0.float_eq(&other.0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3f(Vector4f);

impl Vector3f {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector4f::new_vector3_tuple(x, y, z).into()
    }

    pub fn magnitude(&self) -> f64 {
        self.0.magnitude()
    }

    pub fn normalize(&self) -> Self {
        self.0.normalize().into()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0.dot(&other.0)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.0.vals[1] * other.0.vals[2] - self.0.vals[2] * other.0.vals[1],
            self.0.vals[2] * other.0.vals[0] - self.0.vals[0] * other.0.vals[2],
            self.0.vals[0] * other.0.vals[1] - self.0.vals[1] * other.0.vals[0],
        )
    }

    pub fn x(&self) -> f64 {
        self.0.vals[0]
    }

    pub fn y(&self) -> f64 {
        self.0.vals[1]
    }

    pub fn z(&self) -> f64 {
        self.0.vals[2]
    }
}

impl From<Vector4f> for Vector3f {
    fn from(value: Vector4f) -> Self {
        assert!(value.is_vector3_tuple());
        Vector3f(value)
    }
}

impl Add<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn add(self, rhs: Vector3f) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl Add<Point3f> for Vector3f {
    type Output = Point3f;

    fn add(self, rhs: Point3f) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl Sub<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn sub(self, rhs: Vector3f) -> Self::Output {
        (self.0 - rhs.0).into()
    }
}

impl Neg for Vector3f {
    type Output = Vector3f;

    fn neg(self) -> Self::Output {
        (-self.0).into()
    }
}

impl Mul<f64> for Vector3f {
    type Output = Vector3f;

    fn mul(self, rhs: f64) -> Self::Output {
        (self.0 * rhs).into()
    }
}

impl Div<f64> for Vector3f {
    type Output = Vector3f;

    fn div(self, rhs: f64) -> Self::Output {
        (self.0 / rhs).into()
    }
}

impl FloatEq for Vector3f {
    fn float_eq(&self, other: &Self) -> bool {
        self.0.float_eq(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_float_eq() {
        assert_ne!(0.1_f64 + 0.2, 0.3);
        assert!((0.1 + 0.2).float_eq(&0.3));
    }

    #[test]
    fn test_vector4f_new() {
        assert_eq!(
            Vector4f::new(1.0, 2.0, 3.0, 4.0),
            Vector4f {
                vals: [1.0, 2.0, 3.0, 4.0],
            }
        );
    }

    #[test]
    fn test_vector4f_point_tuple() {
        let v = Vector4f::new(4.3, -4.2, 3.1, 1.0);
        assert!(v.is_point3_tuple());
        assert!(!v.is_vector3_tuple());
    }

    #[test]
    fn test_point3f_from() {
        let v = Vector4f::new(4.3, -4.2, 3.1, 1.0);
        let convert: Point3f = v.into();
        assert_eq!(convert.0, v);
        assert!(convert.0.is_point3_tuple());
        assert!(!convert.0.is_vector3_tuple());
    }

    #[test]
    fn test_vector4f_vector_tuple() {
        let v = Vector4f::new(4.3, -4.2, 3.1, 0.0);
        assert!(!v.is_point3_tuple());
        assert!(v.is_vector3_tuple());
    }

    #[test]
    fn test_vector3f_from() {
        let v = Vector4f::new(4.3, -4.2, 3.1, 0.0);
        let convert: Vector3f = v.into();
        assert_eq!(convert.0, v);
        assert!(!convert.0.is_point3_tuple());
        assert!(convert.0.is_vector3_tuple());
    }

    #[test]
    fn test_vector4f_new_point_tuple() {
        assert_eq!(
            Vector4f::new_point3_tuple(4.0, -4.0, 3.0),
            Vector4f {
                vals: [4.0, -4.0, 3.0, 1.0],
            }
        );
    }

    #[test]
    fn test_point3f_new() {
        assert_eq!(
            Point3f::new(4.0, -4.0, 3.0),
            Point3f(Vector4f {
                vals: [4.0, -4.0, 3.0, 1.0],
            })
        );
    }

    #[test]
    fn test_vector4f_new_vector_tuple() {
        assert_eq!(
            Vector4f::new_vector3_tuple(4.0, -4.0, 3.0),
            Vector4f {
                vals: [4.0, -4.0, 3.0, 0.0],
            }
        );
    }

    #[test]
    fn test_vector3f_new() {
        assert_eq!(
            Vector3f::new(4.0, -4.0, 3.0),
            Vector3f(Vector4f {
                vals: [4.0, -4.0, 3.0, 0.0],
            })
        );
    }

    #[test]
    fn test_vector4f_float_eq() {
        let other = Vector4f {
            vals: [1.0, 2.0, -3.0, 0.0],
        };

        assert!(Vector4f {
            vals: [1.0, 2.0, -3.0, 0.0],
        }
        .float_eq(&other));
        assert!(Vector4f {
            vals: [1.0, 2.0, -3.0, -0.0],
        }
        .float_eq(&other));

        assert!(!Vector4f {
            vals: [-1.0, 2.0, -3.0, 0.0],
        }
        .float_eq(&other));

        assert!(!Vector4f {
            vals: [1.0, -2.0, -3.0, 0.0],
        }
        .float_eq(&other));

        assert!(!Vector4f {
            vals: [1.0, 2.0, 3.0, 0.0],
        }
        .float_eq(&other));

        assert!(!Vector4f {
            vals: [1.0, 2.0, -3.0, 4.0],
        }
        .float_eq(&other));
    }

    #[test]
    fn test_vector3f_float_eq() {
        let other = Vector3f::new(1.0, 2.0, -3.0);

        assert!(Vector3f::new(1.0, 2.0, -3.0).float_eq(&other));
        assert!(!Vector3f::new(-1.0, 2.0, -3.0).float_eq(&other));
        assert!(!Vector3f::new(1.0, -2.0, -3.0).float_eq(&other));
        assert!(!Vector3f::new(1.0, 2.0, 3.0).float_eq(&other));
    }

    #[test]
    fn test_point3f_float_eq() {
        let other = Point3f::new(1.0, 2.0, -3.0);

        assert!(Point3f::new(1.0, 2.0, -3.0).float_eq(&other));
        assert!(!Point3f::new(-1.0, 2.0, -3.0).float_eq(&other));
        assert!(!Point3f::new(1.0, -2.0, -3.0).float_eq(&other));
        assert!(!Point3f::new(1.0, 2.0, 3.0).float_eq(&other));
    }

    #[test]
    fn test_add() {
        let pa = Point3f::new(3.0, -2.0, 5.0);
        let va = Vector3f::new(3.0, -2.0, 5.0);

        let pb = Point3f::new(-2.0, 3.0, 1.0);
        let vb = Vector3f::new(-2.0, 3.0, 1.0);

        let pc = Point3f::new(1.0, 1.0, 6.0);
        let vc = Vector3f::new(1.0, 1.0, 6.0);

        assert_float_eq(pa + vb, pc);
        assert_float_eq(va + pb, pc);
        assert_float_eq(va + vb, vc);
    }

    #[test]
    fn test_sub() {
        let pa = Point3f::new(3.0, 2.0, 1.0);
        let va = Vector3f::new(3.0, 2.0, 1.0);

        let pb = Point3f::new(5.0, 6.0, 7.0);
        let vb = Vector3f::new(5.0, 6.0, 7.0);

        let pc = Point3f::new(-2.0, -4.0, -6.0);
        let vc = Vector3f::new(-2.0, -4.0, -6.0);

        assert_float_eq(pa - pb, vc);
        assert_float_eq(pa - vb, pc);
        assert_float_eq(va - vb, vc);
    }

    #[test]
    fn test_neg() {
        assert_float_eq(
            -Vector4f::new(1.0, -2.0, 3.0, -4.0),
            Vector4f::new(-1.0, 2.0, -3.0, 4.0),
        );

        assert_float_eq(
            -Vector3f::new(1.0, -2.0, 3.0),
            Vector3f::new(-1.0, 2.0, -3.0),
        );
    }

    #[test]
    fn test_mul() {
        assert_float_eq(
            Vector4f::new(1.0, -2.0, 3.0, -4.0) * 3.5,
            Vector4f::new(3.5, -7.0, 10.5, -14.0),
        );

        assert_float_eq(
            Vector4f::new(1.0, -2.0, 3.0, -4.0) * 0.5,
            Vector4f::new(0.5, -1.0, 1.5, -2.0),
        );

        assert_float_eq(
            Vector3f::new(1.0, -2.0, 3.0) * 3.5,
            Vector3f::new(3.5, -7.0, 10.5),
        );

        assert_float_eq(
            Vector3f::new(1.0, -2.0, 3.0) * 0.5,
            Vector3f::new(0.5, -1.0, 1.5),
        );
    }

    #[test]
    fn test_div() {
        assert_float_eq(
            Vector4f::new(1.0, -2.0, 3.0, -4.0) / 2.0,
            Vector4f::new(0.5, -1.0, 1.5, -2.0),
        );

        assert_float_eq(
            Vector3f::new(1.0, -2.0, 3.0) / 2.0,
            Vector3f::new(0.5, -1.0, 1.5),
        );
    }

    #[test]
    fn test_magnitude() {
        assert_float_eq(Vector3f::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_float_eq(Vector3f::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_float_eq(Vector3f::new(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_float_eq(Vector3f::new(1.0, 2.0, 3.0).magnitude(), 14.0_f64.sqrt());
        assert_float_eq(Vector3f::new(-1.0, -2.0, -3.0).magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn test_normalize() {
        assert_float_eq(
            Vector3f::new(4.0, 0.0, 0.0).normalize(),
            Vector3f::new(1.0, 0.0, 0.0),
        );

        assert_float_eq(
            Vector3f::new(1.0, 2.0, 3.0).normalize(),
            Vector3f::new(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt(),
            ),
        );

        assert_float_eq(Vector3f::new(1.0, 2.0, 3.0).normalize().magnitude(), 1.0);
    }

    #[test]
    fn test_dot() {
        assert_float_eq(
            Vector3f::new(1.0, 2.0, 3.0).dot(&Vector3f::new(2.0, 3.0, 4.0)),
            20.0,
        );
    }

    #[test]
    fn test_cross() {
        let a = Vector3f::new(1.0, 2.0, 3.0);
        let b = Vector3f::new(2.0, 3.0, 4.0);

        assert_float_eq(a.cross(&b), Vector3f::new(-1.0, 2.0, -1.0));
        assert_float_eq(b.cross(&a), Vector3f::new(1.0, -2.0, 1.0));

        let a = Vector3f::new(2.0, 3.0, 5.0);
        let b = Vector3f::new(7.0, 11.0, 13.0);

        assert_float_eq(a.cross(&b), Vector3f::new(-16.0, 9.0, 1.0));
        assert_float_eq(b.cross(&a), Vector3f::new(16.0, -9.0, -1.0));
    }

    #[test]
    fn test_get_xyzw() {
        let v = Vector4f {
            vals: [1.0, 2.0, 3.0, 4.0],
        };

        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.w(), 4.0);

        let p = Point3f::new(1.0, 2.0, 3.0);

        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), 2.0);
        assert_eq!(p.z(), 3.0);

        let v = Vector3f::new(1.0, 2.0, 3.0);

        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }
}
