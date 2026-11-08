#![no_std]

#[unsafe(no_mangle)]
pub fn pair(x: (u8, u16)) -> (u8, u16) {
    (x.0.wrapping_add(1), x.1)
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    pair((3, 20)).0
}

// CHECK: fn pair(_1: (u8, u16)) -> (u8, u16) {
// CHECK-NEXT:     debug x => _1;
// CHECK-NEXT:     let mut _0: (u8, u16);
// CHECK-NEXT:     let mut _2: u8;
// CHECK-NEXT:     let mut _3: u8;
// CHECK-NEXT:     let mut _4: u16;
// CHECK-NEXT:     scope 1 (inlined core::num::<impl u8>::wrapping_add) {
// CHECK-NEXT:     }
// CHECK-EMPTY:
// CHECK-NEXT:     bb0: {
// CHECK-NEXT:         _3 = copy (_1.0: u8);
// CHECK-NEXT:         _2 = Add(copy _3, const 1_u8);
// CHECK-NEXT:         _4 = copy (_1.1: u16);
// CHECK-NEXT:         _0 = (move _2, move _4);
// CHECK-NEXT:         return;
// CHECK-NEXT:     }
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: fn gbc_main() -> u8 {
// CHECK-NEXT:     let mut _0: u8;
// CHECK-NEXT:     let mut _1: (u8, u16);
// CHECK-EMPTY:
// CHECK-NEXT:     bb0: {
// CHECK-NEXT:         _1 = pair(const (3_u8, 20_u16)) -> [return: bb1, unwind unreachable];
// CHECK-NEXT:     }
// CHECK-EMPTY:
// CHECK-NEXT:     bb1: {
// CHECK-NEXT:         _0 = copy (_1.0: u8);
// CHECK-NEXT:         return;
// CHECK-NEXT:     }
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: alloc1 (size: 4, align: 2) {
// CHECK-NEXT:     03 __ 14 00                                     │ .░..
// CHECK-NEXT: }
