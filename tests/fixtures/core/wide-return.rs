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
impl Copy for u128 {}
impl Copy for u8 {}
impl Copy for bool {}
#[lang = "freeze"]
pub unsafe auto trait Freeze {}

#[rustc_intrinsic]
pub const fn mul_with_overflow<T: Copy>(x: T, y: T) -> (T, bool);

#[inline(never)]
#[unsafe(no_mangle)]
pub fn mulo(a: u128, b: u128) -> bool {
    mul_with_overflow(a, b).1
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    mulo(3, 5) as u8
}
