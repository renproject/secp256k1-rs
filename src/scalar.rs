use libc::{c_int, c_uchar, c_uint};
use rand::Rng;

#[repr(C)]
pub struct Scalar {
    limbs: [u64; 4],
}

extern "C" {
    fn secp256k1_scalar_clear_export(r: *mut Scalar);
    fn secp256k1_scalar_get_bits_export(a: *const Scalar, offset: c_uint, count: c_uint) -> c_uint;
    fn secp256k1_scalar_get_bits_var_export(
        a: *const Scalar,
        offset: c_uint,
        count: c_uint,
    ) -> c_uint;
    fn secp256k1_scalar_set_b32_export(r: *mut Scalar, bin: *const c_uchar, overflow: *mut c_int);
    fn secp256k1_scalar_set_b32_seckey_export(r: *mut Scalar, bin: *const c_uchar) -> c_int;
    fn secp256k1_scalar_set_int_export(r: *mut Scalar, v: c_uint);
    fn secp256k1_scalar_get_b32_export(bin: *mut c_uchar, a: *const Scalar);
    fn secp256k1_scalar_add_export(r: *mut Scalar, a: *const Scalar, b: *const Scalar) -> c_int;
    fn secp256k1_scalar_cadd_bit_export(r: *mut Scalar, bit: c_uint, flag: c_int);
    fn secp256k1_scalar_mul_export(r: *mut Scalar, a: *const Scalar, b: *const Scalar);
    fn secp256k1_scalar_shr_int_export(r: *mut Scalar, n: c_int) -> c_int;
    fn secp256k1_scalar_sqr_export(r: *mut Scalar, a: *const Scalar);
    fn secp256k1_scalar_inverse_export(r: *mut Scalar, a: *const Scalar);
    fn secp256k1_scalar_inverse_var_export(r: *mut Scalar, a: *const Scalar);
    fn secp256k1_scalar_negate_export(r: *mut Scalar, a: *const Scalar);
    fn secp256k1_scalar_is_zero_export(a: *const Scalar) -> c_int;
    fn secp256k1_scalar_is_one_export(a: *const Scalar) -> c_int;
    fn secp256k1_scalar_is_even_export(a: *const Scalar) -> c_int;
    fn secp256k1_scalar_is_high_export(a: *const Scalar) -> c_int;
    fn secp256k1_scalar_cond_negate_export(a: *mut Scalar, flag: c_int) -> c_int;
    // fn secp256k1_scalar_get_num_export(secp256k1_num *r, a: *const Scalar);
    // fn secp256k1_scalar_order_get_num_export(secp256k1_num *r);
    fn secp256k1_scalar_eq_export(a: *const Scalar, b: *const Scalar) -> c_int;
    fn secp256k1_scalar_split_128_export(r1: *mut Scalar, r2: *mut Scalar, a: *const Scalar);
    fn secp256k1_scalar_split_lambda_export(r1: *mut Scalar, r2: *mut Scalar, a: *const Scalar);
    fn secp256k1_scalar_mul_shift_var_export(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar,
        shift: c_uint,
    );
    fn secp256k1_scalar_cmov_export(r: *mut Scalar, a: *const Scalar, flag: c_int);
}

pub fn clear(r: &mut Scalar) {
    unsafe { secp256k1_scalar_clear_export(r) }
}
pub fn get_bits(a: &Scalar, offset: c_uint, count: c_uint) -> c_uint {
    unsafe { secp256k1_scalar_get_bits_export(a, offset, count) }
}
pub fn get_bits_var(a: &Scalar, offset: c_uint, count: c_uint) -> c_uint {
    unsafe { secp256k1_scalar_get_bits_var_export(a, offset, count) }
}
pub fn set_b32(r: &mut Scalar, bin: &c_uchar, overflow: &mut c_int) {
    unsafe { secp256k1_scalar_set_b32_export(r, bin, overflow) }
}
pub fn set_b32_seckey(r: &mut Scalar, bin: &c_uchar) -> c_int {
    unsafe { secp256k1_scalar_set_b32_seckey_export(r, bin) }
}
pub fn set_int(r: &mut Scalar, v: c_uint) {
    unsafe { secp256k1_scalar_set_int_export(r, v) }
}
pub fn get_b32(bin: &mut c_uchar, a: &Scalar) {
    unsafe { secp256k1_scalar_get_b32_export(bin, a) }
}
pub fn add(r: &mut Scalar, a: &Scalar, b: &Scalar) -> c_int {
    unsafe { secp256k1_scalar_add_export(r, a, b) }
}
pub fn cadd_bit(r: &mut Scalar, bit: c_uint, flag: c_int) {
    unsafe { secp256k1_scalar_cadd_bit_export(r, bit, flag) }
}
pub fn mul(r: &mut Scalar, a: &Scalar, b: &Scalar) {
    unsafe { secp256k1_scalar_mul_export(r, a, b) }
}
pub fn shr_int(r: &mut Scalar, n: c_int) -> c_int {
    unsafe { secp256k1_scalar_shr_int_export(r, n) }
}
pub fn sqr(r: &mut Scalar, a: &Scalar) {
    unsafe { secp256k1_scalar_sqr_export(r, a) }
}
pub fn inverse(r: &mut Scalar, a: &Scalar) {
    unsafe { secp256k1_scalar_inverse_export(r, a) }
}
pub fn inverse_var(r: &mut Scalar, a: &Scalar) {
    unsafe { secp256k1_scalar_inverse_var_export(r, a) }
}
pub fn negate(r: &mut Scalar, a: &Scalar) {
    unsafe { secp256k1_scalar_negate_export(r, a) }
}
pub fn is_zero(a: &Scalar) -> c_int {
    unsafe { secp256k1_scalar_is_zero_export(a) }
}
pub fn is_one(a: &Scalar) -> c_int {
    unsafe { secp256k1_scalar_is_one_export(a) }
}
pub fn is_even(a: &Scalar) -> c_int {
    unsafe { secp256k1_scalar_is_even_export(a) }
}
pub fn is_high(a: &Scalar) -> c_int {
    unsafe { secp256k1_scalar_is_high_export(a) }
}
pub fn cond_negate(a: &mut Scalar, flag: c_int) -> c_int {
    unsafe { secp256k1_scalar_cond_negate_export(a, flag) }
}
pub fn eq(a: &Scalar, b: &Scalar) -> c_int {
    unsafe { secp256k1_scalar_eq_export(a, b) }
}
pub fn split_128(r1: &mut Scalar, r2: &mut Scalar, a: &Scalar) {
    unsafe { secp256k1_scalar_split_128_export(r1, r2, a) }
}
pub fn split_lambda(r1: &mut Scalar, r2: &mut Scalar, a: &Scalar) {
    unsafe { secp256k1_scalar_split_lambda_export(r1, r2, a) }
}
pub fn mul_shift_var(r: &mut Scalar, a: &Scalar, b: &Scalar, shift: c_uint) {
    unsafe { secp256k1_scalar_mul_shift_var_export(r, a, b, shift) }
}
pub fn cmov(r: &mut Scalar, a: &Scalar, flag: c_int) {
    unsafe { secp256k1_scalar_cmov_export(r, a, flag) }
}

impl Scalar {
    pub fn clear(&mut self) {
        unsafe { secp256k1_scalar_clear_export(self) }
    }

    pub fn set_u64(&mut self, v: u64) {
        self.limbs = [v, 0, 0, 0];
    }

    pub fn new_from_u64(v: u64) -> Self {
        Self {
            limbs: [v, 0, 0, 0],
        }
    }

    pub fn randomise<R: Rng>(&mut self, rng: &mut R) {
        self.limbs = rng.gen();
    }

    pub fn randomise_using_thread_rng(&mut self) {
        self.limbs = rand::thread_rng().gen();
    }

    pub fn new_random<R: Rng>(rng: &mut R) -> Self {
        Self { limbs: rng.gen() }
    }

    pub fn new_random_using_thread_rng() -> Self {
        Self {
            limbs: rand::thread_rng().gen(),
        }
    }

    pub fn add(&mut self, a: &Self, b: &Self) -> bool {
        unsafe { secp256k1_scalar_add_export(self, a, b) != 0 }
    }

    pub fn add_assign(&mut self, a: &Self) -> bool {
        unsafe { secp256k1_scalar_add_export(self, self, a) != 0 }
    }

    pub fn double_assign(&mut self) -> bool {
        unsafe { secp256k1_scalar_add_export(self, self, self) != 0 }
    }

    pub fn negate(&mut self, a: &Self) {
        unsafe { secp256k1_scalar_negate_export(self, a) }
    }

    pub fn negate_assign(&mut self) {
        unsafe { secp256k1_scalar_negate_export(self, self) }
    }

    pub fn sub(&mut self, a: &Self, b: &Self) {
        unsafe {
            secp256k1_scalar_negate_export(self, b);
            secp256k1_scalar_add_export(self, self, a);
        }
    }

    pub fn mul(&mut self, a: &Self, b: &Self) {
        unsafe { secp256k1_scalar_mul_export(self, a, b) }
    }

    pub fn mul_assign(&mut self, a: &Self) {
        unsafe { secp256k1_scalar_mul_export(self, self, a) }
    }

    pub fn sqr(&mut self, a: &Self) {
        unsafe { secp256k1_scalar_sqr_export(self, a) }
    }

    pub fn sqr_assign(&mut self) {
        unsafe { secp256k1_scalar_sqr_export(self, self) }
    }

    pub fn inverse(&mut self, a: &Self) {
        // NOTE: We are using the time variant function here because we are not conerned with a
        // timining side channel, and it is faster.
        unsafe { secp256k1_scalar_inverse_var_export(self, a) }
    }

    pub fn inverse_assign(&mut self) {
        // NOTE: We are using the time variant function here because we are not conerned with a
        // timining side channel, and it is faster.
        unsafe { secp256k1_scalar_inverse_var_export(self, self) }
    }

    pub fn divide(&mut self, a: &Self, b: &Self) {
        unsafe {
            // NOTE: We are using the time variant function here because we are not conerned with a
            // timining side channel, and it is faster.
            secp256k1_scalar_inverse_var_export(self, b);
            secp256k1_scalar_mul_export(self, self, a);
        }
    }

    pub fn is_zero(&self) -> bool {
        unsafe { secp256k1_scalar_is_zero_export(self) != 0 }
    }

    pub fn is_one(&self) -> bool {
        unsafe { secp256k1_scalar_is_one_export(self) != 0 }
    }

    pub fn is_even(&self) -> bool {
        unsafe { secp256k1_scalar_is_even_export(self) != 0 }
    }

    pub fn is_high(&self) -> bool {
        unsafe { secp256k1_scalar_is_high_export(self) != 0 }
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        unsafe { secp256k1_scalar_eq_export(self, other) != 0 }
    }
}
impl Eq for Scalar {}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn scalar_addition() {
        let mut r = Scalar { limbs: [0; 4] };
        let a = Scalar {
            limbs: [
                12561100044226495138,
                5589961653028563220,
                8517791565878209618,
                630006259463090245,
            ],
        };
        let b = Scalar {
            limbs: [
                13585510617142601345,
                2522136002439952246,
                1161228564705102482,
                2265091615529285871,
            ],
        };
        add(&mut r, &a, &b);
        assert!(
            r.limbs
                == [
                    7699866587659544867,
                    8112097655468515467,
                    9679020130583312100,
                    2895097874992376116,
                ]
        );
    }

    #[bench]
    fn bench_scalar_add(b: &mut Bencher) {
        let mut r = Scalar { limbs: [0; 4] };
        let mut x = Scalar {
            limbs: [
                12561100044226495138,
                5589961653028563220,
                8517791565878209618,
                630006259463090245,
            ],
        };
        let mut y = Scalar {
            limbs: [
                13585510617142601345,
                2522136002439952246,
                1161228564705102482,
                2265091615529285871,
            ],
        };
        b.iter(|| add(&mut r, &mut x, &mut y));
    }

    #[test]
    fn scalar_multiplication() {
        let mut r = Scalar { limbs: [0; 4] };
        let mut a = Scalar {
            limbs: [
                12561100044226495138,
                5589961653028563220,
                8517791565878209618,
                630006259463090245,
            ],
        };
        let mut b = Scalar {
            limbs: [
                13585510617142601345,
                2522136002439952246,
                1161228564705102482,
                2265091615529285871,
            ],
        };
        mul(&mut r, &mut a, &mut b);
        dbg!(r.limbs);
        assert!(
            r.limbs
                == [
                    1415566385503954678,
                    972744334127566766,
                    515628029826509361,
                    1743495621230648086,
                ]
        );
    }

    #[bench]
    fn bench_scalar_mul(b: &mut Bencher) {
        let mut r = Scalar { limbs: [0; 4] };
        let mut x = Scalar {
            limbs: [
                12561100044226495138,
                5589961653028563220,
                8517791565878209618,
                630006259463090245,
            ],
        };
        let mut y = Scalar {
            limbs: [
                13585510617142601345,
                2522136002439952246,
                1161228564705102482,
                2265091615529285871,
            ],
        };
        b.iter(|| mul(&mut r, &mut x, &mut y));
    }

    #[test]
    fn scalar_inversion() {
        let mut r = Scalar { limbs: [0; 4] };
        let mut a = Scalar {
            limbs: [
                12561100044226495138,
                5589961653028563220,
                8517791565878209618,
                630006259463090245,
            ],
        };
        inverse(&mut r, &mut a);
        dbg!(r.limbs);
        assert!(
            r.limbs
                == [
                    11001831813932638462,
                    9863664224976379268,
                    13877187249603070594,
                    11479251249069358415,
                ]
        );
    }

    #[bench]
    fn bench_scalar_inverse(b: &mut Bencher) {
        let mut r = Scalar { limbs: [0; 4] };
        let mut x = Scalar {
            limbs: [
                12561100044226495138,
                5589961653028563220,
                8517791565878209618,
                630006259463090245,
            ],
        };
        b.iter(|| inverse(&mut r, &mut x));
    }
}
