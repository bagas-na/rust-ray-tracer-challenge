#[derive(Debug, Clone)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

use std::{cmp, ops};

impl cmp::PartialEq for Tuple {
    fn eq(&self, rhs: &Self) -> bool {
        let eps = 0.00001;

        (self.x - rhs.x).abs() < eps
            && (self.y - rhs.y).abs() < eps
            && (self.z - rhs.z).abs() < eps
            && (self.w - rhs.w).abs() < eps
    }
}

impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        match self.w + rhs.w {
            0.0 => Tuple::new_vector(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z),
            1.0 => Tuple::new_point(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z),
            _ => panic!("Resulting Tuple is neither a point nor a vector"),
        }
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        match self.w - rhs.w {
            0.0 => Tuple::new_vector(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z),
            1.0 => Tuple::new_point(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z),
            _ => panic!("Resulting Tuple is neither a point nor a vector"),
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;
    /// Does not check for validity of resulting Tuple
    /// (whether it is a point of a vector)
    fn neg(self) -> Self::Output {
        self * (-1.0)
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;
    /// Does not check for validity of resulting Tuple
    /// (whether it is a point of a vector)
    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;
    /// Does not check for validity of resulting Tuple
    /// (whether it is a point of a vector)
    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Tuple {
    /// Creates a new instance of Tuple{x, y, z, w}, where w
    /// represents an instance of a vector (w = 0) or a point (w = 1)
    /// ## Panics
    /// Will panic if w is neither 0 nor 1.
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        // if w != 0.0 && w != 1.0 {
        //     panic!("Tuple is neither a point nor a vector.")
        // }
        Tuple { x, y, z, w }
    }

    /// Creates a new instance of Tuple{x, y, z, w} that represents a point.
    /// Automatically assigns w = 1.0
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 1.0)
    }

    /// Creates a new instance of Tuple{x, y, z, w} that represents a vector.
    /// Automatically assigns w = 0.0
    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 0.0)
    }

    /// Checks whether the Tuple represents a point.
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Checks whether the Tuple represents a vector.
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// Returns the magnitude of a tuple
    pub fn magnitude(&self) -> f64 {
        let sos = [self.x, self.y, self.z, self.w]
            .iter()
            .map(|num| num * num)
            .reduce(|acc, el| acc + el)
            .unwrap_or(0.0);

        f64::sqrt(sos)
    }

    /// Normalize the tuple to have magnitude 1
    pub fn normalize(&self) -> Tuple {
        let abs = self.magnitude();
        self.to_owned() / abs
    }

    /// Dot product between self and Tuple t
    pub fn dot(&self, t: &Tuple) -> f64 {
        self.x * t.x + self.y * t.y + self.z * t.z + self.w * t.w
    }

    /// Cross product between self (vector) and another vector
    pub fn cross(&self, vector: &Tuple) -> Tuple {
        assert!(self.is_vector(), "Self is not a vector.");
        assert!(vector.is_vector(), "\"Vector\" is not a vector.");

        Tuple::new_vector(
            self.y * vector.z - self.z * vector.y,
            self.z * vector.x - self.x * vector.z,
            self.x * vector.y - self.y * vector.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::*;

    #[test]
    // A tuple with w=1.0 is a point
    fn point() {
        let point = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
        assert!(point.is_point());
        assert!(!point.is_vector());
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
        assert!(point.is_vector());
    }

    #[test]
    // A point is equal to a tuple with w = 1
    fn point_eq() {
        let point = Tuple::new_point(4.0, -4.0, 3.0);
        assert_eq!(point, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    // A vector is equal to a tuple with w = 0
    fn vector_eq() {
        let vector = Tuple::new_vector(4.0, -4.0, 3.0);
        assert_eq!(vector, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn addition() {
        let tuple_a = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let tuple_b = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(tuple_a + tuple_b, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtracting_two_points() {
        let point_a = Tuple::new_point(3.0, 2.0, 1.0);
        let point_b = Tuple::new_point(5.0, 6.0, 7.0);

        assert_eq!(point_a - point_b, Tuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let point = Tuple::new_point(3.0, 2.0, 1.0);
        let vector = Tuple::new_vector(5.0, 6.0, 7.0);

        assert_eq!(point - vector, Tuple::new_point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let vector_a = Tuple::new_vector(3.0, 2.0, 1.0);
        let vector_b = Tuple::new_vector(5.0, 6.0, 7.0);

        assert_eq!(vector_a - vector_b, Tuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negation() {
        let zero_vector = Tuple::new_vector(0.0, 0.0, 0.0);
        let vector = Tuple::new_vector(1.0, -2.0, 3.0);

        assert_eq!(zero_vector - vector, Tuple::new_vector(-1.0, 2.0, -3.0));

        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-tuple, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn scalar_multiplication() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(tuple * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));

        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(tuple * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn scalar_division() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(tuple / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn vector_magnitude() {
        let vector = Tuple::new_vector(1.0, 0.0, 0.0);
        assert_eq!(vector.magnitude(), 1.0);
        let vector = Tuple::new_vector(0.0, 0.0, 1.0);
        assert_eq!(vector.magnitude(), 1.0);
        let vector = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(vector.magnitude(), f64::sqrt(14.0));
        let vector = Tuple::new_vector(-1.0, -2.0, -3.0);
        assert_eq!(vector.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn normalize() {
        let vector = Tuple::new_vector(4.0, 0.0, 0.0);
        assert_eq!(vector.normalize(), Tuple::new_vector(1.0, 0.0, 0.0));

        let vector = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(
            vector.normalize(),
            Tuple::new_vector(
                1.0 / f64::sqrt(14.0),
                2.0 / f64::sqrt(14.0),
                3.0 / f64::sqrt(14.0)
            )
        );

        let vector = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(vector.normalize().magnitude(), 1.0);
    }

    #[test]
    fn dot_product() {
        let vector_a = Tuple::new_vector(1.0, 2.0, 3.0);
        let vector_b = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(vector_a.dot(&vector_b), 20.0);
    }

    #[test]
    fn cross_product() {
        let vector_a = Tuple::new_vector(1.0, 2.0, 3.0);
        let vector_b = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(
            vector_a.cross(&vector_b),
            Tuple::new_vector(-1.0, 2.0, -1.0)
        );
        assert_eq!(vector_b.cross(&vector_a), Tuple::new_vector(1.0, -2.0, 1.0));
    }
}
