#![feature(no_core, lang_items, intrinsics, rustc_attrs, auto_traits)]
#![allow(internal_features)]
#![no_core]

#[lang = "pointee_sized"]
pub trait PointeeSized {}

#[lang = "meta_sized"]
pub trait MetaSized: PointeeSized {}

#[lang = "sized"]
pub trait Sized: MetaSized {}

#[lang = "copy"]
pub trait Copy: Sized {}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    0
}

#[lang = "freeze"]
pub unsafe auto trait Freeze {}

impl Copy for usize {}
impl Copy for isize {}

#[rustc_intrinsic]
pub const unsafe fn arith_offset<T>(pointer: *const T, offset: isize) -> *const T;

#[rustc_intrinsic]
pub const fn wrapping_add<T: Copy>(a: T, b: T) -> T;

#[unsafe(no_mangle)]
pub fn sum_usize(a: usize, b: usize) -> usize {
    wrapping_add(a, b)
}

#[unsafe(no_mangle)]
pub unsafe fn sum_to_memory(a: *const usize, b: *const usize, out: *mut usize) {
    unsafe { *out = wrapping_add(*a, *b) };
}

#[unsafe(no_mangle)]
pub unsafe fn offset_to_memory(base: *const u8, offset: isize, out: *mut usize) {
    unsafe { *out = arith_offset(base, offset) as usize };
}
