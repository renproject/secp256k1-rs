use libc::{c_int, c_uchar, size_t};
use rand::Rng;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Fe {
    limbs: [u64; 5],
}

extern "C" {
    fn secp256k1_fe_normalize_export(r: *mut Fe);
    fn secp256k1_fe_normalize_weak_export(r: *mut Fe);
    fn secp256k1_fe_normalize_var_export(r: *mut Fe);
    fn secp256k1_fe_normalizes_to_zero_export(r: *mut Fe) -> c_int;
    fn secp256k1_fe_normalizes_to_zero_var_export(r: *mut Fe) -> c_int;
    fn secp256k1_fe_set_int_export(r: *mut Fe, a: c_int);
    fn secp256k1_fe_clear_export(a: *mut Fe);
    fn secp256k1_fe_is_zero_export(a: *const Fe) -> c_int;
    fn secp256k1_fe_is_odd_export(a: *const Fe) -> c_int;
    fn secp256k1_fe_equal_export(a: *const Fe, b: *const Fe) -> c_int;
    fn secp256k1_fe_equal_var_export(a: *const Fe, b: *const Fe) -> c_int;
    fn secp256k1_fe_cmp_var_export(a: *const Fe, b: *const Fe) -> c_int;
    fn secp256k1_fe_set_b32_export(r: *mut Fe, a: *const c_uchar) -> c_int;
    fn secp256k1_fe_get_b32_export(r: *mut c_uchar, a: *const Fe);
    fn secp256k1_fe_negate_export(r: *mut Fe, a: *const Fe, m: c_int);
    fn secp256k1_fe_mul_int_export(r: *mut Fe, a: c_int);
    fn secp256k1_fe_add_export(r: *mut Fe, a: *const Fe);
    fn secp256k1_fe_mul_export(r: *mut Fe, a: *const Fe, b: *const Fe);
    fn secp256k1_fe_sqr_export(r: *mut Fe, a: *const Fe);
    fn secp256k1_fe_sqrt_export(r: *mut Fe, a: *const Fe) -> c_int;
    fn secp256k1_fe_is_quad_var_export(a: *const Fe) -> c_int;
    fn secp256k1_fe_inv_export(r: *mut Fe, a: *const Fe);
    fn secp256k1_fe_inv_var_export(r: *mut Fe, a: *const Fe);
    fn secp256k1_fe_inv_all_var_export(r: *mut Fe, a: *const Fe, len: size_t);
    // fn secp256k1_fe_to_storage_export(secp256k1_fe_storage *r, a: *mut Fe);
    // fn secp256k1_fe_from_storage_export(r: *mut Fe, const secp256k1_fe_storage *a);
    // fn secp256k1_fe_storage_cmov_export(secp256k1_fe_storage *r, const secp256k1_fe_storage *a, flag: c_int);
    fn secp256k1_fe_cmov_export(r: *mut Fe, a: *const Fe, flag: c_int);
}

pub fn normalize(r: &mut Fe) {
    unsafe { secp256k1_fe_normalize_export(r) }
}
pub fn normalize_weak(r: &mut Fe) {
    unsafe { secp256k1_fe_normalize_weak_export(r) }
}
pub fn normalize_var(r: &mut Fe) {
    unsafe { secp256k1_fe_normalize_var_export(r) }
}
pub fn normalizes_to_zero(r: &mut Fe) -> c_int {
    unsafe { secp256k1_fe_normalizes_to_zero_export(r) }
}
pub fn normalizes_to_zero_var(r: &mut Fe) -> c_int {
    unsafe { secp256k1_fe_normalizes_to_zero_var_export(r) }
}
pub fn set_int(r: &mut Fe, a: c_int) {
    unsafe { secp256k1_fe_set_int_export(r, a) }
}
pub fn clear(a: &mut Fe) {
    unsafe { secp256k1_fe_clear_export(a) }
}
pub fn is_zero(a: &Fe) -> c_int {
    unsafe { secp256k1_fe_is_zero_export(a) }
}
pub fn is_odd(a: &Fe) -> c_int {
    unsafe { secp256k1_fe_is_odd_export(a) }
}
pub fn equal(a: &Fe, b: &Fe) -> c_int {
    unsafe { secp256k1_fe_equal_export(a, b) }
}
pub fn equal_var(a: &Fe, b: &Fe) -> c_int {
    unsafe { secp256k1_fe_equal_var_export(a, b) }
}
pub fn cmp_var(a: &Fe, b: &Fe) -> c_int {
    unsafe { secp256k1_fe_cmp_var_export(a, b) }
}
pub fn set_b32(r: &mut Fe, a: &[u8]) -> c_int {
    unsafe { secp256k1_fe_set_b32_export(r, a.as_ptr()) }
}
pub fn get_b32(r: &mut c_uchar, a: &Fe) {
    unsafe { secp256k1_fe_get_b32_export(r, a) }
}
pub fn negate(r: &mut Fe, a: &Fe, m: c_int) {
    unsafe { secp256k1_fe_negate_export(r, a, m) }
}
pub fn mul_int(r: &mut Fe, a: c_int) {
    unsafe { secp256k1_fe_mul_int_export(r, a) }
}
pub fn add(r: &mut Fe, a: &Fe) {
    unsafe { secp256k1_fe_add_export(r, a) }
}
pub fn mul(r: &mut Fe, a: &Fe, b: &Fe) {
    unsafe { secp256k1_fe_mul_export(r, a, b) }
}
pub fn sqr(r: &mut Fe, a: &Fe) {
    unsafe { secp256k1_fe_sqr_export(r, a) }
}
pub fn sqrt(r: &mut Fe, a: &Fe) -> c_int {
    unsafe { secp256k1_fe_sqrt_export(r, a) }
}
pub fn is_quad_var(a: &Fe) -> c_int {
    unsafe { secp256k1_fe_is_quad_var_export(a) }
}
pub fn inv(r: &mut Fe, a: &Fe) {
    unsafe { secp256k1_fe_inv_export(r, a) }
}
pub fn inv_var(r: &mut Fe, a: &Fe) {
    unsafe { secp256k1_fe_inv_var_export(r, a) }
}
pub fn inv_all_var(r: &mut Fe, a: &Fe, len: size_t) {
    unsafe { secp256k1_fe_inv_all_var_export(r, a, len) }
}
pub fn cmov(r: &mut Fe, a: &Fe, flag: c_int) {
    unsafe { secp256k1_fe_cmov_export(r, a, flag) }
}

impl Fe {
    pub fn new_random_using_thread_rng() -> Self {
        let bs: [u8; 32] = rand::thread_rng().gen();
        let mut fe = Fe::default();
        fe.set_b32(&bs);
        fe
    }

    pub fn set_b32(&mut self, bs: &[u8]) -> bool {
        // TODO: Panic if the slice if not long enough?
        unsafe { secp256k1_fe_set_b32_export(self, bs.as_ptr()) != 0 }
    }

    pub fn put_b32(&self, bs: &mut [u8]) {
        // TODO: Panic if the slice if not long enough?
        unsafe { secp256k1_fe_get_b32_export(bs.as_mut_ptr(), self) }
    }

    pub fn is_odd(&self) -> bool {
        unsafe { secp256k1_fe_is_odd_export(self) != 0 }
    }

    pub fn mul(&mut self, a: &Fe, b: &Fe) {
        unsafe { secp256k1_fe_mul_export(self, a, b) }
    }

    pub fn mul_assign(&mut self, a: &Fe) {
        unsafe { secp256k1_fe_mul_export(self, self, a) }
    }

    pub fn inv(&mut self, a: &Fe) {
        unsafe { secp256k1_fe_inv_var_export(self, a) }
    }

    pub fn normalize(&mut self) {
        unsafe { secp256k1_fe_normalize_var_export(self) }
    }
}

impl PartialEq for Fe {
    fn eq(&self, other: &Self) -> bool {
        let mut tmp_fe = *self;
        let mut tmp_other_fe = *other;
        tmp_fe.normalize();
        tmp_other_fe.normalize();
        tmp_fe.limbs == tmp_other_fe.limbs
    }
}
impl Eq for Fe {}

#[cfg(test)]
mod tests {}
