use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use crate::vector::Vector;

impl<K: Clone + Add<Output = K> + Sub<Output = K> + Mul<Output = K>> Vector <K>
{
    pub fn add(&mut self, other: Vector <K>)
    {
        if self.data.len() != other.data.len() {
            return println!("undefined behaviour: cannot perform addition of different size Vector \n");
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() + other.data[i].clone();
        }
    }

    pub fn sub(&mut self, other: Vector <K>)
    {

        if self.data.len() != other.data.len() {
            return println!("undefined behaviour: cannot perform substraction of different size Vector \n");
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