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
    pub fn dot(&self, other: &Vector<K>) -> K {
        if self.data.len() != other.data.len() {
            panic!("undefined behaviour: cannot perform dot product of different size Vector \n");
        }
        let mut result = K::default();
        let a = &self.data;
        let b = &other.data;
        for i in 0..a.len() {
            result += a[i] * b[i];
        }
        result
    }
    //norme taxicab (calcul distance a parcourir d'un taxi dans les rues quadrille de manhattan)
    pub fn norm_1(&self) -> K{
        let mut norm: K = K::default();

        for i in self.data.iter(){
            norm += i.abs();
        }
        norm
    }
    //norme euclidienne (calcul disatnce a vol d'oiseau)
    pub fn norm(&self) -> K{
        let mut norm: K = K::default();

        for &i in self.data.iter(){
            norm += i * i;
        }
        norm.sqrt()
    }
    //norme infinie(calucul de la plus grande composante du vecteur)
    pub fn norm_inf(&self) -> K{
        let mut norm = K::default();

        for &i in self.data.iter(){
            let abs_i = i.abs();
            if abs_i > norm {
                norm = abs_i;
            }
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

pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> K
where
    K: Scalar,
{
    let denom = u.norm() * v.norm();
    if denom == K::default(){   
        return K::default();
    }
    u.dot(v) / denom
}

pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where 
    K: Scalar,
{
    let is_3d: bool = u.data.len() == 3 && v.data.len() == 3;

    if !is_3d {
        panic!("undefined behaviour: cross product is only defined for 3D vectors \n");
    }

    let a = u.data.clone();
    let b = v.data.clone();

    let x = a[1] * b[2] - a[2] * b[1];
    let y = a[2] * b[0] - a[0] * b[2];
    let z = a[0] * b[1] - a[1] * b[0];

    Vector::from([x, y, z])
}
