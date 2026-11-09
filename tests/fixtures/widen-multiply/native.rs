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
#[rustc_intrinsic]
pub const fn mul_with_overflow<T: Copy>(a: T, b: T) -> (T, bool);

#[unsafe(no_mangle)]
pub fn audit_widen_u(a: u16, b: u16) -> u32 {
    wrapping_mul(a as u32, b as u32)
}
#[unsafe(no_mangle)]
pub fn audit_widen_s(a: i16, b: i16) -> i32 {
    wrapping_mul(a as i32, b as i32)
}
#[unsafe(no_mangle)]
pub fn audit_widen_mixed(a: i16, b: u16) -> i32 {
    wrapping_mul(a as i32, b as i32)
}
#[unsafe(no_mangle)]
pub fn audit_widen_live(a: u16, b: u16) -> u32 {
    wrapping_add(wrapping_mul(a as u32, b as u32), a as u32)
}
#[unsafe(no_mangle)]
pub fn audit_checked_u(a: u16, b: u16) -> u8 {
    mul_with_overflow(a, b).1 as u8
}
#[unsafe(no_mangle)]
pub fn audit_checked_s(a: i16, b: i16) -> u8 {
    mul_with_overflow(a, b).1 as u8
}
