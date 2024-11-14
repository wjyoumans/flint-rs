/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::bindgen::*;
use crate::acb::*;
use crate::acb_calc::*;
use crate::acb_dft::*;
use crate::acb_dirichlet::*;
use crate::acb_elliptic::*;
use crate::acb_hypgeom::*;
use crate::acb_mat::*;
use crate::acb_modular::*;
use crate::acb_poly::*;
use crate::acb_theta::*;
use crate::acb_types::*;
use crate::acf::*;
use crate::acf_types::*;
use crate::aprcl::*;
use crate::arb::*;
use crate::arb_calc::*;
use crate::arb_fmpz_poly::*;
use crate::arb_fpwrap::*;
use crate::arb_hypgeom::*;
use crate::arb_mat::*;
use crate::arb_poly::*;
use crate::arb_types::*;
use crate::arf::*;
use crate::arf_types::*;
use crate::arith::*;
use crate::bernoulli::*;
use crate::bool_mat::*;
use crate::ca::*;
use crate::ca_ext::*;
use crate::ca_field::*;
use crate::ca_mat::*;
use crate::ca_poly::*;
use crate::ca_vec::*;
use crate::calcium::*;
use crate::d_mat::*;
use crate::d_vec::*;
use crate::dirichlet::*;
use crate::dlog::*;
use crate::double_extras::*;
use crate::double_interval::*;
use crate::fexpr::*;
use crate::fexpr_builtin::*;
use crate::fft::*;
use crate::fft_tuning::*;
use crate::flint_config::*;
use crate::flint::*;
use crate::fmpq::*;
use crate::fmpq_mat::*;
use crate::fmpq_mpoly::*;
use crate::fmpq_mpoly_factor::*;
use crate::fmpq_poly::*;
use crate::fmpq_vec::*;
use crate::fmpz::*;
use crate::fmpz_extras::*;
use crate::fmpz_factor::*;
use crate::fmpz_lll::*;
use crate::fmpz_mat::*;
use crate::fmpz_mod::*;
use crate::fmpz_mod_mat::*;
use crate::fmpz_mod_mpoly::*;
use crate::fmpz_mod_mpoly_factor::*;
use crate::fmpz_mod_poly::*;
use crate::fmpz_mod_poly_factor::*;
use crate::fmpz_mod_types::*;
use crate::fmpz_mod_vec::*;
use crate::fmpz_mpoly::*;
use crate::fmpz_mpoly_factor::*;
use crate::fmpz_mpoly_q::*;
use crate::fmpz_poly::*;
use crate::fmpz_poly_factor::*;
use crate::fmpz_poly_mat::*;
use crate::fmpz_poly_q::*;
use crate::fmpz_types::*;
use crate::fmpz_vec::*;
use crate::fmpzi::*;
use crate::fq::*;
use crate::fq_default::*;
use crate::fq_default_mat::*;
use crate::fq_default_poly::*;
use crate::fq_default_poly_factor::*;
use crate::fq_embed::*;
use crate::fq_embed_templates::*;
use crate::fq_mat::*;
use crate::fq_mat_templates::*;
use crate::fq_nmod::*;
use crate::fq_nmod_embed::*;
use crate::fq_nmod_mat::*;
use crate::fq_nmod_mpoly::*;
use crate::fq_nmod_mpoly_factor::*;
use crate::fq_nmod_poly::*;
use crate::fq_nmod_poly_factor::*;
use crate::fq_nmod_types::*;
use crate::fq_nmod_vec::*;
use crate::fq_poly::*;
use crate::fq_poly_factor::*;
use crate::fq_poly_factor_templates::*;
use crate::fq_poly_templates::*;
use crate::fq_templates::*;
use crate::fq_types::*;
use crate::fq_vec::*;
use crate::fq_vec_templates::*;
use crate::fq_zech::*;
use crate::fq_zech_embed::*;
use crate::fq_zech_mat::*;
use crate::fq_zech_mpoly::*;
use crate::fq_zech_mpoly_factor::*;
use crate::fq_zech_poly::*;
use crate::fq_zech_poly_factor::*;
use crate::fq_zech_types::*;
use crate::fq_zech_vec::*;
use crate::gmpcompat::*;
use crate::gr::*;
use crate::gr_generic::*;
use crate::gr_mat::*;
use crate::gr_mpoly::*;
use crate::gr_poly::*;
use crate::gr_special::*;
use crate::gr_vec::*;
use crate::hypgeom::*;
use crate::limb_types::*;
use crate::long_extras::*;
use crate::longlong::*;
use crate::mag::*;
use crate::mpf_impl::*;
use crate::mpfr_mat::*;
use crate::mpfr_vec::*;
use crate::mpoly::*;
use crate::mpoly_types::*;
use crate::n_poly::*;
use crate::n_poly_types::*;
use crate::nf::*;
use crate::nf_elem::*;
use crate::nmod::*;
use crate::nmod_mat::*;
use crate::nmod_mpoly::*;
use crate::nmod_mpoly_factor::*;
use crate::nmod_poly::*;
use crate::nmod_poly_factor::*;
use crate::nmod_poly_mat::*;
use crate::nmod_types::*;
use crate::nmod_vec::*;
use crate::padic::*;
use crate::padic_mat::*;
use crate::padic_poly::*;
use crate::padic_types::*;
use crate::partitions::*;
use crate::perm::*;
use crate::qadic::*;
use crate::qfb::*;
use crate::qqbar::*;
use crate::qsieve::*;
use crate::templates::*;
use crate::thread_pool::*;
use crate::thread_support::*;
use crate::ulong_extras::*;


#[repr(C)]
pub struct fmpq_mat_struct {
    pub entries: *mut fmpq,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut fmpq,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fmpq_mat_struct"][::std::mem::size_of::<fmpq_mat_struct>() - 32usize];
    ["Alignment of fmpq_mat_struct"][::std::mem::align_of::<fmpq_mat_struct>() - 8usize];
    ["Offset of field: fmpq_mat_struct::entries"]
        [::std::mem::offset_of!(fmpq_mat_struct, entries) - 0usize];
    ["Offset of field: fmpq_mat_struct::r"][::std::mem::offset_of!(fmpq_mat_struct, r) - 8usize];
    ["Offset of field: fmpq_mat_struct::c"][::std::mem::offset_of!(fmpq_mat_struct, c) - 16usize];
    ["Offset of field: fmpq_mat_struct::rows"]
        [::std::mem::offset_of!(fmpq_mat_struct, rows) - 24usize];
};
impl Default for fmpq_mat_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fmpq_mat_t = [fmpq_mat_struct; 1usize];
#[repr(C)]
pub struct fmpq_poly_struct {
    pub coeffs: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub den: fmpz_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fmpq_poly_struct"][::std::mem::size_of::<fmpq_poly_struct>() - 32usize];
    ["Alignment of fmpq_poly_struct"][::std::mem::align_of::<fmpq_poly_struct>() - 8usize];
    ["Offset of field: fmpq_poly_struct::coeffs"]
        [::std::mem::offset_of!(fmpq_poly_struct, coeffs) - 0usize];
    ["Offset of field: fmpq_poly_struct::alloc"]
        [::std::mem::offset_of!(fmpq_poly_struct, alloc) - 8usize];
    ["Offset of field: fmpq_poly_struct::length"]
        [::std::mem::offset_of!(fmpq_poly_struct, length) - 16usize];
    ["Offset of field: fmpq_poly_struct::den"]
        [::std::mem::offset_of!(fmpq_poly_struct, den) - 24usize];
};
impl Default for fmpq_poly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fmpq_poly_t = [fmpq_poly_struct; 1usize];
#[repr(C)]
pub struct fmpq_mpoly_struct {
    pub content: fmpq_t,
    pub zpoly: fmpz_mpoly_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fmpq_mpoly_struct"][::std::mem::size_of::<fmpq_mpoly_struct>() - 56usize];
    ["Alignment of fmpq_mpoly_struct"][::std::mem::align_of::<fmpq_mpoly_struct>() - 8usize];
    ["Offset of field: fmpq_mpoly_struct::content"]
        [::std::mem::offset_of!(fmpq_mpoly_struct, content) - 0usize];
    ["Offset of field: fmpq_mpoly_struct::zpoly"]
        [::std::mem::offset_of!(fmpq_mpoly_struct, zpoly) - 16usize];
};
impl Default for fmpq_mpoly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fmpq_mpoly_t = [fmpq_mpoly_struct; 1usize];
#[repr(C)]
pub struct fmpq_mpoly_factor_struct {
    pub constant: fmpq_t,
    pub poly: *mut fmpq_mpoly_struct,
    pub exp: *mut fmpz,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fmpq_mpoly_factor_struct"]
        [::std::mem::size_of::<fmpq_mpoly_factor_struct>() - 48usize];
    ["Alignment of fmpq_mpoly_factor_struct"]
        [::std::mem::align_of::<fmpq_mpoly_factor_struct>() - 8usize];
    ["Offset of field: fmpq_mpoly_factor_struct::constant"]
        [::std::mem::offset_of!(fmpq_mpoly_factor_struct, constant) - 0usize];
    ["Offset of field: fmpq_mpoly_factor_struct::poly"]
        [::std::mem::offset_of!(fmpq_mpoly_factor_struct, poly) - 16usize];
    ["Offset of field: fmpq_mpoly_factor_struct::exp"]
        [::std::mem::offset_of!(fmpq_mpoly_factor_struct, exp) - 24usize];
    ["Offset of field: fmpq_mpoly_factor_struct::num"]
        [::std::mem::offset_of!(fmpq_mpoly_factor_struct, num) - 32usize];
    ["Offset of field: fmpq_mpoly_factor_struct::alloc"]
        [::std::mem::offset_of!(fmpq_mpoly_factor_struct, alloc) - 40usize];
};
impl Default for fmpq_mpoly_factor_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fmpq_mpoly_factor_t = [fmpq_mpoly_factor_struct; 1usize];