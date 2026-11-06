#![feature(no_core, lang_items, intrinsics, rustc_attrs)]
#![no_core]
#[lang = "pointee_sized"]
pub trait PointeeSized {}
#[lang = "meta_sized"]
pub trait MetaSized: PointeeSized {}
#[lang = "sized"]
pub trait Sized: MetaSized {}
#[lang = "copy"]
pub trait Copy: Sized {}
impl Copy for u16 {}
#[rustc_intrinsic]
pub const fn ctpop<T: Copy>(value: T) -> u32;
#[unsafe(no_mangle)]
pub fn audit_popcount(value: u16) -> u16 {
    ctpop(value) as u16
}
