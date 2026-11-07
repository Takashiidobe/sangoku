#![feature(no_core, lang_items)]
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

#[repr(align(4))]
pub struct Aligned {
    a: u16,
    b: u16,
}

#[inline(never)]
#[unsafe(no_mangle)]
pub fn take(x: Aligned) -> u16 {
    x.a
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    take(Aligned { a: 1, b: 2 }) as u8
}
