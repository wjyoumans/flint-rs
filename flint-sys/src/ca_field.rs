/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::ca::*;
use crate::calcium::*;
use crate::qqbar::*;


extern "C" {
    pub fn ca_field_init_qq(K: *mut ca_field_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_field_init_nf(
        K: *mut ca_field_struct,
        x: *const qqbar_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_field_init_const(
        K: *mut ca_field_struct,
        func: calcium_func_code,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_field_init_fx(
        K: *mut ca_field_struct,
        func: calcium_func_code,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_field_init_fxy(
        K: *mut ca_field_struct,
        func: calcium_func_code,
        x: *const ca_struct,
        y: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_field_init_multi(
        K: *mut ca_field_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_field_clear(K: *mut ca_field_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_field_set_ext(
        K: *mut ca_field_struct,
        i: mp_limb_signed_t,
        x: ca_ext_srcptr,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_field_print(K: *const ca_field_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_field_cmp(
        K1: *const ca_field_struct,
        K2: *const ca_field_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_field_build_ideal(K: *mut ca_field_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_field_build_ideal_erf(K: *mut ca_field_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_field_build_ideal_gamma(K: *mut ca_field_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_field_cache_init(cache: *mut ca_field_cache_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_field_cache_clear(cache: *mut ca_field_cache_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_field_cache_insert_ext(
        cache: *mut ca_field_cache_struct,
        x: *mut *mut ca_ext_struct,
        length: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> ca_field_ptr;
}
