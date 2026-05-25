use super::{Len, Vector};
use crate::core::{LinearOps, Scalar};

impl<K: Scalar> LinearOps<K> for Vector<K> {
    fn add(&mut self, other: Vector<K>) {
        if self.data.len() != other.data.len() {
            return println!(
                "undefined behaviour: cannot perform addition of different size Vector \n"
            );
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() + other.data[i].clone();
        }
    }
    fn sub(&mut self, other: Vector<K>) {
        if self.data.len() != other.data.len() {
            return println!(
                "undefined behaviour: cannot perform substraction of different size Vector \n"
            );
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() - other.data[i].clone();
        }
    }

    fn scl(&mut self, scale: K) {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() * scale.clone();
        }
    }
}

impl<K: Scalar> Vector<K> {
    /** Implementation de la methode de produit scalaire entre deux vecteurs de meme taille
     * si result = 0 alors les vecteurs sont orthogonaux (perpendiculaires dans le plan)
     * si result > 0 alors les vecteurs sont dans le meme sens
     * si result < 0 alors les vecteurs sont dans le sens oppose */
    pub fn dot(&self, other: Vector<K>) -> K {
        if self.data.len() != other.data.len() {
            panic!("undefined behaviour: cannot perform dot product of different size Vector \n");
        }
        let mut result = self.data[0].clone() * other.data[0].clone();
        for i in 1..self.data.len() {
            result = result + self.data[i].clone() * other.data[i].clone();
        }
        result
    }

    pub fn norm_1(&self) -> K{
        let mut norm;

        for i in self.data.iter(){
            norm += i.abs();
        }
        norm
    }
}

pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Scalar,
{
    let mut result: Vector<K> = u[0].clone();

    if u.len() != coefs.len() {
        panic!(
            "undefined behaviour: cannot perform linear combination of different size Vector \n"
        );
    }

    for v in u.iter() {
        if v.data.len() != result.len() {
            panic!("undefined behaviour: vector size mismatch");
        }
    }

    result.scl(coefs[0].clone());
    for (vec, coef) in u.iter().zip(coefs.iter()).skip(1) {
        let mut tmp = vec.clone();
        tmp.scl(coef.clone());
        result.add(tmp);
    }
    result
}
