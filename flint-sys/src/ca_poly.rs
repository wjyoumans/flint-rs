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
use crate::fmpq_types::*;
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
pub struct ca_poly_struct {
    pub coeffs: *mut ca_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ca_poly_struct"][::std::mem::size_of::<ca_poly_struct>() - 24usize];
    ["Alignment of ca_poly_struct"][::std::mem::align_of::<ca_poly_struct>() - 8usize];
    ["Offset of field: ca_poly_struct::coeffs"]
        [::std::mem::offset_of!(ca_poly_struct, coeffs) - 0usize];
    ["Offset of field: ca_poly_struct::alloc"]
        [::std::mem::offset_of!(ca_poly_struct, alloc) - 8usize];
    ["Offset of field: ca_poly_struct::length"]
        [::std::mem::offset_of!(ca_poly_struct, length) - 16usize];
};
impl Default for ca_poly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ca_poly_t = [ca_poly_struct; 1usize];
#[repr(C)]
pub struct ca_poly_vec_struct {
    pub entries: *mut ca_poly_struct,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ca_poly_vec_struct"][::std::mem::size_of::<ca_poly_vec_struct>() - 24usize];
    ["Alignment of ca_poly_vec_struct"][::std::mem::align_of::<ca_poly_vec_struct>() - 8usize];
    ["Offset of field: ca_poly_vec_struct::entries"]
        [::std::mem::offset_of!(ca_poly_vec_struct, entries) - 0usize];
    ["Offset of field: ca_poly_vec_struct::length"]
        [::std::mem::offset_of!(ca_poly_vec_struct, length) - 8usize];
    ["Offset of field: ca_poly_vec_struct::alloc"]
        [::std::mem::offset_of!(ca_poly_vec_struct, alloc) - 16usize];
};
impl Default for ca_poly_vec_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ca_poly_vec_t = [ca_poly_vec_struct; 1usize];
extern "C" {
    #[link_name = "ca_poly_coeff_ptr__extern"]
    pub fn ca_poly_coeff_ptr(poly: *mut ca_poly_struct, i: mp_limb_signed_t) -> ca_ptr;
    pub fn ca_poly_init(poly: *mut ca_poly_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_init2(poly: *mut ca_poly_struct, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_clear(poly: *mut ca_poly_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_fit_length(
        poly: *mut ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_set_length(
        poly: *mut ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_normalise(poly: *mut ca_poly_struct, ctx: *mut ca_ctx_struct);
    #[link_name = "ca_poly_swap__extern"]
    pub fn ca_poly_swap(
        poly1: *mut ca_poly_struct,
        poly2: *mut ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_set_ca(poly: *mut ca_poly_struct, x: *const ca_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_set_si(poly: *mut ca_poly_struct, x: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    #[link_name = "ca_poly_zero__extern"]
    pub fn ca_poly_zero(poly: *mut ca_poly_struct, ctx: *mut ca_ctx_struct);
    #[link_name = "ca_poly_x__extern"]
    pub fn ca_poly_x(poly: *mut ca_poly_struct, ctx: *mut ca_ctx_struct);
    #[link_name = "ca_poly_one__extern"]
    pub fn ca_poly_one(poly: *mut ca_poly_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_set(
        res: *mut ca_poly_struct,
        src: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_set_fmpz_poly(
        res: *mut ca_poly_struct,
        src: *const fmpz_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_set_fmpq_poly(
        res: *mut ca_poly_struct,
        src: *const fmpq_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_transfer(
        res: *mut ca_poly_struct,
        res_ctx: *mut ca_ctx_struct,
        src: *const ca_poly_struct,
        src_ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_set_coeff_ca(
        poly: *mut ca_poly_struct,
        n: mp_limb_signed_t,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_randtest(
        poly: *mut ca_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        depth: mp_limb_signed_t,
        bits: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_randtest_rational(
        poly: *mut ca_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_print(poly: *const ca_poly_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_printn(
        poly: *const ca_poly_struct,
        digits: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_is_proper(poly: *const ca_poly_struct, ctx: *mut ca_ctx_struct) -> libc::c_int;
    pub fn ca_poly_make_monic(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn _ca_poly_reverse(
        res: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_reverse(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_check_equal(
        poly1: ca_srcptr,
        len1: mp_limb_signed_t,
        poly2: ca_srcptr,
        len2: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_poly_check_equal(
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_poly_check_is_zero(poly: *const ca_poly_struct, ctx: *mut ca_ctx_struct) -> truth_t;
    pub fn ca_poly_check_is_one(poly: *const ca_poly_struct, ctx: *mut ca_ctx_struct) -> truth_t;
    pub fn _ca_poly_shift_left(
        res: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_shift_left(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_shift_right(
        res: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_shift_right(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_neg(
        res: *mut ca_poly_struct,
        src: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_add(
        res: ca_ptr,
        poly1: ca_srcptr,
        len1: mp_limb_signed_t,
        poly2: ca_srcptr,
        len2: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_add(
        res: *mut ca_poly_struct,
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_sub(
        res: ca_ptr,
        poly1: ca_srcptr,
        len1: mp_limb_signed_t,
        poly2: ca_srcptr,
        len2: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_sub(
        res: *mut ca_poly_struct,
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_mul(
        C: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_mul(
        res: *mut ca_poly_struct,
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_poly_mul_ca__extern"]
    pub fn ca_poly_mul_ca(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        c: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_poly_div_ca__extern"]
    pub fn ca_poly_div_ca(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        c: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_poly_div_fmpz__extern"]
    pub fn ca_poly_div_fmpz(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        c: *const fmpz,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_mullow_same_nf(
        C: ca_ptr,
        A: ca_srcptr,
        Alen: mp_limb_signed_t,
        B: ca_srcptr,
        Blen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        K: *mut ca_field_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_mullow(
        C: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_mullow(
        res: *mut ca_poly_struct,
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_divrem_basecase(
        Q: ca_ptr,
        R: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        invB: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_divrem_basecase(
        Q: *mut ca_poly_struct,
        R: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn _ca_poly_divrem(
        Q: ca_ptr,
        R: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        invB: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_divrem(
        Q: *mut ca_poly_struct,
        R: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_poly_div(
        Q: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_poly_rem(
        R: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn _ca_poly_pow_ui_trunc(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_pow_ui_trunc(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_pow_ui(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        exp: mp_limb_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_pow_ui(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        exp: mp_limb_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_evaluate_horner(
        res: *mut ca_struct,
        f: ca_srcptr,
        len: mp_limb_signed_t,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_evaluate_horner(
        res: *mut ca_struct,
        f: *const ca_poly_struct,
        a: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_evaluate(
        res: *mut ca_struct,
        f: ca_srcptr,
        len: mp_limb_signed_t,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_evaluate(
        res: *mut ca_struct,
        f: *const ca_poly_struct,
        a: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_compose(
        res: ca_ptr,
        poly1: ca_srcptr,
        len1: mp_limb_signed_t,
        poly2: ca_srcptr,
        len2: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_compose(
        res: *mut ca_poly_struct,
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_derivative(
        res: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_derivative(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_integral(
        res: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_integral(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_gcd_euclidean(
        G: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn ca_poly_gcd_euclidean(
        G: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn _ca_poly_gcd(
        G: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn ca_poly_gcd(
        G: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn _ca_poly_inv_series(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_inv_series(
        res: *mut ca_poly_struct,
        f: *const ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_div_series(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        g: ca_srcptr,
        glen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_div_series(
        res: *mut ca_poly_struct,
        f: *const ca_poly_struct,
        g: *const ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_exp_series(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_exp_series(
        res: *mut ca_poly_struct,
        f: *const ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_log_series(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_log_series(
        res: *mut ca_poly_struct,
        f: *const ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_vec_init(len: mp_limb_signed_t, ctx: *mut ca_ctx_struct)
        -> *mut ca_poly_struct;
    pub fn ca_poly_vec_init(
        res: *mut ca_poly_vec_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_vec_fit_length(
        vec: *mut ca_poly_vec_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_vec_set_length(
        vec: *mut ca_poly_vec_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_vec_clear(
        v: *mut ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_vec_clear(vec: *mut ca_poly_vec_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_vec_append(
        vec: *mut ca_poly_vec_struct,
        f: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_factor_squarefree(
        c: *mut ca_struct,
        fac: *mut ca_poly_vec_struct,
        exp: *mut mp_limb_t,
        F: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_poly_squarefree_part(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn _ca_poly_set_roots(
        poly: ca_ptr,
        roots: ca_srcptr,
        exp: *const mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_set_roots(
        poly: *mut ca_poly_struct,
        roots: *mut ca_vec_struct,
        exp: *const mp_limb_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_roots(
        roots: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_poly_roots(
        roots: *mut ca_vec_struct,
        exp: *mut mp_limb_t,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
}