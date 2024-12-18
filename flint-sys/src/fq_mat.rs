/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::fq_types::*;


pub const FQ_MAT_SOLVE_TRI_ROWS_CUTOFF: u32 = 64;
pub const FQ_MAT_SOLVE_TRI_COLS_CUTOFF: u32 = 64;
pub const FQ_MAT_LU_RECURSIVE_CUTOFF: u32 = 4;
extern "C" {
    pub fn FQ_MAT_MUL_KS_CUTOFF(
        r: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    ) -> libc::c_int;
}
