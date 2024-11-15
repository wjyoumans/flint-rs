/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;


pub const FPWRAP_SUCCESS: u32 = 0;
pub const FPWRAP_UNABLE: u32 = 1;
pub const FPWRAP_ACCURATE_PARTS: u32 = 1;
pub const FPWRAP_CORRECT_ROUNDING: u32 = 2;
pub const FPWRAP_WORK_LIMIT: u32 = 65536;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct complex_double {
    pub real: f64,
    pub imag: f64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of complex_double"][::std::mem::size_of::<complex_double>() - 16usize];
    ["Alignment of complex_double"][::std::mem::align_of::<complex_double>() - 8usize];
    ["Offset of field: complex_double::real"]
        [::std::mem::offset_of!(complex_double, real) - 0usize];
    ["Offset of field: complex_double::imag"]
        [::std::mem::offset_of!(complex_double, imag) - 8usize];
};
extern "C" {
    pub fn arb_fpwrap_double_exp(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_exp(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_expm1(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_expm1(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_log(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_log(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_log1p(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_log1p(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_pow(res: *mut f64, x: f64, y: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_pow(
        res: *mut complex_double,
        x: complex_double,
        y: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_sqrt(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_sqrt(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_rsqrt(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_rsqrt(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_cbrt(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_cbrt(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_sin(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_sin(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_cos(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_cos(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_tan(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_tan(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_cot(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_cot(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_sec(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_sec(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_csc(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_csc(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_sinc(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_sinc(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_sin_pi(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_sin_pi(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_cos_pi(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_cos_pi(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_tan_pi(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_tan_pi(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_cot_pi(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_cot_pi(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_sinc_pi(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_sinc_pi(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_asin(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_asin(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_acos(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_acos(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_atan(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_atan(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_atan2(
        res: *mut f64,
        x1: f64,
        x2: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_asinh(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_asinh(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_acosh(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_acosh(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_atanh(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_atanh(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_lambertw(
        res: *mut f64,
        x: f64,
        branch: mp_limb_signed_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_lambertw(
        res: *mut complex_double,
        x: complex_double,
        branch: mp_limb_signed_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_rising(
        res: *mut f64,
        x: f64,
        n: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_rising(
        res: *mut complex_double,
        x: complex_double,
        n: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_gamma(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_gamma(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_rgamma(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_rgamma(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_lgamma(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_lgamma(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_digamma(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_digamma(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_zeta(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_zeta(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_hurwitz_zeta(
        res: *mut f64,
        s: f64,
        z: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_hurwitz_zeta(
        res: *mut complex_double,
        s: complex_double,
        z: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_barnes_g(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_barnes_g(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_log_barnes_g(res: *mut f64, x: f64, flags: libc::c_int)
        -> libc::c_int;
    pub fn arb_fpwrap_cdouble_log_barnes_g(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_polygamma(
        res: *mut f64,
        s: f64,
        z: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_polygamma(
        res: *mut complex_double,
        s: complex_double,
        z: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_polylog(
        res: *mut f64,
        s: f64,
        z: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_polylog(
        res: *mut complex_double,
        s: complex_double,
        z: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_lerch_phi(
        res: *mut f64,
        z: f64,
        s: f64,
        a: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_lerch_phi(
        res: *mut complex_double,
        z: complex_double,
        s: complex_double,
        a: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_dirichlet_eta(
        res: *mut complex_double,
        s: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_riemann_xi(
        res: *mut complex_double,
        s: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_hardy_theta(
        res: *mut complex_double,
        z: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_hardy_z(
        res: *mut complex_double,
        z: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_zeta_zero(
        res: *mut complex_double,
        n: mp_limb_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_erf(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_erf(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_erfc(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_erfc(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_erfi(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_erfi(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_erfinv(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_double_erfcinv(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_double_fresnel_s(
        res: *mut f64,
        x: f64,
        normalized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_fresnel_s(
        res: *mut complex_double,
        x: complex_double,
        normalized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_fresnel_c(
        res: *mut f64,
        x: f64,
        normalized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_fresnel_c(
        res: *mut complex_double,
        x: complex_double,
        normalized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_gamma_upper(
        res: *mut f64,
        s: f64,
        z: f64,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_gamma_upper(
        res: *mut complex_double,
        s: complex_double,
        z: complex_double,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_gamma_lower(
        res: *mut f64,
        s: f64,
        z: f64,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_gamma_lower(
        res: *mut complex_double,
        s: complex_double,
        z: complex_double,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_beta_lower(
        res: *mut f64,
        a: f64,
        b: f64,
        z: f64,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_beta_lower(
        res: *mut complex_double,
        a: complex_double,
        b: complex_double,
        z: complex_double,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_exp_integral_e(
        res: *mut f64,
        s: f64,
        z: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_exp_integral_e(
        res: *mut complex_double,
        s: complex_double,
        z: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_exp_integral_ei(
        res: *mut f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_exp_integral_ei(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_sin_integral(res: *mut f64, x: f64, flags: libc::c_int)
        -> libc::c_int;
    pub fn arb_fpwrap_cdouble_sin_integral(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_cos_integral(res: *mut f64, x: f64, flags: libc::c_int)
        -> libc::c_int;
    pub fn arb_fpwrap_cdouble_cos_integral(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_sinh_integral(
        res: *mut f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_sinh_integral(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_cosh_integral(
        res: *mut f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_cosh_integral(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_log_integral(
        res: *mut f64,
        x: f64,
        offset: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_log_integral(
        res: *mut complex_double,
        x: complex_double,
        offset: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_dilog(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_dilog(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_bessel_j(
        res: *mut f64,
        nu: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_bessel_j(
        res: *mut complex_double,
        nu: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_bessel_y(
        res: *mut f64,
        nu: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_bessel_y(
        res: *mut complex_double,
        nu: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_bessel_i(
        res: *mut f64,
        nu: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_bessel_i(
        res: *mut complex_double,
        nu: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_bessel_k(
        res: *mut f64,
        nu: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_bessel_k(
        res: *mut complex_double,
        nu: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_bessel_k_scaled(
        res: *mut f64,
        nu: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_bessel_k_scaled(
        res: *mut complex_double,
        nu: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_airy_ai(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_airy_ai(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_airy_ai_prime(
        res: *mut f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_airy_ai_prime(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_airy_bi(res: *mut f64, x: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_airy_bi(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_airy_bi_prime(
        res: *mut f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_airy_bi_prime(
        res: *mut complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_airy_ai_zero(
        res: *mut f64,
        n: mp_limb_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_airy_ai_prime_zero(
        res: *mut f64,
        n: mp_limb_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_airy_bi_zero(
        res: *mut f64,
        n: mp_limb_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_airy_bi_prime_zero(
        res: *mut f64,
        n: mp_limb_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_coulomb_f(
        res: *mut f64,
        l: f64,
        eta: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_coulomb_f(
        res: *mut complex_double,
        l: complex_double,
        eta: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_coulomb_g(
        res: *mut f64,
        l: f64,
        eta: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_coulomb_g(
        res: *mut complex_double,
        l: complex_double,
        eta: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_coulomb_hpos(
        res: *mut complex_double,
        l: complex_double,
        eta: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_coulomb_hneg(
        res: *mut complex_double,
        l: complex_double,
        eta: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_chebyshev_t(
        res: *mut f64,
        n: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_chebyshev_t(
        res: *mut complex_double,
        n: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_chebyshev_u(
        res: *mut f64,
        n: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_chebyshev_u(
        res: *mut complex_double,
        n: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_jacobi_p(
        res: *mut f64,
        n: f64,
        a: f64,
        b: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_jacobi_p(
        res: *mut complex_double,
        n: complex_double,
        a: complex_double,
        b: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_gegenbauer_c(
        res: *mut f64,
        n: f64,
        m: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_gegenbauer_c(
        res: *mut complex_double,
        n: complex_double,
        m: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_laguerre_l(
        res: *mut f64,
        n: f64,
        m: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_laguerre_l(
        res: *mut complex_double,
        n: complex_double,
        m: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_hermite_h(
        res: *mut f64,
        n: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_hermite_h(
        res: *mut complex_double,
        n: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_legendre_p(
        res: *mut f64,
        n: f64,
        m: f64,
        x: f64,
        type_: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_legendre_p(
        res: *mut complex_double,
        n: complex_double,
        m: complex_double,
        x: complex_double,
        type_: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_legendre_q(
        res: *mut f64,
        n: f64,
        m: f64,
        x: f64,
        type_: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_legendre_q(
        res: *mut complex_double,
        n: complex_double,
        m: complex_double,
        x: complex_double,
        type_: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_legendre_root(
        res1: *mut f64,
        res2: *mut f64,
        n: mp_limb_t,
        k: mp_limb_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_spherical_y(
        res: *mut complex_double,
        n: mp_limb_signed_t,
        m: mp_limb_signed_t,
        x1: complex_double,
        x2: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_hypgeom_0f1(
        res: *mut f64,
        a: f64,
        x: f64,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_hypgeom_0f1(
        res: *mut complex_double,
        a: complex_double,
        x: complex_double,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_hypgeom_1f1(
        res: *mut f64,
        a: f64,
        b: f64,
        x: f64,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_hypgeom_1f1(
        res: *mut complex_double,
        a: complex_double,
        b: complex_double,
        x: complex_double,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_hypgeom_u(
        res: *mut f64,
        a: f64,
        b: f64,
        x: f64,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_hypgeom_u(
        res: *mut complex_double,
        a: complex_double,
        b: complex_double,
        x: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_hypgeom_2f1(
        res: *mut f64,
        a: f64,
        b: f64,
        c: f64,
        x: f64,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_hypgeom_2f1(
        res: *mut complex_double,
        a: complex_double,
        b: complex_double,
        c: complex_double,
        x: complex_double,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_hypgeom_pfq(
        res: *mut f64,
        a: *const f64,
        p: mp_limb_signed_t,
        b: *const f64,
        q: mp_limb_signed_t,
        z: f64,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_hypgeom_pfq(
        res: *mut complex_double,
        a: *const complex_double,
        p: mp_limb_signed_t,
        b: *const complex_double,
        q: mp_limb_signed_t,
        z: complex_double,
        regularized: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_double_agm(res: *mut f64, x: f64, y: f64, flags: libc::c_int) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_agm(
        res: *mut complex_double,
        x: complex_double,
        y: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_k(
        res: *mut complex_double,
        m: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_e(
        res: *mut complex_double,
        m: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_pi(
        res: *mut complex_double,
        n: complex_double,
        m: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_f(
        res: *mut complex_double,
        phi: complex_double,
        m: complex_double,
        pi: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_e_inc(
        res: *mut complex_double,
        phi: complex_double,
        m: complex_double,
        pi: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_pi_inc(
        res: *mut complex_double,
        n: complex_double,
        phi: complex_double,
        m: complex_double,
        pi: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_rf(
        res: *mut complex_double,
        x: complex_double,
        y: complex_double,
        z: complex_double,
        option: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_rg(
        res: *mut complex_double,
        x: complex_double,
        y: complex_double,
        z: complex_double,
        option: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_rj(
        res: *mut complex_double,
        x: complex_double,
        y: complex_double,
        z: complex_double,
        w: complex_double,
        option: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_p(
        res: *mut complex_double,
        z: complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_p_prime(
        res: *mut complex_double,
        z: complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_inv_p(
        res: *mut complex_double,
        z: complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_zeta(
        res: *mut complex_double,
        z: complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_elliptic_sigma(
        res: *mut complex_double,
        z: complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_jacobi_theta_1(
        res: *mut complex_double,
        z: complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_jacobi_theta_2(
        res: *mut complex_double,
        z: complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_jacobi_theta_3(
        res: *mut complex_double,
        z: complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_jacobi_theta_4(
        res: *mut complex_double,
        z: complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_dedekind_eta(
        res: *mut complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_modular_j(
        res: *mut complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_modular_lambda(
        res: *mut complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn arb_fpwrap_cdouble_modular_delta(
        res: *mut complex_double,
        tau: complex_double,
        flags: libc::c_int,
    ) -> libc::c_int;
}
