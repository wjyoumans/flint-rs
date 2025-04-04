/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::bindgen::*;
use crate::flint::*;


pub const ARF_NOPTR_LIMBS: u32 = 2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mantissa_noptr_struct {
    pub d: [mp_limb_t; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mantissa_noptr_struct"][::std::mem::size_of::<mantissa_noptr_struct>() - 16usize];
    ["Alignment of mantissa_noptr_struct"]
        [::std::mem::align_of::<mantissa_noptr_struct>() - 8usize];
    ["Offset of field: mantissa_noptr_struct::d"]
        [::std::mem::offset_of!(mantissa_noptr_struct, d) - 0usize];
};
impl Default for mantissa_noptr_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mantissa_ptr_struct {
    pub alloc: mp_size_t,
    pub d: mp_ptr,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mantissa_ptr_struct"][::std::mem::size_of::<mantissa_ptr_struct>() - 16usize];
    ["Alignment of mantissa_ptr_struct"][::std::mem::align_of::<mantissa_ptr_struct>() - 8usize];
    ["Offset of field: mantissa_ptr_struct::alloc"]
        [::std::mem::offset_of!(mantissa_ptr_struct, alloc) - 0usize];
    ["Offset of field: mantissa_ptr_struct::d"]
        [::std::mem::offset_of!(mantissa_ptr_struct, d) - 8usize];
};
impl Default for mantissa_ptr_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mantissa_struct {
    pub noptr: __BindgenUnionField<mantissa_noptr_struct>,
    pub ptr: __BindgenUnionField<mantissa_ptr_struct>,
    pub bindgen_union_field: [u64; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mantissa_struct"][::std::mem::size_of::<mantissa_struct>() - 16usize];
    ["Alignment of mantissa_struct"][::std::mem::align_of::<mantissa_struct>() - 8usize];
    ["Offset of field: mantissa_struct::noptr"]
        [::std::mem::offset_of!(mantissa_struct, noptr) - 0usize];
    ["Offset of field: mantissa_struct::ptr"]
        [::std::mem::offset_of!(mantissa_struct, ptr) - 0usize];
};
impl Default for mantissa_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arf_struct {
    pub exp: fmpz,
    pub size: mp_size_t,
    pub d: mantissa_struct,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of arf_struct"][::std::mem::size_of::<arf_struct>() - 32usize];
    ["Alignment of arf_struct"][::std::mem::align_of::<arf_struct>() - 8usize];
    ["Offset of field: arf_struct::exp"][::std::mem::offset_of!(arf_struct, exp) - 0usize];
    ["Offset of field: arf_struct::size"][::std::mem::offset_of!(arf_struct, size) - 8usize];
    ["Offset of field: arf_struct::d"][::std::mem::offset_of!(arf_struct, d) - 16usize];
};
impl Default for arf_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type arf_t = [arf_struct; 1usize];
pub type arf_ptr = *mut arf_struct;
pub type arf_srcptr = *const arf_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arf_interval_struct {
    pub a: arf_struct,
    pub b: arf_struct,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of arf_interval_struct"][::std::mem::size_of::<arf_interval_struct>() - 64usize];
    ["Alignment of arf_interval_struct"][::std::mem::align_of::<arf_interval_struct>() - 8usize];
    ["Offset of field: arf_interval_struct::a"]
        [::std::mem::offset_of!(arf_interval_struct, a) - 0usize];
    ["Offset of field: arf_interval_struct::b"]
        [::std::mem::offset_of!(arf_interval_struct, b) - 32usize];
};
impl Default for arf_interval_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type arf_interval_t = [arf_interval_struct; 1usize];
pub type arf_interval_ptr = *mut arf_interval_struct;
pub type arf_interval_srcptr = *const arf_interval_struct;
