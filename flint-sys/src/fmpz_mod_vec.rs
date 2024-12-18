/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::flint::*;
use crate::fmpz_mod_types::*;


extern "C" {
    pub fn _fmpz_mod_vec_set_fmpz_vec(
        A: *mut fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_vec_neg(
        A: *mut fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_vec_add(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_vec_sub(
        a: *mut fmpz,
        b: *const fmpz,
        c: *const fmpz,
        n: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_vec_scalar_mul_fmpz_mod(
        A: *mut fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_vec_scalar_addmul_fmpz_mod(
        A: *mut fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_vec_mul(
        A: *mut fmpz,
        B: *const fmpz,
        C: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_vec_scalar_div_fmpz_mod(
        A: *mut fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_vec_dot(
        d: *mut fmpz,
        A: *const fmpz,
        B: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_vec_dot_rev(
        r: *mut fmpz,
        a: *const fmpz,
        b: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
