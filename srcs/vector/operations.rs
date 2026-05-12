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

pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where K: Clone + Add<Output = K>  + Mul<Output = K> + Sub
{
    let mut result: Vector<K> = u[0].clone();

    if u.len() != coefs.len() {
        panic!("undefined behaviour: cannot perform linear combination of different size Vector \n");
    }
    for v in u.iter() {
        if v.data.len() != result.len() {
            panic!("undefined behaviour: vector size mismatch");
        }
    }

    result.scl(coefs[0].clone());
    for (vec, coef)  in u.iter().zip(coefs.iter()).skip(1){
        let mut tmp = vec.clone();
        tmp.scl(coef.clone());
        result.add(tmp);
    }
    result
}