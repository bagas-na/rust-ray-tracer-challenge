use crate::EPSILON;
use std::{cmp, fmt, ops};

#[derive(Debug, Clone)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl cmp::PartialEq for Tuple {
    fn eq(&self, rhs: &Self) -> bool {
        (self.x - rhs.x).abs() < EPSILON
            && (self.y - rhs.y).abs() < EPSILON
            && (self.z - rhs.z).abs() < EPSILON
            && (self.w - rhs.w).abs() < EPSILON
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Tuple {
    /// Creates a new instance of Tuple{x, y, z, w}, where w
    /// represents an instance of a vector (w = 0) or a point (w = 1)
    /// ## Panics
    /// Will panic if w is neither 0 nor 1.
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a new Tuple with zeroed elements
    pub fn zero() -> Self {
        Tuple::new(0.0, 0.0, 0.0, 0.0)
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

    /// Creates a new instance of Tuple{x, y, z, w} from a function 
    /// where x = f(0), y = f(1), z = f(2), w = f(3)
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(usize) -> f64,
    {
        Self {
            x: f(0),
            y: f(1),
            z: f(2),
            w: f(3),
        }
    }

    /// Checks whether the Tuple represents a point.
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Checks whether the Tuple represents a vector.
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// Get elements of Tuple
    pub fn get(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.z, self.w)
    }

    /// Addition between two tuples
    fn add(t1: &Self, t2: &Self) -> Self {
        Self {
            x: t1.x + t2.x,
            y: t1.y + t2.y,
            z: t1.z + t2.z,
            w: t1.w + t2.w,
        }
    }

    /// Subtraction of a tuple from another tuple
    fn sub(t1: &Self, t2: &Self) -> Self {
        Self {
            x: t1.x - t2.x,
            y: t1.y - t2.y,
            z: t1.z - t2.z,
            w: t1.w - t2.w,
        }
    }

    /// Scalar multiplication of a tuple
    fn mul_scal(&self, scal: f64) -> Self {
        Self {
            x: self.x * scal,
            y: self.y * scal,
            z: self.z * scal,
            w: self.w * scal,
        }
    }

    /// Scalar division of a tuple
    fn div_scal(&self, scal: f64) -> Self {
        Self {
            x: self.x / scal,
            y: self.y / scal,
            z: self.z / scal,
            w: self.w / scal,
        }
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
    pub fn dot(tuple_a: &Tuple, tuple_b: &Tuple) -> f64 {
        tuple_a.x * tuple_b.x
            + tuple_a.y * tuple_b.y
            + tuple_a.z * tuple_b.z
            + tuple_a.w * tuple_b.w
    }

    /// Cross product between self (vector) and another vector
    pub fn cross(vector_a: &Tuple, vector_b: &Tuple) -> Tuple {
        assert!(vector_a.is_vector(), "{vector_a} is not a vector.");
        assert!(vector_b.is_vector(), "{vector_b} is not a vector.");

        Tuple::new_vector(
            vector_a.y * vector_b.z - vector_a.z * vector_b.y,
            vector_a.z * vector_b.x - vector_a.x * vector_b.z,
            vector_a.x * vector_b.y - vector_a.y * vector_b.x,
        )
    }
}

impl ops::Add for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Self) -> Self::Output {
        Tuple::add(&self, &rhs)
    }
}
impl ops::Add<&Tuple> for Tuple {
    type Output = Tuple;
    fn add(self, rhs: &Tuple) -> Self::Output {
        Tuple::add(&self, rhs)
    }
}
impl ops::Add<Tuple> for &Tuple {
    type Output = Tuple;
    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple::add(self, &rhs)
    }
}
impl<'a, 'b> ops::Add<&'b Tuple> for &'a Tuple {
    type Output = Tuple;
    fn add(self, rhs: &'b Tuple) -> Self::Output {
        Tuple::add(self, rhs)
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::sub(&self, &rhs)
    }
}
impl ops::Sub<&Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: &Tuple) -> Self::Output {
        Tuple::sub(&self, rhs)
    }
}
impl ops::Sub<Tuple> for &Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple::sub(self, &rhs)
    }
}
impl<'a, 'b> ops::Sub<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    fn sub(self, rhs: &'b Tuple) -> Self::Output {
        Tuple::sub(self, rhs)
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::mul_scal(&self, rhs)
    }
}

impl ops::Mul<f64> for &Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::mul_scal(self, rhs)
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple::div_scal(&self, rhs)
    }
}

impl ops::Div<f64> for &Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple::div_scal(self, rhs)
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}, {}, {}, {}}}", self.x, self.y, self.z, self.w)
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

        assert_eq!(Tuple::dot(&vector_a, &vector_b), 20.0);
    }

    #[test]
    fn cross_product() {
        let vector_a = Tuple::new_vector(1.0, 2.0, 3.0);
        let vector_b = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(
            Tuple::cross(&vector_a, &vector_b),
            Tuple::new_vector(-1.0, 2.0, -1.0)
        );
        assert_eq!(
            Tuple::cross(&vector_b, &vector_a),
            Tuple::new_vector(1.0, -2.0, 1.0)
        );
    }
}
