/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::arb_types::*;
use crate::arf_types::*;
use crate::flint::*;


pub const ARF_RESULT_EXACT: u32 = 0;
pub const ARF_RESULT_INEXACT: u32 = 1;
pub const ARF_EXP_ZERO: u32 = 0;
pub const MUL_MPFR_MIN_LIMBS: u32 = 25;
pub const MUL_MPFR_MAX_LIMBS: u32 = 800;
pub const ARF_MUL_STACK_ALLOC: u32 = 40;
pub const ARF_MUL_TLS_ALLOC: u32 = 1000;
pub const ARF_ADD_STACK_ALLOC: u32 = 40;
pub const ARF_ADD_TLS_ALLOC: u32 = 1000;
pub const arf_rnd_t_ARF_RND_DOWN: arf_rnd_t = 0;
pub const arf_rnd_t_ARF_RND_UP: arf_rnd_t = 1;
pub const arf_rnd_t_ARF_RND_FLOOR: arf_rnd_t = 2;
pub const arf_rnd_t_ARF_RND_CEIL: arf_rnd_t = 3;
pub const arf_rnd_t_ARF_RND_NEAR: arf_rnd_t = 4;
pub type arf_rnd_t = libc::c_uint;
extern "C" {
    #[link_name = "arf_rounds_down__extern"]
    pub fn arf_rounds_down(rnd: arf_rnd_t, sgnbit: libc::c_int) -> libc::c_int;
    #[link_name = "arf_rounds_up__extern"]
    pub fn arf_rounds_up(rnd: arf_rnd_t, sgnbit: libc::c_int) -> libc::c_int;
    pub fn _arf_promote(x: *mut arf_struct, n: mp_size_t);
    pub fn _arf_demote(x: *mut arf_struct);
    #[link_name = "arf_init__extern"]
    pub fn arf_init(x: *mut arf_struct);
    pub fn arf_clear(x: *mut arf_struct);
    pub fn _arf_vec_init(n: mp_limb_signed_t) -> arf_ptr;
    pub fn _arf_vec_clear(v: arf_ptr, n: mp_limb_signed_t);
    #[link_name = "arf_zero__extern"]
    pub fn arf_zero(x: *mut arf_struct);
    #[link_name = "arf_pos_inf__extern"]
    pub fn arf_pos_inf(x: *mut arf_struct);
    #[link_name = "arf_neg_inf__extern"]
    pub fn arf_neg_inf(x: *mut arf_struct);
    #[link_name = "arf_nan__extern"]
    pub fn arf_nan(x: *mut arf_struct);
    #[link_name = "arf_is_special__extern"]
    pub fn arf_is_special(x: *const arf_struct) -> libc::c_int;
    #[link_name = "arf_is_zero__extern"]
    pub fn arf_is_zero(x: *const arf_struct) -> libc::c_int;
    #[link_name = "arf_is_pos_inf__extern"]
    pub fn arf_is_pos_inf(x: *const arf_struct) -> libc::c_int;
    #[link_name = "arf_is_neg_inf__extern"]
    pub fn arf_is_neg_inf(x: *const arf_struct) -> libc::c_int;
    #[link_name = "arf_is_nan__extern"]
    pub fn arf_is_nan(x: *const arf_struct) -> libc::c_int;
    #[link_name = "arf_is_normal__extern"]
    pub fn arf_is_normal(x: *const arf_struct) -> libc::c_int;
    #[link_name = "arf_is_finite__extern"]
    pub fn arf_is_finite(x: *const arf_struct) -> libc::c_int;
    #[link_name = "arf_is_inf__extern"]
    pub fn arf_is_inf(x: *const arf_struct) -> libc::c_int;
    #[link_name = "arf_one__extern"]
    pub fn arf_one(x: *mut arf_struct);
    #[link_name = "arf_is_one__extern"]
    pub fn arf_is_one(x: *const arf_struct) -> libc::c_int;
    #[link_name = "arf_sgn__extern"]
    pub fn arf_sgn(x: *const arf_struct) -> libc::c_int;
    pub fn arf_cmp(x: *const arf_struct, y: *const arf_struct) -> libc::c_int;
    pub fn arf_cmpabs(x: *const arf_struct, y: *const arf_struct) -> libc::c_int;
    pub fn arf_cmpabs_ui(x: *const arf_struct, y: mp_limb_t) -> libc::c_int;
    pub fn arf_cmpabs_d(x: *const arf_struct, y: f64) -> libc::c_int;
    pub fn arf_cmp_si(x: *const arf_struct, y: mp_limb_signed_t) -> libc::c_int;
    pub fn arf_cmp_ui(x: *const arf_struct, y: mp_limb_t) -> libc::c_int;
    pub fn arf_cmp_d(x: *const arf_struct, y: f64) -> libc::c_int;
    #[link_name = "arf_swap__extern"]
    pub fn arf_swap(y: *mut arf_struct, x: *mut arf_struct);
    pub fn arf_set(y: *mut arf_struct, x: *const arf_struct);
    #[link_name = "arf_neg__extern"]
    pub fn arf_neg(y: *mut arf_struct, x: *const arf_struct);
    #[link_name = "arf_init_set_ui__extern"]
    pub fn arf_init_set_ui(x: *mut arf_struct, v: mp_limb_t);
    #[link_name = "arf_init_set_si__extern"]
    pub fn arf_init_set_si(x: *mut arf_struct, v: mp_limb_signed_t);
    #[link_name = "arf_set_ui__extern"]
    pub fn arf_set_ui(x: *mut arf_struct, v: mp_limb_t);
    #[link_name = "arf_set_si__extern"]
    pub fn arf_set_si(x: *mut arf_struct, v: mp_limb_signed_t);
    #[link_name = "arf_init_set_shallow__extern"]
    pub fn arf_init_set_shallow(z: *mut arf_struct, x: *const arf_struct);
    #[link_name = "arf_init_neg_shallow__extern"]
    pub fn arf_init_neg_shallow(z: *mut arf_struct, x: *const arf_struct);
    #[link_name = "arf_init_set_mag_shallow__extern"]
    pub fn arf_init_set_mag_shallow(y: *mut arf_struct, x: *const mag_struct);
    #[link_name = "arf_init_neg_mag_shallow__extern"]
    pub fn arf_init_neg_mag_shallow(z: *mut arf_struct, x: *const mag_struct);
    #[link_name = "arf_cmpabs_mag__extern"]
    pub fn arf_cmpabs_mag(x: *const arf_struct, y: *const mag_struct) -> libc::c_int;
    #[link_name = "arf_mag_cmpabs__extern"]
    pub fn arf_mag_cmpabs(x: *const mag_struct, y: *const arf_struct) -> libc::c_int;
    pub fn arf_set_mpn(y: *mut arf_struct, x: mp_srcptr, xn: mp_size_t, sgnbit: libc::c_int);
    #[link_name = "arf_set_mpz__extern"]
    pub fn arf_set_mpz(y: *mut arf_struct, x: *const __mpz_struct);
    #[link_name = "arf_set_fmpz__extern"]
    pub fn arf_set_fmpz(y: *mut arf_struct, x: *const fmpz);
    pub fn _arf_set_round_ui(
        x: *mut arf_struct,
        v: mp_limb_t,
        sgnbit: libc::c_int,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn _arf_set_round_uiui(
        z: *mut arf_struct,
        fix: *mut mp_limb_signed_t,
        hi: mp_limb_t,
        lo: mp_limb_t,
        sgnbit: libc::c_int,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn _arf_set_round_mpn(
        y: *mut arf_struct,
        exp_shift: *mut mp_limb_signed_t,
        x: mp_srcptr,
        xn: mp_size_t,
        sgnbit: libc::c_int,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_set_round_ui__extern"]
    pub fn arf_set_round_ui(
        x: *mut arf_struct,
        v: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_set_round_si__extern"]
    pub fn arf_set_round_si(
        x: *mut arf_struct,
        v: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_set_round_mpz__extern"]
    pub fn arf_set_round_mpz(
        y: *mut arf_struct,
        x: *const __mpz_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_set_round_fmpz__extern"]
    pub fn arf_set_round_fmpz(
        y: *mut arf_struct,
        x: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_set_round(
        y: *mut arf_struct,
        x: *const arf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_neg_round(
        y: *mut arf_struct,
        x: *const arf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_equal(x: *const arf_struct, y: *const arf_struct) -> libc::c_int;
    pub fn arf_equal_si(x: *const arf_struct, y: mp_limb_signed_t) -> libc::c_int;
    pub fn arf_equal_ui(x: *const arf_struct, y: mp_limb_t) -> libc::c_int;
    pub fn arf_equal_d(x: *const arf_struct, y: f64) -> libc::c_int;
    #[link_name = "arf_min__extern"]
    pub fn arf_min(z: *mut arf_struct, a: *const arf_struct, b: *const arf_struct);
    #[link_name = "arf_max__extern"]
    pub fn arf_max(z: *mut arf_struct, a: *const arf_struct, b: *const arf_struct);
    #[link_name = "arf_abs__extern"]
    pub fn arf_abs(y: *mut arf_struct, x: *const arf_struct);
    #[link_name = "arf_bits__extern"]
    pub fn arf_bits(x: *const arf_struct) -> mp_limb_signed_t;
    #[link_name = "arf_bot__extern"]
    pub fn arf_bot(e: *mut fmpz, x: *const arf_struct);
    pub fn arf_is_int(x: *const arf_struct) -> libc::c_int;
    pub fn arf_is_int_2exp_si(x: *const arf_struct, e: mp_limb_signed_t) -> libc::c_int;
    pub fn arf_cmp_2exp_si(x: *const arf_struct, e: mp_limb_signed_t) -> libc::c_int;
    pub fn arf_cmpabs_2exp_si(x: *const arf_struct, e: mp_limb_signed_t) -> libc::c_int;
    #[link_name = "arf_set_si_2exp_si__extern"]
    pub fn arf_set_si_2exp_si(x: *mut arf_struct, man: mp_limb_signed_t, exp: mp_limb_signed_t);
    #[link_name = "arf_set_ui_2exp_si__extern"]
    pub fn arf_set_ui_2exp_si(x: *mut arf_struct, man: mp_limb_t, exp: mp_limb_signed_t);
    #[link_name = "arf_mul_2exp_si__extern"]
    pub fn arf_mul_2exp_si(y: *mut arf_struct, x: *const arf_struct, e: mp_limb_signed_t);
    #[link_name = "arf_mul_2exp_fmpz__extern"]
    pub fn arf_mul_2exp_fmpz(y: *mut arf_struct, x: *const arf_struct, e: *const fmpz);
    #[link_name = "arf_set_round_fmpz_2exp__extern"]
    pub fn arf_set_round_fmpz_2exp(
        y: *mut arf_struct,
        x: *const fmpz,
        exp: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_abs_bound_lt_2exp_fmpz__extern"]
    pub fn arf_abs_bound_lt_2exp_fmpz(b: *mut fmpz, x: *const arf_struct);
    #[link_name = "arf_abs_bound_le_2exp_fmpz__extern"]
    pub fn arf_abs_bound_le_2exp_fmpz(b: *mut fmpz, x: *const arf_struct);
    pub fn arf_abs_bound_lt_2exp_si(x: *const arf_struct) -> mp_limb_signed_t;
    pub fn arf_frexp(man: *mut arf_struct, exp: *mut fmpz, x: *const arf_struct);
    pub fn arf_get_fmpz_2exp(man: *mut fmpz, exp: *mut fmpz, x: *const arf_struct);
    pub fn _arf_get_integer_mpn(
        y: mp_ptr,
        x: mp_srcptr,
        xn: mp_size_t,
        exp: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn _arf_set_mpn_fixed(
        z: *mut arf_struct,
        xp: mp_srcptr,
        xn: mp_size_t,
        fixn: mp_size_t,
        negative: libc::c_int,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_get_fmpz(z: *mut fmpz, x: *const arf_struct, rnd: arf_rnd_t) -> libc::c_int;
    pub fn arf_get_si(x: *const arf_struct, rnd: arf_rnd_t) -> mp_limb_signed_t;
    pub fn arf_get_fmpz_fixed_fmpz(
        y: *mut fmpz,
        x: *const arf_struct,
        e: *const fmpz,
    ) -> libc::c_int;
    pub fn arf_get_fmpz_fixed_si(
        y: *mut fmpz,
        x: *const arf_struct,
        e: mp_limb_signed_t,
    ) -> libc::c_int;
    #[link_name = "arf_set_fmpz_2exp__extern"]
    pub fn arf_set_fmpz_2exp(x: *mut arf_struct, man: *const fmpz, exp: *const fmpz);
    pub fn arf_floor(z: *mut arf_struct, x: *const arf_struct);
    pub fn arf_ceil(z: *mut arf_struct, x: *const arf_struct);
    pub fn arf_debug(x: *const arf_struct);
    pub fn arf_get_str(x: *const arf_struct, d: mp_limb_signed_t) -> *mut libc::c_char;
    pub fn arf_fprint(file: *mut FILE, x: *const arf_struct);
    pub fn arf_fprintd(file: *mut FILE, y: *const arf_struct, d: mp_limb_signed_t);
    pub fn arf_print(x: *const arf_struct);
    pub fn arf_printd(y: *const arf_struct, d: mp_limb_signed_t);
    pub fn arf_randtest(
        x: *mut arf_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arf_randtest_not_zero(
        x: *mut arf_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arf_randtest_special(
        x: *mut arf_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arf_urandom(
        x: *mut arf_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        rnd: arf_rnd_t,
    );
    pub static mut __arf_mul_tmp: mp_ptr;
    pub static mut __arf_mul_alloc: mp_limb_signed_t;
    pub fn _arf_mul_tmp_cleanup();
    pub fn arf_mul_special(z: *mut arf_struct, x: *const arf_struct, y: *const arf_struct);
    pub fn arf_mul_via_mpfr(
        z: *mut arf_struct,
        x: *const arf_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_mul_rnd_any(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_mul_rnd_down(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
    ) -> libc::c_int;
    #[link_name = "arf_neg_mul__extern"]
    pub fn arf_neg_mul(
        z: *mut arf_struct,
        x: *const arf_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_mul_ui__extern"]
    pub fn arf_mul_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_mul_si__extern"]
    pub fn arf_mul_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_mul_mpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *const __mpz_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_mul_fmpz__extern"]
    pub fn arf_mul_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub static mut __arf_add_tmp: mp_ptr;
    pub static mut __arf_add_alloc: mp_limb_signed_t;
    pub fn _arf_add_tmp_cleanup();
    pub fn _arf_add_mpn(
        z: *mut arf_struct,
        xp: mp_srcptr,
        xn: mp_size_t,
        xsgnbit: libc::c_int,
        xexp: *const fmpz,
        yp: mp_srcptr,
        yn: mp_size_t,
        ysgnbit: libc::c_int,
        shift: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_add(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_add_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_add_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_add_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_add_fmpz_2exp(
        z: arf_ptr,
        x: arf_srcptr,
        y: *const fmpz,
        exp: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_sub(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_sub_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_sub_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_sub_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_addmul(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_addmul_ui__extern"]
    pub fn arf_addmul_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_addmul_si__extern"]
    pub fn arf_addmul_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_addmul_mpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *const __mpz_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_addmul_fmpz__extern"]
    pub fn arf_addmul_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_submul(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_submul_ui__extern"]
    pub fn arf_submul_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_submul_si__extern"]
    pub fn arf_submul_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_submul_mpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *const __mpz_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_submul_fmpz__extern"]
    pub fn arf_submul_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_fma(
        res: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        z: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_sosq(
        z: *mut arf_struct,
        x: *const arf_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_div(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_div_ui__extern"]
    pub fn arf_div_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_ui_div__extern"]
    pub fn arf_ui_div(
        z: arf_ptr,
        x: mp_limb_t,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_div_si__extern"]
    pub fn arf_div_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_si_div__extern"]
    pub fn arf_si_div(
        z: arf_ptr,
        x: mp_limb_signed_t,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_div_fmpz__extern"]
    pub fn arf_div_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_fmpz_div__extern"]
    pub fn arf_fmpz_div(
        z: arf_ptr,
        x: *const fmpz,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "arf_fmpz_div_fmpz__extern"]
    pub fn arf_fmpz_div_fmpz(
        z: arf_ptr,
        x: *const fmpz,
        y: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_sqrt(
        z: arf_ptr,
        x: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_sqrt_ui(
        z: *mut arf_struct,
        x: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_sqrt_fmpz(
        z: *mut arf_struct,
        x: *const fmpz,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_rsqrt(
        z: arf_ptr,
        x: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_root(
        z: arf_ptr,
        x: arf_srcptr,
        k: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_get_mag(y: *mut mag_struct, x: *const arf_struct);
    pub fn arf_get_mag_lower(y: *mut mag_struct, x: *const arf_struct);
    #[link_name = "arf_set_mag__extern"]
    pub fn arf_set_mag(y: *mut arf_struct, x: *const mag_struct);
    #[link_name = "mag_init_set_arf__extern"]
    pub fn mag_init_set_arf(y: *mut mag_struct, x: *const arf_struct);
    #[link_name = "mag_fast_init_set_arf__extern"]
    pub fn mag_fast_init_set_arf(y: *mut mag_struct, x: *const arf_struct);
    #[link_name = "arf_mag_fast_add_ulp__extern"]
    pub fn arf_mag_fast_add_ulp(
        z: *mut mag_struct,
        x: *const mag_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    #[link_name = "arf_mag_add_ulp__extern"]
    pub fn arf_mag_add_ulp(
        z: *mut mag_struct,
        x: *const mag_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    #[link_name = "arf_mag_set_ulp__extern"]
    pub fn arf_mag_set_ulp(z: *mut mag_struct, y: *const arf_struct, prec: mp_limb_signed_t);
    pub fn arf_get_fmpq(y: *mut fmpq, x: *const arf_struct);
    #[link_name = "arf_set_fmpq__extern"]
    pub fn arf_set_fmpq(
        y: *mut arf_struct,
        x: *const fmpq,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_complex_mul(
        e: *mut arf_struct,
        f: *mut arf_struct,
        a: *const arf_struct,
        b: *const arf_struct,
        c: *const arf_struct,
        d: *const arf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_complex_mul_fallback(
        e: *mut arf_struct,
        f: *mut arf_struct,
        a: *const arf_struct,
        b: *const arf_struct,
        c: *const arf_struct,
        d: *const arf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_complex_sqr(
        e: *mut arf_struct,
        f: *mut arf_struct,
        a: *const arf_struct,
        b: *const arf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_sum(
        s: *mut arf_struct,
        terms: arf_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn arf_get_d(x: *const arf_struct, rnd: arf_rnd_t) -> f64;
    pub fn arf_set_d(x: *mut arf_struct, v: f64);
    #[link_name = "arf_allocated_bytes__extern"]
    pub fn arf_allocated_bytes(x: *const arf_struct) -> mp_limb_signed_t;
    pub fn arf_load_str(res: *mut arf_struct, data: *const libc::c_char) -> libc::c_int;
    pub fn arf_dump_str(x: *const arf_struct) -> *mut libc::c_char;
    pub fn arf_load_file(res: *mut arf_struct, stream: *mut FILE) -> libc::c_int;
    pub fn arf_dump_file(stream: *mut FILE, x: *const arf_struct) -> libc::c_int;
    pub fn arf_approx_dot(
        res: *mut arf_struct,
        initial: *const arf_struct,
        subtract: libc::c_int,
        x: arf_srcptr,
        xstep: mp_limb_signed_t,
        y: arf_srcptr,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    );
}
