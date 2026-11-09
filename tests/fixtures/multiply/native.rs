#![feature(no_core, lang_items, intrinsics, rustc_attrs, auto_traits)]
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
impl Copy for u8 {}
impl Copy for u16 {}
impl Copy for u32 {}
impl Copy for i8 {}
impl Copy for i16 {}
impl Copy for i32 {}
#[rustc_intrinsic]
pub const fn wrapping_mul<T: Copy>(a: T, b: T) -> T;
#[rustc_intrinsic]
pub const fn wrapping_add<T: Copy>(a: T, b: T) -> T;
#[unsafe(no_mangle)]
pub fn audit_mul8(a: u8, b: u8) -> u8 {
    wrapping_mul(a, b)
}
#[unsafe(no_mangle)]
pub fn audit_mul8_signed(a: i8, b: i8) -> i8 {
    wrapping_mul(a, b)
}
#[unsafe(no_mangle)]
pub fn audit_mul8_live(a: u8, b: u8) -> u8 {
    wrapping_add(wrapping_mul(a, b), a)
}
#[unsafe(no_mangle)]
pub fn audit_mul16(a: u16, b: u16) -> u16 {
    wrapping_mul(a, b)
}
#[unsafe(no_mangle)]
pub fn audit_mul16_signed(a: i16, b: i16) -> i16 {
    wrapping_mul(a, b)
}
#[unsafe(no_mangle)]
pub fn audit_mul16_live(a: u16, b: u16) -> u16 {
    wrapping_add(wrapping_mul(a, b), a)
}
#[unsafe(no_mangle)]
pub fn audit_mul32(a: u32, b: u32) -> u32 {
    wrapping_mul(a, b)
}
#[unsafe(no_mangle)]
pub fn audit_mul32_signed(a: i32, b: i32) -> i32 {
    wrapping_mul(a, b)
}
#[unsafe(no_mangle)]
pub fn audit_mul32_live(a: u32, b: u32) -> u32 {
    wrapping_add(wrapping_mul(a, b), a)
}
