use std::ops::*;
use std::default::Default;

pub trait Scalar:
    Default +
    Clone + 
    Copy +
    PartialOrd +
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> +
    Neg<Output = Self> +
    Div<Output = Self> +
    AddAssign + From<f32>
{
    fn abs(&self) -> Self;
    fn sqrt(&self) -> Self;
}

impl<T> Scalar for T
where
    T: Clone + 
    Default +
    Copy +
    PartialOrd +
    Add<Output = T> + 
    Sub<Output = T> + 
    Mul<Output = T> +
    Neg<Output = T> +
    Div<Output = T> +
    AddAssign +
    From<f32>
{
    fn abs(&self) -> Self {
        if self < &T::default() {
            -self.clone()
        } else {
            self.clone()
        }
    }
    fn sqrt(&self) -> Self {
        if self <= &T::default() {
            return T::default()
        }

        let two = T::from(2.0);

        // estimation initiale
        let mut x = self.clone() / two.clone();

        // nombre fixe d'itérations
        for _ in 0..20 {
            x = (x.clone() + self.clone() / x.clone()) / two.clone();
        }
        x
    }
}

pub trait LinearOps<K>
where K: Scalar
{
    fn add(&mut self, other: Self);
    fn sub(&mut self, other: Self);
    fn scl(&mut self, scale: K);
}

impl LinearOps<f32> for f32 {
    fn add(&mut self, other: Self) {
        *self = *self + other;
    }

    fn sub(&mut self, other: Self) {
        *self = *self - other;
    }

    fn scl(&mut self, scale: f32) {
        *self = *self * scale;
    }
}
