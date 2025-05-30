/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::fq_nmod_types::*;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_struct {
    pub value: mp_limb_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_struct"][::std::mem::size_of::<fq_zech_struct>() - 8usize];
    ["Alignment of fq_zech_struct"][::std::mem::align_of::<fq_zech_struct>() - 8usize];
    ["Offset of field: fq_zech_struct::value"]
        [::std::mem::offset_of!(fq_zech_struct, value) - 0usize];
};
impl Default for fq_zech_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_t = [fq_zech_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_ctx_struct {
    pub qm1: mp_limb_t,
    pub qm1o2: mp_limb_t,
    pub qm1opm1: mp_limb_t,
    pub p: mp_limb_t,
    pub ppre: f64,
    pub prime_root: mp_limb_t,
    pub zech_log_table: *mut mp_limb_t,
    pub prime_field_table: *mut mp_limb_t,
    pub eval_table: *mut mp_limb_t,
    pub fq_nmod_ctx: *mut fq_nmod_ctx_struct,
    pub owns_fq_nmod_ctx: libc::c_int,
    pub is_conway: libc::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_ctx_struct"][::std::mem::size_of::<fq_zech_ctx_struct>() - 88usize];
    ["Alignment of fq_zech_ctx_struct"][::std::mem::align_of::<fq_zech_ctx_struct>() - 8usize];
    ["Offset of field: fq_zech_ctx_struct::qm1"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, qm1) - 0usize];
    ["Offset of field: fq_zech_ctx_struct::qm1o2"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, qm1o2) - 8usize];
    ["Offset of field: fq_zech_ctx_struct::qm1opm1"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, qm1opm1) - 16usize];
    ["Offset of field: fq_zech_ctx_struct::p"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, p) - 24usize];
    ["Offset of field: fq_zech_ctx_struct::ppre"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, ppre) - 32usize];
    ["Offset of field: fq_zech_ctx_struct::prime_root"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, prime_root) - 40usize];
    ["Offset of field: fq_zech_ctx_struct::zech_log_table"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, zech_log_table) - 48usize];
    ["Offset of field: fq_zech_ctx_struct::prime_field_table"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, prime_field_table) - 56usize];
    ["Offset of field: fq_zech_ctx_struct::eval_table"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, eval_table) - 64usize];
    ["Offset of field: fq_zech_ctx_struct::fq_nmod_ctx"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, fq_nmod_ctx) - 72usize];
    ["Offset of field: fq_zech_ctx_struct::owns_fq_nmod_ctx"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, owns_fq_nmod_ctx) - 80usize];
    ["Offset of field: fq_zech_ctx_struct::is_conway"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, is_conway) - 84usize];
};
impl Default for fq_zech_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_ctx_t = [fq_zech_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_mat_struct {
    pub entries: *mut fq_zech_struct,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut fq_zech_struct,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_mat_struct"][::std::mem::size_of::<fq_zech_mat_struct>() - 32usize];
    ["Alignment of fq_zech_mat_struct"][::std::mem::align_of::<fq_zech_mat_struct>() - 8usize];
    ["Offset of field: fq_zech_mat_struct::entries"]
        [::std::mem::offset_of!(fq_zech_mat_struct, entries) - 0usize];
    ["Offset of field: fq_zech_mat_struct::r"]
        [::std::mem::offset_of!(fq_zech_mat_struct, r) - 8usize];
    ["Offset of field: fq_zech_mat_struct::c"]
        [::std::mem::offset_of!(fq_zech_mat_struct, c) - 16usize];
    ["Offset of field: fq_zech_mat_struct::rows"]
        [::std::mem::offset_of!(fq_zech_mat_struct, rows) - 24usize];
};
impl Default for fq_zech_mat_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_mat_t = [fq_zech_mat_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_poly_struct {
    pub coeffs: *mut fq_zech_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_poly_struct"][::std::mem::size_of::<fq_zech_poly_struct>() - 24usize];
    ["Alignment of fq_zech_poly_struct"][::std::mem::align_of::<fq_zech_poly_struct>() - 8usize];
    ["Offset of field: fq_zech_poly_struct::coeffs"]
        [::std::mem::offset_of!(fq_zech_poly_struct, coeffs) - 0usize];
    ["Offset of field: fq_zech_poly_struct::alloc"]
        [::std::mem::offset_of!(fq_zech_poly_struct, alloc) - 8usize];
    ["Offset of field: fq_zech_poly_struct::length"]
        [::std::mem::offset_of!(fq_zech_poly_struct, length) - 16usize];
};
impl Default for fq_zech_poly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_poly_t = [fq_zech_poly_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_zech_poly_factor_struct {
    pub poly: *mut fq_zech_poly_struct,
    pub exp: *mut mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_poly_factor_struct"]
        [::std::mem::size_of::<fq_zech_poly_factor_struct>() - 32usize];
    ["Alignment of fq_zech_poly_factor_struct"]
        [::std::mem::align_of::<fq_zech_poly_factor_struct>() - 8usize];
    ["Offset of field: fq_zech_poly_factor_struct::poly"]
        [::std::mem::offset_of!(fq_zech_poly_factor_struct, poly) - 0usize];
    ["Offset of field: fq_zech_poly_factor_struct::exp"]
        [::std::mem::offset_of!(fq_zech_poly_factor_struct, exp) - 8usize];
    ["Offset of field: fq_zech_poly_factor_struct::num"]
        [::std::mem::offset_of!(fq_zech_poly_factor_struct, num) - 16usize];
    ["Offset of field: fq_zech_poly_factor_struct::alloc"]
        [::std::mem::offset_of!(fq_zech_poly_factor_struct, alloc) - 24usize];
};
impl Default for fq_zech_poly_factor_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_poly_factor_t = [fq_zech_poly_factor_struct; 1usize];
