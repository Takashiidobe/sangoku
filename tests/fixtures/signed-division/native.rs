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
pub const unsafe fn unchecked_div<T: Copy>(a: T, b: T) -> T;
#[rustc_intrinsic]
pub const unsafe fn unchecked_rem<T: Copy>(a: T, b: T) -> T;
#[unsafe(no_mangle)]
pub unsafe fn audit_sdiv8(a: i8, b: i8) -> i8 {
    unsafe { unchecked_div(a, b) }
}
#[unsafe(no_mangle)]
pub unsafe fn audit_srem8(a: i8, b: i8) -> i8 {
    unsafe { unchecked_rem(a, b) }
}
#[unsafe(no_mangle)]
pub unsafe fn audit_sdiv16(a: i16, b: i16) -> i16 {
    unsafe { unchecked_div(a, b) }
}
#[unsafe(no_mangle)]
pub unsafe fn audit_srem16(a: i16, b: i16) -> i16 {
    unsafe { unchecked_rem(a, b) }
}
#[unsafe(no_mangle)]
pub unsafe fn audit_sdiv32(a: i32, b: i32) -> i32 {
    unsafe { unchecked_div(a, b) }
}
#[unsafe(no_mangle)]
pub unsafe fn audit_srem32(a: i32, b: i32) -> i32 {
    unsafe { unchecked_rem(a, b) }
}
