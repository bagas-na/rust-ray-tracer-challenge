#![allow(dead_code)]

struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

use std::cmp;

impl cmp::PartialEq for Tuple {
    fn eq(&self, t: &Self) -> bool {
        let eps = 0.00001;

        (self.x - t.x).abs() < eps &&
        (self.y - t.y).abs() < eps &&
        (self.z - t.z).abs() < eps &&
        (self.w - t.w).abs() < eps
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A tuple with w=1.0 is a point
    fn point() {
        let point = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
        assert!(point.is_point());
        assert!(!point.is_vector())
    }

    #[test]
    // A tuple with w=0.0 is a vector
    fn vector() {
        let point = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 0.0);
        assert!(!point.is_point());
        assert!(point.is_vector())
    }
}
