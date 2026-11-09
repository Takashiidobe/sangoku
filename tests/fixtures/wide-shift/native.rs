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
impl Copy for u32 {}
impl Copy for u64 {}
impl Copy for i64 {}
impl Copy for u128 {}
impl Copy for i128 {}
#[rustc_intrinsic]
pub const unsafe fn unchecked_shl<T: Copy, U: Copy>(value: T, count: U) -> T;
#[rustc_intrinsic]
pub const unsafe fn unchecked_shr<T: Copy, U: Copy>(value: T, count: U) -> T;
#[rustc_intrinsic]
pub const fn rotate_left<T: Copy>(value: T, count: u32) -> T;
#[rustc_intrinsic]
pub const fn rotate_right<T: Copy>(value: T, count: u32) -> T;

#[unsafe(no_mangle)]
pub unsafe fn shl64(p: *mut u64, n: u32) {
    let r = unsafe { unchecked_shl(*p, n) };
    *p = r;
}
#[unsafe(no_mangle)]
pub unsafe fn ushr64(p: *mut u64, n: u32) {
    let r = unsafe { unchecked_shr(*p, n) };
    *p = r;
}
#[unsafe(no_mangle)]
pub unsafe fn sshr64(p: *mut i64, n: u32) {
    let r = unsafe { unchecked_shr(*p, n) };
    *p = r;
}
#[unsafe(no_mangle)]
pub unsafe fn rotl64(p: *mut u64, n: u32) {
    let r = rotate_left(*p, n);
    *p = r;
}
#[unsafe(no_mangle)]
pub unsafe fn rotr64(p: *mut u64, n: u32) {
    let r = rotate_right(*p, n);
    *p = r;
}
#[unsafe(no_mangle)]
pub unsafe fn shl128(p: *mut u128, n: u32) {
    let r = unsafe { unchecked_shl(*p, n) };
    *p = r;
}
#[unsafe(no_mangle)]
pub unsafe fn ushr128(p: *mut u128, n: u32) {
    let r = unsafe { unchecked_shr(*p, n) };
    *p = r;
}
#[unsafe(no_mangle)]
pub unsafe fn sshr128(p: *mut i128, n: u32) {
    let r = unsafe { unchecked_shr(*p, n) };
    *p = r;
}
#[unsafe(no_mangle)]
pub unsafe fn rotl128(p: *mut u128, n: u32) {
    let r = rotate_left(*p, n);
    *p = r;
}
#[unsafe(no_mangle)]
pub unsafe fn rotr128(p: *mut u128, n: u32) {
    let r = rotate_right(*p, n);
    *p = r;
}
