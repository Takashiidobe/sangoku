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

impl Copy for u128 {}

#[rustc_intrinsic]
pub const fn wrapping_add<T: Copy>(a: T, b: T) -> T;

#[unsafe(no_mangle)]
pub fn sum_u128(a: u128, b: u128) -> u128 {
    wrapping_add(a, b)
}

#[unsafe(no_mangle)]
pub unsafe fn sum_to_memory(a: *const u128, b: *const u128, out: *mut u128) {
    unsafe { *out = wrapping_add(*a, *b) };
}
