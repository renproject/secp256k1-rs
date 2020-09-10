#![feature(test)]

extern crate test;

use secp256k1::group::fe::{self, Fe};
use test::Bencher;

#[bench]
fn bench_fe_add(b: &mut Bencher) {
    let mut r = Fe::new_random_using_thread_rng();
    let mut x = Fe::new_random_using_thread_rng();
    b.iter(|| fe::add(&mut r, &mut x));
}

#[bench]
fn bench_fe_mul(b: &mut Bencher) {
    let mut r = Fe::default();
    let mut x = Fe::new_random_using_thread_rng();
    let mut y = Fe::new_random_using_thread_rng();
    b.iter(|| fe::mul(&mut r, &mut x, &mut y));
}

#[bench]
fn bench_fe_inverse(b: &mut Bencher) {
    let mut r = Fe::new_random_using_thread_rng();
    let mut x = Fe::new_random_using_thread_rng();
    b.iter(|| fe::inv_var(&mut r, &mut x));
}

#[bench]
fn bench_fe_normalize(b: &mut Bencher) {
    let mut r = Fe::new_random_using_thread_rng();
    b.iter(|| fe::normalize_var(&mut r));
}
