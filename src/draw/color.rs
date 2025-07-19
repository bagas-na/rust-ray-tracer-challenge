#[derive(Debug, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

use crate::EPSILON;
use std::{cmp, ops};

impl cmp::PartialEq for Color {
    fn eq(&self, rhs: &Self) -> bool {
        (self.red - rhs.red).abs() < EPSILON
            && (self.green - rhs.green).abs() < EPSILON
            && (self.blue - rhs.blue).abs() < EPSILON
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl ops::Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

impl ops::Neg for Color {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            red: -self.red,
            green: -self.green,
            blue: -self.blue,
        }
    }
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    /// Component wise addition between two colors
    fn add(c1: &Self, c2: &Self) -> Self {
        Self {
            red: c1.red + c2.red,
            green: c1.green + c2.green,
            blue: c1.blue + c2.blue,
        }
    }

    /// Component wise subtraction between two colors
    fn sub(c1: &Self, c2: &Self) -> Self {
        Self {
            red: c1.red - c2.red,
            green: c1.green - c2.green,
            blue: c1.blue - c2.blue,
        }
    }

    /// The Hadamard product (or Schur product) between two colors
    pub fn hadamard_product(c1: &Self, c2: &Self) -> Self {
        Self {
            red: c1.red * c2.red,
            green: c1.green * c2.green,
            blue: c1.blue * c2.blue,
        }
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::add(&self, &rhs)
    }
}

impl ops::Add<&Color> for Color {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::add(&self, rhs)
    }
}

impl<'a, 'b> ops::Add<&'b Color> for &'a Color {
    type Output = Color;

    fn add(self, rhs: &'b Color) -> Self::Output {
        Color::add(self, rhs)
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::hadamard_product(&self, &rhs)
    }
}

impl ops::Mul<&Color> for Color {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        Self::hadamard_product(&self, rhs)
    }
}

impl<'a, 'b> ops::Mul<&'b Color> for &'a Color {
    type Output = Color;

    fn mul(self, rhs: &'b Color) -> Self::Output {
        Color::hadamard_product(self, rhs)
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::sub(&self, &rhs)
    }
}

impl ops::Sub<&Color> for Color {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self::sub(&self, rhs)
    }
}

impl<'a, 'b> ops::Sub<&'b Color> for &'a Color {
    type Output = Color;

    fn sub(self, rhs: &'b Color) -> Self::Output {
        Color::sub(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_tuple() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(color.red, -0.5);
        assert_eq!(color.green, 0.4);
        assert_eq!(color.blue, 1.7);
    }

    #[test]
    fn addition() {
        let color_a = Color::new(0.9, 0.6, 0.75);
        let color_b = Color::new(0.7, 0.1, 0.25);
        let color_result = Color::new(1.6, 0.7, 1.0);
        assert_eq!(&color_a + &color_b, color_result);
        assert_eq!(color_a + &color_b, color_result);

        let color_a = Color::new(0.9, 0.6, 0.75);
        assert_eq!(color_a + color_b, color_result);
    }

    #[test]
    fn subtraction() {
        let color_a = Color::new(0.9, 0.6, 0.75);
        let color_b = Color::new(0.7, 0.1, 0.25);
        assert_eq!(&color_a - &color_b, Color::new(0.2, 0.5, 0.5));
        assert_eq!(color_a - &color_b, Color::new(0.2, 0.5, 0.5));

        let color_a = Color::new(0.9, 0.6, 0.75);
        assert_eq!(color_a - color_b, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn mult_with_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);
        assert_eq!(color * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn mult_two_colors() {
        let color_a = Color::new(1.0, 0.2, 0.4);
        let color_b = Color::new(0.9, 1.0, 0.1);
        assert_eq!(color_a * color_b, Color::new(0.9, 0.2, 0.04));
    }
}
