use libc::{c_int, size_t};

pub mod fe;

use crate::scalar::Scalar;
use fe::Fe;

#[repr(C)]
pub struct Ge {
    x: Fe,
    y: Fe,
    infinity: c_int,
}

#[repr(C)]
pub struct Gej {
    x: Fe,
    y: Fe,
    z: Fe,
    infinity: c_int,
}

extern "C" {
    fn secp256k1_ge_set_xy_export(r: *mut Ge, x: *const Fe, y: *const Fe);
    fn secp256k1_ge_set_xquad_export(r: *mut Ge, x: *const Fe) -> c_int;
    fn secp256k1_ge_set_xo_var_export(r: *mut Ge, x: *const Fe, odd: c_int) -> c_int;
    fn secp256k1_ge_is_infinity_export(a: *const Ge) -> c_int;
    fn secp256k1_ge_is_valid_var_export(a: *const Ge) -> c_int;
    fn secp256k1_ge_neg_export(r: *mut Ge, a: *const Ge);
    fn secp256k1_ge_set_gej_export(r: *mut Ge, a: *mut Gej);
    fn secp256k1_ge_set_all_gej_var_export(r: *mut Ge, a: *const Gej, len: size_t);
    fn secp256k1_ge_globalz_set_table_gej_export(
        len: size_t,
        r: *mut Ge,
        globalz: *mut Fe,
        a: *const Gej,
        zr: *const Fe,
    );
    fn secp256k1_ge_set_infinity_export(r: *mut Ge);
    fn secp256k1_gej_set_infinity_export(r: *mut Gej);
    fn secp256k1_gej_set_ge_export(r: *mut Gej, a: *const Ge);
    fn secp256k1_gej_eq_x_var_export(x: *const Fe, a: *const Gej) -> c_int;
    fn secp256k1_gej_neg_export(r: *mut Gej, a: *const Gej);
    fn secp256k1_gej_is_infinity_export(a: *const Gej) -> c_int;
    fn secp256k1_gej_has_quad_y_var_export(a: *const Gej) -> c_int;
    fn secp256k1_gej_double_export(r: *mut Gej, a: *const Gej);
    fn secp256k1_gej_double_var_export(r: *mut Gej, a: *const Gej, rzr: *mut Fe);
    fn secp256k1_gej_add_var_export(r: *mut Gej, a: *const Gej, b: *const Gej, rzr: *mut Fe);
    fn secp256k1_gej_add_ge_export(r: *mut Gej, a: *const Gej, b: *const Ge);
    fn secp256k1_gej_add_ge_var_export(r: *mut Gej, a: *const Gej, b: *const Ge, rzr: *mut Fe);
    fn secp256k1_gej_add_zinv_var_export(
        r: *mut Gej,
        a: *const Gej,
        b: *const Ge,
        bzinv: *const Fe,
    );
    // fn secp256k1_ge_mul_lambda_export(*mut Ge r, *const Ge a);
    fn secp256k1_gej_clear_export(r: *mut Gej);
    fn secp256k1_ge_clear_export(r: *mut Ge);
    // fn secp256k1_ge_to_storage_export(secp256k1_ge_storage *r, *const Ge a);
    // fn secp256k1_ge_from_storage_export(*mut Ge r,  const secp256k1_ge_storage *a);
    // fn secp256k1_ge_storage_cmov_export(secp256k1_ge_storage *r, const secp256k1_ge_storage *a, c_int flag);
    fn secp256k1_gej_rescale_export(r: *mut Gej, b: *const Fe);

    fn secp256k1_ecmult_const_export(r: *mut Gej, a: *const Ge, q: *const Scalar, bits: c_int);
}
