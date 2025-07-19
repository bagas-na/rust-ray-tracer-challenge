use std::{cmp, fmt, ops};

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

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row >= 2 || col >= 2 {
            return None;
        }
        let index = row * 2 + col;
        Some(self.data[index])
    }

    pub fn set(&mut self, row: usize, col: usize, val: f64) -> Result<(), Matrix2Error> {
        if row >= 2 || col >= 2 {
            return Err(Matrix2Error::OutOfBounds);
        }
        let index = row * 2 + col;
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
}

impl Default for Matrix2 {
    fn default() -> Self {
        Self::new()
    }
}

impl cmp::PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..9 {
            if self.data[i] != other.data[i] {
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
impl<'a, 'b> ops::Add<&'b Self> for &'a Matrix2 {
    type Output = Matrix2;
    fn add(self, rhs: &'b Self) -> Self::Output {
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
impl<'a, 'b> ops::Sub<&'b Self> for &'a Matrix2 {
    type Output = Matrix2;
    fn sub(self, rhs: &'b Self) -> Self::Output {
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
impl<'a, 'b> ops::Mul<&'b Self> for &'a Matrix2 {
    type Output = Matrix2;
    fn mul(self, rhs: &'b Self) -> Self::Output {
        Matrix2::mult_mat(self, rhs)
    }
}

#[cfg(test)]
mod tests {}
