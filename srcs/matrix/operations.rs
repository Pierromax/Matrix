use super::Matrix;
use crate::vector::Vector;
use crate::core::{LinearOps, Scalar};

impl<K : Scalar> LinearOps<K> for Matrix<K>
{
    fn add(&mut self, other: Matrix<K>)
    {
        if self.rows != other.rows || self.cols != other.cols {
            return println!("undefined behaviour: cannot perform addition of different size matrix\n");
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() + other.data[i].clone();
        }
    }

    fn sub(&mut self, other: Matrix<K>)
    {

        if self.rows != other.rows || self.cols != other.cols {
            return println!("undefined behaviour: cannot perform substraction of different size matrix\n");
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() - other.data[i].clone();
        }
    }

    fn scl(&mut self, scale: K)
    {
        for i in 0..self.data.len(){
            self.data[i] = self.data[i].clone() * scale.clone();
        }       
    }
}

impl<K: Scalar> Matrix::<K>
{
    pub fn mul_vec(&mut self, vec: Vector<K>) -> Vector<K>
    {
        if self.cols != vec.data.len() {
            panic!("undefined behaviour: cannot perform matrix-vector multiplication of incompatible size\n");
        }
        
        let mut result_data: Vec<K> = Vec::with_capacity(self.rows);
        
        let a = self.data.clone();
        let b = vec.data.clone();

        for i in 0..self.rows {
            let mut sum: K = K::default();
            for j in 0..self.cols {
                sum += a[i * self.cols + j] * b[j];
            }
            result_data.push(sum);
        }
        Vector { data: result_data }
    }

    pub fn mul_mat(&mut self, other: Matrix<K>) -> Matrix<K>
    {
        if self.cols != other.rows {
            panic!("undefined behaviour: cannot perform matrix-matrix multiplication of incompatible size\n");
        }
        
        let mut result_data: Vec<K> = Vec::with_capacity(self.rows * other.cols);
        
        let a = self.data.clone();
        let b = other.data.clone();

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum: K = K::default();
                for k in 0..self.cols {
                    sum += a[i * self.cols + k] * b[k * other.cols + j];
                }
                result_data.push(sum);
            }
        }
        Matrix { data: result_data, rows: self.rows, cols: other.cols }
    }

    pub fn trace(&mut self) -> K
    {
        if self.rows != self.cols {
            panic!("undefined behaviour: trace is only defined for square matrices\n");
        }
        let mut sum = K::default();
        let data = self.data.clone();
        for i in 0..self.rows {
            sum += data[i * self.cols + i].clone();
        }
        sum
    }
}   