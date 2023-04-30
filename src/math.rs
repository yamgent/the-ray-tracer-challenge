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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let vec = Vec4f {
            vals: [4.3, -4.2, 3.1, 1.0],
        };
        assert!(vec.is_point());
        assert!(!vec.is_vector());
    }

    #[test]
    fn test_vector() {
        let vec = Vec4f {
            vals: [4.3, -4.2, 3.1, 0.0],
        };
        assert!(!vec.is_point());
        assert!(vec.is_vector());
    }

    #[test]
    fn test_new_point() {
        assert_eq!(
            Vec4f::new_point(4.0, -4.0, 3.0),
            Vec4f {
                vals: [4.0, -4.0, 3.0, 1.0],
            }
        );
    }

    #[test]
    fn test_new_vector() {
        assert_eq!(
            Vec4f::new_vector(4.0, -4.0, 3.0),
            Vec4f {
                vals: [4.0, -4.0, 3.0, 0.0],
            }
        );
    }
}
