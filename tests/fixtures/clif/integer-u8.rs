#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn integer_methods(x: u8, y: u8) -> u8 {
    let add = (1, true);
    let sub = (u8::MAX - 2, false);
    let mul = (u8::MAX - 1, true);
    (x.checked_add(y).is_none() as u8)
        | ((x.checked_sub(y) == Some(u8::MAX - 2)) as u8) << 1
        | (x.checked_mul(y).is_none() as u8) << 2
        | ((x.wrapping_add(y) == add.0) as u8) << 3
        | ((x.wrapping_sub(y) == sub.0) as u8) << 4
        | ((x.wrapping_mul(y) == mul.0) as u8) << 5
        | ((x.overflowing_add(y) == add
            && x.overflowing_sub(y) == sub
            && x.overflowing_mul(y) == mul) as u8) << 6
        | ((x.leading_zeros() == 0
            && x.trailing_zeros() == 0
            && x.count_ones() == 8) as u8) << 7
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    (integer_methods(u8::MAX, 2) == 255) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCsdyHBBVKw7bQ_10integer_u8
// CHECK-NEXT: function u0:0() system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCsdyHBBVKw7bQ_10integer_u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleThbENtNtB7_3cmp9PartialEq2eqCsdyHBBVKw7bQ_10integer_u8
// CHECK-NEXT: function u0:1(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleThbENtNtB7_3cmp9PartialEq2eqCsdyHBBVKw7bQ_10integer_u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} (u8, bool)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} (u8, bool)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} (u8, bool)             2b 2                storage=ss0
// CHECK-NEXT: ; stack _2    &'{erased} (u8, bool)             2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _4    &'{erased} u8                     2b 2                var=3
// CHECK-NEXT: ; ssa   _5    &'{erased} u8                     2b 2                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} bool                   2b 2                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} bool                   2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:3 sig0 ; Instance { def: Item(DefId(1:29972 ~ core[..]::cmp::impls::{impl#22}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:2 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (u8, bool) <- ByVal(v0): &'{erased} (u8, bool)
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u8, bool) <- ByVal(v1): &'{erased} (u8, bool)
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = &((*_1).0: U)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): &'{erased} u8 <- ByVal(v5): &'{erased} u8
// CHECK-NEXT: ; _5 = &((*_2).0: U)
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): &'{erased} u8 <- ByVal(v7): &'{erased} u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = <U as core::cmp::PartialEq>::eq(move _4, move _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; _6 = &((*_1).1: T)
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     v11 = load.i16 notrap v10
// CHECK-NEXT:     v12 = iconst.i16 1
// CHECK-NEXT:     v13 = iadd v11, v12  ; v12 = 1
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): &'{erased} bool <- ByVal(v13): &'{erased} bool
// CHECK-NEXT: ; _7 = &((*_2).1: T)
// CHECK-NEXT:     v14 = stack_addr.i16 ss1
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = iconst.i16 1
// CHECK-NEXT:     v17 = iadd v15, v16  ; v16 = 1
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): &'{erased} bool <- ByVal(v17): &'{erased} bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::cmp::PartialEq>::eq(move _6, move _7)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &bool, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &bool, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v18 = call fn1(v13, v17)
// CHECK-NEXT:     v20 -> v18
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v18): bool
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const false
// CHECK-NEXT:     v19 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v19): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v19  ; v19 = 0
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
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCsdyHBBVKw7bQ_10integer_u8
// CHECK-NEXT: function u0:2(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCsdyHBBVKw7bQ_10integer_u8
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
// CHECK-NEXT: ; _0 = Eq(move _3, move _4)
// CHECK-NEXT:     v6 = icmp eq v3, v5
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v6): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v6
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXsk_NtNtCs40R3J0iFVxM_4core3cmp5implshNtB7_9PartialEq2eqCsdyHBBVKw7bQ_10integer_u8
// CHECK-NEXT: function u0:3(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsk_NtNtCs40R3J0iFVxM_4core3cmp5implshNtB7_9PartialEq2eqCsdyHBBVKw7bQ_10integer_u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} u8                     2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} u8                     2b 2                var=3
// CHECK-NEXT: ; ssa   _3    u8                                1b 1                var=4
// CHECK-NEXT: ; ssa   _4    u8                                1b 1                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} u8 <- ByVal(v0): &'{erased} u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} u8 <- ByVal(v1): &'{erased} u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): u8 <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v3 = load.i8 notrap v2
// CHECK-NEXT: ; _4 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): u8 <- ByRef(Pointer { base: Addr(v4), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v5 = load.i8 notrap v4
// CHECK-NEXT: ; _0 = Eq(move _3, move _4)
// CHECK-NEXT:     v6 = icmp eq v3, v5
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v6): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v6
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:4() -> i8 system_v {
// CHECK-NEXT: ; symbol gbc_main
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _2    u8                                1b 1                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i8, i8) -> i8 system_v
// CHECK-NEXT:     fn0 = colocated u0:5 sig0 ; Instance { def: Item(DefId(0:3 ~ integer_u8[..]::integer_methods)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = integer_methods(const u8::MAX, const 2_u8)
// CHECK-NEXT:     v0 = iconst.i8 -1
// CHECK-NEXT:     v1 = iconst.i8 2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0(v0, v1)  ; v0 = -1, v1 = 2
// CHECK-NEXT:     v3 -> v2
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u8 <- ByVal(v2): u8
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = Eq(move _2, const u8::MAX)
// CHECK-NEXT:     v4 = iconst.i8 -1
// CHECK-NEXT:     v5 = icmp.i8 eq v3, v4  ; v4 = -1
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v5): bool
// CHECK-NEXT: ; _0 = move _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v5): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol integer_methods
// CHECK-NEXT: function u0:5(i8, i8) -> i8 system_v {
// CHECK-NEXT: ; symbol integer_methods
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u8                                1b 1                var=2
// CHECK-NEXT: ; ssa   _2    u8                                1b 1                var=3
// CHECK-NEXT: ; stack _3    (u8, bool)                        2b 1                storage=ss0
// CHECK-NEXT: ; stack _4    (u8, bool)                        2b 1                storage=ss1
// CHECK-NEXT: ; stack _5    (u8, bool)                        2b 1                storage=ss2
// CHECK-NEXT: ; ssa   _6    u8                                1b 1                var=4
// CHECK-NEXT: ; ssa   _7    u8                                1b 1                var=5
// CHECK-NEXT: ; ssa   _8    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=7
// CHECK-NEXT: ; ssa   _10   u8                                1b 1                var=8
// CHECK-NEXT: ; ssa   _11   u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=10
// CHECK-NEXT: ; ssa   _13   bool                              1b 1                var=11
// CHECK-NEXT: ; stack _14   core::option::Option<u8>          2b 1                storage=ss3
// CHECK-NEXT: ; ssa   _15   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _16   u8                                1b 1                var=13
// CHECK-NEXT: ; ssa   _17   bool                              1b 1                var=14
// CHECK-NEXT: ; stack _18   core::option::Option<u8>          2b 1                storage=ss4
// CHECK-NEXT: ; ssa   _19   u8                                1b 1                var=15
// CHECK-NEXT: ; ssa   _20   u8                                1b 1                var=16
// CHECK-NEXT: ; ssa   _21   bool                              1b 1                var=17
// CHECK-NEXT: ; stack _22   core::option::Option<u8>          2b 1                storage=ss5
// CHECK-NEXT: ; ssa   _23   u8                                1b 1                var=18
// CHECK-NEXT: ; ssa   _24   u8                                1b 1                var=19
// CHECK-NEXT: ; ssa   _25   bool                              1b 1                var=20
// CHECK-NEXT: ; ssa   _26   u8                                1b 1                var=21
// CHECK-NEXT: ; ssa   _27   u8                                1b 1                var=22
// CHECK-NEXT: ; ssa   _28   u8                                1b 1                var=23
// CHECK-NEXT: ; ssa   _29   bool                              1b 1                var=24
// CHECK-NEXT: ; ssa   _30   u8                                1b 1                var=25
// CHECK-NEXT: ; ssa   _31   u8                                1b 1                var=26
// CHECK-NEXT: ; ssa   _32   u8                                1b 1                var=27
// CHECK-NEXT: ; ssa   _33   bool                              1b 1                var=28
// CHECK-NEXT: ; ssa   _34   u8                                1b 1                var=29
// CHECK-NEXT: ; ssa   _35   u8                                1b 1                var=30
// CHECK-NEXT: ; ssa   _36   u8                                1b 1                var=31
// CHECK-NEXT: ; ssa   _37   bool                              1b 1                var=32
// CHECK-NEXT: ; ssa   _38   bool                              1b 1                var=33
// CHECK-NEXT: ; ssa   _39   &'{erased} (u8, bool)             2b 2                var=34
// CHECK-NEXT: ; stack _40   (u8, bool)                        2b 1                storage=ss6
// CHECK-NEXT: ; ssa   _41   &'{erased} (u8, bool)             2b 2                var=35
// CHECK-NEXT: ; ssa   _42   bool                              1b 1                var=36
// CHECK-NEXT: ; ssa   _43   &'{erased} (u8, bool)             2b 2                var=37
// CHECK-NEXT: ; stack _44   (u8, bool)                        2b 1                storage=ss7
// CHECK-NEXT: ; ssa   _45   &'{erased} (u8, bool)             2b 2                var=38
// CHECK-NEXT: ; ssa   _46   &'{erased} (u8, bool)             2b 2                var=39
// CHECK-NEXT: ; stack _47   (u8, bool)                        2b 1                storage=ss8
// CHECK-NEXT: ; ssa   _48   &'{erased} (u8, bool)             2b 2                var=40
// CHECK-NEXT: ; ssa   _49   u8                                1b 1                var=41
// CHECK-NEXT: ; ssa   _50   u8                                1b 1                var=42
// CHECK-NEXT: ; ssa   _51   bool                              1b 1                var=43
// CHECK-NEXT: ; ssa   _52   u32                               4b 2                var=44
// CHECK-NEXT: ; ssa   _53   u32                               4b 2                var=45
// CHECK-NEXT: ; ssa   _54   u32                               4b 2                var=46
// CHECK-NEXT: ; ssa   _55   bool                              1b 1                var=47
// CHECK-NEXT: ; ssa   _56   (u8, bool)                        2b 1                var=(48, 49)
// CHECK-NEXT: ; ssa   _57   u8                                1b 1                var=50
// CHECK-NEXT: ; zst   _58   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _59   bool                              1b 1                var=51
// CHECK-NEXT: ; ssa   _60   isize                             2b 2                var=52
// CHECK-NEXT: ; ssa   _61   bool                              1b 1                var=53
// CHECK-NEXT: ; ssa   _62   u8                                1b 1                var=54
// CHECK-NEXT: ; ssa   _63   u8                                1b 1                var=55
// CHECK-NEXT: ; ssa   _64   u8                                1b 1                var=56
// CHECK-NEXT: ; ssa   _65   bool                              1b 1                var=57
// CHECK-NEXT: ; ssa   _66   (u8, bool)                        2b 1                var=(58, 59)
// CHECK-NEXT: ; zst   _67   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _68   bool                              1b 1                var=60
// CHECK-NEXT: ; ssa   _69   isize                             2b 2                var=61
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2
// CHECK-NEXT:     ss1 = explicit_slot 2
// CHECK-NEXT:     ss2 = explicit_slot 2
// CHECK-NEXT:     ss3 = explicit_slot 2
// CHECK-NEXT:     ss4 = explicit_slot 2
// CHECK-NEXT:     ss5 = explicit_slot 2
// CHECK-NEXT:     ss6 = explicit_slot 2
// CHECK-NEXT:     ss7 = explicit_slot 2
// CHECK-NEXT:     ss8 = explicit_slot 2
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc7
// CHECK-NEXT:     gv1 = symbol colocated userextname1 ; alloc8
// CHECK-NEXT:     gv2 = symbol colocated userextname2 ; alloc9
// CHECK-NEXT:     gv3 = symbol colocated userextname4 ; alloc4
// CHECK-NEXT:     gv4 = symbol colocated userextname5 ; alloc6
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u8, bool] }
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u8, bool] }
// CHECK-NEXT:     fn2 = colocated u0:1 sig2 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u8, bool] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v17 -> v0
// CHECK-NEXT:     v75 -> v0
// CHECK-NEXT:     v167 -> v0
// CHECK-NEXT:     v18 -> v1
// CHECK-NEXT:     v76 -> v1
// CHECK-NEXT:     v168 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = const (1_u8, true)
// CHECK-NEXT:     v2 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): (u8, bool) <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): (u8, bool)
// CHECK-NEXT:     v3 = load.i8 notrap v2
// CHECK-NEXT:     v4 = load.i8 notrap v2+1
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v3, v5
// CHECK-NEXT:     v6 = stack_addr.i16 ss0+1
// CHECK-NEXT:     store notrap v4, v6
// CHECK-NEXT: ; _4 = const (253_u8, false)
// CHECK-NEXT:     v7 = symbol_value.i16 gv1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): (u8, bool) <- ByRef(Pointer { base: Addr(v7), offset: Offset32(0) }, None): (u8, bool)
// CHECK-NEXT:     v8 = load.i8 notrap v7
// CHECK-NEXT:     v9 = load.i8 notrap v7+1
// CHECK-NEXT:     v10 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v8, v10
// CHECK-NEXT:     v11 = stack_addr.i16 ss1+1
// CHECK-NEXT:     store notrap v9, v11
// CHECK-NEXT: ; _5 = const (254_u8, true)
// CHECK-NEXT:     v12 = symbol_value.i16 gv2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): (u8, bool) <- ByRef(Pointer { base: Addr(v12), offset: Offset32(0) }, None): (u8, bool)
// CHECK-NEXT:     v13 = load.i8 notrap v12
// CHECK-NEXT:     v14 = load.i8 notrap v12+1
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v13, v15
// CHECK-NEXT:     v16 = stack_addr.i16 ss2+1
// CHECK-NEXT:     store notrap v14, v16
// CHECK-NEXT: ; _56 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v19 = iadd.i8 v17, v18
// CHECK-NEXT:     v175 -> v19
// CHECK-NEXT:     v176 -> v19
// CHECK-NEXT:     v183 -> v19
// CHECK-NEXT:     v184 -> v19
// CHECK-NEXT:     v20 = icmp ult v19, v17
// CHECK-NEXT:     v177 -> v20
// CHECK-NEXT:     v178 -> v20
// CHECK-NEXT:     v185 -> v20
// CHECK-NEXT:     v186 -> v20
// CHECK-NEXT: ; write_cvalue: VarPair(_56, var48, var49): (u8, bool) <- ByValPair(v19, v20): (u8, bool)
// CHECK-NEXT: ; _55 = copy (_56.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_55, var47): bool <- ByVal(v20): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _55)
// CHECK-NEXT:     brif v20, block13, block15
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _38)
// CHECK-NEXT:     brif.i8 v21, block3, block6
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _44 = SubWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v24 = isub.i8 v22, v23
// CHECK-NEXT:     v25 = icmp ugt v24, v22
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): (u8, bool) <- ByValPair(v24, v25): (u8, bool)
// CHECK-NEXT:     v26 = stack_addr.i16 ss7
// CHECK-NEXT:     store notrap v24, v26
// CHECK-NEXT:     v27 = stack_addr.i16 ss7+1
// CHECK-NEXT:     store notrap v25, v27
// CHECK-NEXT: ; _43 = &_44
// CHECK-NEXT:     v28 = stack_addr.i16 ss7
// CHECK-NEXT: ; write_cvalue: Var(_43, var37): &'{erased} (u8, bool) <- ByVal(v28): &'{erased} (u8, bool)
// CHECK-NEXT: ; _45 = &_4
// CHECK-NEXT:     v29 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_45, var38): &'{erased} (u8, bool) <- ByVal(v29): &'{erased} (u8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _42 = <(u8, bool) as core::cmp::PartialEq>::eq(move _43, move _45)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v30 = call fn0(v28, v29)
// CHECK-NEXT:     v31 -> v30
// CHECK-NEXT: ; write_cvalue: Var(_42, var36): bool <- ByVal(v30): bool
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _42)
// CHECK-NEXT:     brif.i8 v31, block5, block6
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _47 = copy _66
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): (u8, bool) <- ByValPair(v32, v33): (u8, bool)
// CHECK-NEXT:     v34 = stack_addr.i16 ss8
// CHECK-NEXT:     store.i8 notrap v32, v34
// CHECK-NEXT:     v35 = stack_addr.i16 ss8+1
// CHECK-NEXT:     store.i8 notrap v33, v35
// CHECK-NEXT: ; _46 = &_47
// CHECK-NEXT:     v36 = stack_addr.i16 ss8
// CHECK-NEXT: ; write_cvalue: Var(_46, var39): &'{erased} (u8, bool) <- ByVal(v36): &'{erased} (u8, bool)
// CHECK-NEXT: ; _48 = &_5
// CHECK-NEXT:     v37 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_48, var40): &'{erased} (u8, bool) <- ByVal(v37): &'{erased} (u8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _37 = <(u8, bool) as core::cmp::PartialEq>::eq(move _46, move _48)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v38 = call fn1(v36, v37)
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): bool <- ByVal(v38): bool
// CHECK-NEXT:     jump block7(v38)
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _37 = const false
// CHECK-NEXT:     v39 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): bool <- ByVal(v39): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block7(v39)  ; v39 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block7(v40: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _36 = move _37 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_36, var31): u8 <- ByVal(v40): u8
// CHECK-NEXT: ; _35 = Shl(move _36, const 6_i32)
// CHECK-NEXT:     v41 = iconst.i32 6
// CHECK-NEXT:     v42 = ishl v40, v41  ; v41 = 6
// CHECK-NEXT: ; write_cvalue: Var(_35, var30): u8 <- ByVal(v42): u8
// CHECK-NEXT: ; _6 = BitOr(move _7, move _35)
// CHECK-NEXT:     v44 = bor.i8 v43, v42
// CHECK-NEXT:     v164 -> v44
// CHECK-NEXT:     v165 -> v44
// CHECK-NEXT:     v166 -> v44
// CHECK-NEXT: ; write_cvalue: Var(_6, var4): u8 <- ByVal(v44): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _52 = core::intrinsics::ctlz::<u8>(copy _1)
// CHECK-NEXT:     v46 = clz.i8 v45
// CHECK-NEXT:     v47 = uextend.i32 v46
// CHECK-NEXT:     v152 -> v47
// CHECK-NEXT: ; write_cvalue: Var(_52, var44): u32 <- ByVal(v47): u32
// CHECK-NEXT:     jump block23
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _53 = core::intrinsics::cttz::<u8>(copy _1)
// CHECK-NEXT:     v49 = ctz.i8 v48
// CHECK-NEXT:     v50 = uextend.i32 v49
// CHECK-NEXT:     v153 -> v50
// CHECK-NEXT: ; write_cvalue: Var(_53, var45): u32 <- ByVal(v50): u32
// CHECK-NEXT:     jump block24
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _54 = core::intrinsics::ctpop::<u8>(move _1)
// CHECK-NEXT:     v52 = popcnt.i8 v51
// CHECK-NEXT:     v53 = uextend.i32 v52
// CHECK-NEXT:     v154 -> v53
// CHECK-NEXT: ; write_cvalue: Var(_54, var46): u32 <- ByVal(v53): u32
// CHECK-NEXT:     jump block25
// CHECK-EMPTY:
// CHECK-NEXT: block10(v163: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _51 = const false
// CHECK-NEXT:     v54 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_51, var43): bool <- ByVal(v54): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block11(v54, v163)  ; v54 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block11(v55: i8, v58: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _50 = move _51 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_50, var42): u8 <- ByVal(v55): u8
// CHECK-NEXT: ; _49 = Shl(move _50, const 7_i32)
// CHECK-NEXT:     v56 = iconst.i32 7
// CHECK-NEXT:     v57 = ishl v55, v56  ; v56 = 7
// CHECK-NEXT: ; write_cvalue: Var(_49, var41): u8 <- ByVal(v57): u8
// CHECK-NEXT: ; _0 = BitOr(move _6, move _49)
// CHECK-NEXT:     v59 = bor v58, v57
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v59): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v59
// CHECK-EMPTY:
// CHECK-NEXT: block12(v67: i8, v68: i8):
// CHECK-NEXT:     v82 -> v67
// CHECK-NEXT:     v97 -> v67
// CHECK-NEXT:     v171 -> v67
// CHECK-NEXT:     v172 -> v67
// CHECK-NEXT:     v83 -> v68
// CHECK-NEXT:     v98 -> v68
// CHECK-NEXT:     v173 -> v68
// CHECK-NEXT:     v174 -> v68
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = discriminant(_14)
// CHECK-NEXT:     v60 = stack_addr.i16 ss3
// CHECK-NEXT:     v61 = load.i8 notrap v60
// CHECK-NEXT:     v62 = uextend.i16 v61
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): isize <- ByVal(v62): isize
// CHECK-NEXT: ; _59 = Eq(copy _60, const 1_isize)
// CHECK-NEXT:     v63 = iconst.i16 1
// CHECK-NEXT:     v64 = icmp eq v62, v63  ; v63 = 1
// CHECK-NEXT: ; write_cvalue: Var(_59, var51): bool <- ByVal(v64): bool
// CHECK-NEXT: ; _13 = Not(move _59)
// CHECK-NEXT:     v65 = iconst.i8 0
// CHECK-NEXT:     v66 = icmp eq v64, v65  ; v65 = 0
// CHECK-NEXT:     v95 -> v66
// CHECK-NEXT: ; write_cvalue: Var(_13, var11): bool <- ByVal(v66): bool
// CHECK-NEXT: ; _12 = move _13 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): u8 <- ByVal(v66): u8
// CHECK-NEXT: ; _61 = Lt(copy _1, copy _2)
// CHECK-NEXT:     v69 = icmp ult v67, v68
// CHECK-NEXT: ; write_cvalue: Var(_61, var53): bool <- ByVal(v69): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _61)
// CHECK-NEXT:     brif v69, block16, block17
// CHECK-EMPTY:
// CHECK-NEXT: block13 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _58 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block14
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _14 = const core::option::Option::<u8>::None
// CHECK-NEXT:     v70 = symbol_value.i16 gv3
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): core::option::Option<u8> <- ByRef(Pointer { base: Addr(v70), offset: Offset32(0) }, None): core::option::Option<u8>
// CHECK-NEXT:     v71 = load.i8 notrap v70
// CHECK-NEXT:     v72 = load.i8 notrap v70+1
// CHECK-NEXT:     v73 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v71, v73
// CHECK-NEXT:     v74 = stack_addr.i16 ss3+1
// CHECK-NEXT:     store notrap v72, v74
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v167, v168)
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _57 = AddUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v77 = iadd.i8 v75, v76
// CHECK-NEXT: ; write_cvalue: Var(_57, var50): u8 <- ByVal(v77): u8
// CHECK-NEXT: ; _14 = core::option::Option::<u8>::Some(move _57)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(1) }, None): u8 <- ByVal(v77): u8
// CHECK-NEXT:     v78 = stack_addr.i16 ss3+1
// CHECK-NEXT:     store notrap v77, v78
// CHECK-NEXT:     v79 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u8 <- ByVal(v79): u8
// CHECK-NEXT:     v80 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v79, v80  ; v79 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v75, v76)
// CHECK-EMPTY:
// CHECK-NEXT: block16:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _17 = const false
// CHECK-NEXT:     v81 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_17, var14): bool <- ByVal(v81): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block18(v81)  ; v81 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block17:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _62 = SubUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v84 = isub.i8 v82, v83
// CHECK-NEXT: ; write_cvalue: Var(_62, var54): u8 <- ByVal(v84): u8
// CHECK-NEXT: ; _18 = core::option::Option::<u8>::Some(move _62)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(1) }, None): u8 <- ByVal(v84): u8
// CHECK-NEXT:     v85 = stack_addr.i16 ss4+1
// CHECK-NEXT:     store notrap v84, v85
// CHECK-NEXT:     v86 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u8 <- ByVal(v86): u8
// CHECK-NEXT:     v87 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v86, v87  ; v86 = 1
// CHECK-NEXT: ; _63 = copy ((_18 as Some).0: u8)
// CHECK-NEXT: ; write_cvalue: Var(_63, var55): u8 <- ByRef(Pointer { base: Stack(ss4), offset: Offset32(1) }, None): u8
// CHECK-NEXT:     v88 = stack_addr.i16 ss4+1
// CHECK-NEXT:     v89 = load.i8 notrap v88
// CHECK-NEXT: ; _17 = Eq(move _63, const 253_u8)
// CHECK-NEXT:     v90 = iconst.i8 -3
// CHECK-NEXT:     v91 = icmp eq v89, v90  ; v90 = -3
// CHECK-NEXT: ; write_cvalue: Var(_17, var14): bool <- ByVal(v91): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block18(v91)
// CHECK-EMPTY:
// CHECK-NEXT: block18(v92: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = move _17 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_16, var13): u8 <- ByVal(v92): u8
// CHECK-NEXT: ; _15 = Shl(move _16, const 1_i32)
// CHECK-NEXT:     v93 = iconst.i32 1
// CHECK-NEXT:     v94 = ishl v92, v93  ; v93 = 1
// CHECK-NEXT: ; write_cvalue: Var(_15, var12): u8 <- ByVal(v94): u8
// CHECK-NEXT: ; _11 = BitOr(move _12, move _15)
// CHECK-NEXT:     v96 = bor.i8 v95, v94
// CHECK-NEXT:     v169 -> v96
// CHECK-NEXT:     v170 -> v96
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): u8 <- ByVal(v96): u8
// CHECK-NEXT: ; _66 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v99 = uextend.i16 v97
// CHECK-NEXT:     v100 = uextend.i16 v98
// CHECK-NEXT:     v101 = imul v99, v100
// CHECK-NEXT:     v102 = iconst.i16 255
// CHECK-NEXT:     v103 = icmp ugt v101, v102  ; v102 = 255
// CHECK-NEXT:     v181 -> v103
// CHECK-NEXT:     v182 -> v103
// CHECK-NEXT:     v104 = ireduce.i8 v101
// CHECK-NEXT:     v148 -> v104
// CHECK-NEXT:     v179 -> v104
// CHECK-NEXT:     v180 -> v104
// CHECK-NEXT: ; write_cvalue: VarPair(_66, var58, var59): (u8, bool) <- ByValPair(v104, v103): (u8, bool)
// CHECK-NEXT: ; _64 = copy (_66.0: u8)
// CHECK-NEXT: ; write_cvalue: Var(_64, var56): u8 <- ByVal(v104): u8
// CHECK-NEXT: ; _65 = copy (_66.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_65, var57): bool <- ByVal(v103): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _65)
// CHECK-NEXT:     brif v103, block20, block22
// CHECK-EMPTY:
// CHECK-NEXT: block19(v114: i8, v116: i8, v117: i8, v136: i8, v137: i8, v157: i8, v158: i8):
// CHECK-NEXT:     v22 -> v116
// CHECK-NEXT:     v45 -> v116
// CHECK-NEXT:     v160 -> v116
// CHECK-NEXT:     v161 -> v116
// CHECK-NEXT:     v162 -> v116
// CHECK-NEXT:     v51 -> v162
// CHECK-NEXT:     v48 -> v161
// CHECK-NEXT:     v23 -> v117
// CHECK-NEXT:     v32 -> v157
// CHECK-NEXT:     v33 -> v158
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _69 = discriminant(_22)
// CHECK-NEXT:     v105 = stack_addr.i16 ss5
// CHECK-NEXT:     v106 = load.i8 notrap v105
// CHECK-NEXT:     v107 = uextend.i16 v106
// CHECK-NEXT: ; write_cvalue: Var(_69, var61): isize <- ByVal(v107): isize
// CHECK-NEXT: ; _68 = Eq(copy _69, const 1_isize)
// CHECK-NEXT:     v108 = iconst.i16 1
// CHECK-NEXT:     v109 = icmp eq v107, v108  ; v108 = 1
// CHECK-NEXT: ; write_cvalue: Var(_68, var60): bool <- ByVal(v109): bool
// CHECK-NEXT: ; _21 = Not(move _68)
// CHECK-NEXT:     v110 = iconst.i8 0
// CHECK-NEXT:     v111 = icmp eq v109, v110  ; v110 = 0
// CHECK-NEXT: ; write_cvalue: Var(_21, var17): bool <- ByVal(v111): bool
// CHECK-NEXT: ; _20 = move _21 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_20, var16): u8 <- ByVal(v111): u8
// CHECK-NEXT: ; _19 = Shl(move _20, const 2_i32)
// CHECK-NEXT:     v112 = iconst.i32 2
// CHECK-NEXT:     v113 = ishl v111, v112  ; v112 = 2
// CHECK-NEXT: ; write_cvalue: Var(_19, var15): u8 <- ByVal(v113): u8
// CHECK-NEXT: ; _10 = BitOr(move _11, move _19)
// CHECK-NEXT:     v115 = bor v114, v113
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): u8 <- ByVal(v115): u8
// CHECK-NEXT: ; _26 = Add(copy _1, copy _2)
// CHECK-NEXT:     v118 = iadd v116, v117
// CHECK-NEXT: ; write_cvalue: Var(_26, var21): u8 <- ByVal(v118): u8
// CHECK-NEXT: ; _25 = Eq(move _26, const 1_u8)
// CHECK-NEXT:     v119 = iconst.i8 1
// CHECK-NEXT:     v120 = icmp eq v118, v119  ; v119 = 1
// CHECK-NEXT: ; write_cvalue: Var(_25, var20): bool <- ByVal(v120): bool
// CHECK-NEXT: ; _24 = move _25 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_24, var19): u8 <- ByVal(v120): u8
// CHECK-NEXT: ; _23 = Shl(move _24, const 3_i32)
// CHECK-NEXT:     v121 = iconst.i32 3
// CHECK-NEXT:     v122 = ishl v120, v121  ; v121 = 3
// CHECK-NEXT: ; write_cvalue: Var(_23, var18): u8 <- ByVal(v122): u8
// CHECK-NEXT: ; _9 = BitOr(move _10, move _23)
// CHECK-NEXT:     v123 = bor v115, v122
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): u8 <- ByVal(v123): u8
// CHECK-NEXT: ; _30 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v124 = isub v116, v117
// CHECK-NEXT: ; write_cvalue: Var(_30, var25): u8 <- ByVal(v124): u8
// CHECK-NEXT: ; _29 = Eq(move _30, const 253_u8)
// CHECK-NEXT:     v125 = iconst.i8 -3
// CHECK-NEXT:     v126 = icmp eq v124, v125  ; v125 = -3
// CHECK-NEXT: ; write_cvalue: Var(_29, var24): bool <- ByVal(v126): bool
// CHECK-NEXT: ; _28 = move _29 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_28, var23): u8 <- ByVal(v126): u8
// CHECK-NEXT: ; _27 = Shl(move _28, const 4_i32)
// CHECK-NEXT:     v127 = iconst.i32 4
// CHECK-NEXT:     v128 = ishl v126, v127  ; v127 = 4
// CHECK-NEXT: ; write_cvalue: Var(_27, var22): u8 <- ByVal(v128): u8
// CHECK-NEXT: ; _8 = BitOr(move _9, move _27)
// CHECK-NEXT:     v129 = bor v123, v128
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): u8 <- ByVal(v129): u8
// CHECK-NEXT: ; _34 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v130 = imul v116, v117
// CHECK-NEXT: ; write_cvalue: Var(_34, var29): u8 <- ByVal(v130): u8
// CHECK-NEXT: ; _33 = Eq(move _34, const 254_u8)
// CHECK-NEXT:     v131 = iconst.i8 -2
// CHECK-NEXT:     v132 = icmp eq v130, v131  ; v131 = -2
// CHECK-NEXT: ; write_cvalue: Var(_33, var28): bool <- ByVal(v132): bool
// CHECK-NEXT: ; _32 = move _33 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_32, var27): u8 <- ByVal(v132): u8
// CHECK-NEXT: ; _31 = Shl(move _32, const 5_i32)
// CHECK-NEXT:     v133 = iconst.i32 5
// CHECK-NEXT:     v134 = ishl v132, v133  ; v133 = 5
// CHECK-NEXT: ; write_cvalue: Var(_31, var26): u8 <- ByVal(v134): u8
// CHECK-NEXT: ; _7 = BitOr(move _8, move _31)
// CHECK-NEXT:     v135 = bor v129, v134
// CHECK-NEXT:     v43 -> v135
// CHECK-NEXT:     v159 -> v135
// CHECK-NEXT: ; write_cvalue: Var(_7, var5): u8 <- ByVal(v135): u8
// CHECK-NEXT: ; _40 = copy _56
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): (u8, bool) <- ByValPair(v136, v137): (u8, bool)
// CHECK-NEXT:     v138 = stack_addr.i16 ss6
// CHECK-NEXT:     store notrap v136, v138
// CHECK-NEXT:     v139 = stack_addr.i16 ss6+1
// CHECK-NEXT:     store notrap v137, v139
// CHECK-NEXT: ; _39 = &_40
// CHECK-NEXT:     v140 = stack_addr.i16 ss6
// CHECK-NEXT: ; write_cvalue: Var(_39, var34): &'{erased} (u8, bool) <- ByVal(v140): &'{erased} (u8, bool)
// CHECK-NEXT: ; _41 = &_3
// CHECK-NEXT:     v141 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_41, var35): &'{erased} (u8, bool) <- ByVal(v141): &'{erased} (u8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _38 = <(u8, bool) as core::cmp::PartialEq>::eq(move _39, move _41)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v142 = call fn2(v140, v141)
// CHECK-NEXT:     v21 -> v142
// CHECK-NEXT: ; write_cvalue: Var(_38, var33): bool <- ByVal(v142): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block20 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _67 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block21
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = const core::option::Option::<u8>::None
// CHECK-NEXT:     v143 = symbol_value.i16 gv4
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): core::option::Option<u8> <- ByRef(Pointer { base: Addr(v143), offset: Offset32(0) }, None): core::option::Option<u8>
// CHECK-NEXT:     v144 = load.i8 notrap v143
// CHECK-NEXT:     v145 = load.i8 notrap v143+1
// CHECK-NEXT:     v146 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v144, v146
// CHECK-NEXT:     v147 = stack_addr.i16 ss5+1
// CHECK-NEXT:     store notrap v145, v147
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block19(v169, v171, v173, v175, v177, v179, v181)
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = core::option::Option::<u8>::Some(copy _64)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(1) }, None): u8 <- ByVal(v148): u8
// CHECK-NEXT:     v149 = stack_addr.i16 ss5+1
// CHECK-NEXT:     store.i8 notrap v148, v149
// CHECK-NEXT:     v150 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): u8 <- ByVal(v150): u8
// CHECK-NEXT:     v151 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v150, v151  ; v150 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block19(v170, v172, v174, v176, v178, v180, v182)
// CHECK-EMPTY:
// CHECK-NEXT: block23:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _52)
// CHECK-NEXT:     brif.i32 v152, block10(v164), block8
// CHECK-EMPTY:
// CHECK-NEXT: block24:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _53)
// CHECK-NEXT:     brif.i32 v153, block10(v165), block9
// CHECK-EMPTY:
// CHECK-NEXT: block25:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _51 = Eq(move _54, const 8_u32)
// CHECK-NEXT:     v155 = iconst.i32 8
// CHECK-NEXT:     v156 = icmp.i32 eq v154, v155  ; v155 = 8
// CHECK-NEXT: ; write_cvalue: Var(_51, var43): bool <- ByVal(v156): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block11(v156, v166)
// CHECK-NEXT: }
