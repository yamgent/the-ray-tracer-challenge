use std::ops::{Add, Mul, Sub};

use crate::math::FloatEq;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    vals: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { vals: [r, g, b] }
    }

    pub fn r(&self) -> f64 {
        self.vals[0]
    }

    pub fn g(&self) -> f64 {
        self.vals[1]
    }

    pub fn b(&self) -> f64 {
        self.vals[2]
    }

    fn unary_op<F>(&self, op: F) -> Self
    where
        F: Fn(&f64) -> f64,
    {
        Self {
            vals: [op(&self.vals[0]), op(&self.vals[1]), op(&self.vals[2])],
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
            ],
        }
    }
}

impl FloatEq for Color {
    fn float_eq(&self, other: &Self) -> bool {
        self.vals
            .iter()
            .zip(other.vals.iter())
            .all(|(a, b)| a.float_eq(&b))
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        self.binary_op(&rhs, |a, b| a + b)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        self.binary_op(&rhs, |a, b| a - b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        self.unary_op(|a| a * rhs)
    }
}

impl Mul for Color {
    type Output = Color;

    /// Hadamard product / Schur product (multiple each component by other)
    fn mul(self, rhs: Self) -> Self::Output {
        self.binary_op(&rhs, |a, b| a * b)
    }
}

pub struct Canvas {
    px: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            px: std::iter::repeat(
                std::iter::repeat(Color::new(0.0, 0.0, 0.0))
                    .take(w)
                    .collect(),
            )
            .take(h)
            .collect(),
        }
    }

    pub fn w(&self) -> usize {
        if self.px.is_empty() {
            0
        } else {
            self.px[0].len()
        }
    }

    pub fn h(&self) -> usize {
        self.px.len()
    }

    pub fn px(&self, x: usize, y: usize) -> Color {
        assert!(
            y < self.px.len(),
            "Out of range: ({}, {}) for size ({}, {})",
            x,
            y,
            self.w(),
            self.h()
        );
        assert!(
            x < self.px[y].len(),
            "Out of range: ({}, {}) for size ({}, {})",
            x,
            y,
            self.w(),
            self.h()
        );
        self.px[y][x]
    }

    pub fn write_px(&mut self, x: usize, y: usize, color: Color) {
        assert!(
            y < self.px.len(),
            "Out of range: ({}, {}) for size ({}, {})",
            x,
            y,
            self.w(),
            self.h()
        );
        assert!(
            x < self.px[y].len(),
            "Out of range: ({}, {}) for size ({}, {})",
            x,
            y,
            self.w(),
            self.h()
        );
        self.px[y][x] = color;
    }
}

#[cfg(test)]
mod tests {
    use crate::math::assert_float_eq;

    use super::*;

    #[test]
    fn test_color_new() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_float_eq(
            c,
            Color {
                vals: [-0.5, 0.4, 1.7],
            },
        );
        assert_float_eq(c.r(), -0.5);
        assert_float_eq(c.g(), 0.4);
        assert_float_eq(c.b(), 1.7);
    }

    #[test]
    fn test_color_add() {
        assert_float_eq(
            Color::new(0.9, 0.6, 0.75) + Color::new(0.7, 0.1, 0.25),
            Color::new(1.6, 0.7, 1.0),
        );
    }

    #[test]
    fn test_color_sub() {
        assert_float_eq(
            Color::new(0.9, 0.6, 0.75) - Color::new(0.7, 0.1, 0.25),
            Color::new(0.2, 0.5, 0.5),
        );
    }

    #[test]
    fn test_color_mul_scalar() {
        assert_float_eq(Color::new(0.2, 0.3, 0.4) * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_color_mul() {
        assert_float_eq(
            Color::new(1.0, 0.2, 0.4) * Color::new(0.9, 1.0, 0.1),
            Color::new(0.9, 0.2, 0.04),
        );
    }

    #[test]
    fn test_canvas_new() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.w(), 10);
        assert_eq!(c.h(), 20);
        (0..10).for_each(|x| {
            (0..20).for_each(|y| {
                assert_float_eq(c.px(x, y), Color::new(0.0, 0.0, 0.0));
            });
        });
    }

    #[test]
    fn test_canvas_write() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_px(2, 3, red);
        assert_float_eq(c.px(2, 3), red);
    }
}
