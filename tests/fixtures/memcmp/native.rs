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
#[rustc_intrinsic]
pub const unsafe fn compare_bytes(a: *const u8, b: *const u8, len: usize) -> i32;
#[rustc_intrinsic]
pub const unsafe fn raw_eq<T>(a: &T, b: &T) -> bool;

unsafe extern "C" {
    #[link_name = "memcmp"]
    fn named_memcmp(a: *const u8, b: *const u8, len: usize) -> i32;
}

#[unsafe(no_mangle)]
pub unsafe fn audit_order(a: *const u8, b: *const u8, len: usize) -> i32 {
    unsafe { compare_bytes(a, b, len) }
}

#[unsafe(no_mangle)]
pub unsafe fn audit_named(a: *const u8, b: *const u8, len: usize) -> i32 {
    unsafe { named_memcmp(a, b, len) }
}

#[unsafe(no_mangle)]
pub unsafe fn audit_equal(a: &[u8; 5], b: &[u8; 5]) -> bool {
    unsafe { raw_eq(a, b) }
}

#[unsafe(no_mangle)]
pub unsafe fn audit_equal_small(a: &[u8; 2], b: &[u8; 2]) -> bool {
    unsafe { raw_eq(a, b) }
}
