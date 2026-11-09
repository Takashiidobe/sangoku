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
pub const fn ctpop<T: Copy>(value: T) -> u32;
#[rustc_intrinsic]
pub const fn wrapping_add<T: Copy>(left: T, right: T) -> T;
#[rustc_intrinsic]
pub const fn ctlz<T: Copy>(value: T) -> u32;
#[unsafe(no_mangle)]
pub fn audit_popcount8(value: u8) -> u32 {
    ctpop(value)
}
#[unsafe(no_mangle)]
pub fn audit_popcount8_signed(value: i8) -> u32 {
    ctpop(value)
}
#[unsafe(no_mangle)]
pub fn audit_popcount8_live(value: u8) -> u8 {
    wrapping_add(value, ctpop(value) as u8)
}
#[unsafe(no_mangle)]
pub fn audit_popcount8_clz(value: u8) -> u32 {
    ctlz(value)
}
#[unsafe(no_mangle)]
pub fn audit_popcount16(value: u16) -> u32 {
    ctpop(value)
}
#[unsafe(no_mangle)]
pub fn audit_popcount16_signed(value: i16) -> u32 {
    ctpop(value)
}
#[unsafe(no_mangle)]
pub fn audit_popcount16_live(value: u16) -> u16 {
    wrapping_add(value, ctpop(value) as u16)
}
#[unsafe(no_mangle)]
pub fn audit_popcount16_clz(value: u16) -> u32 {
    ctlz(value)
}
#[unsafe(no_mangle)]
pub fn audit_popcount32(value: u32) -> u32 {
    ctpop(value)
}
#[unsafe(no_mangle)]
pub fn audit_popcount32_signed(value: i32) -> u32 {
    ctpop(value)
}
#[unsafe(no_mangle)]
pub fn audit_popcount32_live(value: u32) -> u32 {
    wrapping_add(value, ctpop(value) as u32)
}
#[unsafe(no_mangle)]
pub fn audit_popcount32_clz(value: u32) -> u32 {
    ctlz(value)
}
