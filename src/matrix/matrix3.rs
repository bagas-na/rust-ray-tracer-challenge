use std::{cmp, fmt, ops};

use crate::{EPSILON, matrix::Matrix2};

#[derive(Debug)]
pub struct Matrix3 {
    data: [f64; 9], // row major
}

pub enum Matrix3Error {
    OutOfBounds,
}

impl fmt::Display for Matrix3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = &self.data;
        write!(
            f,
            concat!(
                "| {:>10.4} | {:>10.4} | {:>10.4}|\n",
                "| {:>10.4} | {:>10.4} | {:>10.4}|\n",
                "| {:>10.4} | {:>10.4} | {:>10.4}|",
            ),
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
        )
    }
}

impl Matrix3 {
    /// Creates a 4x4 zero Matrix
    /// identical to Matrix4::new()
    pub fn zero() -> Self {
        Self { data: [0.0; 9] }
    }

    /// Creates a 4x4 zero Matrix
    /// identical to Matrix4::zero()
    pub fn new() -> Self {
        Self { data: [0.0; 9] }
    }

    /// Create a 3×3 matrix from a flat array of 9 elements.
    /// The elements are arranged in row-major order.
    /// Matrix = | arr[0] arr[1] arr[2] |
    ///          | arr[3] arr[4] arr[5] |
    ///          | arr[6] arr[7] arr[8] |
    pub fn from_array(arr: [f64; 9]) -> Self {
        Self { data: arr }
    }

    /// Create a 3×3 matrix from a flat array of 9 elements.
    /// The elements are arranged in column-major order.
    /// Matrix = | arr[0] arr[3] arr[6] |
    ///          | arr[1] arr[4] arr[7] |
    ///          | arr[2] arr[5] arr[8] |
    pub fn from_array_by_col(arr: [f64; 9]) -> Self {
        Self::from_array([
            arr[0], arr[3], arr[6], // first row
            arr[1], arr[4], arr[7], // second row
            arr[2], arr[5], arr[8], // third row
        ])
    }

    /// Get element at given row and column. 0-indexed
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row >= 3 || col >= 3 {
            return None;
        }
        let index = row * 3 + col;
        Some(self.data[index])
    }

    /// Set element at given row and column. 0-indexed
    pub fn set(&mut self, row: usize, col: usize, val: f64) -> Result<(), Matrix3Error> {
        if row >= 3 || col >= 3 {
            return Err(Matrix3Error::OutOfBounds);
        }
        let index = row * 3 + col;
        self.data[index] = val;
        Ok(())
    }

    pub fn identity() -> Self {
        let indices = [0, 4, 8];
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
            let row = i / 3;
            let col = i % 3;
            let mut sum = 0.0;
            for j in 0..3 {
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
        let mut transposed = [0.0; 9];
        for i in 0..3 {
            for j in 0..3 {
                transposed[i * 3 + j] = data[j * 3 + i];
            }
        }
        Self { data: transposed }
    }

    /// Extracts a submatrix (2x2 matrix) of a 3x3 matrix given row and column
    /// to be removed
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let mut submatrix_data = [0.0; 4];
        let mut index: usize = 0;

        for (i, val) in self.data.iter().enumerate() {
            if (i / 3 == row) || (i % 3 == col) {
                continue;
            } else {
                submatrix_data[index] = *val;
                index += 1;
            }
        }
        Matrix2::from_array(submatrix_data)
    }

    /// Computes the minor of an element at given row and column,
    /// Which is the determinant of
    /// ```text
    /// matrix3.submatrix(row: usize, column: usize)
    /// ```
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let submatrix = self.submatrix(row, col);
        submatrix.det()
    }

    /// Computes the cofactor of an element at given row (i) and column (i),
    /// ```text
    /// Cofactor = (-1)^(i + j) matrix3.minor(row: usize, column: usize)
    /// ```
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if let 0 = (row + col) % 2 {
            minor
        } else {
            -minor
        }
    }

    /// Calculate the determinant of the matrix. Given:
    /// ```text
    /// M = ⎡a  b  c⎤
    ///     ⎢d  e  f⎥
    ///     ⎣g  h  i⎦
    ///
    /// det(M) = a * M.cofactor(0,0) + b * M.cofactor(0, 1) + c * M.cofactor(0, 2)
    /// ```
    pub fn det(&self) -> f64 {
        let data = self.data;
        data[0] * self.cofactor(0, 0)
            + data[1] * self.cofactor(0, 1)
            + data[2] * self.cofactor(0, 2)
    }

    pub fn invertible(&self) -> bool {
        self.det().abs() > EPSILON
    }

    pub fn inverse(self) -> Option<Self> {
        if let false = self.invertible() {
            None
        } else {
            let adjoint = Self::from_array(core::array::from_fn(|i| {
                let row = i / 3;
                let col = i % 3;
                self.cofactor(col, row) // inline transpose
            }));
            Some(adjoint / self.det())
        }
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Self::zero()
    }
}

impl cmp::PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..9 {
            if (self.data[i] - other.data[i]).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl ops::Add for Matrix3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::add(&self, &rhs)
    }
}
impl ops::Add<&Self> for Matrix3 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self::add(&self, rhs)
    }
}
impl<'a, 'b> ops::Add<&'b Matrix3> for &'a Matrix3 {
    type Output = Matrix3;
    fn add(self, rhs: &'b Matrix3) -> Self::Output {
        Matrix3::add(self, rhs)
    }
}

impl ops::Sub for Matrix3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::sub(&self, &rhs)
    }
}
impl ops::Sub<&Self> for Matrix3 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        Self::sub(&self, rhs)
    }
}
impl<'a, 'b> ops::Sub<&'b Matrix3> for &'a Matrix3 {
    type Output = Matrix3;
    fn sub(self, rhs: &'b Matrix3) -> Self::Output {
        Matrix3::sub(self, rhs)
    }
}

impl ops::Mul for Matrix3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::mult_mat(&self, &rhs)
    }
}
impl ops::Mul<&Self> for Matrix3 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self::Output {
        Self::mult_mat(&self, rhs)
    }
}
impl<'a, 'b> ops::Mul<&'b Matrix3> for &'a Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: &'b Matrix3) -> Self::Output {
        Matrix3::mult_mat(self, rhs)
    }
}

impl ops::Mul<f64> for Matrix3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix3::mult_scal(&self, rhs)
    }
}
impl ops::Mul<f64> for &Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix3::mult_scal(self, rhs)
    }
}

impl ops::Div<f64> for Matrix3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Matrix3::div_scal(&self, rhs)
    }
}
impl ops::Div<f64> for &Matrix3 {
    type Output = Matrix3;
    fn div(self, rhs: f64) -> Self::Output {
        Matrix3::div_scal(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_and_inspection() {
        let m = Matrix3::from_array([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(m.get(0, 0), Some(-3.0));
        assert_eq!(m.get(1, 0), Some(1.0));
        assert_eq!(m.get(1, 2), Some(-7.0));
        assert_eq!(m.get(2, 2), Some(1.0));
    }

    #[test]
    fn equality_two_matrices() {
        let a = Matrix3::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let b = Matrix3::from_array([
            1.000005, 2.000005, 3.000005, 4.000005, 5.000005, 6.000005, 7.000005, 8.000005,
            9.000005,
        ]);
        let c = Matrix3::from_array([
            1.000005, 2.000005, 3.000005, 4.000005, 5.000005, 6.000005, 7.000005, 8.000005, 9.00005,
        ]);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn multiplication() {
        let matrix_a = Matrix3::from_array([5.0, 3.0, 7.0, -3.0, -4.0, -5.0, -6.0, -8.0, 9.0]);
        let matrix_b = Matrix3::from_array([4.0, 4.0, -5.0, -7.0, 1.0, 9.0, 9.0, 1.0, -4.0]);
        let result = Matrix3::from_array([62., 30., -26., -29., -21., -1., 113., -23., -78.]);

        assert_eq!(matrix_a * matrix_b, result);
    }

    #[test]
    fn multiplicative_identity() {
        let identity = Matrix3::identity();
        let matrix_a = Matrix3::from_array([4.0, 4.0, -5.0, -7.0, 1.0, 9.0, 9.0, 1.0, -4.0]);
        assert_eq!(&identity * &matrix_a, matrix_a);
    }

    #[test]
    fn transpose() {
        let matrix_a = Matrix3::from_array([4.0, 4.0, -5.0, -7.0, 1.0, 9.0, 9.0, 1.0, -4.0]);
        let result = Matrix3::from_array([4.0, -7.0, 9.0, 4.0, 1.0, 1.0, -5.0, 9.0, -4.0]);
        assert_eq!(matrix_a.transpose(), result);
    }

    #[test]
    fn submatrix() {
        let matrix_a = Matrix3::from_array([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        let result = Matrix2::from_array([-3.0, 2.0, 0.0, 6.0]);
        assert_eq!(matrix_a.submatrix(0, 2), result);
    }

    #[test]
    fn minor() {
        let matrix_a = Matrix3::from_array([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        let submatrix_b = matrix_a.submatrix(1, 0);
        assert_eq!(submatrix_b.det(), 25.0);
        assert_eq!(matrix_a.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor() {
        let matrix_a = Matrix3::from_array([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(matrix_a.minor(0, 0), -12.0);
        assert_eq!(matrix_a.cofactor(0, 0), -12.0);
        assert_eq!(matrix_a.minor(1, 0), 25.0);
        assert_eq!(matrix_a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant() {
        let matrix_a = Matrix3::from_array([1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        assert_eq!(matrix_a.cofactor(0, 0), 56.);
        assert_eq!(matrix_a.cofactor(0, 1), 12.);
        assert_eq!(matrix_a.cofactor(0, 2), -46.);
        assert_eq!(matrix_a.det(), -196.);
    }
}
