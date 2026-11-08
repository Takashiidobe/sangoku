#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn unsigned_multiply(lhs: u64, rhs: u64) -> (u64, bool) {
    lhs.overflowing_mul(rhs)
}

static CASES: [(u64, u64, u64, bool); 12] = [
    (0, u64::MAX, 0, false),
    (u64::MAX, 0, 0, false),
    (u64::MAX, 1, u64::MAX, false),
    (1, u64::MAX, u64::MAX, false),
    (u64::MAX, 2, u64::MAX - 1, true),
    (1 << 63, 2, 0, true),
    (1 << 62, 2, 1 << 63, false),
    ((1 << 32) - 1, (1 << 32) - 1, 0xffff_fffe_0000_0001, false),
    (1 << 32, 1 << 32, 0, true),
    (u64::MAX / 3, 3, u64::MAX, false),
    (u64::MAX / 3 + 1, 3, 2, true),
    (u64::MAX, u64::MAX, 1, true),
];

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    for &(lhs, rhs, value, overflow) in &CASES {
        if unsigned_multiply(lhs, rhs) != (value, overflow) {
            return 0;
        }
    }
    1
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTybENtNtB7_3cmp9PartialEq2neCs1wKLaDni5OV_23unsigned_overflow_mul64
// CHECK-NEXT: function u0:0(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTybENtNtB7_3cmp9PartialEq2neCs1wKLaDni5OV_23unsigned_overflow_mul64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (u64, bool)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (u64, bool)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} (u64, bool)            2b 2                storage=ss0
// CHECK-NEXT: ; stack _2    &'{erased} (u64, bool)            2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _4    &'{erased} u64                    2b 2                var=3
// CHECK-NEXT: ; ssa   _5    &'{erased} u64                    2b 2                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} bool                   2b 2                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} bool                   2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:2 sig0 ; Instance { def: Item(DefId(1:29982 ~ core[..]::cmp::impls::{impl#25}::ne)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(1:29964 ~ core[..]::cmp::impls::{impl#19}::ne)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (u64, bool) <- ByVal(v0): &'{erased} (u64, bool)
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u64, bool) <- ByVal(v1): &'{erased} (u64, bool)
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = &((*_1).0: U)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): &'{erased} u64 <- ByVal(v5): &'{erased} u64
// CHECK-NEXT: ; _5 = &((*_2).0: U)
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): &'{erased} u64 <- ByVal(v7): &'{erased} u64
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = <U as core::cmp::PartialEq>::ne(move _4, move _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &u64, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &u64, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v8 = call fn0(v5, v7)
// CHECK-NEXT:     v9 -> v8
// CHECK-NEXT: ; write_cvalue: Var(_3, var2): bool <- ByVal(v8): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _3)
// CHECK-NEXT:     brif.i8 v9, block3, block4
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const true
// CHECK-NEXT:     v10 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v10): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v10  ; v10 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = &((*_1).1: T)
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT:     v12 = load.i16 notrap v11
// CHECK-NEXT:     v13 = iconst.i16 8
// CHECK-NEXT:     v14 = iadd v12, v13  ; v13 = 8
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): &'{erased} bool <- ByVal(v14): &'{erased} bool
// CHECK-NEXT: ; _7 = &((*_2).1: T)
// CHECK-NEXT:     v15 = stack_addr.i16 ss1
// CHECK-NEXT:     v16 = load.i16 notrap v15
// CHECK-NEXT:     v17 = iconst.i16 8
// CHECK-NEXT:     v18 = iadd v16, v17  ; v17 = 8
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): &'{erased} bool <- ByVal(v18): &'{erased} bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::cmp::PartialEq>::ne(move _6, move _7)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &bool, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &bool, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v19 = call fn1(v14, v18)
// CHECK-NEXT:     v20 -> v19
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v19): bool
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v20
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2neCs1wKLaDni5OV_23unsigned_overflow_mul64
// CHECK-NEXT: function u0:1(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2neCs1wKLaDni5OV_23unsigned_overflow_mul64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} bool
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} bool
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} bool                   2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} bool                   2b 2                var=3
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} bool <- ByVal(v0): &'{erased} bool
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} bool <- ByVal(v1): &'{erased} bool
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): bool <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): bool
// CHECK-NEXT:     v3 = load.i8 notrap v2
// CHECK-NEXT: ; _4 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): bool <- ByRef(Pointer { base: Addr(v4), offset: Offset32(0) }, None): bool
// CHECK-NEXT:     v5 = load.i8 notrap v4
// CHECK-NEXT: ; _0 = Ne(move _3, move _4)
// CHECK-NEXT:     v6 = icmp ne v3, v5
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v6): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v6
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXsn_NtNtCs40R3J0iFVxM_4core3cmp5implsyNtB7_9PartialEq2neCs1wKLaDni5OV_23unsigned_overflow_mul64
// CHECK-NEXT: function u0:2(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsn_NtNtCs40R3J0iFVxM_4core3cmp5implsyNtB7_9PartialEq2neCs1wKLaDni5OV_23unsigned_overflow_mul64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} u64
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} u64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} u64                    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} u64                    2b 2                var=3
// CHECK-NEXT: ; ssa   _3    u64                               8b 2                var=4
// CHECK-NEXT: ; ssa   _4    u64                               8b 2                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} u64 <- ByVal(v0): &'{erased} u64
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} u64 <- ByVal(v1): &'{erased} u64
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): u64 <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): u64
// CHECK-NEXT:     v3 = load.i64 notrap v2
// CHECK-NEXT: ; _4 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): u64 <- ByRef(Pointer { base: Addr(v4), offset: Offset32(0) }, None): u64
// CHECK-NEXT:     v5 = load.i64 notrap v4
// CHECK-NEXT: ; _0 = Ne(move _3, move _4)
// CHECK-NEXT:     v6 = icmp ne v3, v5
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v6): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v6
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:3() -> i8 system_v {
// CHECK-NEXT: ; symbol gbc_main
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    core::slice::Iter<'{erased}, (u64, u64, u64, bool)>    4b 2                storage=ss0
// CHECK-NEXT: ; stack _2    core::option::Option<&'{erased} (u64, u64, u64, bool)>    2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    u64                               8b 2                var=2
// CHECK-NEXT: ; ssa   _4    u64                               8b 2                var=3
// CHECK-NEXT: ; ssa   _5    u64                               8b 2                var=4
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=5
// CHECK-NEXT: ; ssa   _7    bool                              1b 1                var=6
// CHECK-NEXT: ; ssa   _8    &'{erased} (u64, bool)            2b 2                var=7
// CHECK-NEXT: ; stack _9    (u64, bool)                      10b 2                storage=ss2
// CHECK-NEXT: ; ssa   _10   &'{erased} (u64, bool)            2b 2                var=8
// CHECK-NEXT: ; stack _11   (u64, bool)                      10b 2                storage=ss3
// CHECK-NEXT: ; ssa   _12   &'{erased} (u64, u64, u64, bool)    2b 2                var=9
// CHECK-NEXT: ; ssa   _13   &'{erased} (u64, u64, u64, bool)    2b 2                var=10
// CHECK-NEXT: ; ssa   _14   &'{erased} (u64, u64, u64, bool)    2b 2                var=11
// CHECK-NEXT: ; ssa   _15   &'{erased} (u64, u64, u64, bool)    2b 2                var=12
// CHECK-NEXT: ; stack _16   &'{erased} [(u64, u64, u64, bool)]    4b 2                storage=ss4
// CHECK-NEXT: ; stack _17   core::ptr::NonNull<(u64, u64, u64, bool)>    2b 2                storage=ss5
// CHECK-NEXT: ; stack _18   core::ptr::NonNull<[(u64, u64, u64, bool)]>    4b 2                storage=ss6
// CHECK-NEXT: ; ssa   _19   *const (u64, u64, u64, bool)      2b 2                var=13
// CHECK-NEXT: ; ssa   _20   *mut (u64, u64, u64, bool)        2b 2                var=14
// CHECK-NEXT: ; ssa   _21   *mut (u64, u64, u64, bool)        2b 2                var=15
// CHECK-NEXT: ; ssa   _22   *const [(u64, u64, u64, bool)]    4b 2                var=(16, 17)
// CHECK-NEXT: ; ssa   _23   *mut (u64, u64, u64, bool)        2b 2                var=18
// CHECK-NEXT: ; ssa   _24   *mut [(u64, u64, u64, bool)]      4b 2                var=(19, 20)
// CHECK-NEXT: ; stack _25   core::ptr::NonNull<(u64, u64, u64, bool)>    2b 2                storage=ss7
// CHECK-NEXT: ; ssa   _26   *const (u64, u64, u64, bool)      2b 2                var=21
// CHECK-NEXT: ; ssa   _27   bool                              1b 1                var=22
// CHECK-NEXT: ; stack _28   core::ptr::NonNull<(u64, u64, u64, bool)>    2b 2                storage=ss8
// CHECK-NEXT: ; stack _29   core::ptr::NonNull<(u64, u64, u64, bool)>    2b 2                storage=ss9
// CHECK-NEXT: ; ssa   _30   &'{erased} (u64, u64, u64, bool)    2b 2                var=23
// CHECK-NEXT: ; ssa   _31   *mut (u64, u64, u64, bool)        2b 2                var=24
// CHECK-NEXT: ; ssa   _32   *mut (u64, u64, u64, bool)        2b 2                var=25
// CHECK-NEXT: ; ssa   _33   *mut (u64, u64, u64, bool)        2b 2                var=26
// CHECK-NEXT: ; stack _34   *const (u64, u64, u64, bool)      2b 2                storage=ss10
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss3 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss4 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss5 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss6 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss7 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss8 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss9 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss10 = explicit_slot 2, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; DefId(0:4 ~ unsigned_overflow_mul64[..]::CASES)
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16 sret, i64, i64) system_v
// CHECK-NEXT:     fn0 = colocated u0:0 sig0 ; Instance { def: Item(DefId(1:46740 ~ core[..]::tuple::{impl#10}::ne)), args: [u64, bool] }
// CHECK-NEXT:     fn1 = colocated u0:4 sig1 ; Instance { def: Item(DefId(0:3 ~ unsigned_overflow_mul64[..]::unsigned_multiply)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = const {alloc1: &[(u64, u64, u64, bool); 12]} as &[(u64, u64, u64, bool)] (PointerCoercion(Unsize, Implicit))
// CHECK-NEXT:     v0 = symbol_value.i16 gv0
// CHECK-NEXT:     v1 = iconst.i16 12
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): &'{erased} [(u64, u64, u64, bool)] <- ByValPair(v0, v1): &'{erased} [(u64, u64, u64, bool)]
// CHECK-NEXT:     v2 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT:     v3 = stack_addr.i16 ss4+2
// CHECK-NEXT:     store notrap v1, v3  ; v1 = 12
// CHECK-NEXT: ; _22 = &raw const (*_16)
// CHECK-NEXT:     v4 = stack_addr.i16 ss4
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT:     v6 = stack_addr.i16 ss4+2
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: VarPair(_22, var16, var17): *const [(u64, u64, u64, bool)] <- ByValPair(v5, v7): *const [(u64, u64, u64, bool)]
// CHECK-NEXT: ; _18 = copy _22 as core::ptr::NonNull<[(u64, u64, u64, bool)]> (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): core::ptr::NonNull<[(u64, u64, u64, bool)]> <- ByValPair(v5, v7): *const [(u64, u64, u64, bool)]
// CHECK-NEXT:     v8 = stack_addr.i16 ss6
// CHECK-NEXT:     store notrap v5, v8
// CHECK-NEXT:     v9 = stack_addr.i16 ss6+2
// CHECK-NEXT:     store notrap v7, v9
// CHECK-NEXT: ; _24 = copy _18 as *mut [(u64, u64, u64, bool)] (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: VarPair(_24, var19, var20): *mut [(u64, u64, u64, bool)] <- ByRef(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): core::ptr::NonNull<[(u64, u64, u64, bool)]>
// CHECK-NEXT:     v10 = stack_addr.i16 ss6
// CHECK-NEXT:     v11 = load.i16 notrap v10
// CHECK-NEXT:     v12 = stack_addr.i16 ss6+2
// CHECK-NEXT:     v13 = load.i16 notrap v12
// CHECK-NEXT: ; _23 = copy _24 as *mut (u64, u64, u64, bool) (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_23, var18): *mut (u64, u64, u64, bool) <- ByVal(v11): *mut (u64, u64, u64, bool)
// CHECK-NEXT: ; _17 = copy _23 as core::ptr::NonNull<(u64, u64, u64, bool)> (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)> <- ByVal(v11): *mut (u64, u64, u64, bool)
// CHECK-NEXT:     v14 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v11, v14
// CHECK-NEXT: ; _21 = copy _17 as *mut (u64, u64, u64, bool) (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_21, var15): *mut (u64, u64, u64, bool) <- ByRef(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)>
// CHECK-NEXT:     v15 = stack_addr.i16 ss5
// CHECK-NEXT:     v16 = load.i16 notrap v15
// CHECK-NEXT: ; _20 = Offset(copy _21, const 12_usize)
// CHECK-NEXT:     v17 = iconst.i16 12
// CHECK-NEXT:     v18 = iconst.i16 26
// CHECK-NEXT:     v19 = imul v17, v18  ; v17 = 12, v18 = 26
// CHECK-NEXT:     v20 = iadd v16, v19
// CHECK-NEXT: ; write_cvalue: Var(_20, var14): *mut (u64, u64, u64, bool) <- ByVal(v20): *mut (u64, u64, u64, bool)
// CHECK-NEXT: ; _19 = copy _20 as *const (u64, u64, u64, bool) (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_19, var13): *const (u64, u64, u64, bool) <- ByVal(v20): *const (u64, u64, u64, bool)
// CHECK-NEXT: ; _1 = core::slice::Iter::<'_, (u64, u64, u64, bool)> { ptr: copy _17, end_or_len: copy _19, _marker: const core::marker::PhantomData::<&(u64, u64, u64, bool)> }
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)> <- ByRef(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)>
// CHECK-NEXT:     v21 = stack_addr.i16 ss5
// CHECK-NEXT:     v22 = load.i16 notrap v21
// CHECK-NEXT:     v23 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v22, v23
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): *const (u64, u64, u64, bool) <- ByVal(v20): *const (u64, u64, u64, bool)
// CHECK-NEXT:     v24 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v20, v24
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(4) }, None): core::marker::PhantomData<&'{erased} (u64, u64, u64, bool)> <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): core::marker::PhantomData<&'{erased} (u64, u64, u64, bool)>
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _25 = copy (_1.0: core::ptr::NonNull<(u64, u64, u64, bool)>)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)> <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)>
// CHECK-NEXT:     v25 = stack_addr.i16 ss0
// CHECK-NEXT:     v26 = load.i16 notrap v25
// CHECK-NEXT:     v27 = stack_addr.i16 ss7
// CHECK-NEXT:     store notrap v26, v27
// CHECK-NEXT: ; _26 = copy (_1.1: *const (u64, u64, u64, bool))
// CHECK-NEXT: ; write_cvalue: Var(_26, var21): *const (u64, u64, u64, bool) <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): *const (u64, u64, u64, bool)
// CHECK-NEXT:     v28 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v29 = load.i16 notrap v28
// CHECK-NEXT: ; _28 = copy _26 as core::ptr::NonNull<(u64, u64, u64, bool)> (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)> <- ByVal(v29): *const (u64, u64, u64, bool)
// CHECK-NEXT:     v30 = stack_addr.i16 ss8
// CHECK-NEXT:     store notrap v29, v30
// CHECK-NEXT: ; _31 = copy _25 as *mut (u64, u64, u64, bool) (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_31, var24): *mut (u64, u64, u64, bool) <- ByRef(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)>
// CHECK-NEXT:     v31 = stack_addr.i16 ss7
// CHECK-NEXT:     v32 = load.i16 notrap v31
// CHECK-NEXT:     v47 -> v32
// CHECK-NEXT: ; _32 = copy _28 as *mut (u64, u64, u64, bool) (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_32, var25): *mut (u64, u64, u64, bool) <- ByRef(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)>
// CHECK-NEXT:     v33 = stack_addr.i16 ss8
// CHECK-NEXT:     v34 = load.i16 notrap v33
// CHECK-NEXT: ; _27 = Eq(copy _31, copy _32)
// CHECK-NEXT:     v35 = icmp eq v32, v34
// CHECK-NEXT: ; write_cvalue: Var(_27, var22): bool <- ByVal(v35): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _27)
// CHECK-NEXT:     brif v35, block7, block8
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = &_9
// CHECK-NEXT:     v36 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_8, var7): &'{erased} (u64, bool) <- ByVal(v36): &'{erased} (u64, bool)
// CHECK-NEXT: ; _11 = (copy _5, copy _6)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u64 <- ByVal(v37): u64
// CHECK-NEXT:     v38 = stack_addr.i16 ss3
// CHECK-NEXT:     store.i64 notrap v37, v38
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(8) }, None): bool <- ByVal(v39): bool
// CHECK-NEXT:     v40 = stack_addr.i16 ss3+8
// CHECK-NEXT:     store.i8 notrap v39, v40
// CHECK-NEXT: ; _10 = &_11
// CHECK-NEXT:     v41 = stack_addr.i16 ss3
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): &'{erased} (u64, bool) <- ByVal(v41): &'{erased} (u64, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = <(u64, bool) as core::cmp::PartialEq>::ne(move _8, move _10)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u64, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u64, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v42 = call fn0(v36, v41)
// CHECK-NEXT:     v43 -> v42
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): bool <- ByVal(v42): bool
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _7)
// CHECK-NEXT:     brif.i8 v43, block5, block2
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const 0_u8
// CHECK-NEXT:     v44 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v44): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v44  ; v44 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     v75 = iconst.i8 0
// CHECK-NEXT:     v45 -> v75
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v45  ; v45 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const 1_u8
// CHECK-NEXT:     v46 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v46): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v46  ; v46 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _33 = Offset(copy _31, const 1_usize)
// CHECK-NEXT:     v48 = iconst.i16 1
// CHECK-NEXT:     v49 = iconst.i16 26
// CHECK-NEXT:     v50 = imul v48, v49  ; v48 = 1, v49 = 26
// CHECK-NEXT:     v51 = iadd.i16 v47, v50
// CHECK-NEXT: ; write_cvalue: Var(_33, var26): *mut (u64, u64, u64, bool) <- ByVal(v51): *mut (u64, u64, u64, bool)
// CHECK-NEXT: ; _29 = copy _33 as core::ptr::NonNull<(u64, u64, u64, bool)> (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss9), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)> <- ByVal(v51): *mut (u64, u64, u64, bool)
// CHECK-NEXT:     v52 = stack_addr.i16 ss9
// CHECK-NEXT:     store notrap v51, v52
// CHECK-NEXT: ; (_1.0: core::ptr::NonNull<(u64, u64, u64, bool)>) = move _29
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)> <- ByRef(Pointer { base: Stack(ss9), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)>
// CHECK-NEXT:     v53 = stack_addr.i16 ss9
// CHECK-NEXT:     v54 = load.i16 notrap v53
// CHECK-NEXT:     v55 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v54, v55
// CHECK-NEXT: ; _34 = copy _25 as *const (u64, u64, u64, bool) (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss10), offset: Offset32(0) }, None): *const (u64, u64, u64, bool) <- ByRef(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): core::ptr::NonNull<(u64, u64, u64, bool)>
// CHECK-NEXT:     v56 = stack_addr.i16 ss7
// CHECK-NEXT:     v57 = load.i16 notrap v56
// CHECK-NEXT:     v58 = stack_addr.i16 ss10
// CHECK-NEXT:     store notrap v57, v58
// CHECK-NEXT: ; _30 = &(*_34)
// CHECK-NEXT:     v59 = stack_addr.i16 ss10
// CHECK-NEXT:     v60 = load.i16 notrap v59
// CHECK-NEXT: ; write_cvalue: Var(_30, var23): &'{erased} (u64, u64, u64, bool) <- ByVal(v60): &'{erased} (u64, u64, u64, bool)
// CHECK-NEXT: ; _2 = core::option::Option::<&(u64, u64, u64, bool)>::Some(copy _30)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u64, u64, u64, bool) <- ByVal(v60): &'{erased} (u64, u64, u64, bool)
// CHECK-NEXT:     v61 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v60, v61
// CHECK-NEXT: ; _12 = no_retag copy ((_2 as Some).0: &(u64, u64, u64, bool))
// CHECK-NEXT: ; write_cvalue: Var(_12, var9): &'{erased} (u64, u64, u64, bool) <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u64, u64, u64, bool)
// CHECK-NEXT:     v62 = stack_addr.i16 ss1
// CHECK-NEXT:     v63 = load.i16 notrap v62
// CHECK-NEXT: ; _3 = copy ((*_12).0: u64)
// CHECK-NEXT: ; write_cvalue: Var(_3, var2): u64 <- ByRef(Pointer { base: Addr(v63), offset: Offset32(0) }, None): u64
// CHECK-NEXT:     v64 = load.i64 notrap v63
// CHECK-NEXT: ; _13 = no_retag copy ((_2 as Some).0: &(u64, u64, u64, bool))
// CHECK-NEXT: ; write_cvalue: Var(_13, var10): &'{erased} (u64, u64, u64, bool) <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u64, u64, u64, bool)
// CHECK-NEXT:     v65 = stack_addr.i16 ss1
// CHECK-NEXT:     v66 = load.i16 notrap v65
// CHECK-NEXT: ; _4 = copy ((*_13).1: u64)
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): u64 <- ByRef(Pointer { base: Addr(v66), offset: Offset32(8) }, None): u64
// CHECK-NEXT:     v67 = load.i64 notrap v66+8
// CHECK-NEXT: ; _14 = no_retag copy ((_2 as Some).0: &(u64, u64, u64, bool))
// CHECK-NEXT: ; write_cvalue: Var(_14, var11): &'{erased} (u64, u64, u64, bool) <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u64, u64, u64, bool)
// CHECK-NEXT:     v68 = stack_addr.i16 ss1
// CHECK-NEXT:     v69 = load.i16 notrap v68
// CHECK-NEXT: ; _5 = copy ((*_14).2: u64)
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): u64 <- ByRef(Pointer { base: Addr(v69), offset: Offset32(16) }, None): u64
// CHECK-NEXT:     v70 = load.i64 notrap v69+16
// CHECK-NEXT:     v37 -> v70
// CHECK-NEXT: ; _15 = no_retag copy ((_2 as Some).0: &(u64, u64, u64, bool))
// CHECK-NEXT: ; write_cvalue: Var(_15, var12): &'{erased} (u64, u64, u64, bool) <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u64, u64, u64, bool)
// CHECK-NEXT:     v71 = stack_addr.i16 ss1
// CHECK-NEXT:     v72 = load.i16 notrap v71
// CHECK-NEXT: ; _6 = copy ((*_15).3: bool)
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): bool <- ByRef(Pointer { base: Addr(v72), offset: Offset32(24) }, None): bool
// CHECK-NEXT:     v73 = load.i8 notrap v72+24
// CHECK-NEXT:     v39 -> v73
// CHECK-NEXT: ;
// CHECK-NEXT: ; _9 = unsigned_multiply(move _3, move _4)
// CHECK-NEXT:     v74 = stack_addr.i16 ss2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u64, layout: Layout { size: Size(8 bytes), align: Align(2 bytes), backend_repr: Scalar(u64 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934599 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u64, layout: Layout { size: Size(8 bytes), align: Align(2 bytes), backend_repr: Scalar(u64 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934599 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u64, bool), layout: Layout { size: Size(10 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u64 is .., b: u8 is 0..=1, b_offset: Size(8 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(8 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(8 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934599 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(10 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn1(v74, v64, v67)
// CHECK-NEXT:     jump block3
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol unsigned_multiply
// CHECK-NEXT: function u0:4(i16 sret, i64, i64) system_v {
// CHECK-NEXT: ; symbol unsigned_multiply
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(10 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } (u64, bool)
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u64
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u64                               8b 2                var=1
// CHECK-NEXT: ; ssa   _2    u64                               8b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 16, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 16, align = 2
// CHECK-NEXT:     sig0 = (i16, i16, i16) system_v
// CHECK-NEXT:     fn0 = u0:5 sig0 ; "__gbc_u128_mulo"
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i64, v2: i64):
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v4 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): u64 <- ByVal(v1): u64
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): u64 <- ByVal(v2): u64
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v5 = uextend.i128 v3
// CHECK-NEXT:     v6 = uextend.i128 v4
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u128 <- ByVal(v5): u128
// CHECK-NEXT:     v7 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v5, v7
// CHECK-NEXT:     v8 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u128 <- ByVal(v6): u128
// CHECK-NEXT:     v9 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v6, v9
// CHECK-NEXT:     v10 = stack_addr.i16 ss2
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT: ; lib_call __gbc_u128_mulo
// CHECK-NEXT:     call fn0(v11, v8, v10)
// CHECK-NEXT:     v12 = stack_addr.i16 ss0
// CHECK-NEXT:     v13 = load.i128 notrap v12
// CHECK-NEXT:     v14 = stack_addr.i16 ss0+16
// CHECK-NEXT:     v15 = load.i8 notrap v14
// CHECK-NEXT:     v16 = iconst.i64 64
// CHECK-NEXT:     v17 = uextend.i128 v16  ; v16 = 64
// CHECK-NEXT:     v18 = ushr v13, v17
// CHECK-NEXT:     v19 = ireduce.i64 v18
// CHECK-NEXT:     v20 = iconst.i64 0
// CHECK-NEXT:     v21 = icmp ne v19, v20  ; v20 = 0
// CHECK-NEXT:     v22 = bor v15, v21
// CHECK-NEXT:     v23 = ireduce.i64 v13
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): (u64, bool) <- ByValPair(v23, v22): (u64, bool)
// CHECK-NEXT:     store notrap v23, v0
// CHECK-NEXT:     store notrap v22, v0+8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
