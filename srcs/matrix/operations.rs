use super::Matrix;
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

