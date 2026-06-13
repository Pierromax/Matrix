use crate::core::*;
use crate::vector::*;
use crate::matrix::*;

mod core;
mod matrix;
mod vector;

fn ex00() {
    println!("ex00:\n");

    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(v);
    println!("1. {}", u);
    // [7.0]
    // [10.0]
    println!("expected value: \n[7.0]\n[10.0]\n");
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(v);
    println!("2. {}", u);
    // [-3.0]
    // [-4.0]
    println!("expected value: \n[-3.0]\n[-4.0]\n");
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    println!("3. {}", u);
    // [4.0]
    // [6.0]
    println!("expected value: \n[4.0]\n[6.0]\n");
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(v);

    println!("4. {}", u);
    // [8.0, 6.0]
    println!("expected value: \n[8.0, 6.0]\n[1.0, 6.0]\n");
    // [1.0, 6.0]
    println!("expected value: \n[1.0, 6.0]\n");
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(v);
    println!("5. {}", u);
    // [-6.0, -2.0]
    println!("expected value: \n[-6.0, -2.0]\n[5.0, 2.0]\n");
    // [5.0, 2.0]
    println!("expected value: \n[5.0, 2.0]\n");
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("6. {}", u);
    // [2.0, 4.0]
    println!("expected value: \n[2.0, 4.0]\n[6.0, 8.0]\n");
    // [6.0, 8.0]
    println!("expected value: \n[6.0, 8.0]\n");
}

fn ex01() {
    println!("ex01:\n");

    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("1. {}", linear_combination(&[e1, e2, e3], &[10., -2., 0.5]));
    println!("expected value: \n[10.]\n[-2.]\n[0.5]\n");
    // [10.]
    // [-2.]
    // [0.5]
    println!("2. {}", linear_combination(&[v1, v2], &[10., -2.]));
    println!("expected value: \n[10.]\n[0.]\n[230.]\n");
    // [10.]
    // [0.]
    // [230.]
}

fn ex02() {
    println!("\n=====ex02=====");
    println!("1. {}", lerp(0., 1., 0.));
    // 0.0
    println!("expected value = 0.0\n");
    println!("2, {}", lerp(0., 1., 1.));
    // 1.0
    println!("expected value = 1.0\n");
    println!("3. {}", lerp(0., 1., 0.5));
    // 0.5
    println!("expected value = 0.5\n");
    println!("4. {}", lerp(21., 42., 0.3));
    // 27.3
    println!("expected value = 27.3\n");
    println!(
        "5. {}\n",
        lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3)
    );
    // [2.6]
    // [1.3]
    println!("expected value: \n[2.6]\n[1.3]\n");
    println!(
        "6. {}",
        lerp(
            Matrix::from([[2., 1.], [3., 4.]]),
            Matrix::from([[20., 10.], [30., 40.]]),
            0.5
        )
    );
    println!("expected value: \n[[11., 5.5]\n[16.5, 22.]]\n");
    // [[11., 5.5]
    // [16.5, 22.]]
}

fn ex03() {
    println!("\n====ex03=====");
    let  u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("1. {}", u.dot(&v));
    // 0.0
    println!("expected value = 0.0\n");
    let  u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("2. {}", u.dot(&v));
    // 2.0
    println!("expected value = 2.0\n");
    let  u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("3. {}", u.dot(&v));
    // 9.0
    println!("expected value = 9.0\n");
}

fn ex04() {
    println!("\n=====ex04=====");
    let u = Vector::from([0., 0., 0.]);
    println!("1. {},{},{}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    println!("expected value = 0.0, 0.0, 0.0\n");
    let u = Vector::from([1., 2., 3.]);
    println!("2. {},{},{}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    println!("expected value = 6.0, 3.74165738, 3.0\n");
    let u = Vector::from([-1., -2.]);
    println!("4. {},{},{}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
    println!("expected value = 3.0, 2.236067977, 2.0\n");
}

fn ex05(){
    println!("\n=====ex05====");
    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("1. {}", angle_cos(&u, &v));
    // 1.0
    println!("expected value = 1.0\n");
    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("2. {}", angle_cos(&u, &v));
    // 0.0
    println!("expected value = 0.0\n");
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([ 1., -1.]);
    println!("3. {}", angle_cos(&u, &v));
    // -1.0
    println!("expected value = -1.0\n");
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("4. {}", angle_cos(&u, &v));
    // 1.0
    println!("expected value = 1.0\n");
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("5. {}", angle_cos(&u, &v));
    // 0.974631846
    println!("expected value = 0.974631846\n");
}

fn ex06(){
    println!("\n=====ex06====");
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
}

fn ex07() {
    println!("\n=====ex07====");

    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from([4., 2.]);
    println!("1. {}", u.mul_vec(v));
    // [4.]
    // [2.]
    println!("expected value: \n[4.]\n[2.]\n");

    let mut u = Matrix::from([[2., 0.], [0., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("2. {}", u.mul_vec(v));
    // [8.]
    // [4.]
    println!("expected value: \n[8.]\n[4.]\n");

    let mut u = Matrix::from([[2., -2.], [-2., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("3. {}", u.mul_vec(v));
    // [4.]
    // [-4.]
    println!("expected value: \n[4.]\n[-4.]\n");

    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[1., 0.], [0., 1.]]);
    println!("4. {}", u.mul_mat(v));
    // [1., 0.]
    // [0., 1.]
    println!("expected value: \n[1., 0.]\n[0., 1.]\n");

    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("5. {}", u.mul_mat(v));
    // [2., 1.]
    // [4., 2.]
    println!("expected value: \n[2., 1.]\n[4., 2.]\n");

    let mut u = Matrix::from([[3., -5.], [6., 8.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("6. {}", u.mul_mat(v));
    // [-14., -7.]
    // [44., 22.]
    println!("expected value: \n[-14., -7.]\n[44., 22.]\n");
}

fn ex08() {
    println!("\n=====ex08====");
    let u = Matrix::from([
[1., 0.],
[0., 1.],
]);
println!("{}", u.trace());
// 2.0
let u = Matrix::from([
[2., -5., 0.],
[4., 3., 7.],
[-2., 3., 4.],
]);
println!("{}", u.trace());
// 9.0
let u = Matrix::from([
[-2., -8., 4.],
[1., -23., 4.],
[0., 6., 4.],
]);
println!("{}", u.trace());
// -21.0
}

fn main() {
    // ex00();
    // ex01();
    // ex02();
    // ex03();
    // ex04();
    // ex05();
    // ex06();
    ex07();
    ex08();
}
