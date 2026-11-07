#![feature(f16, f128, no_core, lang_items, auto_traits)]
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

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 { 0 }

impl Copy for f128 {}
impl Copy for f16 {}
impl Copy for f32 {}
impl Copy for f64 {}
impl Copy for i128 {}
impl Copy for i64 {}
impl Copy for i8 {}
impl Copy for u128 {}
impl Copy for u64 {}

#[unsafe(no_mangle)]
pub unsafe fn i64_to_f16(memory: *mut i64) {
    unsafe { *(memory as *mut f16) = *memory as f16 };
}

#[unsafe(no_mangle)]
pub unsafe fn u64_to_f32(memory: *mut u64) {
    unsafe { *(memory as *mut f32) = *memory as f32 };
}

#[unsafe(no_mangle)]
pub unsafe fn i128_to_f64(memory: *mut i128) {
    unsafe { *(memory as *mut f64) = *memory as f64 };
}

#[unsafe(no_mangle)]
pub unsafe fn u128_to_f128(memory: *mut u128) {
    unsafe { *(memory as *mut f128) = *memory as f128 };
}

#[unsafe(no_mangle)]
pub unsafe fn f64_to_i8(memory: *mut f64) {
    unsafe { *(memory as *mut i8) = *memory as i8 };
}

#[unsafe(no_mangle)]
pub unsafe fn f128_to_u128(memory: *mut f128) {
    unsafe { *(memory as *mut u128) = *memory as u128 };
}

#[unsafe(no_mangle)]
pub unsafe fn f64_to_f16(memory: *mut f64) {
    unsafe { *(memory as *mut f16) = *memory as f16 };
}

#[unsafe(no_mangle)]
pub unsafe fn f16_to_f128(memory: *mut f16) {
    unsafe { *(memory as *mut f128) = *memory as f128 };
}

#[unsafe(no_mangle)]
pub unsafe fn f128_to_f32(memory: *mut f128) {
    unsafe { *(memory as *mut f32) = *memory as f32 };
}

