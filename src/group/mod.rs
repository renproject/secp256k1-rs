use libc::c_int;

pub mod fe;

use crate::scalar::Scalar;
use fe::Fe;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Ge {
    x: Fe,
    y: Fe,
    infinity: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Gej {
    x: Fe,
    y: Fe,
    z: Fe,
    infinity: c_int,
}

extern "C" {
    static secp256k1_ge_const_g: Ge;

    fn secp256k1_ge_set_xy_export(r: *mut Ge, x: *const Fe, y: *const Fe);
    fn secp256k1_ge_set_xquad_export(r: *mut Ge, x: *const Fe) -> c_int;
    fn secp256k1_ge_set_xo_var_export(r: *mut Ge, x: *const Fe, odd: c_int) -> c_int;
    fn secp256k1_ge_is_infinity_export(a: *const Ge) -> c_int;
    fn secp256k1_ge_is_valid_var_export(a: *const Ge) -> c_int;
    fn secp256k1_ge_neg_export(r: *mut Ge, a: *const Ge);
    fn secp256k1_ge_set_gej_export(r: *mut Ge, a: *mut Gej);
    // fn secp256k1_ge_set_all_gej_var_export(r: *mut Ge, a: *const Gej, len: size_t);
    /*
        fn secp256k1_ge_globalz_set_table_gej_export(
            len: size_t,
            r: *mut Ge,
            globalz: *mut Fe,
            a: *const Gej,
            zr: *const Fe,
        );
    */
    fn secp256k1_ge_set_infinity_export(r: *mut Ge);
    fn secp256k1_gej_set_infinity_export(r: *mut Gej);
    fn secp256k1_gej_set_ge_export(r: *mut Gej, a: *const Ge);
    fn secp256k1_gej_eq_x_var_export(x: *const Fe, a: *const Gej) -> c_int;
    fn secp256k1_gej_neg_export(r: *mut Gej, a: *const Gej);
    fn secp256k1_gej_is_infinity_export(a: *const Gej) -> c_int;
    fn secp256k1_gej_has_quad_y_var_export(a: *const Gej) -> c_int;
    // fn secp256k1_gej_double_export(r: *mut Gej, a: *const Gej);
    fn secp256k1_gej_double_var_export(r: *mut Gej, a: *const Gej, rzr: *mut Fe);
    fn secp256k1_gej_add_var_export(r: *mut Gej, a: *const Gej, b: *const Gej, rzr: *mut Fe);
    // fn secp256k1_gej_add_ge_export(r: *mut Gej, a: *const Gej, b: *const Ge);
    fn secp256k1_gej_add_ge_var_export(r: *mut Gej, a: *const Gej, b: *const Ge, rzr: *mut Fe);
    /*
        fn secp256k1_gej_add_zinv_var_export(
            r: *mut Gej,
            a: *const Gej,
            b: *const Ge,
            bzinv: *const Fe,
        );
    */
    // fn secp256k1_ge_mul_lambda_export(*mut Ge r, *const Ge a);
    fn secp256k1_gej_clear_export(r: *mut Gej);
    fn secp256k1_ge_clear_export(r: *mut Ge);
    // fn secp256k1_ge_to_storage_export(secp256k1_ge_storage *r, *const Ge a);
    // fn secp256k1_ge_from_storage_export(*mut Ge r,  const secp256k1_ge_storage *a);
    // fn secp256k1_ge_storage_cmov_export(secp256k1_ge_storage *r, const secp256k1_ge_storage *a, c_int flag);
    fn secp256k1_gej_rescale_export(r: *mut Gej, b: *const Fe);

    fn secp256k1_ecmult_const_export(r: *mut Gej, a: *const Ge, q: *const Scalar, bits: c_int);
}

impl Ge {
    pub fn set_xy(&mut self, x: &Fe, y: &Fe) {
        unsafe { secp256k1_ge_set_xy_export(self, x, y) }
    }

    pub fn set_xquad(&mut self, x: &Fe) -> bool {
        unsafe { secp256k1_ge_set_xquad_export(self, x) != 0 }
    }

    pub fn set_xo_var(&mut self, x: &Fe, odd: bool) -> bool {
        let odd_cint = if odd { 1 } else { 0 };
        unsafe { secp256k1_ge_set_xo_var_export(self, x, odd_cint) != 0 }
    }
    pub fn is_infinity(&self) -> bool {
        unsafe { secp256k1_ge_is_infinity_export(self) != 0 }
    }

    pub fn is_valid_var(&self) -> bool {
        unsafe { secp256k1_ge_is_valid_var_export(self) != 0 }
    }

    pub fn neg(&mut self, a: &Ge) {
        unsafe { secp256k1_ge_neg_export(self, a) }
    }

    pub fn set_gej(&mut self, a: &mut Gej) {
        unsafe { secp256k1_ge_set_gej_export(self, a) }
    }

    pub fn set_infinity(&mut self) {
        unsafe { secp256k1_ge_set_infinity_export(self) }
    }

    pub fn clear(&mut self) {
        unsafe { secp256k1_ge_clear_export(self) }
    }

    pub fn scalar_mul(&mut self, a: &Ge, q: &Scalar) {
        let mut tmp_gej = Gej::default();
        let bits = 257;
        unsafe {
            secp256k1_ecmult_const_export(&mut tmp_gej, a, q, bits);
            secp256k1_ge_set_gej_export(self, &mut tmp_gej);
        }
    }

    pub fn scalar_base_mul(&mut self, q: &Scalar) {
        let mut tmp_gej = Gej::default();
        let bits = 257;
        unsafe {
            secp256k1_ecmult_const_export(&mut tmp_gej, &secp256k1_ge_const_g, q, bits);
            secp256k1_ge_set_gej_export(self, &mut tmp_gej);
        }
    }
}

impl Gej {
    pub fn set_infinity(&mut self) {
        unsafe { secp256k1_gej_set_infinity_export(self) }
    }

    pub fn set_ge(&mut self, a: &Ge) {
        unsafe { secp256k1_gej_set_ge_export(self, a) }
    }

    pub fn eq_x(x: &Fe, a: &Gej) -> bool {
        unsafe { secp256k1_gej_eq_x_var_export(x, a) != 0 }
    }

    pub fn neg(&mut self, a: &Gej) {
        unsafe { secp256k1_gej_neg_export(self, a) }
    }

    pub fn is_infinity(a: &Gej) -> bool {
        unsafe { secp256k1_gej_is_infinity_export(a) != 0 }
    }

    pub fn has_quad_y(a: &Gej) -> bool {
        unsafe { secp256k1_gej_has_quad_y_var_export(a) != 0 }
    }

    pub fn double(&mut self, a: &Gej, rzr: &mut Fe) {
        // NOTE: We are using the time variant function here because we are not conerned with a
        // timining side channel, and it is faster.
        unsafe { secp256k1_gej_double_var_export(self, a, rzr) }
    }

    pub fn add(&mut self, a: &Gej, b: &Gej, rzr: &mut Fe) {
        unsafe { secp256k1_gej_add_var_export(self, a, b, rzr) }
    }

    pub fn add_ge_var(&mut self, a: &Gej, b: &Ge, rzr: &mut Fe) {
        // NOTE: We are using the time variant function here because we are not conerned with a
        // timining side channel, and it is faster.
        unsafe { secp256k1_gej_add_ge_var_export(self, a, b, rzr) }
    }

    pub fn clear(&mut self) {
        unsafe { secp256k1_gej_clear_export(self) }
    }

    pub fn rescale(&mut self, b: &Fe) {
        unsafe { secp256k1_gej_rescale_export(self, b) }
    }

    pub fn scalar_mul(&mut self, a: &Ge, q: &Scalar) {
        let bits = 257;
        unsafe { secp256k1_ecmult_const_export(self, a, q, bits) }
    }

    pub fn scalar_base_mul(&mut self, q: &Scalar) {
        let bits = 257;
        unsafe { secp256k1_ecmult_const_export(self, &secp256k1_ge_const_g, q, bits) }
    }
}

impl PartialEq for Gej {
    fn eq(&self, other: &Self) -> bool {
        let mut scale = Fe::default();
        let mut tmp_gej = *self;
        let mut tmp_other_gej = *other;
        scale.inv(&self.z);
        scale.mul_assign(&other.z);
        tmp_gej.rescale(&scale);
        tmp_gej.x.normalize();
        tmp_gej.y.normalize();
        tmp_other_gej.x.normalize();
        tmp_other_gej.y.normalize();
        (tmp_gej.x == tmp_other_gej.x) && (tmp_gej.y == tmp_other_gej.y)
    }
}
impl Eq for Gej {}
