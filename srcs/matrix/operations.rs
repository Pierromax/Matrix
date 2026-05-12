use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use crate::matrix::Matrix;

impl<K: Clone + Add<Output = K> + Sub<Output = K> + Mul<Output = K>> Matrix<K>
{
    pub fn add(&mut self, other: Matrix<K>)
    {
        if self.rows != other.rows || self.cols != other.cols {
            return println!("undefined behaviour: cannot perform addition of different size matrix\n");
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() + other.data[i].clone();
        }
    }

    pub fn sub(&mut self, other: Matrix<K>)
    {

        if self.rows != other.rows || self.cols != other.cols {
            return println!("undefined behaviour: cannot perform substraction of different size matrix\n");
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() - other.data[i].clone();
        }
    }

    pub fn scl(&mut self, scale: K)
    {
        for i in 0..self.data.len(){
            self.data[i] = self.data[i].clone() * scale.clone();
        }       
    }
}

