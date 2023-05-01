pub fn f64_eq(a: &f64, b: &f64) -> bool {
    (a - b).abs() < std::f64::EPSILON
}

#[derive(Debug, PartialEq)]
pub struct Vec4f {
    vals: [f64; 4],
}

impl Vec4f {
    fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            vals: [x, y, z, 0.0],
        }
    }

    fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self {
            vals: [x, y, z, 1.0],
        }
    }

    fn is_vector(&self) -> bool {
        self.vals[3].eq(&0.0)
    }

    fn is_point(&self) -> bool {
        self.vals[3].eq(&1.0)
    }

    fn vec_eq(&self, other: &Vec4f) -> bool {
        self.vals
            .iter()
            .zip(other.vals.iter())
            .all(|(a, b)| f64_eq(a, b))
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
}
