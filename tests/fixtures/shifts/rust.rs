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
impl Copy for u32 {}
#[rustc_intrinsic]
pub const unsafe fn unchecked_shl<T: Copy, U: Copy>(value: T, count: U) -> T;
#[rustc_intrinsic]
pub const unsafe fn unchecked_shr<T: Copy, U: Copy>(value: T, count: U) -> T;
#[unsafe(no_mangle)]
pub fn rust_shl16_8(value: u16) -> u16 {
    unsafe { unchecked_shl(value, 8u32) }
}
#[unsafe(no_mangle)]
pub fn rust_shr32_8(value: u32) -> u32 {
    unsafe { unchecked_shr(value, 8u32) }
}
#[lang = "partial_ord"]
pub trait PartialOrd<Rhs = Self> {
    fn lt(&self, other: &Rhs) -> bool;
}
impl PartialOrd for u32 {
    fn lt(&self, other: &u32) -> bool {
        *self < *other
    }
}
#[unsafe(no_mangle)]
pub fn checked16_15(value: u16) -> u16 {
    if 15u32 < 16u32 {
        unsafe { unchecked_shl(value, 15u32) }
    } else {
        0
    }
}
#[unsafe(no_mangle)]
pub fn checked16_16(value: u16) -> u16 {
    if 16u32 < 16u32 {
        unsafe { unchecked_shl(value, 16u32) }
    } else {
        0
    }
}
#[lang = "legacy_receiver"]
pub trait LegacyReceiver: PointeeSized {}
impl<T: PointeeSized> LegacyReceiver for &T {}
#[unsafe(no_mangle)]
pub fn checked32_31(value: u32) -> u32 {
    if 31u32 < 32u32 {
        unsafe { unchecked_shr(value, 31u32) }
    } else {
        0
    }
}
#[unsafe(no_mangle)]
pub fn checked32_32(value: u32) -> u32 {
    if 32u32 < 32u32 {
        unsafe { unchecked_shr(value, 32u32) }
    } else {
        0
    }
}
#[unsafe(no_mangle)]
pub fn checked16_valid_15(_value: u16) -> bool {
    15u32 < 16u32
}
#[unsafe(no_mangle)]
pub fn checked16_valid_16(_value: u16) -> bool {
    16u32 < 16u32
}
#[unsafe(no_mangle)]
pub fn checked32_valid_31(_value: u32) -> bool {
    31u32 < 32u32
}
#[unsafe(no_mangle)]
pub fn checked32_valid_32(_value: u32) -> bool {
    32u32 < 32u32
}
#[unsafe(no_mangle)]
pub fn checked16_65536(value: u16) -> u16 {
    if 65536u32 < 16u32 {
        unsafe { unchecked_shl(value, 65536u32) }
    } else {
        0
    }
}
#[unsafe(no_mangle)]
pub fn checked16_valid_65536(_value: u16) -> bool {
    65536u32 < 16u32
}
