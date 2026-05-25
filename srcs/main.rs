use crate::core::operations::lerp;
use crate::core::traits::LinearOps;
use crate::vector::operations::linear_combination;
use crate::{matrix::Matrix, vector::Vector};

mod core;
mod matrix;
mod vector;

fn ex00() {
    println!("ex00:\n");

    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(v);
    println!("{}", u);
    // [7.0]
    // [10.0]
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(v);
    println!("{}", u);
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    println!("{}", u);
    // [4.0]
    // [6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(v);

    println!("{}", u);
    // [8.0, 6.0]
    // [1.0, 6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(v);
    println!("{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]
}

fn ex01() {
    println!("ex01:\n");

    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("{}", linear_combination(&[e1, e2, e3], &[10., -2., 0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}", linear_combination(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]
}

fn ex02() {
    println!("ex02:\n");
    println!("{}", lerp(0., 1., 0.));
    // 0.0
    println!("{}", lerp(0., 1., 1.));
    // 1.0
    println!("{}", lerp(0., 1., 0.5));
    // 0.5
    println!("{}", lerp(21., 42., 0.3));
    // 27.3
    println!(
        "{}\n",
        lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3)
    );
    // [2.6]
    // [1.3]
    println!(
        "{}",
        lerp(
            Matrix::from([[2., 1.], [3., 4.]]),
            Matrix::from([[20., 10.], [30., 40.]]),
            0.5
        )
    );
    // [[11., 5.5]
    // [16.5, 22.]]
}

fn ex03() {
    println!("ex03:\n");
    let  u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(v));
    // 0.0
    let  u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(v));
    // 2.0
    let  u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("{}", u.dot(v));
    // 9.0
}
fn main() {
    ex00();
    ex01();
    ex02();
    ex03();
}
