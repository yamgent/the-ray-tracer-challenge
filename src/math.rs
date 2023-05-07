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

pub fn assert_float_ne<T>(left: T, right: T)
where
    T: FloatEq + std::fmt::Debug,
{
    assert!(
        !left.float_eq(&right),
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

pub trait Determinant {
    fn determinant(&self) -> f64;
}

impl Determinant for f64 {
    fn determinant(&self) -> f64 {
        *self
    }
}

pub trait Submatrix {
    type Output: Determinant;

    fn submatrix(&self, remove_r: usize, remove_c: usize) -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
pub struct BaseMatrix<const N: usize, const O: usize> {
    vals: [f64; N],
}

impl<const N: usize, const O: usize> BaseMatrix<N, O> {
    const MAT_ORDER: usize = O;

    fn assert_bounds(r: usize, c: usize) {
        assert!(
            r < Self::MAT_ORDER,
            "r = {} is not valid for a {}x{}",
            r,
            Self::MAT_ORDER,
            Self::MAT_ORDER
        );
        assert!(
            c < Self::MAT_ORDER,
            "c = {} is not valid for a {}x{}",
            c,
            Self::MAT_ORDER,
            Self::MAT_ORDER
        );
    }

    pub fn new(vals: [f64; N]) -> Self {
        Self { vals }
    }

    pub fn get(&self, r: usize, c: usize) -> f64 {
        Self::assert_bounds(r, c);
        self.vals[r * Self::MAT_ORDER + c]
    }

    pub fn identity() -> Self {
        Self {
            vals: (0..Self::MAT_ORDER)
                .flat_map(|r| {
                    (0..Self::MAT_ORDER)
                        .map(|c| if r == c { 1.0 } else { 0.0 })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn transpose(&self) -> Self {
        Self {
            vals: (0..Self::MAT_ORDER)
                .flat_map(|r| {
                    (0..Self::MAT_ORDER)
                        .map(|c| self.get(c, r))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    fn submatrix_vals(&self, remove_r: usize, remove_c: usize) -> Vec<f64> {
        Self::assert_bounds(remove_r, remove_c);

        (0..Self::MAT_ORDER)
            .filter(|r| *r != remove_r)
            .flat_map(|r| {
                (0..Self::MAT_ORDER)
                    .filter(|c| *c != remove_c)
                    .map(|c| self.get(r, c))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl<const N: usize, const O: usize> BaseMatrix<N, O>
where
    BaseMatrix<N, O>: Submatrix,
{
    pub fn minor(&self, i: usize, j: usize) -> f64 {
        self.submatrix(i, j).determinant()
    }

    pub fn cofactor(&self, i: usize, j: usize) -> f64 {
        let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
        self.minor(i, j) * sign
    }
}

impl<const N: usize, const O: usize> FloatEq for BaseMatrix<N, O> {
    fn float_eq(&self, other: &Self) -> bool {
        self.vals
            .iter()
            .zip(other.vals.iter())
            .all(|(a, b)| a.float_eq(b))
    }
}

impl<const N: usize, const O: usize> Mul for BaseMatrix<N, O> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let vals = (0..Self::MAT_ORDER)
            .flat_map(|r| {
                (0..Self::MAT_ORDER)
                    .map(|c| {
                        (0..Self::MAT_ORDER)
                            .map(|i| self.get(r, i) * rhs.get(i, c))
                            .sum()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            vals: vals.try_into().unwrap(),
        }
    }
}

impl<const N: usize, const O: usize> Determinant for BaseMatrix<N, O>
where
    BaseMatrix<N, O>: Submatrix,
{
    fn determinant(&self) -> f64 {
        (0..Self::MAT_ORDER)
            .map(|c| self.get(0, c) * self.cofactor(0, c))
            .sum()
    }
}

pub type Matrix4x4f = BaseMatrix<16, 4>;

impl Submatrix for Matrix4x4f {
    type Output = Matrix3x3f;

    fn submatrix(&self, remove_r: usize, remove_c: usize) -> Matrix3x3f {
        Matrix3x3f {
            vals: self.submatrix_vals(remove_r, remove_c).try_into().unwrap(),
        }
    }
}

impl Mul<Vector4f> for Matrix4x4f {
    type Output = Vector4f;

    fn mul(self, rhs: Vector4f) -> Self::Output {
        let vals = (0..4)
            .map(|r| (0..4).map(|i| self.get(r, i) * rhs.vals[i]).sum())
            .collect::<Vec<_>>();

        Vector4f {
            vals: vals.try_into().unwrap(),
        }
    }
}

pub type Matrix3x3f = BaseMatrix<9, 3>;

impl Submatrix for Matrix3x3f {
    type Output = Matrix2x2f;

    fn submatrix(&self, remove_r: usize, remove_c: usize) -> Matrix2x2f {
        Matrix2x2f {
            vals: self.submatrix_vals(remove_r, remove_c).try_into().unwrap(),
        }
    }
}

pub type Matrix2x2f = BaseMatrix<4, 2>;

impl Submatrix for Matrix2x2f {
    type Output = f64;

    fn submatrix(&self, remove_r: usize, remove_c: usize) -> f64 {
        self.submatrix_vals(remove_r, remove_c)[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_float_eq() {
        assert_ne!(0.1 + 0.2, 0.3);
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
    fn test_vec_add() {
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
    fn test_vec_sub() {
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
    fn test_vec_neg() {
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
    fn test_vec_mul() {
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
    fn test_vec_div() {
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
    fn test_vec_magnitude() {
        assert_float_eq(Vector3f::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_float_eq(Vector3f::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_float_eq(Vector3f::new(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_float_eq(Vector3f::new(1.0, 2.0, 3.0).magnitude(), 14.0_f64.sqrt());
        assert_float_eq(Vector3f::new(-1.0, -2.0, -3.0).magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn test_vec_normalize() {
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
    fn test_vec_dot() {
        assert_float_eq(
            Vector3f::new(1.0, 2.0, 3.0).dot(&Vector3f::new(2.0, 3.0, 4.0)),
            20.0,
        );
    }

    #[test]
    fn test_vec_cross() {
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
    fn test_vec_get_xyzw() {
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

    #[test]
    fn test_matrix_new() {
        let m = Matrix4x4f::new([
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ]);
        [
            (0, 0, 1.0),
            (0, 3, 4.0),
            (1, 0, 5.5),
            (1, 2, 7.5),
            (2, 2, 11.0),
            (3, 0, 13.5),
            (3, 2, 15.5),
        ]
        .into_iter()
        .for_each(|(r, c, expected)| {
            assert_float_eq(m.get(r, c), expected);
        });

        let m = Matrix3x3f::new([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
        [(0, 0, -3.0), (1, 1, -2.0), (2, 2, 1.0), (1, 2, -7.0)]
            .into_iter()
            .for_each(|(r, c, expected)| {
                assert_float_eq(m.get(r, c), expected);
            });

        let m = Matrix2x2f::new([-3.0, 5.0, 1.0, -2.0]);
        [(0, 0, -3.0), (0, 1, 5.0), (1, 0, 1.0), (1, 1, -2.0)]
            .into_iter()
            .for_each(|(r, c, expected)| {
                assert_float_eq(m.get(r, c), expected);
            });
    }

    #[test]
    fn test_matrix_eq() {
        {
            let a = Matrix4x4f::new([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]);
            let b = Matrix4x4f::new([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]);
            let c = Matrix4x4f::new([
                2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
            ]);
            assert_float_eq(a, b);
            assert_float_ne(a, c);
        }

        {
            let a = Matrix3x3f::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
            let b = Matrix3x3f::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
            let c = Matrix3x3f::new([2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0]);
            assert_float_eq(a, b);
            assert_float_ne(a, c);
        }

        {
            let a = Matrix2x2f::new([1.0, 2.0, 3.0, 4.0]);
            let b = Matrix2x2f::new([1.0, 2.0, 3.0, 4.0]);
            let c = Matrix2x2f::new([2.0, 3.0, 4.0, 5.0]);
            assert_float_eq(a, b);
            assert_float_ne(a, c);
        }
    }

    #[test]
    fn test_matrix4x4f_mul() {
        assert_float_eq(
            Matrix4x4f::new([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]) * Matrix4x4f::new([
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ]),
            Matrix4x4f::new([
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ]),
        );

        assert_float_eq(
            Matrix4x4f::new([
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ]) * Vector4f::new(1.0, 2.0, 3.0, 1.0),
            Vector4f::new(18.0, 24.0, 33.0, 1.0),
        );
    }

    #[test]
    fn test_matrix4x4f_identity() {
        assert_float_eq(
            Matrix4x4f::identity(),
            Matrix4x4f::new([
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ]),
        );
        let m = Matrix4x4f::new([
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        ]);
        assert_float_eq(m * Matrix4x4f::identity(), m);

        let v = Vector4f::new(1.0, 2.0, 3.0, 4.0);
        assert_float_eq(Matrix4x4f::identity() * v, v);
    }

    #[test]
    fn test_matrix4x4f_transpose() {
        assert_float_eq(
            Matrix4x4f::new([
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ])
            .transpose(),
            Matrix4x4f::new([
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
            ]),
        );
        assert_float_eq(Matrix4x4f::identity().transpose(), Matrix4x4f::identity());
    }

    #[test]
    fn test_matrix2x2f_determinant() {
        assert_float_eq(Matrix2x2f::new([1.0, 5.0, -3.0, 2.0]).determinant(), 17.0);
    }

    #[test]
    fn test_matrix_submatrix() {
        assert_float_eq(Matrix2x2f::new([1.0, 2.0, 3.0, 4.0]).submatrix(0, 1), 3.0);
        assert_float_eq(
            Matrix3x3f::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]).submatrix(0, 2),
            Matrix2x2f::new([-3.0, 2.0, 0.0, 6.0]),
        );

        assert_float_eq(
            Matrix4x4f::new([
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ])
            .submatrix(2, 1),
            Matrix3x3f::new([-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]),
        );
    }

    #[test]
    fn test_matrix3x3f_minor() {
        let m = Matrix3x3f::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        assert_float_eq(m.submatrix(1, 0).determinant(), 25.0);
        assert_float_eq(m.minor(1, 0), 25.0);
    }

    #[test]
    fn test_matrix3x3f_cofactor() {
        let m = Matrix3x3f::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        assert_float_eq(m.minor(0, 0), -12.0);
        assert_float_eq(m.cofactor(0, 0), -12.0);
        assert_float_eq(m.minor(1, 0), 25.0);
        assert_float_eq(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_matrix3x3f_determinant() {
        let m = Matrix3x3f::new([1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);

        assert_float_eq(m.cofactor(0, 0), 56.0);
        assert_float_eq(m.cofactor(0, 1), 12.0);
        assert_float_eq(m.cofactor(0, 2), -46.0);
    }

    #[test]
    fn test_matrix4x4f_determinant() {
        let m = Matrix4x4f::new([
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        ]);

        assert_float_eq(m.cofactor(0, 0), 690.0);
        assert_float_eq(m.cofactor(0, 1), 447.0);
        assert_float_eq(m.cofactor(0, 2), 210.0);
        assert_float_eq(m.cofactor(0, 3), 51.0);
        assert_float_eq(m.determinant(), -4071.0);
    }
}
