#![feature(f128, no_core, lang_items, intrinsics, rustc_attrs, auto_traits)]
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

impl Copy for f128 {}

#[rustc_intrinsic]
pub const fn fma<T: Copy>(a: T, b: T, c: T) -> T;

#[unsafe(no_mangle)]
pub unsafe fn fma_to_memory(a: *const f128, b: *const f128, c: *const f128, out: *mut f128) {
    unsafe { *out = fma(*a, *b, *c) };
}
