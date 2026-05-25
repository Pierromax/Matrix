use super::*;

// interpolation lineaire dans un scope de [0, 1].
pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: LinearOps<f32> + Clone,
{
    let mut u = u.clone();
    let mut v = v.clone();
    u.scl(1.0 - t);
    v.scl(t);
    u.add(v);
    u
} // u * (1 - t) + v * t

