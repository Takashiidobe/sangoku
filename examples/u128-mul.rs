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
pub const fn wrapping_mul<T: Copy>(a: T, b: T) -> T;

#[unsafe(no_mangle)]
pub fn product_u128(a: u128, b: u128) -> u128 {
    wrapping_mul(a, b)
}
