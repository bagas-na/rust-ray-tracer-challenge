use crate::tuple::Tuple;
use std::{cmp, fmt, ops};

#[derive(Debug)]
pub struct Matrix4 {
    data: [f64; 16], // row major
}

pub enum Matrix4Error {
    OutOfBounds,
}

impl fmt::Display for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = &self.data;
        write!(
            f,
            concat!(
                "| {:>10.4} | {:>10.4} | {:>10.4} | {:>10.4} |\n",
                "| {:>10.4} | {:>10.4} | {:>10.4} | {:>10.4} |\n",
                "| {:>10.4} | {:>10.4} | {:>10.4} | {:>10.4} |\n",
                "| {:>10.4} | {:>10.4} | {:>10.4} | {:>10.4} |"
            ),
            data[0],
            data[1],
            data[2],
            data[3],
            data[4],
            data[5],
            data[6],
            data[7],
            data[8],
            data[9],
            data[10],
            data[11],
            data[12],
            data[13],
            data[14],
            data[15]
        )
    }
}

impl Matrix4 {
    /// Creates a 4x4 zero Matrix
    /// identical to Matrix4::new()
    pub fn zero() -> Self {
        Self { data: [0.0; 16] }
    }

    /// Creates a 4x4 zero Matrix
    /// identical to Matrix4::zero()
    pub fn new() -> Self {
        Self { data: [0.0; 16] }
    }

    /// Create a matrix using 4 columnes of Tuples.
    /// Given tuples t1, t2, t3, and t4, with each elements defined as
    /// x, y, z, w (t1 -> t1.x, t1.y, t1.z, t1.w)
    /// Matrix = | t1.x | t2.x | t3.x | t4.x |
    ///          | t1.y | t2.y | t3.y | t4.y |
    ///          | t1.z | t2.z | t3.z | t4.z |
    ///          | t1.w | t2.w | t3.w | t4.w |
    pub fn from_tuples(t1: Tuple, t2: Tuple, t3: Tuple, t4: Tuple) -> Self {
        let (x1, y1, z1, w1) = t1.get();
        let (x2, y2, z2, w2) = t2.get();
        let (x3, y3, z3, w3) = t3.get();
        let (x4, y4, z4, w4) = t4.get();

        let data = [
            x1, x2, x3, x4, y1, y2, y3, y4, z1, z2, z3, z4, w1, w2, w3, w4,
        ];

        Self { data }
    }

    /// Create a matrix using 4 rows of Tuples.
    /// Given tuples t1, t2, t3, and t4, with each elements defined as
    /// x, y, z, w (t1 -> t1.x, t1.y, t1.z, t1.w)
    /// Matrix = | t1.x | t1.y | t1.z | t1.w |
    ///          | t2.x | t2.y | t2.z | t2.w |
    ///          | t3.x | t3.y | t3.z | t3.w |
    ///          | t4.x | t4.y | t4.z | t4.w |
    pub fn from_tuples_by_row(t1: Tuple, t2: Tuple, t3: Tuple, t4: Tuple) -> Matrix4 {
        let (x1, y1, z1, w1) = t1.get();
        let (x2, y2, z2, w2) = t2.get();
        let (x3, y3, z3, w3) = t3.get();
        let (x4, y4, z4, w4) = t4.get();

        let data = [
            x1, y1, z1, w1, x2, y2, z2, w2, x3, y3, z3, w3, x4, y4, z4, w4,
        ];

        Self { data }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row >= 4 || col >= 4 {
            return None;
        }
        let index = row * 4 + col;
        Some(self.data[index])
    }

    pub fn set(&mut self, row: usize, col: usize, val: f64) -> Result<(), Matrix4Error> {
        if row >= 4 || col >= 4 {
            return Err(Matrix4Error::OutOfBounds);
        }
        let index = row * 4 + col;
        self.data[index] = val;
        Ok(())
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
            let row = i / 4;
            let col = i % 4;
            let mut sum = 0.0;
            for j in 0..4 {
                sum += matrix_a.get(row, j).unwrap() * matrix_b.get(j, col).unwrap();
            }
            sum
        });

        Self { data }
    }

    fn mult_vec(matrix_a: &Self, vec_b: &Tuple) -> Tuple {
        let matrix = &matrix_a.data;
        let vec = vec_b.get();
        Tuple::new(
            matrix[0] * vec.0 + matrix[1] * vec.1 + matrix[2] * vec.2 + matrix[3] * vec.3,
            matrix[4] * vec.0 + matrix[5] * vec.1 + matrix[6] * vec.2 + matrix[7] * vec.3,
            matrix[8] * vec.0 + matrix[9] * vec.1 + matrix[10] * vec.2 + matrix[11] * vec.3,
            matrix[12] * vec.0 + matrix[13] * vec.1 + matrix[14] * vec.2 + matrix[15] * vec.3,
        )
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self::new()
    }
}

impl cmp::PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..16 {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

impl ops::Add for Matrix4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::add(&self, &rhs)
    }
}
impl ops::Add<&Self> for Matrix4 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self::add(&self, rhs)
    }
}
impl<'a, 'b> ops::Add<&'b Self> for &'a Matrix4 {
    type Output = Matrix4;
    fn add(self, rhs: &'b Self) -> Self::Output {
        Matrix4::add(self, rhs)
    }
}

impl ops::Sub for Matrix4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::sub(&self, &rhs)
    }
}
impl ops::Sub<&Self> for Matrix4 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        Self::sub(&self, rhs)
    }
}
impl<'a, 'b> ops::Sub<&'b Self> for &'a Matrix4 {
    type Output = Matrix4;
    fn sub(self, rhs: &'b Self) -> Self::Output {
        Matrix4::sub(self, rhs)
    }
}

impl ops::Mul for Matrix4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::mult_mat(&self, &rhs)
    }
}
impl ops::Mul<&Self> for Matrix4 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self::Output {
        Self::mult_mat(&self, rhs)
    }
}
impl<'a, 'b> ops::Mul<&'b Self> for &'a Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: &'b Self) -> Self::Output {
        Matrix4::mult_mat(self, rhs)
    }
}

impl ops::Mul<Tuple> for Matrix4 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        Self::mult_vec(&self, &rhs)
    }
}
impl ops::Mul<&Tuple> for Matrix4 {
    type Output = Tuple;
    fn mul(self, rhs: &Tuple) -> Self::Output {
        Self::mult_vec(&self, rhs)
    }
}
impl<'a, 'b> ops::Mul<&'b Tuple> for &'a Matrix4 {
    type Output = Tuple;
    fn mul(self, rhs: &'b Tuple) -> Self::Output {
        Matrix4::mult_vec(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix4_creation() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(5.0, 6.0, 7.0, 8.0);
        let t3 = Tuple::new(9.0, 10.0, 11.0, 12.0);
        let t4 = Tuple::new(13.0, 14.0, 15.0, 16.0);

        let m = Matrix4::from_tuples(t1, t2, t3, t4);

        assert_eq!(m.data[0], 1.0);
        assert_eq!(m.data[2], 9.0);
        assert_eq!(m.data[10], 11.0);
        assert_eq!(m.data[13], 8.0);
        assert_eq!(m.data[15], 16.0);
    }
    #[test]
    fn matrix4_creation_by_row() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(5.0, 6.0, 7.0, 8.0);
        let t3 = Tuple::new(9.0, 10.0, 11.0, 12.0);
        let t4 = Tuple::new(13.0, 14.0, 15.0, 16.0);

        let m = Matrix4::from_tuples_by_row(t1, t2, t3, t4);

        assert_eq!(m.data[0], 1.0);
        assert_eq!(m.data[2], 3.0);
        assert_eq!(m.data[10], 11.0);
        assert_eq!(m.data[13], 14.0);
        assert_eq!(m.data[15], 16.0);
    }
}
