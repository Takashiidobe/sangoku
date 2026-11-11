#![feature(no_core, lang_items, auto_traits, intrinsics)]
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
#[lang = "freeze"]
pub unsafe auto trait Freeze {}
#[lang = "sync"]
pub unsafe auto trait Sync {}

#[lang = "drop_glue"]
pub unsafe fn drop_glue<T: PointeeSized>(_: &mut T) {}

impl Copy for u8 {}
impl Copy for u16 {}

#[rustc_intrinsic]
pub const fn wrapping_mul<T: Copy>(a: T, b: T) -> T;

#[inline(never)]
#[unsafe(no_mangle)]
pub fn dependency_answer() -> u8 {
    42
}

#[inline(never)]
pub fn identity<T>(value: T) -> T {
    value
}

#[unsafe(no_mangle)]
pub static BYTES: [u8; 3] = [7, 8, 9];

#[repr(C)]
pub struct Pointers {
    pub bytes: *const u8,
    pub function: fn() -> u8,
}

unsafe impl Sync for Pointers {}

pub mod nested {
    #[unsafe(no_mangle)]
    pub fn second_answer() -> u8 {
        7
    }
}
