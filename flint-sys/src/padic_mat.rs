/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::flint::*;
use crate::fmpq_types::*;
use crate::fmpz_types::*;
use crate::padic_types::*;


extern "C" {
    #[link_name = "padic_mat__extern"]
    pub fn padic_mat(A: *const padic_mat_struct) -> *mut fmpz_mat_struct;
    #[link_name = "padic_mat_entry__extern"]
    pub fn padic_mat_entry(
        A: *const padic_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> *mut fmpz;
    #[link_name = "padic_mat_get_val__extern"]
    pub fn padic_mat_get_val(A: *const padic_mat_struct) -> mp_limb_signed_t;
    #[link_name = "padic_mat_get_prec__extern"]
    pub fn padic_mat_get_prec(A: *const padic_mat_struct) -> mp_limb_signed_t;
    #[link_name = "padic_mat_nrows__extern"]
    pub fn padic_mat_nrows(A: *const padic_mat_struct) -> mp_limb_signed_t;
    #[link_name = "padic_mat_ncols__extern"]
    pub fn padic_mat_ncols(A: *const padic_mat_struct) -> mp_limb_signed_t;
    pub fn padic_mat_init(A: *mut padic_mat_struct, r: mp_limb_signed_t, c: mp_limb_signed_t);
    pub fn padic_mat_init2(
        A: *mut padic_mat_struct,
        r: mp_limb_signed_t,
        c: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn padic_mat_clear(A: *mut padic_mat_struct);
    pub fn _padic_mat_canonicalise(A: *mut padic_mat_struct, ctx: *const padic_ctx_struct);
    pub fn _padic_mat_reduce(A: *mut padic_mat_struct, ctx: *const padic_ctx_struct);
    pub fn padic_mat_reduce(A: *mut padic_mat_struct, ctx: *const padic_ctx_struct);
    #[link_name = "padic_mat_is_empty__extern"]
    pub fn padic_mat_is_empty(A: *const padic_mat_struct) -> libc::c_int;
    #[link_name = "padic_mat_is_square__extern"]
    pub fn padic_mat_is_square(A: *const padic_mat_struct) -> libc::c_int;
    pub fn padic_mat_is_canonical(
        A: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_mat_is_reduced(
        A: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_mat_set(
        B: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_swap(A: *mut padic_mat_struct, B: *mut padic_mat_struct);
    #[link_name = "padic_mat_swap_entrywise__extern"]
    pub fn padic_mat_swap_entrywise(mat1: *mut padic_mat_struct, mat2: *mut padic_mat_struct);
    pub fn padic_mat_zero(A: *mut padic_mat_struct);
    pub fn padic_mat_one(A: *mut padic_mat_struct);
    pub fn padic_mat_set_fmpq_mat(
        B: *mut padic_mat_struct,
        A: *const fmpq_mat_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_get_fmpq_mat(
        B: *mut fmpq_mat_struct,
        A: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_get_entry_padic(
        rop: *mut padic_struct,
        op: *const padic_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_set_entry_padic(
        rop: *mut padic_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_equal(A: *const padic_mat_struct, B: *const padic_mat_struct) -> libc::c_int;
    pub fn padic_mat_is_zero(A: *const padic_mat_struct) -> libc::c_int;
    pub fn padic_mat_fprint(
        file: *mut FILE,
        A: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_mat_fprint_pretty(
        file: *mut FILE,
        A: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_mat_print(A: *const padic_mat_struct, ctx: *const padic_ctx_struct)
        -> libc::c_int;
    pub fn padic_mat_print_pretty(
        A: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_mat_randtest(
        mat: *mut padic_mat_struct,
        state: *mut flint_rand_s,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_transpose(B: *mut padic_mat_struct, A: *const padic_mat_struct);
    pub fn _padic_mat_add(
        C: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        B: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_add(
        C: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        B: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn _padic_mat_sub(
        C: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        B: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_sub(
        C: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        B: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn _padic_mat_neg(B: *mut padic_mat_struct, A: *const padic_mat_struct);
    pub fn padic_mat_neg(
        B: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn _padic_mat_scalar_mul_padic(
        B: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        c: *const padic_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_scalar_mul_padic(
        B: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        c: *const padic_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn _padic_mat_scalar_mul_fmpz(
        B: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        c: *const fmpz,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_scalar_mul_fmpz(
        B: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        c: *const fmpz,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_scalar_div_fmpz(
        B: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        c: *const fmpz,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_mat_mul(
        C: *mut padic_mat_struct,
        A: *const padic_mat_struct,
        B: *const padic_mat_struct,
        ctx: *const padic_ctx_struct,
    );
}
