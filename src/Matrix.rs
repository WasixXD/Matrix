use rand::Rng;
use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign};

#[warn(unused_mut)]
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

#[derive(Debug, PartialEq)]
pub enum MatrixError {
    InvalidDimension,
    NotImplemented,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn random(&mut self) {
        let mut rng = rand::thread_rng();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let random_number = rng.gen_range(1f64..5.0f64);
                self.data[row][col] = random_number;
            }
        }
    }

    pub fn add(&mut self, other: &Matrix) -> Result<bool, MatrixError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MatrixError::InvalidDimension);
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                self.data[row][col] += other.data[row][col];
            }
        }

        Ok(true)
    }

    pub fn sub(&mut self, other: &Matrix) -> Result<bool, MatrixError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MatrixError::InvalidDimension);
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                self.data[row][col] -= other.data[row][col];
            }
        }

        Ok(true)
    }

    pub fn print(self, title: String) {
        println!("{title}");
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("| {:?} | ", self.data[row][col]);
            }
            print!("\n");
        }
    }

    pub fn mul(a: &Matrix, b: &Matrix) -> Result<Matrix, MatrixError> {
        if a.cols != b.rows {
            return Err(MatrixError::InvalidDimension);
        }

        let mut new_matrix = Matrix::new(a.rows, b.cols);
        for row in 0..a.rows {
            for col in 0..b.cols {
                for k in 0..a.cols {
                    new_matrix.data[row][col] = a.data[row][k] * b.data[k][col];
                }
            }
        }

        Ok(new_matrix)
    }

    pub fn t(&mut self) {
        let mut new_matrix = Matrix::new(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                new_matrix.data[col][row] = self.data[row][col];
            }
        }

        *self = new_matrix;
    }


    // pub fn det(&self) -> Result<f64, MatrixError> {
    //     if self.rows != self.cols {
    //         return Err(MatrixError::InvalidDimension);
    //     }

    //     if self.cols == 1 && self.rows == 1 {
    //         return Ok(self.data[0][0].clone());
    //     }

    //     if self.cols == 2 && self.rows == 2 {
    //         return Ok((self.data[0][0] * self.data[1][1]) - (self.data[0][1] * self.data[1][0]));
    //     }

    //     if self.cols == 3 && self.rows == 3 {
    //         // sarrus
    //         let mut sarrus = Matrix::new(self.rows, self.cols + 2);
    //         //copy the first two cols
    //         let s_rule = &self.data[0..2].to_vec();
    //         sarrus.data = self.data.clone();
    //         for i in s_rule.iter() {
    //             sarrus.data.push(i.to_vec());
    //         }

    //         let det = (sarrus.data[0][0] * sarrus.data[1][1] * sarrus.data[2][2]) - ()
    //     }

    //     Err(MatrixError::NotImplemented)
    // }
}

impl AddAssign<f64> for Matrix {
    fn add_assign(&mut self, other: f64) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.data[row][col] = self.data[row][col] + other;
            }
        }
    }
}

impl SubAssign<f64> for Matrix {
    fn sub_assign(&mut self, other: f64) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.data[row][col] = self.data[row][col] - other;
            }
        }
    }
}

impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, other: f64) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.data[row][col] = self.data[row][col] * other;
            }
        }
    }
}

impl DivAssign<f64> for Matrix {
    fn div_assign(&mut self, other: f64) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.data[row][col] = self.data[row][col] / other;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_matrix() {
        let matrix = Matrix::new(2, 2);
        let mock = Matrix {
            rows: 2 as usize,
            cols: 2 as usize,
            data: vec![vec![0.0; 2]; 2],
        };
        assert_eq!(matrix, mock);
    }

    #[test]
    fn random_matrix() {
        let mut matrix = Matrix::new(2, 2);
        matrix.random();

        let nonzero = if matrix.data[0][0] != 0.0 {
            true
        } else {
            false
        };

        assert!(nonzero);
    }

    #[test]
    fn add_scalar() {
        let scalar = 5.0f64;
        let mut matrix = Matrix::new(2, 2);

        matrix += scalar;
        let valor = matrix.data[0][0];

        assert_eq!(valor, scalar);
    }

    #[test]
    fn add_two_matrixes() {
        let mut a = Matrix::new(2, 2);
        let mut b = Matrix::new(2, 2);

        a += 1f64;
        b += 1f64;
        let _ = a.add(&b);

        let valor = a.data[0][0];

        assert_eq!(valor, 2.0);
    }

    #[test]
    fn add_two_matrixes_error() {
        let mut a = Matrix::new(3, 2);
        let mut b = Matrix::new(2, 3);

        let result = a.add(&b);

        assert_eq!(result, Err(MatrixError::InvalidDimension));
    }

    #[test]
    fn sub_two_matrixes() {
        let mut a = Matrix::new(2, 2);
        let mut b = Matrix::new(2, 2);

        a += 4f64;
        b += 3f64;

        let ok = a.sub(&b).unwrap();

        assert!(ok);
    }

    #[test]
    fn sub_two_matrixes_fail() {
        let mut a = Matrix::new(3, 2);
        let mut b = Matrix::new(2, 2);

        let ok = a.sub(&b);
        assert_eq!(ok, Err(MatrixError::InvalidDimension));
    }

    #[test]
    fn sub_scalar() {
        let mut a = Matrix::new(2, 2);

        a += 3f64;

        a -= 2f64;

        let valor = a.data[0][0];

        assert_eq!(valor, 1.0f64);
    }

    #[test]
    fn mul_scalar() {
        let mut a = Matrix::new(2, 2);

        a += 3f64;

        a *= 2f64;

        let valor = a.data[0][0];

        assert_eq!(valor, 6.0f64);
    }

    #[test]
    fn mul_two_matrixes() {
        let mut a = Matrix::new(2, 2);
        let mut b = Matrix::new(2, 2);

        a += 2f64;
        b += 3f64;

        let c = Matrix::mul(&a, &b).unwrap();

        let valor = c.data[0][0];

        assert_eq!(valor, 6f64);
    }

    #[test]
    fn transpose() {
        let mut a = Matrix::new(2, 3);
        let mut b = Matrix::new(3, 2);

        a.random();
        a.t();

        assert_eq!(a.rows, b.rows);
    }

    #[test]
    fn div_scalar() {
        let scalar = 2f64;
        let mut a = Matrix::new(2, 2);
        a += 8f64;

        a /= scalar;

        let valor = a.data[0][0];

        assert_eq!(valor, 4.0f64);
    }
}
