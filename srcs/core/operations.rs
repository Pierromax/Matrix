use super::*;

// interpolation lineaire dans un scope de [0, 1].
pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: LinearOps<f32> + Clone,
{
    if t < 0.0 || t > 1.0 {
        panic!("undefined behaviour: t should be in the range [0, 1] \n");
    }

    let mut u = u.clone();
    let mut v = v.clone();
    u.scl(1.0 - t);
    v.scl(t);
    u.add(v);
    u
} // u * (1 - t) + v * t

