use std::ops::{Add, Div, Mul, Neg, Sub};

pub fn f64_eq(left: &f64, right: &f64) -> bool {
    (left - right).abs() < std::f64::EPSILON
}

#[derive(Debug, PartialEq)]
pub struct Vec4f {
    vals: [f64; 4],
}

impl Vec4f {
    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            vals: [x, y, z, 0.0],
        }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self {
            vals: [x, y, z, 1.0],
        }
    }

    pub fn is_vector(&self) -> bool {
        f64_eq(&self.vals[3], &0.0)
    }

    pub fn is_point(&self) -> bool {
        f64_eq(&self.vals[3], &1.0)
    }

    pub fn vec_eq(&self, other: &Vec4f) -> bool {
        self.vals
            .iter()
            .zip(other.vals.iter())
            .all(|(a, b)| f64_eq(a, b))
    }

    fn unary_op<F>(&self, op: F) -> Vec4f
    where
        F: Fn(&f64) -> f64,
    {
        Vec4f {
            vals: [
                op(&self.vals[0]),
                op(&self.vals[1]),
                op(&self.vals[2]),
                op(&self.vals[3]),
            ],
        }
    }

    fn binary_op<F>(&self, other: &Vec4f, op: F) -> Vec4f
    where
        F: Fn(&f64, &f64) -> f64,
    {
        Vec4f {
            vals: [
                op(&self.vals[0], &other.vals[0]),
                op(&self.vals[1], &other.vals[1]),
                op(&self.vals[2], &other.vals[2]),
                op(&self.vals[3], &other.vals[3]),
            ],
        }
    }

    fn binary_scalar_op<F>(&self, other: &f64, op: F) -> Vec4f
    where
        F: Fn(&f64, &f64) -> f64,
    {
        Vec4f {
            vals: [
                op(&self.vals[0], &other),
                op(&self.vals[1], &other),
                op(&self.vals[2], &other),
                op(&self.vals[3], &other),
            ],
        }
    }
}

pub fn assert_vec_eq(left: &Vec4f, right: &Vec4f) {
    assert!(left.vec_eq(right), "left = {:?}, right = {:?}", left, right);
}

impl Add for Vec4f {
    type Output = Vec4f;

    fn add(self, rhs: Self) -> Self::Output {
        self.binary_op(&rhs, |a, b| a + b)
    }
}

impl Sub for Vec4f {
    type Output = Vec4f;

    fn sub(self, rhs: Self) -> Self::Output {
        self.binary_op(&rhs, |a, b| a - b)
    }
}

impl Neg for Vec4f {
    type Output = Vec4f;

    fn neg(self) -> Self::Output {
        self.unary_op(|a| -a)
    }
}

impl Mul<f64> for Vec4f {
    type Output = Vec4f;

    fn mul(self, rhs: f64) -> Self::Output {
        self.binary_scalar_op(&rhs, |a, b| a * b)
    }
}

impl Div<f64> for Vec4f {
    type Output = Vec4f;

    fn div(self, rhs: f64) -> Self::Output {
        self.binary_scalar_op(&rhs, |a, b| a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_eq() {
        assert_ne!(0.1 + 0.2, 0.3);
        assert!(f64_eq(&(0.1 + 0.2), &0.3));
    }

    #[test]
    fn test_vec4f_point() {
        let vec = Vec4f {
            vals: [4.3, -4.2, 3.1, 1.0],
        };
        assert!(vec.is_point());
        assert!(!vec.is_vector());
    }

    #[test]
    fn test_vec4f_vector() {
        let vec = Vec4f {
            vals: [4.3, -4.2, 3.1, 0.0],
        };
        assert!(!vec.is_point());
        assert!(vec.is_vector());
    }

    #[test]
    fn test_vec4f_new_point() {
        assert_eq!(
            Vec4f::new_point(4.0, -4.0, 3.0),
            Vec4f {
                vals: [4.0, -4.0, 3.0, 1.0],
            }
        );
    }

    #[test]
    fn test_vec4f_new_vector() {
        assert_eq!(
            Vec4f::new_vector(4.0, -4.0, 3.0),
            Vec4f {
                vals: [4.0, -4.0, 3.0, 0.0],
            }
        );
    }

    #[test]
    fn test_vec4f_vec_eq() {
        let other = Vec4f {
            vals: [1.0, 2.0, -3.0, 0.0],
        };

        assert!(Vec4f {
            vals: [1.0, 2.0, -3.0, 0.0],
        }
        .vec_eq(&other));
        assert!(Vec4f {
            vals: [1.0, 2.0, -3.0, -0.0],
        }
        .vec_eq(&other));

        assert!(!Vec4f {
            vals: [-1.0, 2.0, -3.0, 0.0],
        }
        .vec_eq(&other));

        assert!(!Vec4f {
            vals: [1.0, -2.0, -3.0, 0.0],
        }
        .vec_eq(&other));

        assert!(!Vec4f {
            vals: [1.0, 2.0, 3.0, 0.0],
        }
        .vec_eq(&other));

        assert!(!Vec4f {
            vals: [1.0, 2.0, -3.0, 4.0],
        }
        .vec_eq(&other));
    }

    #[test]
    fn test_vec4f_add() {
        assert_vec_eq(
            &(Vec4f {
                vals: [3.0, -2.0, 5.0, 1.0],
            } + Vec4f {
                vals: [-2.0, 3.0, 1.0, 0.0],
            }),
            &Vec4f {
                vals: [1.0, 1.0, 6.0, 1.0],
            },
        );
    }

    #[test]
    fn test_vec4f_sub() {
        [
            (
                Vec4f::new_point(3.0, 2.0, 1.0),
                Vec4f::new_point(5.0, 6.0, 7.0),
                Vec4f::new_vector(-2.0, -4.0, -6.0),
            ),
            (
                Vec4f::new_point(3.0, 2.0, 1.0),
                Vec4f::new_vector(5.0, 6.0, 7.0),
                Vec4f::new_point(-2.0, -4.0, -6.0),
            ),
            (
                Vec4f::new_vector(3.0, 2.0, 1.0),
                Vec4f::new_vector(5.0, 6.0, 7.0),
                Vec4f::new_vector(-2.0, -4.0, -6.0),
            ),
        ]
        .into_iter()
        .for_each(|(a, b, c)| {
            assert_vec_eq(&(a - b), &c);
        });
    }

    #[test]
    fn test_vec4f_neg() {
        assert_vec_eq(
            &-Vec4f {
                vals: [1.0, -2.0, 3.0, -4.0],
            },
            &Vec4f {
                vals: [-1.0, 2.0, -3.0, 4.0],
            },
        );
    }

    #[test]
    fn test_vec4f_mul() {
        assert_vec_eq(
            &(Vec4f {
                vals: [1.0, -2.0, 3.0, -4.0],
            } * 3.5),
            &Vec4f {
                vals: [3.5, -7.0, 10.5, -14.0],
            },
        );

        assert_vec_eq(
            &(Vec4f {
                vals: [1.0, -2.0, 3.0, -4.0],
            } * 0.5),
            &Vec4f {
                vals: [0.5, -1.0, 1.5, -2.0],
            },
        );
    }

    #[test]
    fn test_vec4f_div() {
        assert_vec_eq(
            &(Vec4f {
                vals: [1.0, -2.0, 3.0, -4.0],
            } / 2.0),
            &Vec4f {
                vals: [0.5, -1.0, 1.5, -2.0],
            },
        );
    }
}
