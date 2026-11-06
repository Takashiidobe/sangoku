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
pub fn callee(x: u8) -> u8 {
    x
}

#[unsafe(no_mangle)]
pub fn caller(x: u8) -> u8 {
    callee(x)
}

// CHECK: fn callee(_1: u8) -> u8 {
// CHECK-NEXT: debug x => _1;
// CHECK-NEXT: let mut _0: u8;
// CHECK-EMPTY:
// CHECK-NEXT: bb0: {
// CHECK-NEXT: _0 = copy _1;
// CHECK-NEXT: return;
// CHECK-NEXT: }
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: fn caller(_1: u8) -> u8 {
// CHECK-NEXT: debug x => _1;
// CHECK-NEXT: let mut _0: u8;
// CHECK-EMPTY:
// CHECK-NEXT: bb0: {
// CHECK-NEXT: _0 = callee(copy _1) -> [return: bb1, unwind unreachable];
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: bb1: {
// CHECK-NEXT: return;
// CHECK-NEXT: }
// CHECK-NEXT: }
