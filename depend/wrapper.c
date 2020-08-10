#define USE_NUM_GMP

#define HAVE___INT128
#define USE_SCALAR_4X64
#define USE_SCALAR_INV_BUILTIN

#include "secp256k1/include/secp256k1.h"
#include "secp256k1/src/util.h"
#include "secp256k1/src/num_gmp_impl.h"
#include "secp256k1/src/scalar.h"
#include "secp256k1/src/scalar_impl.h"
#include "secp256k1/src/scalar_4x64_impl.h"

/* Scalar */

void secp256k1_scalar_clear_export(secp256k1_scalar *r) {
	return secp256k1_scalar_clear(r);
}
unsigned int secp256k1_scalar_get_bits_export(const secp256k1_scalar *a, unsigned int offset, unsigned int count) {
	return secp256k1_scalar_get_bits(a,  offset,  count);
}
unsigned int secp256k1_scalar_get_bits_var_export(const secp256k1_scalar *a, unsigned int offset, unsigned int count) {
	return secp256k1_scalar_get_bits_var(a,  offset,  count);
}
void secp256k1_scalar_set_b32_export(secp256k1_scalar *r, const unsigned char *bin, int *overflow) {
	return secp256k1_scalar_set_b32(r, bin, overflow);
}
int secp256k1_scalar_set_b32_seckey_export(secp256k1_scalar *r, const unsigned char *bin) {
	return secp256k1_scalar_set_b32_seckey(r, bin);
}
void secp256k1_scalar_set_int_export(secp256k1_scalar *r, unsigned v) {
	return secp256k1_scalar_set_int(r,  v);
}
void secp256k1_scalar_get_b32_export(unsigned char *bin, const secp256k1_scalar* a) {
	return secp256k1_scalar_get_b32(bin, a);
}
int secp256k1_scalar_add_export(secp256k1_scalar *r, const secp256k1_scalar *a, const secp256k1_scalar *b) {
	return secp256k1_scalar_add(r, a, b);
}
void secp256k1_scalar_cadd_bit_export(secp256k1_scalar *r, unsigned int bit, int flag) {
	return secp256k1_scalar_cadd_bit(r,  bit, flag);
}
void secp256k1_scalar_mul_export(secp256k1_scalar *r, const secp256k1_scalar *a, const secp256k1_scalar *b) {
	return secp256k1_scalar_mul(r, a, b);
}
int secp256k1_scalar_shr_int_export(secp256k1_scalar *r, int n) {
	return secp256k1_scalar_shr_int(r, n);
}
void secp256k1_scalar_sqr_export(secp256k1_scalar *r, const secp256k1_scalar *a) {
	return secp256k1_scalar_sqr(r, a);
}
void secp256k1_scalar_inverse_export(secp256k1_scalar *r, const secp256k1_scalar *a) {
	return secp256k1_scalar_inverse(r, a);
}
void secp256k1_scalar_inverse_var_export(secp256k1_scalar *r, const secp256k1_scalar *a) {
	return secp256k1_scalar_inverse_var(r, a);
}
void secp256k1_scalar_negate_export(secp256k1_scalar *r, const secp256k1_scalar *a) {
	return secp256k1_scalar_negate(r, a);
}
int secp256k1_scalar_is_zero_export(const secp256k1_scalar *a) {
	return secp256k1_scalar_is_zero(a);
}
int secp256k1_scalar_is_one_export(const secp256k1_scalar *a) {
	return secp256k1_scalar_is_one(a);
}
int secp256k1_scalar_is_even_export(const secp256k1_scalar *a) {
	return secp256k1_scalar_is_even(a);
}
int secp256k1_scalar_is_high_export(const secp256k1_scalar *a) {
	return secp256k1_scalar_is_high(a);
}
int secp256k1_scalar_cond_negate_export(secp256k1_scalar *a, int flag) {
	return secp256k1_scalar_cond_negate(a, flag);
}
void secp256k1_scalar_get_num_export(secp256k1_num *r, const secp256k1_scalar *a) {
	return secp256k1_scalar_get_num(r, a);
}
void secp256k1_scalar_order_get_num_export(secp256k1_num *r) {
	return secp256k1_scalar_order_get_num(r);
}
int secp256k1_scalar_eq_export(const secp256k1_scalar *a, const secp256k1_scalar *b) {
	return secp256k1_scalar_eq(a, b);
}
void secp256k1_scalar_split_128_export(secp256k1_scalar *r1, secp256k1_scalar *r2, const secp256k1_scalar *a) {
	return secp256k1_scalar_split_128(r1, r2, a);
}
void secp256k1_scalar_split_lambda_export(secp256k1_scalar *r1, secp256k1_scalar *r2, const secp256k1_scalar *a) {
	return secp256k1_scalar_split_lambda(r1, r2, a);
}
void secp256k1_scalar_mul_shift_var_export(secp256k1_scalar *r, const secp256k1_scalar *a, const secp256k1_scalar *b, unsigned int shift) {
	return secp256k1_scalar_mul_shift_var(r, a, b,  shift);
}
void secp256k1_scalar_cmov_export(secp256k1_scalar *r, const secp256k1_scalar *a, int flag) {
	return secp256k1_scalar_cmov(r, a, flag);
}

/* Field */

// #define HAVE___INT128
#define USE_FIELD_5X52
#define USE_FIELD_INV_BUILTIN
// #define USE_NUM_GMP
#define USE_ASM_X86_64

// #include "secp256k1/include/secp256k1.h"
#include "secp256k1/src/field.h"
#include "secp256k1/src/field_impl.h"
#include "secp256k1/src/field_5x52_impl.h"
#include "secp256k1/src/num_impl.h"

void secp256k1_fe_normalize_export(secp256k1_fe *r){
	return secp256k1_fe_normalize(r);
}
void secp256k1_fe_normalize_weak_export(secp256k1_fe *r){
	return secp256k1_fe_normalize_weak(r);
}
void secp256k1_fe_normalize_var_export(secp256k1_fe *r){
	return secp256k1_fe_normalize_var(r);
}
int secp256k1_fe_normalizes_to_zero_export(secp256k1_fe *r){
	return secp256k1_fe_normalizes_to_zero(r);
}
int secp256k1_fe_normalizes_to_zero_var_export(secp256k1_fe *r){
	return secp256k1_fe_normalizes_to_zero_var(r);
}
void secp256k1_fe_set_int_export(secp256k1_fe *r, int a){
	return secp256k1_fe_set_int(r, a);
}
void secp256k1_fe_clear_export(secp256k1_fe *a){
	return secp256k1_fe_clear(a);
}
int secp256k1_fe_is_zero_export(const secp256k1_fe *a){
	return secp256k1_fe_is_zero(a);
}
int secp256k1_fe_is_odd_export(const secp256k1_fe *a){
	return secp256k1_fe_is_odd(a);
}
int secp256k1_fe_equal_export(const secp256k1_fe *a, const secp256k1_fe *b){
	return secp256k1_fe_equal(a, b);
}
int secp256k1_fe_equal_var_export(const secp256k1_fe *a, const secp256k1_fe *b){
	return secp256k1_fe_equal_var(a, b);
}
int secp256k1_fe_cmp_var_export(const secp256k1_fe *a, const secp256k1_fe *b){
	return secp256k1_fe_cmp_var(a, b);
}
int secp256k1_fe_set_b32_export(secp256k1_fe *r, const unsigned char *a){
	return secp256k1_fe_set_b32(r, a);
}
void secp256k1_fe_get_b32_export(unsigned char *r, const secp256k1_fe *a){
	return secp256k1_fe_get_b32(r, a);
}
void secp256k1_fe_negate_export(secp256k1_fe *r, const secp256k1_fe *a, int m){
	return secp256k1_fe_negate(r, a, m);
}
void secp256k1_fe_mul_int_export(secp256k1_fe *r, int a){
	return secp256k1_fe_mul_int(r, a);
}
void secp256k1_fe_add_export(secp256k1_fe *r, const secp256k1_fe *a){
	return secp256k1_fe_add(r, a);
}
void secp256k1_fe_mul_export(secp256k1_fe *r, const secp256k1_fe *a, const secp256k1_fe * SECP256K1_RESTRICT b){
	return secp256k1_fe_mul(r, a, b);
}
void secp256k1_fe_sqr_export(secp256k1_fe *r, const secp256k1_fe *a){
	return secp256k1_fe_sqr(r, a);
}
int secp256k1_fe_sqrt_export(secp256k1_fe *r, const secp256k1_fe *a){
	return secp256k1_fe_sqrt(r, a);
}
int secp256k1_fe_is_quad_var_export(const secp256k1_fe *a){
	return secp256k1_fe_is_quad_var(a);
}
void secp256k1_fe_inv_export(secp256k1_fe *r, const secp256k1_fe *a){
	return secp256k1_fe_inv(r, a);
}
void secp256k1_fe_inv_var_export(secp256k1_fe *r, const secp256k1_fe *a){
	return secp256k1_fe_inv_var(r, a);
}
void secp256k1_fe_inv_all_var_export(secp256k1_fe *r, const secp256k1_fe *a, size_t len){
	return secp256k1_fe_inv_all_var(r, a, len);
}
void secp256k1_fe_to_storage_export(secp256k1_fe_storage *r, const secp256k1_fe *a){
	return secp256k1_fe_to_storage(r, a);
}
void secp256k1_fe_from_storage_export(secp256k1_fe *r, const secp256k1_fe_storage *a){
	return secp256k1_fe_from_storage(r, a);
}
void secp256k1_fe_storage_cmov_export(secp256k1_fe_storage *r, const secp256k1_fe_storage *a, int flag){
	return secp256k1_fe_storage_cmov(r, a, flag);
}
void secp256k1_fe_cmov_export(secp256k1_fe *r, const secp256k1_fe *a, int flag){
	return secp256k1_fe_cmov(r, a, flag);
}

/* Group */

#include "secp256k1/src/group.h"
#include "secp256k1/src/group_impl.h"
// #include "secp256k1/src/scratch.h"
// #include "secp256k1/src/scratch_impl.h"


void secp256k1_ge_set_xy_export(secp256k1_ge *r, const secp256k1_fe *x, const secp256k1_fe *y) {
	return secp256k1_ge_set_xy(r, x, y);
}
int secp256k1_ge_set_xquad_export(secp256k1_ge *r, const secp256k1_fe *x) {
	return secp256k1_ge_set_xquad(r, x);
}
int secp256k1_ge_set_xo_var_export(secp256k1_ge *r, const secp256k1_fe *x, int odd) {
	return secp256k1_ge_set_xo_var(r, x, odd);
}
int secp256k1_ge_is_infinity_export(const secp256k1_ge *a) {
	return secp256k1_ge_is_infinity(a);
}
int secp256k1_ge_is_valid_var_export(const secp256k1_ge *a) {
	return secp256k1_ge_is_valid_var(a);
}
void secp256k1_ge_neg_export(secp256k1_ge *r, const secp256k1_ge *a) {
	return secp256k1_ge_neg(r, a);
}
void secp256k1_ge_set_gej_export(secp256k1_ge *r, secp256k1_gej *a) {
	return secp256k1_ge_set_gej(r, a);
}
void secp256k1_ge_set_all_gej_var_export(secp256k1_ge *r, const secp256k1_gej *a, size_t len) {
	return secp256k1_ge_set_all_gej_var(r, a, len);
}
void secp256k1_ge_globalz_set_table_gej_export(size_t len, secp256k1_ge *r, secp256k1_fe *globalz, const secp256k1_gej *a, const secp256k1_fe *zr) {
	return secp256k1_ge_globalz_set_table_gej(len, r, globalz, a, zr);
}
void secp256k1_ge_set_infinity_export(secp256k1_ge *r) {
	return secp256k1_ge_set_infinity(r);
}
void secp256k1_gej_set_infinity_export(secp256k1_gej *r) {
	return secp256k1_gej_set_infinity(r);
}
void secp256k1_gej_set_ge_export(secp256k1_gej *r, const secp256k1_ge *a) {
	return secp256k1_gej_set_ge(r, a);
}
int secp256k1_gej_eq_x_var_export(const secp256k1_fe *x, const secp256k1_gej *a) {
	return secp256k1_gej_eq_x_var(x, a);
}
void secp256k1_gej_neg_export(secp256k1_gej *r, const secp256k1_gej *a) {
	return secp256k1_gej_neg(r, a);
}
int secp256k1_gej_is_infinity_export(const secp256k1_gej *a) {
	return secp256k1_gej_is_infinity(a);
}
int secp256k1_gej_has_quad_y_var_export(const secp256k1_gej *a) {
	return secp256k1_gej_has_quad_y_var(a);
}
void secp256k1_gej_double_export(secp256k1_gej *r, const secp256k1_gej *a) {
	return secp256k1_gej_double(r, a);
}
void secp256k1_gej_double_var_export(secp256k1_gej *r, const secp256k1_gej *a, secp256k1_fe *rzr) {
	return secp256k1_gej_double_var(r, a, rzr);
}
void secp256k1_gej_add_var_export(secp256k1_gej *r, const secp256k1_gej *a, const secp256k1_gej *b, secp256k1_fe *rzr) {
	return secp256k1_gej_add_var(r, a, b, rzr);
}
void secp256k1_gej_add_ge_export(secp256k1_gej *r, const secp256k1_gej *a, const secp256k1_ge *b) {
	return secp256k1_gej_add_ge(r, a, b);
}
void secp256k1_gej_add_ge_var_export(secp256k1_gej *r, const secp256k1_gej *a, const secp256k1_ge *b, secp256k1_fe *rzr) {
	return secp256k1_gej_add_ge_var(r, a, b, rzr);
}
void secp256k1_gej_add_zinv_var_export(secp256k1_gej *r, const secp256k1_gej *a, const secp256k1_ge *b, const secp256k1_fe *bzinv) {
	return secp256k1_gej_add_zinv_var(r, a, b, bzinv);
}
#ifdef USE_ENDOMORPHISM
void secp256k1_ge_mul_lambda_export(secp256k1_ge *r, const secp256k1_ge *a) {
	return secp256k1_ge_mul_lambda(r, a);
}
#endif
void secp256k1_gej_clear_export(secp256k1_gej *r) {
	return secp256k1_gej_clear(r);
}
void secp256k1_ge_clear_export(secp256k1_ge *r) {
	return secp256k1_ge_clear(r);
}
void secp256k1_ge_to_storage_export(secp256k1_ge_storage *r, const secp256k1_ge *a) {
	return secp256k1_ge_to_storage(r, a);
}
void secp256k1_ge_from_storage_export(secp256k1_ge *r,  const secp256k1_ge_storage *a) {
	return secp256k1_ge_from_storage(r, a);
}
void secp256k1_ge_storage_cmov_export(secp256k1_ge_storage *r, const secp256k1_ge_storage *a, int flag) {
	return secp256k1_ge_storage_cmov(r, a, flag);
}
void secp256k1_gej_rescale_export(secp256k1_gej *r, const secp256k1_fe *b) {
	return secp256k1_gej_rescale(r, b);
}

/* Elliptic curve multiplication */

// TODO: What should this be set to?
#define ECMULT_WINDOW_SIZE 24

#include "secp256k1/src/ecmult_const.h"
#include "secp256k1/src/ecmult_const_impl.h"

void secp256k1_ecmult_const_export(secp256k1_gej *r, const secp256k1_ge *a, const secp256k1_scalar *q, int bits) {
	return secp256k1_ecmult_const(r, a, q, bits);
}
