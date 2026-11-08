#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn integer_methods(x: usize, y: usize) -> u8 {
    let add = (1, true);
    let sub = (usize::MAX - 2, false);
    let mul = (usize::MAX - 1, true);
    (x.checked_add(y).is_none() as u8)
        | ((x.checked_sub(y) == Some(usize::MAX - 2)) as u8) << 1
        | (x.checked_mul(y).is_none() as u8) << 2
        | ((x.wrapping_add(y) == add.0) as u8) << 3
        | ((x.wrapping_sub(y) == sub.0) as u8) << 4
        | ((x.wrapping_mul(y) == mul.0) as u8) << 5
        | ((x.overflowing_add(y) == add
            && x.overflowing_sub(y) == sub
            && x.overflowing_mul(y) == mul) as u8) << 6
        | ((x.leading_zeros() == 0
            && x.trailing_zeros() == 0
            && x.count_ones() == 16) as u8) << 7
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    (integer_methods(usize::MAX, 2) == 255) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCsa0LCqDJU2Ss_13integer_usize
// CHECK-NEXT: function u0:0() system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCsa0LCqDJU2Ss_13integer_usize
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
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTjbENtNtB7_3cmp9PartialEq2eqCsa0LCqDJU2Ss_13integer_usize
// CHECK-NEXT: function u0:1(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTjbENtNtB7_3cmp9PartialEq2eqCsa0LCqDJU2Ss_13integer_usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (usize, bool)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (usize, bool)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} (usize, bool)          2b 2                storage=ss0
// CHECK-NEXT: ; stack _2    &'{erased} (usize, bool)          2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _4    &'{erased} usize                  2b 2                var=3
// CHECK-NEXT: ; ssa   _5    &'{erased} usize                  2b 2                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} bool                   2b 2                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} bool                   2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:3 sig0 ; Instance { def: Item(DefId(1:29969 ~ core[..]::cmp::impls::{impl#21}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:2 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (usize, bool) <- ByVal(v0): &'{erased} (usize, bool)
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (usize, bool) <- ByVal(v1): &'{erased} (usize, bool)
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = &((*_1).0: U)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): &'{erased} usize <- ByVal(v5): &'{erased} usize
// CHECK-NEXT: ; _5 = &((*_2).0: U)
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): &'{erased} usize <- ByVal(v7): &'{erased} usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = <U as core::cmp::PartialEq>::eq(move _4, move _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT:     v12 = iconst.i16 2
// CHECK-NEXT:     v13 = iadd v11, v12  ; v12 = 2
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): &'{erased} bool <- ByVal(v13): &'{erased} bool
// CHECK-NEXT: ; _7 = &((*_2).1: T)
// CHECK-NEXT:     v14 = stack_addr.i16 ss1
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = iconst.i16 2
// CHECK-NEXT:     v17 = iadd v15, v16  ; v16 = 2
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
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCsa0LCqDJU2Ss_13integer_usize
// CHECK-NEXT: function u0:2(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCsa0LCqDJU2Ss_13integer_usize
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
// CHECK-NEXT: ; symbol _RNvXsj_NtNtCs40R3J0iFVxM_4core3cmp5implsjNtB7_9PartialEq2eqCsa0LCqDJU2Ss_13integer_usize
// CHECK-NEXT: function u0:3(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsj_NtNtCs40R3J0iFVxM_4core3cmp5implsjNtB7_9PartialEq2eqCsa0LCqDJU2Ss_13integer_usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} usize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} usize                  2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} usize                  2b 2                var=3
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=4
// CHECK-NEXT: ; ssa   _4    usize                             2b 2                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} usize <- ByVal(v0): &'{erased} usize
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} usize <- ByVal(v1): &'{erased} usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): usize <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): usize
// CHECK-NEXT:     v3 = load.i16 notrap v2
// CHECK-NEXT: ; _4 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): usize <- ByRef(Pointer { base: Addr(v4), offset: Offset32(0) }, None): usize
// CHECK-NEXT:     v5 = load.i16 notrap v4
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
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 system_v
// CHECK-NEXT:     fn0 = colocated u0:5 sig0 ; Instance { def: Item(DefId(0:3 ~ integer_usize[..]::integer_methods)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = integer_methods(const usize::MAX, const 2_usize)
// CHECK-NEXT:     v0 = iconst.i16 -1
// CHECK-NEXT:     v1 = iconst.i16 2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: function u0:5(i16, i16) -> i8 system_v {
// CHECK-NEXT: ; symbol integer_methods
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    usize                             2b 2                var=2
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=3
// CHECK-NEXT: ; stack _3    (usize, bool)                     4b 2                storage=ss0
// CHECK-NEXT: ; stack _4    (usize, bool)                     4b 2                storage=ss1
// CHECK-NEXT: ; stack _5    (usize, bool)                     4b 2                storage=ss2
// CHECK-NEXT: ; ssa   _6    u8                                1b 1                var=4
// CHECK-NEXT: ; ssa   _7    u8                                1b 1                var=5
// CHECK-NEXT: ; ssa   _8    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=7
// CHECK-NEXT: ; ssa   _10   u8                                1b 1                var=8
// CHECK-NEXT: ; ssa   _11   u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=10
// CHECK-NEXT: ; ssa   _13   bool                              1b 1                var=11
// CHECK-NEXT: ; stack _14   core::option::Option<usize>       4b 2                storage=ss3
// CHECK-NEXT: ; ssa   _15   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _16   u8                                1b 1                var=13
// CHECK-NEXT: ; ssa   _17   bool                              1b 1                var=14
// CHECK-NEXT: ; stack _18   core::option::Option<usize>       4b 2                storage=ss4
// CHECK-NEXT: ; ssa   _19   u8                                1b 1                var=15
// CHECK-NEXT: ; ssa   _20   u8                                1b 1                var=16
// CHECK-NEXT: ; ssa   _21   bool                              1b 1                var=17
// CHECK-NEXT: ; stack _22   core::option::Option<usize>       4b 2                storage=ss5
// CHECK-NEXT: ; ssa   _23   u8                                1b 1                var=18
// CHECK-NEXT: ; ssa   _24   u8                                1b 1                var=19
// CHECK-NEXT: ; ssa   _25   bool                              1b 1                var=20
// CHECK-NEXT: ; ssa   _26   usize                             2b 2                var=21
// CHECK-NEXT: ; ssa   _27   u8                                1b 1                var=22
// CHECK-NEXT: ; ssa   _28   u8                                1b 1                var=23
// CHECK-NEXT: ; ssa   _29   bool                              1b 1                var=24
// CHECK-NEXT: ; ssa   _30   usize                             2b 2                var=25
// CHECK-NEXT: ; ssa   _31   u8                                1b 1                var=26
// CHECK-NEXT: ; ssa   _32   u8                                1b 1                var=27
// CHECK-NEXT: ; ssa   _33   bool                              1b 1                var=28
// CHECK-NEXT: ; ssa   _34   usize                             2b 2                var=29
// CHECK-NEXT: ; ssa   _35   u8                                1b 1                var=30
// CHECK-NEXT: ; ssa   _36   u8                                1b 1                var=31
// CHECK-NEXT: ; ssa   _37   bool                              1b 1                var=32
// CHECK-NEXT: ; ssa   _38   bool                              1b 1                var=33
// CHECK-NEXT: ; ssa   _39   &'{erased} (usize, bool)          2b 2                var=34
// CHECK-NEXT: ; stack _40   (usize, bool)                     4b 2                storage=ss6
// CHECK-NEXT: ; ssa   _41   &'{erased} (usize, bool)          2b 2                var=35
// CHECK-NEXT: ; ssa   _42   bool                              1b 1                var=36
// CHECK-NEXT: ; ssa   _43   &'{erased} (usize, bool)          2b 2                var=37
// CHECK-NEXT: ; stack _44   (usize, bool)                     4b 2                storage=ss7
// CHECK-NEXT: ; ssa   _45   &'{erased} (usize, bool)          2b 2                var=38
// CHECK-NEXT: ; ssa   _46   &'{erased} (usize, bool)          2b 2                var=39
// CHECK-NEXT: ; stack _47   (usize, bool)                     4b 2                storage=ss8
// CHECK-NEXT: ; ssa   _48   &'{erased} (usize, bool)          2b 2                var=40
// CHECK-NEXT: ; ssa   _49   u8                                1b 1                var=41
// CHECK-NEXT: ; ssa   _50   u8                                1b 1                var=42
// CHECK-NEXT: ; ssa   _51   bool                              1b 1                var=43
// CHECK-NEXT: ; ssa   _52   u32                               4b 2                var=44
// CHECK-NEXT: ; ssa   _53   u32                               4b 2                var=45
// CHECK-NEXT: ; ssa   _54   u32                               4b 2                var=46
// CHECK-NEXT: ; ssa   _55   bool                              1b 1                var=47
// CHECK-NEXT: ; ssa   _56   (usize, bool)                     4b 2                var=(48, 49)
// CHECK-NEXT: ; ssa   _57   usize                             2b 2                var=50
// CHECK-NEXT: ; zst   _58   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _59   bool                              1b 1                var=51
// CHECK-NEXT: ; ssa   _60   isize                             2b 2                var=52
// CHECK-NEXT: ; ssa   _61   bool                              1b 1                var=53
// CHECK-NEXT: ; ssa   _62   usize                             2b 2                var=54
// CHECK-NEXT: ; ssa   _63   usize                             2b 2                var=55
// CHECK-NEXT: ; ssa   _64   u16                               2b 2                var=56
// CHECK-NEXT: ; ssa   _65   bool                              1b 1                var=57
// CHECK-NEXT: ; ssa   _66   (u16, bool)                       4b 2                var=(58, 59)
// CHECK-NEXT: ; ssa   _67   u16                               2b 2                var=60
// CHECK-NEXT: ; ssa   _68   u16                               2b 2                var=61
// CHECK-NEXT: ; ssa   _69   usize                             2b 2                var=62
// CHECK-NEXT: ; zst   _70   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _71   bool                              1b 1                var=63
// CHECK-NEXT: ; ssa   _72   isize                             2b 2                var=64
// CHECK-NEXT: ; ssa   _73   u16                               2b 2                var=65
// CHECK-NEXT: ; ssa   _74   bool                              1b 1                var=66
// CHECK-NEXT: ; ssa   _75   (u16, bool)                       4b 2                var=(67, 68)
// CHECK-NEXT: ; ssa   _76   usize                             2b 2                var=69
// CHECK-NEXT: ; ssa   _77   u16                               2b 2                var=70
// CHECK-NEXT: ; ssa   _78   bool                              1b 1                var=71
// CHECK-NEXT: ; ssa   _79   (u16, bool)                       4b 2                var=(72, 73)
// CHECK-NEXT: ; ssa   _80   usize                             2b 2                var=74
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss3 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss4 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss5 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss6 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss7 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss8 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc7
// CHECK-NEXT:     gv1 = symbol colocated userextname1 ; alloc8
// CHECK-NEXT:     gv2 = symbol colocated userextname2 ; alloc9
// CHECK-NEXT:     gv3 = symbol colocated userextname4 ; alloc4
// CHECK-NEXT:     gv4 = symbol colocated userextname5 ; alloc6
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [usize, bool] }
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [usize, bool] }
// CHECK-NEXT:     fn2 = colocated u0:1 sig2 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [usize, bool] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v17 -> v0
// CHECK-NEXT:     v74 -> v0
// CHECK-NEXT:     v167 -> v0
// CHECK-NEXT:     v18 -> v1
// CHECK-NEXT:     v75 -> v1
// CHECK-NEXT:     v168 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): usize <- ByVal(v0): usize
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = const (1_usize, true)
// CHECK-NEXT:     v2 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): (usize, bool) <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): (usize, bool)
// CHECK-NEXT:     v3 = load.i16 notrap v2
// CHECK-NEXT:     v4 = load.i8 notrap v2+2
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v3, v5
// CHECK-NEXT:     v6 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v4, v6
// CHECK-NEXT: ; _4 = const (65533_usize, false)
// CHECK-NEXT:     v7 = symbol_value.i16 gv1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): (usize, bool) <- ByRef(Pointer { base: Addr(v7), offset: Offset32(0) }, None): (usize, bool)
// CHECK-NEXT:     v8 = load.i16 notrap v7
// CHECK-NEXT:     v9 = load.i8 notrap v7+2
// CHECK-NEXT:     v10 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v8, v10
// CHECK-NEXT:     v11 = stack_addr.i16 ss1+2
// CHECK-NEXT:     store notrap v9, v11
// CHECK-NEXT: ; _5 = const (65534_usize, true)
// CHECK-NEXT:     v12 = symbol_value.i16 gv2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): (usize, bool) <- ByRef(Pointer { base: Addr(v12), offset: Offset32(0) }, None): (usize, bool)
// CHECK-NEXT:     v13 = load.i16 notrap v12
// CHECK-NEXT:     v14 = load.i8 notrap v12+2
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v13, v15
// CHECK-NEXT:     v16 = stack_addr.i16 ss2+2
// CHECK-NEXT:     store notrap v14, v16
// CHECK-NEXT: ; _56 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v19 = iadd.i16 v17, v18
// CHECK-NEXT:     v20 = icmp ult v19, v17
// CHECK-NEXT: ; write_cvalue: VarPair(_56, var48, var49): (usize, bool) <- ByValPair(v19, v20): (usize, bool)
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
// CHECK-NEXT: ; _79 = SubWithOverflow(copy _67, copy _68)
// CHECK-NEXT:     v24 = isub.i16 v22, v23
// CHECK-NEXT:     v25 = icmp ugt v24, v22
// CHECK-NEXT: ; write_cvalue: VarPair(_79, var72, var73): (u16, bool) <- ByValPair(v24, v25): (u16, bool)
// CHECK-NEXT: ; _77 = copy (_79.0: u16)
// CHECK-NEXT: ; write_cvalue: Var(_77, var70): u16 <- ByVal(v24): u16
// CHECK-NEXT: ; _78 = copy (_79.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_78, var71): bool <- ByVal(v25): bool
// CHECK-NEXT: ; _80 = copy _77 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_80, var74): usize <- ByVal(v24): usize
// CHECK-NEXT: ; _44 = (move _80, copy _78)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): usize <- ByVal(v24): usize
// CHECK-NEXT:     v26 = stack_addr.i16 ss7
// CHECK-NEXT:     store notrap v24, v26
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(2) }, None): bool <- ByVal(v25): bool
// CHECK-NEXT:     v27 = stack_addr.i16 ss7+2
// CHECK-NEXT:     store notrap v25, v27
// CHECK-NEXT: ; _43 = &_44
// CHECK-NEXT:     v28 = stack_addr.i16 ss7
// CHECK-NEXT: ; write_cvalue: Var(_43, var37): &'{erased} (usize, bool) <- ByVal(v28): &'{erased} (usize, bool)
// CHECK-NEXT: ; _45 = &_4
// CHECK-NEXT:     v29 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_45, var38): &'{erased} (usize, bool) <- ByVal(v29): &'{erased} (usize, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _42 = <(usize, bool) as core::cmp::PartialEq>::eq(move _43, move _45)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; _47 = (copy _69, copy _65)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): usize <- ByVal(v32): usize
// CHECK-NEXT:     v33 = stack_addr.i16 ss8
// CHECK-NEXT:     store.i16 notrap v32, v33
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(2) }, None): bool <- ByVal(v34): bool
// CHECK-NEXT:     v35 = stack_addr.i16 ss8+2
// CHECK-NEXT:     store.i8 notrap v34, v35
// CHECK-NEXT: ; _46 = &_47
// CHECK-NEXT:     v36 = stack_addr.i16 ss8
// CHECK-NEXT: ; write_cvalue: Var(_46, var39): &'{erased} (usize, bool) <- ByVal(v36): &'{erased} (usize, bool)
// CHECK-NEXT: ; _48 = &_5
// CHECK-NEXT:     v37 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_48, var40): &'{erased} (usize, bool) <- ByVal(v37): &'{erased} (usize, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _37 = <(usize, bool) as core::cmp::PartialEq>::eq(move _46, move _48)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; _52 = core::intrinsics::ctlz::<u16>(move _67)
// CHECK-NEXT:     v46 = clz.i16 v45
// CHECK-NEXT:     v47 = uextend.i32 v46
// CHECK-NEXT:     v152 -> v47
// CHECK-NEXT: ; write_cvalue: Var(_52, var44): u32 <- ByVal(v47): u32
// CHECK-NEXT:     jump block23
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _53 = core::intrinsics::cttz::<usize>(copy _1)
// CHECK-NEXT:     v49 = ctz.i16 v48
// CHECK-NEXT:     v50 = uextend.i32 v49
// CHECK-NEXT:     v153 -> v50
// CHECK-NEXT: ; write_cvalue: Var(_53, var45): u32 <- ByVal(v50): u32
// CHECK-NEXT:     jump block24
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _54 = core::intrinsics::ctpop::<usize>(move _1)
// CHECK-NEXT:     v52 = popcnt.i16 v51
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
// CHECK-NEXT: block12(v66: i16, v67: i16):
// CHECK-NEXT:     v81 -> v66
// CHECK-NEXT:     v96 -> v66
// CHECK-NEXT:     v171 -> v66
// CHECK-NEXT:     v172 -> v66
// CHECK-NEXT:     v175 -> v66
// CHECK-NEXT:     v176 -> v66
// CHECK-NEXT:     v82 -> v67
// CHECK-NEXT:     v97 -> v67
// CHECK-NEXT:     v173 -> v67
// CHECK-NEXT:     v174 -> v67
// CHECK-NEXT:     v177 -> v67
// CHECK-NEXT:     v178 -> v67
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = discriminant(_14)
// CHECK-NEXT:     v60 = stack_addr.i16 ss3
// CHECK-NEXT:     v61 = load.i16 notrap v60
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): isize <- ByVal(v61): isize
// CHECK-NEXT: ; _59 = Eq(copy _60, const 1_isize)
// CHECK-NEXT:     v62 = iconst.i16 1
// CHECK-NEXT:     v63 = icmp eq v61, v62  ; v62 = 1
// CHECK-NEXT: ; write_cvalue: Var(_59, var51): bool <- ByVal(v63): bool
// CHECK-NEXT: ; _13 = Not(move _59)
// CHECK-NEXT:     v64 = iconst.i8 0
// CHECK-NEXT:     v65 = icmp eq v63, v64  ; v64 = 0
// CHECK-NEXT:     v94 -> v65
// CHECK-NEXT: ; write_cvalue: Var(_13, var11): bool <- ByVal(v65): bool
// CHECK-NEXT: ; _12 = move _13 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): u8 <- ByVal(v65): u8
// CHECK-NEXT: ; _61 = Lt(copy _1, copy _2)
// CHECK-NEXT:     v68 = icmp ult v66, v67
// CHECK-NEXT: ; write_cvalue: Var(_61, var53): bool <- ByVal(v68): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _61)
// CHECK-NEXT:     brif v68, block16, block17
// CHECK-EMPTY:
// CHECK-NEXT: block13 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _58 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block14
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _14 = const core::option::Option::<usize>::None
// CHECK-NEXT:     v69 = symbol_value.i16 gv3
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): core::option::Option<usize> <- ByRef(Pointer { base: Addr(v69), offset: Offset32(0) }, None): core::option::Option<usize>
// CHECK-NEXT:     v70 = load.i16 notrap v69
// CHECK-NEXT:     v71 = load.i16 notrap v69+2
// CHECK-NEXT:     v72 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v70, v72
// CHECK-NEXT:     v73 = stack_addr.i16 ss3+2
// CHECK-NEXT:     store notrap v71, v73
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v167, v168)
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _57 = AddUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v76 = iadd.i16 v74, v75
// CHECK-NEXT: ; write_cvalue: Var(_57, var50): usize <- ByVal(v76): usize
// CHECK-NEXT: ; _14 = core::option::Option::<usize>::Some(move _57)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(2) }, None): usize <- ByVal(v76): usize
// CHECK-NEXT:     v77 = stack_addr.i16 ss3+2
// CHECK-NEXT:     store notrap v76, v77
// CHECK-NEXT:     v78 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u16 <- ByVal(v78): u16
// CHECK-NEXT:     v79 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v78, v79  ; v78 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v74, v75)
// CHECK-EMPTY:
// CHECK-NEXT: block16:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _17 = const false
// CHECK-NEXT:     v80 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_17, var14): bool <- ByVal(v80): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block18(v80)  ; v80 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block17:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _62 = SubUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v83 = isub.i16 v81, v82
// CHECK-NEXT: ; write_cvalue: Var(_62, var54): usize <- ByVal(v83): usize
// CHECK-NEXT: ; _18 = core::option::Option::<usize>::Some(move _62)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(2) }, None): usize <- ByVal(v83): usize
// CHECK-NEXT:     v84 = stack_addr.i16 ss4+2
// CHECK-NEXT:     store notrap v83, v84
// CHECK-NEXT:     v85 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u16 <- ByVal(v85): u16
// CHECK-NEXT:     v86 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v85, v86  ; v85 = 1
// CHECK-NEXT: ; _63 = copy ((_18 as Some).0: usize)
// CHECK-NEXT: ; write_cvalue: Var(_63, var55): usize <- ByRef(Pointer { base: Stack(ss4), offset: Offset32(2) }, None): usize
// CHECK-NEXT:     v87 = stack_addr.i16 ss4+2
// CHECK-NEXT:     v88 = load.i16 notrap v87
// CHECK-NEXT: ; _17 = Eq(move _63, const 65533_usize)
// CHECK-NEXT:     v89 = iconst.i16 -3
// CHECK-NEXT:     v90 = icmp eq v88, v89  ; v89 = -3
// CHECK-NEXT: ; write_cvalue: Var(_17, var14): bool <- ByVal(v90): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block18(v90)
// CHECK-EMPTY:
// CHECK-NEXT: block18(v91: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = move _17 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_16, var13): u8 <- ByVal(v91): u8
// CHECK-NEXT: ; _15 = Shl(move _16, const 1_i32)
// CHECK-NEXT:     v92 = iconst.i32 1
// CHECK-NEXT:     v93 = ishl v91, v92  ; v92 = 1
// CHECK-NEXT: ; write_cvalue: Var(_15, var12): u8 <- ByVal(v93): u8
// CHECK-NEXT: ; _11 = BitOr(move _12, move _15)
// CHECK-NEXT:     v95 = bor.i8 v94, v93
// CHECK-NEXT:     v169 -> v95
// CHECK-NEXT:     v170 -> v95
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): u8 <- ByVal(v95): u8
// CHECK-NEXT: ; _67 = copy _1 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_67, var60): u16 <- ByVal(v96): u16
// CHECK-NEXT: ; _68 = copy _2 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_68, var61): u16 <- ByVal(v97): u16
// CHECK-NEXT: ; _66 = MulWithOverflow(copy _67, copy _68)
// CHECK-NEXT:     v98 = uextend.i32 v96
// CHECK-NEXT:     v99 = uextend.i32 v97
// CHECK-NEXT:     v100 = imul v98, v99
// CHECK-NEXT:     v101 = iconst.i32 0xffff
// CHECK-NEXT:     v102 = icmp ugt v100, v101  ; v101 = 0xffff
// CHECK-NEXT:     v180 -> v102
// CHECK-NEXT:     v181 -> v102
// CHECK-NEXT:     v103 = ireduce.i16 v100
// CHECK-NEXT:     v148 -> v103
// CHECK-NEXT:     v179 -> v103
// CHECK-NEXT: ; write_cvalue: VarPair(_66, var58, var59): (u16, bool) <- ByValPair(v103, v102): (u16, bool)
// CHECK-NEXT: ; _64 = copy (_66.0: u16)
// CHECK-NEXT: ; write_cvalue: Var(_64, var56): u16 <- ByVal(v103): u16
// CHECK-NEXT: ; _65 = copy (_66.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_65, var57): bool <- ByVal(v102): bool
// CHECK-NEXT: ; _69 = copy _64 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_69, var62): usize <- ByVal(v103): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _65)
// CHECK-NEXT:     brif v102, block20, block22
// CHECK-EMPTY:
// CHECK-NEXT: block19(v112: i8, v114: i16, v115: i16, v134: i16, v135: i16, v157: i16, v158: i8):
// CHECK-NEXT:     v161 -> v114
// CHECK-NEXT:     v162 -> v114
// CHECK-NEXT:     v182 -> v114
// CHECK-NEXT:     v183 -> v114
// CHECK-NEXT:     v51 -> v162
// CHECK-NEXT:     v48 -> v161
// CHECK-NEXT:     v22 -> v134
// CHECK-NEXT:     v45 -> v134
// CHECK-NEXT:     v160 -> v134
// CHECK-NEXT:     v23 -> v135
// CHECK-NEXT:     v32 -> v157
// CHECK-NEXT:     v34 -> v158
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _72 = discriminant(_22)
// CHECK-NEXT:     v104 = stack_addr.i16 ss5
// CHECK-NEXT:     v105 = load.i16 notrap v104
// CHECK-NEXT: ; write_cvalue: Var(_72, var64): isize <- ByVal(v105): isize
// CHECK-NEXT: ; _71 = Eq(copy _72, const 1_isize)
// CHECK-NEXT:     v106 = iconst.i16 1
// CHECK-NEXT:     v107 = icmp eq v105, v106  ; v106 = 1
// CHECK-NEXT: ; write_cvalue: Var(_71, var63): bool <- ByVal(v107): bool
// CHECK-NEXT: ; _21 = Not(move _71)
// CHECK-NEXT:     v108 = iconst.i8 0
// CHECK-NEXT:     v109 = icmp eq v107, v108  ; v108 = 0
// CHECK-NEXT: ; write_cvalue: Var(_21, var17): bool <- ByVal(v109): bool
// CHECK-NEXT: ; _20 = move _21 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_20, var16): u8 <- ByVal(v109): u8
// CHECK-NEXT: ; _19 = Shl(move _20, const 2_i32)
// CHECK-NEXT:     v110 = iconst.i32 2
// CHECK-NEXT:     v111 = ishl v109, v110  ; v110 = 2
// CHECK-NEXT: ; write_cvalue: Var(_19, var15): u8 <- ByVal(v111): u8
// CHECK-NEXT: ; _10 = BitOr(move _11, move _19)
// CHECK-NEXT:     v113 = bor v112, v111
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): u8 <- ByVal(v113): u8
// CHECK-NEXT: ; _26 = Add(copy _1, copy _2)
// CHECK-NEXT:     v116 = iadd v114, v115
// CHECK-NEXT: ; write_cvalue: Var(_26, var21): usize <- ByVal(v116): usize
// CHECK-NEXT: ; _25 = Eq(move _26, const 1_usize)
// CHECK-NEXT:     v117 = iconst.i16 1
// CHECK-NEXT:     v118 = icmp eq v116, v117  ; v117 = 1
// CHECK-NEXT: ; write_cvalue: Var(_25, var20): bool <- ByVal(v118): bool
// CHECK-NEXT: ; _24 = move _25 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_24, var19): u8 <- ByVal(v118): u8
// CHECK-NEXT: ; _23 = Shl(move _24, const 3_i32)
// CHECK-NEXT:     v119 = iconst.i32 3
// CHECK-NEXT:     v120 = ishl v118, v119  ; v119 = 3
// CHECK-NEXT: ; write_cvalue: Var(_23, var18): u8 <- ByVal(v120): u8
// CHECK-NEXT: ; _9 = BitOr(move _10, move _23)
// CHECK-NEXT:     v121 = bor v113, v120
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): u8 <- ByVal(v121): u8
// CHECK-NEXT: ; _30 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v122 = isub v114, v115
// CHECK-NEXT: ; write_cvalue: Var(_30, var25): usize <- ByVal(v122): usize
// CHECK-NEXT: ; _29 = Eq(move _30, const 65533_usize)
// CHECK-NEXT:     v123 = iconst.i16 -3
// CHECK-NEXT:     v124 = icmp eq v122, v123  ; v123 = -3
// CHECK-NEXT: ; write_cvalue: Var(_29, var24): bool <- ByVal(v124): bool
// CHECK-NEXT: ; _28 = move _29 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_28, var23): u8 <- ByVal(v124): u8
// CHECK-NEXT: ; _27 = Shl(move _28, const 4_i32)
// CHECK-NEXT:     v125 = iconst.i32 4
// CHECK-NEXT:     v126 = ishl v124, v125  ; v125 = 4
// CHECK-NEXT: ; write_cvalue: Var(_27, var22): u8 <- ByVal(v126): u8
// CHECK-NEXT: ; _8 = BitOr(move _9, move _27)
// CHECK-NEXT:     v127 = bor v121, v126
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): u8 <- ByVal(v127): u8
// CHECK-NEXT: ; _34 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v128 = imul v114, v115
// CHECK-NEXT: ; write_cvalue: Var(_34, var29): usize <- ByVal(v128): usize
// CHECK-NEXT: ; _33 = Eq(move _34, const 65534_usize)
// CHECK-NEXT:     v129 = iconst.i16 -2
// CHECK-NEXT:     v130 = icmp eq v128, v129  ; v129 = -2
// CHECK-NEXT: ; write_cvalue: Var(_33, var28): bool <- ByVal(v130): bool
// CHECK-NEXT: ; _32 = move _33 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_32, var27): u8 <- ByVal(v130): u8
// CHECK-NEXT: ; _31 = Shl(move _32, const 5_i32)
// CHECK-NEXT:     v131 = iconst.i32 5
// CHECK-NEXT:     v132 = ishl v130, v131  ; v131 = 5
// CHECK-NEXT: ; write_cvalue: Var(_31, var26): u8 <- ByVal(v132): u8
// CHECK-NEXT: ; _7 = BitOr(move _8, move _31)
// CHECK-NEXT:     v133 = bor v127, v132
// CHECK-NEXT:     v43 -> v133
// CHECK-NEXT:     v159 -> v133
// CHECK-NEXT: ; write_cvalue: Var(_7, var5): u8 <- ByVal(v133): u8
// CHECK-NEXT: ; _75 = AddWithOverflow(copy _67, copy _68)
// CHECK-NEXT:     v136 = iadd v134, v135
// CHECK-NEXT:     v137 = icmp ult v136, v134
// CHECK-NEXT: ; write_cvalue: VarPair(_75, var67, var68): (u16, bool) <- ByValPair(v136, v137): (u16, bool)
// CHECK-NEXT: ; _73 = copy (_75.0: u16)
// CHECK-NEXT: ; write_cvalue: Var(_73, var65): u16 <- ByVal(v136): u16
// CHECK-NEXT: ; _74 = copy (_75.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_74, var66): bool <- ByVal(v137): bool
// CHECK-NEXT: ; _76 = copy _73 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_76, var69): usize <- ByVal(v136): usize
// CHECK-NEXT: ; _40 = (move _76, copy _74)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): usize <- ByVal(v136): usize
// CHECK-NEXT:     v138 = stack_addr.i16 ss6
// CHECK-NEXT:     store notrap v136, v138
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(2) }, None): bool <- ByVal(v137): bool
// CHECK-NEXT:     v139 = stack_addr.i16 ss6+2
// CHECK-NEXT:     store notrap v137, v139
// CHECK-NEXT: ; _39 = &_40
// CHECK-NEXT:     v140 = stack_addr.i16 ss6
// CHECK-NEXT: ; write_cvalue: Var(_39, var34): &'{erased} (usize, bool) <- ByVal(v140): &'{erased} (usize, bool)
// CHECK-NEXT: ; _41 = &_3
// CHECK-NEXT:     v141 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_41, var35): &'{erased} (usize, bool) <- ByVal(v141): &'{erased} (usize, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _38 = <(usize, bool) as core::cmp::PartialEq>::eq(move _39, move _41)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v142 = call fn2(v140, v141)
// CHECK-NEXT:     v21 -> v142
// CHECK-NEXT: ; write_cvalue: Var(_38, var33): bool <- ByVal(v142): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block20 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _70 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block21
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = const core::option::Option::<usize>::None
// CHECK-NEXT:     v143 = symbol_value.i16 gv4
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): core::option::Option<usize> <- ByRef(Pointer { base: Addr(v143), offset: Offset32(0) }, None): core::option::Option<usize>
// CHECK-NEXT:     v144 = load.i16 notrap v143
// CHECK-NEXT:     v145 = load.i16 notrap v143+2
// CHECK-NEXT:     v146 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v144, v146
// CHECK-NEXT:     v147 = stack_addr.i16 ss5+2
// CHECK-NEXT:     store notrap v145, v147
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block19(v169, v171, v173, v175, v177, v179, v180)
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = core::option::Option::<usize>::Some(copy _69)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(2) }, None): usize <- ByVal(v148): usize
// CHECK-NEXT:     v149 = stack_addr.i16 ss5+2
// CHECK-NEXT:     store.i16 notrap v148, v149
// CHECK-NEXT:     v150 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): u16 <- ByVal(v150): u16
// CHECK-NEXT:     v151 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v150, v151  ; v150 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block19(v170, v172, v174, v176, v178, v148, v181)
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
// CHECK-NEXT: ; _51 = Eq(move _54, const 16_u32)
// CHECK-NEXT:     v155 = iconst.i32 16
// CHECK-NEXT:     v156 = icmp.i32 eq v154, v155  ; v155 = 16
// CHECK-NEXT: ; write_cvalue: Var(_51, var43): bool <- ByVal(v156): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block11(v156, v166)
// CHECK-NEXT: }
