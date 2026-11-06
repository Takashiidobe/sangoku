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
pub fn pick(flag: bool, a: u8, b: u8) -> u8 {
    if flag { a } else { b }
}

// CHECK: fn pick(_1: bool, _2: u8, _3: u8) -> u8 {
// CHECK-NEXT: debug flag => _1;
// CHECK-NEXT: debug a => _2;
// CHECK-NEXT: debug b => _3;
// CHECK-NEXT: let mut _0: u8;
// CHECK-EMPTY:
// CHECK-NEXT: bb0: {
// CHECK-NEXT: switchInt(copy _1) -> [0: bb2, otherwise: bb1];
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: bb1: {
// CHECK-NEXT: _0 = copy _2;
// CHECK-NEXT: goto -> bb3;
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: bb2: {
// CHECK-NEXT: _0 = copy _3;
// CHECK-NEXT: goto -> bb3;
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: bb3: {
// CHECK-NEXT: return;
// CHECK-NEXT: }
// CHECK-NEXT: }
