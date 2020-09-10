use libc::{c_int, c_uchar};
use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash)]
pub struct Scalar {
    limbs: [u64; 4],
}

extern "C" {
    fn secp256k1_scalar_clear_export(r: *mut Scalar);
    // fn secp256k1_scalar_get_bits_export(a: *const Scalar, offset: c_uint, count: c_uint) -> c_uint;
    /*
    fn secp256k1_scalar_get_bits_var_export(
        a: *const Scalar,
        offset: c_uint,
        count: c_uint,
    ) -> c_uint;
    */
    fn secp256k1_scalar_set_b32_export(r: *mut Scalar, bin: *const c_uchar, overflow: *mut c_int);
    // fn secp256k1_scalar_set_b32_seckey_export(r: *mut Scalar, bin: *const c_uchar) -> c_int;
    // fn secp256k1_scalar_set_int_export(r: *mut Scalar, v: c_uint);
    fn secp256k1_scalar_get_b32_export(bin: *mut c_uchar, a: *const Scalar);
    fn secp256k1_scalar_add_export(r: *mut Scalar, a: *const Scalar, b: *const Scalar) -> c_int;
    // fn secp256k1_scalar_cadd_bit_export(r: *mut Scalar, bit: c_uint, flag: c_int);
    fn secp256k1_scalar_mul_export(r: *mut Scalar, a: *const Scalar, b: *const Scalar);
    // fn secp256k1_scalar_shr_int_export(r: *mut Scalar, n: c_int) -> c_int;
    fn secp256k1_scalar_sqr_export(r: *mut Scalar, a: *const Scalar);
    // fn secp256k1_scalar_inverse_export(r: *mut Scalar, a: *const Scalar);
    fn secp256k1_scalar_inverse_var_export(r: *mut Scalar, a: *const Scalar);
    fn secp256k1_scalar_negate_export(r: *mut Scalar, a: *const Scalar);
    fn secp256k1_scalar_is_zero_export(a: *const Scalar) -> c_int;
    fn secp256k1_scalar_is_one_export(a: *const Scalar) -> c_int;
    fn secp256k1_scalar_is_even_export(a: *const Scalar) -> c_int;
    fn secp256k1_scalar_is_high_export(a: *const Scalar) -> c_int;
    // fn secp256k1_scalar_cond_negate_export(a: *mut Scalar, flag: c_int) -> c_int;
    // fn secp256k1_scalar_get_num_export(secp256k1_num *r, a: *const Scalar);
    // fn secp256k1_scalar_order_get_num_export(secp256k1_num *r);
    fn secp256k1_scalar_eq_export(a: *const Scalar, b: *const Scalar) -> c_int;
// fn secp256k1_scalar_split_128_export(r1: *mut Scalar, r2: *mut Scalar, a: *const Scalar);
// fn secp256k1_scalar_split_lambda_export(r1: *mut Scalar, r2: *mut Scalar, a: *const Scalar);
/*
fn secp256k1_scalar_mul_shift_var_export(
    r: *mut Scalar,
    a: *const Scalar,
    b: *const Scalar,
    shift: c_uint,
);
*/
// fn secp256k1_scalar_cmov_export(r: *mut Scalar, a: *const Scalar, flag: c_int);
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

    pub fn zero() -> Self {
        Self {
            limbs: [0, 0, 0, 0],
        }
    }

    pub fn one() -> Self {
        Self {
            limbs: [1, 0, 0, 0],
        }
    }

    pub fn set_b32(&mut self, bs: &[u8]) -> bool {
        let mut overflow = 0;
        unsafe { secp256k1_scalar_set_b32_export(self, bs.as_ptr(), &mut overflow) }
        overflow != 0
    }

    pub fn put_b32(&self, bs: &mut [u8]) {
        unsafe { secp256k1_scalar_get_b32_export(bs.as_mut_ptr(), self) }
    }

    pub fn add_mut(&mut self, a: &Self, b: &Self) -> bool {
        unsafe { secp256k1_scalar_add_export(self, a, b) != 0 }
    }

    pub fn add_assign_mut(&mut self, a: &Self) -> bool {
        unsafe { secp256k1_scalar_add_export(self, self, a) != 0 }
    }

    pub fn double_assign_mut(&mut self) -> bool {
        unsafe { secp256k1_scalar_add_export(self, self, self) != 0 }
    }

    pub fn negate_mut(&mut self, a: &Self) {
        unsafe { secp256k1_scalar_negate_export(self, a) }
    }

    pub fn negate_assign_mut(&mut self) {
        unsafe { secp256k1_scalar_negate_export(self, self) }
    }

    pub fn sub_mut(&mut self, a: &Self, b: &Self) {
        unsafe {
            secp256k1_scalar_negate_export(self, b);
            secp256k1_scalar_add_export(self, self, a);
        }
    }

    pub fn mul_mut(&mut self, a: &Self, b: &Self) {
        unsafe { secp256k1_scalar_mul_export(self, a, b) }
    }

    pub fn mul_assign_mut(&mut self, a: &Self) {
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

    pub fn divide_mut(&mut self, a: &Self, b: &Self) {
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

impl Default for Scalar {
    fn default() -> Self {
        Scalar { limbs: [0; 4] }
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        unsafe { secp256k1_scalar_eq_export(self, other) != 0 }
    }
}
impl Eq for Scalar {}

impl Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = Scalar::default();
        ret.add_mut(&self, &rhs);
        ret
    }
}

impl AddAssign for Scalar {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign_mut(&rhs);
    }
}

impl Div for Scalar {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut ret = Scalar::default();
        ret.divide_mut(&self, &rhs);
        ret
    }
}

impl DivAssign for Scalar {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Mul for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = Scalar::default();
        ret.mul_mut(&self, &rhs);
        ret
    }
}

impl MulAssign for Scalar {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign_mut(&rhs);
    }
}

impl Neg for Scalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut ret = Scalar::default();
        ret.negate_mut(&self);
        ret
    }
}

impl Neg for &Scalar {
    type Output = Scalar;

    fn neg(self) -> Self::Output {
        let mut ret = Scalar::default();
        ret.negate_mut(self);
        ret
    }
}

impl Sub for Scalar {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = Scalar::default();
        ret.sub_mut(&self, &rhs);
        ret
    }
}

impl SubAssign for Scalar {
    fn sub_assign(&mut self, rhs: Self) {
        let mut tmp = Scalar::default();
        tmp.negate_mut(&rhs);
        self.add_assign_mut(&tmp);
    }
}

macro_rules! impl_bin_op_ref {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

macro_rules! impl_bin_op_assign_ref {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
    };
}

impl_bin_op_ref!(impl Add, add for Scalar, Scalar);
impl_bin_op_ref!(impl Div, div for Scalar, Scalar);
impl_bin_op_ref!(impl Mul, mul for Scalar, Scalar);
impl_bin_op_ref!(impl Sub, sub for Scalar, Scalar);

impl_bin_op_assign_ref!(impl AddAssign, add_assign for Scalar, Scalar);
impl_bin_op_assign_ref!(impl DivAssign, div_assign for Scalar, Scalar);
impl_bin_op_assign_ref!(impl MulAssign, mul_assign for Scalar, Scalar);
impl_bin_op_assign_ref!(impl SubAssign, sub_assign for Scalar, Scalar);

pub fn randomise_scalars_using_thread_rng(scalars: &mut [Scalar]) {
    scalars
        .iter_mut()
        .for_each(|s| s.randomise_using_thread_rng())
}

pub fn random_scalars_using_thread_rng(n: usize) -> Vec<Scalar> {
    let mut scalars = Vec::with_capacity(n);
    for _ in 0..n {
        scalars.push(Scalar::new_random_using_thread_rng());
    }
    scalars
}

pub fn randomise_scalars<R: Rng>(scalars: &mut [Scalar], rng: &mut R) {
    scalars.iter_mut().for_each(|s| s.randomise(rng))
}

pub fn random_scalars<R: Rng>(n: usize, rng: &mut R) -> Vec<Scalar> {
    let mut scalars = Vec::with_capacity(n);
    for _ in 0..n {
        scalars.push(Scalar::new_random(rng));
    }
    scalars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
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
        r.add_mut(&a, &b);
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

    #[test]
    fn multiplication() {
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
        r.mul_mut(&a, &b);
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

    #[test]
    fn inversion() {
        let mut r = Scalar { limbs: [0; 4] };
        let a = Scalar {
            limbs: [
                12561100044226495138,
                5589961653028563220,
                8517791565878209618,
                630006259463090245,
            ],
        };
        r.inverse(&a);
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

    #[test]
    fn negate_is_additive_inverse() {
        let mut a = Scalar::default();
        let mut b = Scalar::default();
        let mut c = Scalar::default();

        for _ in 0..100 {
            a.randomise_using_thread_rng();
            b.negate_mut(&a);
            c.add_mut(&a, &b);
            assert!(c.is_zero());
        }
    }

    #[test]
    fn subtracting_self_yields_zero() {
        let mut a = Scalar::default();
        let mut b: Scalar;
        let mut c = Scalar::default();

        for _ in 0..100 {
            a.randomise_using_thread_rng();
            b = a;
            c.sub_mut(&a, &b);
            assert!(c.is_zero());
        }
    }

    #[test]
    fn inverse_is_multiplicative_inverse() {
        let mut a = Scalar::default();
        let mut b = Scalar::default();
        let mut c = Scalar::default();

        for _ in 0..100 {
            a.randomise_using_thread_rng();
            b.inverse(&a);
            c.mul_mut(&a, &b);
            assert!(c.is_one());
        }
    }

    #[test]
    fn dividing_self_yields_one() {
        let mut a = Scalar::default();
        let mut b: Scalar;
        let mut c = Scalar::default();

        for _ in 0..100 {
            a.randomise_using_thread_rng();
            b = a;
            c.divide_mut(&a, &b);
            assert!(c.is_one());
        }
    }
}
