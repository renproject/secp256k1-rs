#![feature(test)]

extern crate test;

use secp256k1::scalar::Scalar;
use test::Bencher;

#[bench]
fn bench_scalar_add(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    let y = Scalar::new_random_using_thread_rng();
    b.iter(|| r = x + y);
}

#[bench]
fn bench_scalar_add_mut(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    let y = Scalar::new_random_using_thread_rng();
    b.iter(|| r.add_mut(&x, &y));
}

#[bench]
fn bench_scalar_add_assign(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    b.iter(|| r += x);
}

#[bench]
fn bench_scalar_add_assign_mut(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    b.iter(|| r.add_assign_mut(&x));
}

#[bench]
fn bench_scalar_sub(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    let y = Scalar::new_random_using_thread_rng();
    b.iter(|| r = x - y);
}

#[bench]
fn bench_scalar_sub_mut(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    let y = Scalar::new_random_using_thread_rng();
    b.iter(|| r.sub_mut(&x, &y));
}

#[bench]
fn bench_scalar_sub_assign(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    b.iter(|| r -= x);
}

#[bench]
fn bench_scalar_mul(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    let y = Scalar::new_random_using_thread_rng();
    b.iter(|| r = x * y);
}

#[bench]
fn bench_scalar_mul_mut(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    let y = Scalar::new_random_using_thread_rng();
    b.iter(|| r.mul_mut(&x, &y));
}

#[bench]
fn bench_scalar_mul_assign(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    b.iter(|| r *= x);
}

#[bench]
fn bench_scalar_mul_assign_mut(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    b.iter(|| r.mul_assign_mut(&x));
}

#[bench]
fn bench_scalar_div(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    let y = Scalar::new_random_using_thread_rng();
    b.iter(|| r = x / y);
}

#[bench]
fn bench_scalar_div_mut(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    let y = Scalar::new_random_using_thread_rng();
    b.iter(|| r.divide_mut(&x, &y));
}

#[bench]
fn bench_scalar_div_assign(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    b.iter(|| r /= x);
}

#[bench]
fn bench_scalar_neg(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    b.iter(|| r = -x);
}

#[bench]
fn bench_scalar_neg_mut(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    b.iter(|| r.negate_mut(&x));
}

#[bench]
fn bench_scalar_neg_assign(b: &mut Bencher) {
    let mut r = Scalar::new_random_using_thread_rng();
    b.iter(|| r = -r);
}

#[bench]
fn bench_scalar_neg_assign_mut(b: &mut Bencher) {
    let mut r = Scalar::new_random_using_thread_rng();
    b.iter(|| r.negate_assign_mut());
}

#[bench]
fn bench_scalar_inverse(b: &mut Bencher) {
    let mut r = Scalar::default();
    let x = Scalar::new_random_using_thread_rng();
    b.iter(|| r.inverse(&x));
}
