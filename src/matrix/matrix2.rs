use std::{cmp, fmt, ops};

use crate::EPSILON;

#[derive(Debug)]
pub struct Matrix2 {
    data: [f64; 4], // row major
}

pub enum Matrix2Error {
    OutOfBounds,
}

impl fmt::Display for Matrix2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = &self.data;
        write!(
            f,
            concat!("| {:>10.4} | {:>10.4} |\n", "| {:>10.4} | {:>10.4} |",),
            data[0], data[1], data[2], data[3],
        )
    }
}

impl Matrix2 {
    /// Creates a 4x4 zero Matrix
    /// identical to Matrix4::new()
    pub fn zero() -> Self {
        Self { data: [0.0; 4] }
    }

    /// Creates a 4x4 zero Matrix
    /// identical to Matrix4::zero()
    pub fn new() -> Self {
        Self { data: [0.0; 4] }
    }

    /// Create a 2×2 matrix from a flat array of 4 elements.
    /// The elements are arranged in row-major order.
    /// Matrix = | arr[0] arr[1] |
    ///          | arr[2] arr[3] |
    pub fn from_array(arr: [f64; 4]) -> Self {
        Self { data: arr }
    }

    /// Create a 2×2 matrix from a flat array of 4 elements.
    /// The elements are arranged in column-major order.
    /// Matrix = | arr[0] arr[2] |
    ///          | arr[1] arr[3] |
    pub fn from_array_by_col(arr: [f64; 4]) -> Self {
        Self::from_array([
            arr[0], arr[2], // first row
            arr[1], arr[3], // second row
        ])
    }

    /// Get element at given row and column. 0-indexed
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row >= 2 || col >= 2 {
            return None;
        }
        let index = row * 2 + col;
        Some(self.data[index])
    }

    /// Set element at given row and column. 0-indexed
    pub fn set(&mut self, row: usize, col: usize, val: f64) -> Result<(), Matrix2Error> {
        if row >= 2 || col >= 2 {
            return Err(Matrix2Error::OutOfBounds);
        }
        let index = row * 2 + col;
        self.data[index] = val;
        Ok(())
    }

    pub fn identity() -> Self {
        let indices = [0, 3];
        let data = core::array::from_fn(|i| if indices.contains(&i) { 1.0 } else { 0.0 });
        Self { data }
    }

    fn add(matrix_a: &Self, matrix_b: &Self) -> Self {
        let data = core::array::from_fn(|i| matrix_a.data[i] + matrix_b.data[i]);
        Self { data }
    }

    fn sub(matrix_a: &Self, matrix_b: &Self) -> Self {
        let data = core::array::from_fn(|i| matrix_a.data[i] - matrix_b.data[i]);
        Self { data }
    }

    fn mult_mat(matrix_a: &Self, matrix_b: &Self) -> Self {
        let data = core::array::from_fn(|i| {
            let row = i / 2;
            let col = i % 2;
            let mut sum = 0.0;
            for j in 0..2 {
                sum += matrix_a.get(row, j).unwrap() * matrix_b.get(j, col).unwrap();
            }
            sum
        });

        Self { data }
    }

    fn mult_scal(matrix_a: &Self, scal: f64) -> Self {
        let data = core::array::from_fn(|i| matrix_a.data[i] * scal);
        Self { data }
    }

    fn div_scal(matrix_a: &Self, scal: f64) -> Self {
        let data = core::array::from_fn(|i| matrix_a.data[i] / scal);
        Self { data }
    }

    pub fn transpose(self) -> Self {
        let data = self.data;
        Self {
            data: [data[0], data[2], data[1], data[3]],
        }
    }

    /// Calculate the determinant of the matrix
    /// ```text
    /// determinant ⎡a  b⎤ = ad - bc
    ///             ⎣c  d⎦
    /// ```
    pub fn det(&self) -> f64 {
        let data = self.data;
        data[0] * data[3] - data[1] * data[2]
    }

    pub fn invertible(&self) -> bool {
        self.det().abs() > EPSILON
    }

    pub fn inverse(self) -> Option<Self> {
        if let false = self.invertible() {
            None
        } else {
            let det = self.det();
            let data = self.data;
            let adjoint = Matrix2::from_array([data[3], -data[1], -data[2], data[0]]);
            Some(adjoint / det)
        }
    }
}

impl Default for Matrix2 {
    fn default() -> Self {
        Self::zero()
    }
}

impl cmp::PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            if (self.data[i] - other.data[i]).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl ops::Add for Matrix2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::add(&self, &rhs)
    }
}
impl ops::Add<&Self> for Matrix2 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self::add(&self, rhs)
    }
}
impl<'a, 'b> ops::Add<&'b Matrix2> for &'a Matrix2 {
    type Output = Matrix2;
    fn add(self, rhs: &'b Matrix2) -> Self::Output {
        Matrix2::add(self, rhs)
    }
}

impl ops::Sub for Matrix2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::sub(&self, &rhs)
    }
}
impl ops::Sub<&Self> for Matrix2 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        Self::sub(&self, rhs)
    }
}
impl<'a, 'b> ops::Sub<&'b Matrix2> for &'a Matrix2 {
    type Output = Matrix2;
    fn sub(self, rhs: &'b Matrix2) -> Self::Output {
        Matrix2::sub(self, rhs)
    }
}

impl ops::Mul for Matrix2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::mult_mat(&self, &rhs)
    }
}
impl ops::Mul<&Self> for Matrix2 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self::Output {
        Self::mult_mat(&self, rhs)
    }
}
impl<'a, 'b> ops::Mul<&'b Matrix2> for &'a Matrix2 {
    type Output = Matrix2;
    fn mul(self, rhs: &'b Matrix2) -> Self::Output {
        Matrix2::mult_mat(self, rhs)
    }
}

impl ops::Mul<f64> for Matrix2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix2::mult_scal(&self, rhs)
    }
}
impl ops::Mul<f64> for &Matrix2 {
    type Output = Matrix2;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix2::mult_scal(self, rhs)
    }
}

impl ops::Div<f64> for Matrix2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Matrix2::div_scal(&self, rhs)
    }
}
impl ops::Div<f64> for &Matrix2 {
    type Output = Matrix2;
    fn div(self, rhs: f64) -> Self::Output {
        Matrix2::div_scal(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_and_inspection() {
        let m = Matrix2::from_array([-3.0, 5.0, 1.0, -2.0]);

        assert_eq!(m.get(0, 0), Some(-3.0));
        assert_eq!(m.get(0, 1), Some(5.0));
        assert_eq!(m.get(1, 0), Some(1.0));
        assert_eq!(m.get(1, 1), Some(-2.0));
    }

    #[test]
    fn equality_two_matrices() {
        let a = Matrix2::from_array([1.0, 2.0, 3.0, 4.0]);
        let b = Matrix2::from_array([1.000005, 2.000005, 3.000005, 4.000005]);
        let c = Matrix2::from_array([1.000005, 2.000005, 3.000005, 4.00005]);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn multiplication() {
        let matrix_a = Matrix2::from_array([1.0, 2.0, 3.0, 4.0]);
        let matrix_b = Matrix2::from_array([-2.0, 1.0, 2.0, 3.0]);
        let result = Matrix2::from_array([2.0, 7.0, 2.0, 15.0]);

        assert_eq!(matrix_a * matrix_b, result);
    }

    #[test]
    fn multiplicative_identity() {
        let identity = Matrix2::identity();
        let matrix_a = Matrix2::from_array([1.0, 3.0, 8.0, 2.0]);
        assert_eq!(&identity * &matrix_a, matrix_a);
    }

    #[test]
    fn transpose() {
        let matrix_a = Matrix2::from_array([1.0, 5.0, -3.0, 2.0]);
        let result = Matrix2::from_array([1.0, -3.0, 5.0, 2.0]);
        assert_eq!(matrix_a.transpose(), result);
    }

    #[test]
    fn determinant() {
        let matrix_a = Matrix2::from_array([1.0, 5.0, -3.0, 2.0]);
        assert_eq!(matrix_a.det(), 17.0);
    }
}
