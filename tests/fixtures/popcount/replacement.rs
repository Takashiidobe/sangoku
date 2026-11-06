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
impl Copy for u16 {}
#[rustc_intrinsic]
pub const fn ctpop<T: Copy>(value: T) -> u32;
#[rustc_intrinsic]
pub const fn wrapping_add<T: Copy>(left: T, right: T) -> T;
#[unsafe(no_mangle)]
pub fn audit_popcount(value: u16) -> u16 {
    ctpop(value) as u16
}
#[unsafe(no_mangle)]
pub fn audit_popcount_wide(value: u16) -> u32 {
    ctpop(value)
}
#[unsafe(no_mangle)]
pub fn audit_popcount_live(value: u16) -> u16 {
    wrapping_add(value, ctpop(value) as u16)
}
#[unsafe(no_mangle)]
pub fn audit_popcount_pressure(value: u16, other: u16) -> u16 {
    wrapping_add(
        wrapping_add(ctpop(value) as u16, ctpop(other) as u16),
        wrapping_add(value, other),
    )
}
