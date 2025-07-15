#[derive(Debug, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

use std::{cmp, ops};

impl cmp::PartialEq for Color {
    fn eq(&self, rhs: &Self) -> bool {
        let eps = 0.00001;

        (self.red - rhs.red).abs() < eps
            && (self.green - rhs.green).abs() < eps
            && (self.blue - rhs.blue).abs() < eps
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

/// The Hadamard product (or Shur product) between two colors
impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

impl ops::Neg for Color {
    type Output = Color;

    fn neg(self) -> Self::Output {
        self * (-1.0)
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
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
        assert_eq!(color_a + color_b, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtraction() {
        let color_a = Color::new(0.9, 0.6, 0.75);
        let color_b = Color::new(0.7, 0.1, 0.25);
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
