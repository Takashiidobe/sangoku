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

#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for u16 {
    type Output = u16;
    fn add(self, rhs: u16) -> u16 {
        self + rhs
    }
}

#[unsafe(no_mangle)]
pub fn add_u16(a: u16, b: u16) -> u16 {
    a + b
}

// CHECK: fn <impl at ./tests/fixtures/mir/add.rs:27:1: 27:17>::add(_1: u16, _2: u16) -> u16 {
// CHECK-NEXT: debug self => _1;
// CHECK-NEXT: debug rhs => _2;
// CHECK-NEXT: let mut _0: u16;
// CHECK-NEXT: let mut _3: (u16, bool);
// CHECK-EMPTY:
// CHECK-NEXT: bb0: {
// CHECK-NEXT: _3 = AddWithOverflow(copy _1, copy _2);
// CHECK-NEXT: assert(!move (_3.1: bool), "attempt to compute `{} + {}`, which would overflow", copy _1, copy _2) -> [success: bb1, unwind unreachable];
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: bb1: {
// CHECK-NEXT: _0 = move (_3.0: u16);
// CHECK-NEXT: return;
// CHECK-NEXT: }
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: fn add_u16(_1: u16, _2: u16) -> u16 {
// CHECK-NEXT: debug a => _1;
// CHECK-NEXT: debug b => _2;
// CHECK-NEXT: let mut _0: u16;
// CHECK-NEXT: let mut _3: (u16, bool);
// CHECK-EMPTY:
// CHECK-NEXT: bb0: {
// CHECK-NEXT: _3 = AddWithOverflow(copy _1, copy _2);
// CHECK-NEXT: assert(!move (_3.1: bool), "attempt to compute `{} + {}`, which would overflow", copy _1, copy _2) -> [success: bb1, unwind unreachable];
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: bb1: {
// CHECK-NEXT: _0 = move (_3.0: u16);
// CHECK-NEXT: return;
// CHECK-NEXT: }
// CHECK-NEXT: }
