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

impl Copy for u8 {}
impl Copy for u16 {}
impl Copy for bool {}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    0
}

// CHECK: fn gbc_main() -> u8 {
// CHECK-NEXT: let mut _0: u8;
// CHECK-EMPTY:
// CHECK-NEXT: bb0: {
// CHECK-NEXT: _0 = const 0_u8;
// CHECK-NEXT: return;
// CHECK-NEXT: }
// CHECK-NEXT: }
