#![feature(no_core, lang_items, intrinsics, rustc_attrs, auto_traits)]
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

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    0
}

#[lang = "freeze"]
pub unsafe auto trait Freeze {}

impl Copy for f64 {}

#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for f64 {
    type Output = f64;
    fn add(self, rhs: f64) -> f64 {
        self + rhs
    }
}

#[unsafe(no_mangle)]
pub unsafe fn sum_to_memory(a: *const f64, b: *const f64, out: *mut f64) {
    unsafe { *out = *a + *b };
}
