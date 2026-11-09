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
pub const fn ctlz<T: Copy>(value: T) -> u32;
#[rustc_intrinsic]
pub const fn cttz<T: Copy>(value: T) -> u32;
#[rustc_intrinsic]
pub const fn bitreverse<T: Copy>(value: T) -> T;
#[rustc_intrinsic]
pub const fn wrapping_add<T: Copy>(a: T, b: T) -> T;
#[unsafe(no_mangle)]
pub fn audit_clz8(value: u8) -> u32 {
    ctlz(value)
}
#[unsafe(no_mangle)]
pub fn audit_clz8_signed(value: i8) -> u32 {
    ctlz(value)
}
#[unsafe(no_mangle)]
pub fn audit_clz8_live(value: u8) -> u8 {
    wrapping_add(value, ctlz(value) as u8)
}
#[unsafe(no_mangle)]
pub fn audit_clz16(value: u16) -> u32 {
    ctlz(value)
}
#[unsafe(no_mangle)]
pub fn audit_clz16_signed(value: i16) -> u32 {
    ctlz(value)
}
#[unsafe(no_mangle)]
pub fn audit_clz16_live(value: u16) -> u16 {
    wrapping_add(value, ctlz(value) as u16)
}
#[unsafe(no_mangle)]
pub fn audit_clz32(value: u32) -> u32 {
    ctlz(value)
}
#[unsafe(no_mangle)]
pub fn audit_clz32_signed(value: i32) -> u32 {
    ctlz(value)
}
#[unsafe(no_mangle)]
pub fn audit_clz32_live(value: u32) -> u32 {
    wrapping_add(value, ctlz(value) as u32)
}
#[unsafe(no_mangle)]
pub fn audit_ctz8(value: u8) -> u32 {
    cttz(value)
}
#[unsafe(no_mangle)]
pub fn audit_ctz8_signed(value: i8) -> u32 {
    cttz(value)
}
#[unsafe(no_mangle)]
pub fn audit_ctz8_live(value: u8) -> u8 {
    wrapping_add(value, cttz(value) as u8)
}
#[unsafe(no_mangle)]
pub fn audit_ctz16(value: u16) -> u32 {
    cttz(value)
}
#[unsafe(no_mangle)]
pub fn audit_ctz16_signed(value: i16) -> u32 {
    cttz(value)
}
#[unsafe(no_mangle)]
pub fn audit_ctz16_live(value: u16) -> u16 {
    wrapping_add(value, cttz(value) as u16)
}
#[unsafe(no_mangle)]
pub fn audit_ctz32(value: u32) -> u32 {
    cttz(value)
}
#[unsafe(no_mangle)]
pub fn audit_ctz32_signed(value: i32) -> u32 {
    cttz(value)
}
#[unsafe(no_mangle)]
pub fn audit_ctz32_live(value: u32) -> u32 {
    wrapping_add(value, cttz(value) as u32)
}
#[unsafe(no_mangle)]
pub fn audit_bitrev8(value: u8) -> u8 {
    bitreverse(value)
}
#[unsafe(no_mangle)]
pub fn audit_bitrev8_signed(value: i8) -> i8 {
    bitreverse(value)
}
#[unsafe(no_mangle)]
pub fn audit_bitrev8_live(value: u8) -> u8 {
    wrapping_add(value, bitreverse(value) as u8)
}
#[unsafe(no_mangle)]
pub fn audit_bitrev16(value: u16) -> u16 {
    bitreverse(value)
}
#[unsafe(no_mangle)]
pub fn audit_bitrev16_signed(value: i16) -> i16 {
    bitreverse(value)
}
#[unsafe(no_mangle)]
pub fn audit_bitrev16_live(value: u16) -> u16 {
    wrapping_add(value, bitreverse(value) as u16)
}
#[unsafe(no_mangle)]
pub fn audit_bitrev32(value: u32) -> u32 {
    bitreverse(value)
}
#[unsafe(no_mangle)]
pub fn audit_bitrev32_signed(value: i32) -> i32 {
    bitreverse(value)
}
#[unsafe(no_mangle)]
pub fn audit_bitrev32_live(value: u32) -> u32 {
    wrapping_add(value, bitreverse(value) as u32)
}
