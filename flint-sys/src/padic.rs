/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::flint::*;
use crate::padic_types::*;


extern "C" {
    #[link_name = "padic_unit__extern"]
    pub fn padic_unit(x: *const padic_struct) -> *mut fmpz;
    #[link_name = "padic_get_val__extern"]
    pub fn padic_get_val(x: *const padic_struct) -> mp_limb_signed_t;
    #[link_name = "padic_get_prec__extern"]
    pub fn padic_get_prec(x: *const padic_struct) -> mp_limb_signed_t;
    pub fn padic_ctx_init(
        ctx: *mut padic_ctx_struct,
        p: *const fmpz,
        min: mp_limb_signed_t,
        max: mp_limb_signed_t,
        mode: padic_print_mode,
    );
    pub fn padic_ctx_clear(ctx: *mut padic_ctx_struct);
    #[link_name = "_padic_ctx_pow_ui__extern"]
    pub fn _padic_ctx_pow_ui(
        rop: *mut fmpz,
        e: mp_limb_t,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "padic_ctx_pow_ui__extern"]
    pub fn padic_ctx_pow_ui(rop: *mut fmpz, e: mp_limb_t, ctx: *const padic_ctx_struct);
    pub fn padic_init(rop: *mut padic_struct);
    pub fn padic_init2(rop: *mut padic_struct, N: mp_limb_signed_t);
    pub fn padic_clear(rop: *mut padic_struct);
    #[link_name = "_padic_canonicalise__extern"]
    pub fn _padic_canonicalise(rop: *mut padic_struct, ctx: *const padic_ctx_struct);
    pub fn _padic_reduce(rop: *mut padic_struct, ctx: *const padic_ctx_struct);
    pub fn padic_reduce(rop: *mut padic_struct, ctx: *const padic_ctx_struct);
    pub fn padic_randtest(
        rop: *mut padic_struct,
        state: *mut flint_rand_s,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_randtest_not_zero(
        rop: *mut padic_struct,
        state: *mut flint_rand_s,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_randtest_int(
        rop: *mut padic_struct,
        state: *mut flint_rand_s,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_set(rop: *mut padic_struct, op: *const padic_struct, ctx: *const padic_ctx_struct);
    pub fn padic_set_si(rop: *mut padic_struct, op: mp_limb_signed_t, ctx: *const padic_ctx_struct);
    pub fn padic_set_ui(rop: *mut padic_struct, op: mp_limb_t, ctx: *const padic_ctx_struct);
    pub fn padic_set_fmpz(rop: *mut padic_struct, op: *const fmpz, ctx: *const padic_ctx_struct);
    pub fn padic_set_fmpq(rop: *mut padic_struct, op: *const fmpq, ctx: *const padic_ctx_struct);
    pub fn padic_set_mpz(
        rop: *mut padic_struct,
        op: *const __mpz_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_set_mpq(
        rop: *mut padic_struct,
        op: *const __mpq_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_get_fmpz(rop: *mut fmpz, op: *const padic_struct, ctx: *const padic_ctx_struct);
    pub fn padic_get_fmpq(rop: *mut fmpq, op: *const padic_struct, ctx: *const padic_ctx_struct);
    pub fn padic_get_mpz(
        rop: *mut __mpz_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_get_mpq(
        rop: *mut __mpq_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    );
    #[link_name = "padic_swap__extern"]
    pub fn padic_swap(op1: *mut padic_struct, op2: *mut padic_struct);
    #[link_name = "padic_zero__extern"]
    pub fn padic_zero(rop: *mut padic_struct);
    #[link_name = "padic_one__extern"]
    pub fn padic_one(rop: *mut padic_struct);
    #[link_name = "padic_is_zero__extern"]
    pub fn padic_is_zero(op: *const padic_struct) -> libc::c_int;
    #[link_name = "padic_is_one__extern"]
    pub fn padic_is_one(op: *const padic_struct) -> libc::c_int;
    #[link_name = "padic_equal__extern"]
    pub fn padic_equal(op1: *const padic_struct, op2: *const padic_struct) -> libc::c_int;
    pub fn _padic_lifts_exps(
        n: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
    ) -> *mut mp_limb_signed_t;
    pub fn _padic_lifts_pows(
        pow: *mut fmpz,
        a: *const mp_limb_signed_t,
        n: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn padic_add(
        rop: *mut padic_struct,
        op1: *const padic_struct,
        op2: *const padic_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_sub(
        rop: *mut padic_struct,
        op1: *const padic_struct,
        op2: *const padic_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_neg(rop: *mut padic_struct, op: *const padic_struct, ctx: *const padic_ctx_struct);
    pub fn padic_mul(
        rop: *mut padic_struct,
        op1: *const padic_struct,
        op2: *const padic_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_shift(
        rop: *mut padic_struct,
        op: *const padic_struct,
        v: mp_limb_signed_t,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_div(
        rop: *mut padic_struct,
        op1: *const padic_struct,
        op2: *const padic_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn _padic_inv_precompute(S: *mut padic_inv_struct, p: *const fmpz, N: mp_limb_signed_t);
    pub fn _padic_inv_clear(S: *mut padic_inv_struct);
    pub fn _padic_inv_precomp(rop: *mut fmpz, op: *const fmpz, S: *const padic_inv_struct);
    pub fn _padic_inv(rop: *mut fmpz, op: *const fmpz, p: *const fmpz, N: mp_limb_signed_t);
    pub fn padic_inv(rop: *mut padic_struct, op: *const padic_struct, ctx: *const padic_ctx_struct);
    pub fn padic_sqrt(
        rop: *mut padic_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_pow_si(
        rop: *mut padic_struct,
        op: *const padic_struct,
        e: mp_limb_signed_t,
        ctx: *const padic_ctx_struct,
    );
    pub fn _padic_exp_bound(
        v: mp_limb_signed_t,
        N: mp_limb_signed_t,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn _padic_exp(
        rop: *mut fmpz,
        u: *const fmpz,
        v: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn _padic_exp_rectangular(
        rop: *mut fmpz,
        u: *const fmpz,
        v: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn _padic_exp_balanced(
        rop: *mut fmpz,
        u: *const fmpz,
        v: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn padic_exp(
        rop: *mut padic_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_exp_rectangular(
        rop: *mut padic_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_exp_balanced(
        rop: *mut padic_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn _padic_log_bound(
        v: mp_limb_signed_t,
        N: mp_limb_signed_t,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn _padic_log(
        z: *mut fmpz,
        y: *const fmpz,
        v: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn _padic_log_rectangular(
        z: *mut fmpz,
        y: *const fmpz,
        v: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn _padic_log_satoh(
        z: *mut fmpz,
        y: *const fmpz,
        v: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn _padic_log_balanced(
        z: *mut fmpz,
        y: *const fmpz,
        v: mp_limb_signed_t,
        p: *const fmpz,
        N: mp_limb_signed_t,
    );
    pub fn padic_log(
        rop: *mut padic_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_log_rectangular(
        rop: *mut padic_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_log_satoh(
        rop: *mut padic_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_log_balanced(
        rop: *mut padic_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn _padic_teichmuller(rop: *mut fmpz, op: *const fmpz, p: *const fmpz, N: mp_limb_signed_t);
    pub fn padic_teichmuller(
        rop: *mut padic_struct,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    );
    pub fn padic_val_fac_ui_2(N: mp_limb_t) -> mp_limb_t;
    pub fn padic_val_fac_ui(N: mp_limb_t, p: *const fmpz) -> mp_limb_t;
    pub fn padic_val_fac(rop: *mut fmpz, op: *const fmpz, p: *const fmpz);
    pub fn _padic_get_str(
        str_: *mut libc::c_char,
        op: *const padic_struct,
        p: *const fmpz,
        mode: padic_print_mode,
    ) -> *mut libc::c_char;
    pub fn padic_get_str(
        str_: *mut libc::c_char,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    ) -> *mut libc::c_char;
    pub fn _padic_fprint(
        file: *mut FILE,
        u: *const fmpz,
        v: mp_limb_signed_t,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_fprint(
        file: *mut FILE,
        op: *const padic_struct,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn _padic_print(
        u: *const fmpz,
        v: mp_limb_signed_t,
        ctx: *const padic_ctx_struct,
    ) -> libc::c_int;
    pub fn padic_print(op: *const padic_struct, ctx: *const padic_ctx_struct) -> libc::c_int;
    pub fn padic_debug(op: *const padic_struct);
}
