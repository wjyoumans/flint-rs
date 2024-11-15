/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::flint::*;
use crate::fmpz_types::*;


extern "C" {
    pub fn fmpz_poly_q_canonicalise(rop: *mut fmpz_poly_q_struct);
    pub fn fmpz_poly_q_is_canonical(op: *const fmpz_poly_q_struct) -> libc::c_int;
    pub fn fmpz_poly_q_init(rop: *mut fmpz_poly_q_struct);
    pub fn fmpz_poly_q_clear(rop: *mut fmpz_poly_q_struct);
    pub fn fmpz_poly_q_randtest(
        poly: *mut fmpz_poly_q_struct,
        state: *mut flint_rand_s,
        len1: mp_limb_signed_t,
        bits1: mp_limb_t,
        len2: mp_limb_signed_t,
        bits2: mp_limb_t,
    );
    pub fn fmpz_poly_q_randtest_not_zero(
        poly: *mut fmpz_poly_q_struct,
        state: *mut flint_rand_s,
        len1: mp_limb_signed_t,
        bits1: mp_limb_t,
        len2: mp_limb_signed_t,
        bits2: mp_limb_t,
    );
    pub fn fmpz_poly_q_set(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_set_si(rop: *mut fmpz_poly_q_struct, op: mp_limb_signed_t);
    pub fn fmpz_poly_q_swap(op1: *mut fmpz_poly_q_struct, op2: *mut fmpz_poly_q_struct);
    #[link_name = "fmpz_poly_q_zero__extern"]
    pub fn fmpz_poly_q_zero(rop: *mut fmpz_poly_q_struct);
    #[link_name = "fmpz_poly_q_one__extern"]
    pub fn fmpz_poly_q_one(rop: *mut fmpz_poly_q_struct);
    #[link_name = "fmpz_poly_q_neg__extern"]
    pub fn fmpz_poly_q_neg(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_inv(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    #[link_name = "fmpz_poly_q_is_zero__extern"]
    pub fn fmpz_poly_q_is_zero(op: *const fmpz_poly_q_struct) -> libc::c_int;
    #[link_name = "fmpz_poly_q_is_one__extern"]
    pub fn fmpz_poly_q_is_one(op: *const fmpz_poly_q_struct) -> libc::c_int;
    #[link_name = "fmpz_poly_q_equal__extern"]
    pub fn fmpz_poly_q_equal(
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    ) -> libc::c_int;
    pub fn fmpz_poly_q_add_in_place(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_sub_in_place(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_add(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_sub(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_addmul(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_submul(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_scalar_mul_si(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: mp_limb_signed_t,
    );
    pub fn fmpz_poly_q_scalar_mul_fmpz(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: *const fmpz,
    );
    pub fn fmpz_poly_q_scalar_mul_fmpq(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: *const fmpq,
    );
    pub fn fmpz_poly_q_scalar_div_si(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: mp_limb_signed_t,
    );
    pub fn fmpz_poly_q_scalar_div_fmpz(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: *const fmpz,
    );
    pub fn fmpz_poly_q_scalar_div_fmpq(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        x: *const fmpq,
    );
    pub fn fmpz_poly_q_mul(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_div(
        rop: *mut fmpz_poly_q_struct,
        op1: *const fmpz_poly_q_struct,
        op2: *const fmpz_poly_q_struct,
    );
    pub fn fmpz_poly_q_pow(
        rop: *mut fmpz_poly_q_struct,
        op: *const fmpz_poly_q_struct,
        exp: mp_limb_t,
    );
    pub fn fmpz_poly_q_derivative(rop: *mut fmpz_poly_q_struct, op: *const fmpz_poly_q_struct);
    pub fn fmpz_poly_q_evaluate_fmpq(
        rop: *mut fmpq,
        f: *const fmpz_poly_q_struct,
        a: *const fmpq,
    ) -> libc::c_int;
    pub fn fmpz_poly_q_set_str(rop: *mut fmpz_poly_q_struct, s: *const libc::c_char)
        -> libc::c_int;
    pub fn fmpz_poly_q_get_str(op: *const fmpz_poly_q_struct) -> *mut libc::c_char;
    pub fn fmpz_poly_q_get_str_pretty(
        op: *const fmpz_poly_q_struct,
        x: *const libc::c_char,
    ) -> *mut libc::c_char;
    pub fn fmpz_poly_q_print(op: *const fmpz_poly_q_struct) -> libc::c_int;
    pub fn fmpz_poly_q_print_pretty(
        op: *const fmpz_poly_q_struct,
        x: *const libc::c_char,
    ) -> libc::c_int;
}
