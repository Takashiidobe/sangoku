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
pub const unsafe fn unchecked_div<T: Copy>(x: T, y: T) -> T;
#[rustc_intrinsic]
pub const unsafe fn unchecked_rem<T: Copy>(x: T, y: T) -> T;
#[rustc_intrinsic]
pub const fn wrapping_mul<T: Copy>(a: T, b: T) -> T;

#[repr(C)]
pub struct Pair<T> {
    a: T,
    b: T,
}
#[unsafe(no_mangle)]
pub unsafe fn div_u64(p: *mut Pair<u64>) {
    unsafe { (*p).a = unchecked_div((*p).a, (*p).b) };
}
#[unsafe(no_mangle)]
pub unsafe fn rem_u64(p: *mut Pair<u64>) {
    unsafe { (*p).a = unchecked_rem((*p).a, (*p).b) };
}
#[unsafe(no_mangle)]
pub unsafe fn div_i64(p: *mut Pair<i64>) {
    unsafe { (*p).a = unchecked_div((*p).a, (*p).b) };
}
#[unsafe(no_mangle)]
pub unsafe fn rem_i64(p: *mut Pair<i64>) {
    unsafe { (*p).a = unchecked_rem((*p).a, (*p).b) };
}
#[unsafe(no_mangle)]
pub unsafe fn div_u128(p: *mut Pair<u128>) {
    unsafe { (*p).a = unchecked_div((*p).a, (*p).b) };
}
#[unsafe(no_mangle)]
pub unsafe fn rem_u128(p: *mut Pair<u128>) {
    unsafe { (*p).a = unchecked_rem((*p).a, (*p).b) };
}
#[unsafe(no_mangle)]
pub unsafe fn div_i128(p: *mut Pair<i128>) {
    unsafe { (*p).a = unchecked_div((*p).a, (*p).b) };
}
#[unsafe(no_mangle)]
pub unsafe fn rem_i128(p: *mut Pair<i128>) {
    unsafe { (*p).a = unchecked_rem((*p).a, (*p).b) };
}
#[unsafe(no_mangle)]
pub unsafe fn mul_u128(p: *mut Pair<u128>) {
    unsafe { (*p).a = wrapping_mul((*p).a, (*p).b) };
}
