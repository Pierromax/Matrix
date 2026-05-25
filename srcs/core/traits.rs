use std::ops::{Add, Sub, Mul};

pub trait Scalar:
    Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> {}

impl<T> Scalar for T
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
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