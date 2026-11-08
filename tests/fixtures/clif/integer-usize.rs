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
// CHECK-NEXT: ; symbol _RNvMs9_NtCs40R3J0iFVxM_4core3numj11checked_mulCsa0LCqDJU2Ss_13integer_usize
// CHECK-NEXT: function u0:0(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs9_NtCs40R3J0iFVxM_4core3numj11checked_mulCsa0LCqDJU2Ss_13integer_usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::option::Option<usize>       4b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::option::Option<usize>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    usize                             2b 2                var=1
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=2
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (usize, bool)                     4b 2                var=(5, 6)
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=7
// CHECK-NEXT: ; ssa   _7    u16                               2b 2                var=8
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=9
// CHECK-NEXT: ; ssa   _9    (u16, bool)                       4b 2                var=(10, 11)
// CHECK-NEXT: ; ssa   _10   u16                               2b 2                var=12
// CHECK-NEXT: ; ssa   _11   u16                               2b 2                var=13
// CHECK-NEXT: ; ssa   _12   usize                             2b 2                var=14
// CHECK-NEXT: ; zst   _13   ()                                0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): usize <- ByVal(v0): usize
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = copy _1 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): u16 <- ByVal(v2): u16
// CHECK-NEXT: ; _11 = copy _2 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): u16 <- ByVal(v3): u16
// CHECK-NEXT: ; _9 = MulWithOverflow(move _10, move _11)
// CHECK-NEXT:     v4 = uextend.i32 v2
// CHECK-NEXT:     v5 = uextend.i32 v3
// CHECK-NEXT:     v6 = imul v4, v5
// CHECK-NEXT:     v7 = iconst.i32 0xffff
// CHECK-NEXT:     v8 = icmp ugt v6, v7  ; v7 = 0xffff
// CHECK-NEXT:     v9 = ireduce.i16 v6
// CHECK-NEXT:     v32 -> v9
// CHECK-NEXT:     v33 -> v9
// CHECK-NEXT: ; write_cvalue: VarPair(_9, var10, var11): (u16, bool) <- ByValPair(v9, v8): (u16, bool)
// CHECK-NEXT: ; _7 = copy (_9.0: u16)
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): u16 <- ByVal(v9): u16
// CHECK-NEXT: ; _8 = copy (_9.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): bool <- ByVal(v8): bool
// CHECK-NEXT: ; _12 = copy _7 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_12, var14): usize <- ByVal(v9): usize
// CHECK-NEXT: ; _5 = (move _12, copy _8)
// CHECK-NEXT: ; write_cvalue: Var(_5, var5): usize <- ByVal(v9): usize
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): bool <- ByVal(v8): bool
// CHECK-NEXT: ; _3 = copy (_5.0: usize)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v9): usize
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v8): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _4)
// CHECK-NEXT:     brif v8, block6, block8
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<usize>::None
// CHECK-NEXT:     v10 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u16 <- ByVal(v10): u16
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v10, v11  ; v10 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v12 = stack_addr.i16 ss0
// CHECK-NEXT:     v13 = load.i16 notrap v12
// CHECK-NEXT:     v14 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     return v13, v15
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<usize>::Some(copy _3)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): usize <- ByVal(v16): usize
// CHECK-NEXT:     v17 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store.i16 notrap v16, v17
// CHECK-NEXT:     v18 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u16 <- ByVal(v18): u16
// CHECK-NEXT:     v19 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v18, v19  ; v18 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v20 = stack_addr.i16 ss0
// CHECK-NEXT:     v21 = load.i16 notrap v20
// CHECK-NEXT:     v22 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v23 = load.i16 notrap v22
// CHECK-NEXT:     return v21, v23
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v24 = stack_addr.i16 ss0
// CHECK-NEXT:     v25 = load.i16 notrap v24
// CHECK-NEXT:     v26 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v27 = load.i16 notrap v26
// CHECK-NEXT:     return v25, v27
// CHECK-EMPTY:
// CHECK-NEXT: block5(v28: i8, v31: i16):
// CHECK-NEXT:     v16 -> v31
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     brif v28, block2, block3
// CHECK-EMPTY:
// CHECK-NEXT: block6 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = const true
// CHECK-NEXT:     v29 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v29): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block5(v29, v32)  ; v29 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = const false
// CHECK-NEXT:     v30 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v30): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block5(v30, v33)  ; v30 = 0
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCsa0LCqDJU2Ss_13integer_usize
// CHECK-NEXT: function u0:1() system_v {
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
// CHECK-NEXT: function u0:2(i16, i16) -> i8 uext system_v {
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
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(1:29969 ~ core[..]::cmp::impls::{impl#21}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
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
// CHECK-NEXT: function u0:3(i16, i16) -> i8 uext system_v {
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
// CHECK-NEXT: function u0:4(i16, i16) -> i8 uext system_v {
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
// CHECK-NEXT: function u0:5() -> i8 system_v {
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
// CHECK-NEXT:     fn0 = colocated u0:6 sig0 ; Instance { def: Item(DefId(0:3 ~ integer_usize[..]::integer_methods)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = integer_methods(const core::num::<impl usize>::MAX, const 2_usize)
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
// CHECK-NEXT: function u0:6(i16, i16) -> i8 system_v {
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
// CHECK-NEXT: ; ssa   _5    usize                             2b 2                var=4
// CHECK-NEXT: ; stack _6    (usize, bool)                     4b 2                storage=ss2
// CHECK-NEXT: ; ssa   _7    usize                             2b 2                var=5
// CHECK-NEXT: ; ssa   _8    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=7
// CHECK-NEXT: ; ssa   _10   u8                                1b 1                var=8
// CHECK-NEXT: ; ssa   _11   u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=10
// CHECK-NEXT: ; ssa   _13   u8                                1b 1                var=11
// CHECK-NEXT: ; ssa   _14   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _15   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _16   &'{erased} core::option::Option<usize>    2b 2                var=14
// CHECK-NEXT: ; stack _17   core::option::Option<usize>       4b 2                storage=ss3
// CHECK-NEXT: ; ssa   _18   u8                                1b 1                var=15
// CHECK-NEXT: ; ssa   _19   u8                                1b 1                var=16
// CHECK-NEXT: ; ssa   _20   bool                              1b 1                var=17
// CHECK-NEXT: ; ssa   _21   &'{erased} core::option::Option<usize>    2b 2                var=18
// CHECK-NEXT: ; stack _22   core::option::Option<usize>       4b 2                storage=ss4
// CHECK-NEXT: ; ssa   _23   &'{erased} core::option::Option<usize>    2b 2                var=19
// CHECK-NEXT: ; ssa   _24   u8                                1b 1                var=20
// CHECK-NEXT: ; ssa   _25   u8                                1b 1                var=21
// CHECK-NEXT: ; ssa   _26   bool                              1b 1                var=22
// CHECK-NEXT: ; ssa   _27   &'{erased} core::option::Option<usize>    2b 2                var=23
// CHECK-NEXT: ; stack _28   core::option::Option<usize>       4b 2                storage=ss5
// CHECK-NEXT: ; ssa   _29   u8                                1b 1                var=24
// CHECK-NEXT: ; ssa   _30   u8                                1b 1                var=25
// CHECK-NEXT: ; ssa   _31   bool                              1b 1                var=26
// CHECK-NEXT: ; ssa   _32   usize                             2b 2                var=27
// CHECK-NEXT: ; ssa   _33   usize                             2b 2                var=28
// CHECK-NEXT: ; ssa   _34   u8                                1b 1                var=29
// CHECK-NEXT: ; ssa   _35   u8                                1b 1                var=30
// CHECK-NEXT: ; ssa   _36   bool                              1b 1                var=31
// CHECK-NEXT: ; ssa   _37   usize                             2b 2                var=32
// CHECK-NEXT: ; ssa   _38   usize                             2b 2                var=33
// CHECK-NEXT: ; ssa   _39   u8                                1b 1                var=34
// CHECK-NEXT: ; ssa   _40   u8                                1b 1                var=35
// CHECK-NEXT: ; ssa   _41   bool                              1b 1                var=36
// CHECK-NEXT: ; ssa   _42   usize                             2b 2                var=37
// CHECK-NEXT: ; ssa   _43   usize                             2b 2                var=38
// CHECK-NEXT: ; ssa   _44   u8                                1b 1                var=39
// CHECK-NEXT: ; ssa   _45   u8                                1b 1                var=40
// CHECK-NEXT: ; ssa   _46   bool                              1b 1                var=41
// CHECK-NEXT: ; ssa   _47   bool                              1b 1                var=42
// CHECK-NEXT: ; ssa   _48   &'{erased} (usize, bool)          2b 2                var=43
// CHECK-NEXT: ; stack _49   (usize, bool)                     4b 2                storage=ss6
// CHECK-NEXT: ; ssa   _50   &'{erased} (usize, bool)          2b 2                var=44
// CHECK-NEXT: ; ssa   _51   bool                              1b 1                var=45
// CHECK-NEXT: ; ssa   _52   &'{erased} (usize, bool)          2b 2                var=46
// CHECK-NEXT: ; stack _53   (usize, bool)                     4b 2                storage=ss7
// CHECK-NEXT: ; ssa   _54   &'{erased} (usize, bool)          2b 2                var=47
// CHECK-NEXT: ; ssa   _55   &'{erased} (usize, bool)          2b 2                var=48
// CHECK-NEXT: ; stack _56   (usize, bool)                     4b 2                storage=ss8
// CHECK-NEXT: ; ssa   _57   &'{erased} (usize, bool)          2b 2                var=49
// CHECK-NEXT: ; ssa   _58   u8                                1b 1                var=50
// CHECK-NEXT: ; ssa   _59   u8                                1b 1                var=51
// CHECK-NEXT: ; ssa   _60   bool                              1b 1                var=52
// CHECK-NEXT: ; ssa   _61   bool                              1b 1                var=53
// CHECK-NEXT: ; ssa   _62   u32                               4b 2                var=54
// CHECK-NEXT: ; ssa   _63   bool                              1b 1                var=55
// CHECK-NEXT: ; ssa   _64   u32                               4b 2                var=56
// CHECK-NEXT: ; ssa   _65   u32                               4b 2                var=57
// CHECK-NEXT: ; ssa   _66   bool                              1b 1                var=58
// CHECK-NEXT: ; ssa   _67   bool                              1b 1                var=59
// CHECK-NEXT: ; ssa   _68   (usize, bool)                     4b 2                var=(60, 61)
// CHECK-NEXT: ; ssa   _69   usize                             2b 2                var=62
// CHECK-NEXT: ; zst   _70   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _71   bool                              1b 1                var=63
// CHECK-NEXT: ; ssa   _72   isize                             2b 2                var=64
// CHECK-NEXT: ; ssa   _73   bool                              1b 1                var=65
// CHECK-NEXT: ; ssa   _74   usize                             2b 2                var=66
// CHECK-NEXT: ; ssa   _75   bool                              1b 1                var=67
// CHECK-NEXT: ; ssa   _76   isize                             2b 2                var=68
// CHECK-NEXT: ; ssa   _77   u16                               2b 2                var=69
// CHECK-NEXT: ; ssa   _78   bool                              1b 1                var=70
// CHECK-NEXT: ; ssa   _79   (u16, bool)                       4b 2                var=(71, 72)
// CHECK-NEXT: ; ssa   _80   u16                               2b 2                var=73
// CHECK-NEXT: ; ssa   _81   u16                               2b 2                var=74
// CHECK-NEXT: ; ssa   _82   usize                             2b 2                var=75
// CHECK-NEXT: ; ssa   _83   u16                               2b 2                var=76
// CHECK-NEXT: ; ssa   _84   bool                              1b 1                var=77
// CHECK-NEXT: ; ssa   _85   (u16, bool)                       4b 2                var=(78, 79)
// CHECK-NEXT: ; ssa   _86   u16                               2b 2                var=80
// CHECK-NEXT: ; ssa   _87   u16                               2b 2                var=81
// CHECK-NEXT: ; ssa   _88   usize                             2b 2                var=82
// CHECK-NEXT: ; ssa   _89   u16                               2b 2                var=83
// CHECK-NEXT: ; ssa   _90   bool                              1b 1                var=84
// CHECK-NEXT: ; ssa   _91   (u16, bool)                       4b 2                var=(85, 86)
// CHECK-NEXT: ; ssa   _92   u16                               2b 2                var=87
// CHECK-NEXT: ; ssa   _93   u16                               2b 2                var=88
// CHECK-NEXT: ; ssa   _94   usize                             2b 2                var=89
// CHECK-NEXT: ; ssa   _95   u16                               2b 2                var=90
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
// CHECK-NEXT:     gv0 = symbol colocated userextname2 ; alloc3
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig4 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:0 sig0 ; Instance { def: Item(DefId(1:28338 ~ core[..]::num::{impl#11}::checked_mul)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:2 sig1 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [usize, bool] }
// CHECK-NEXT:     fn2 = colocated u0:2 sig2 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [usize, bool] }
// CHECK-NEXT:     fn3 = u0:7 sig3 ; Instance { def: Item(DefId(1:11950 ~ core[..]::option::{impl#17}::eq)), args: [usize] }
// CHECK-NEXT:     fn4 = colocated u0:2 sig4 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [usize, bool] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v18 -> v0
// CHECK-NEXT:     v175 -> v0
// CHECK-NEXT:     v176 -> v0
// CHECK-NEXT:     v19 -> v1
// CHECK-NEXT:     v177 -> v1
// CHECK-NEXT:     v178 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): usize <- ByVal(v0): usize
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (const 1_usize, const true)
// CHECK-NEXT:     v2 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): usize <- ByVal(v2): usize
// CHECK-NEXT:     v3 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v2, v3  ; v2 = 1
// CHECK-NEXT:     v4 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): bool <- ByVal(v4): bool
// CHECK-NEXT:     v5 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v4, v5  ; v4 = 1
// CHECK-NEXT: ; _5 = Sub(const core::num::<impl usize>::MAX, const 2_usize)
// CHECK-NEXT:     v6 = iconst.i16 -1
// CHECK-NEXT:     v7 = iconst.i16 2
// CHECK-NEXT:     v8 = isub v6, v7  ; v6 = -1, v7 = 2
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): usize <- ByVal(v8): usize
// CHECK-NEXT: ; _4 = (move _5, const false)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): usize <- ByVal(v8): usize
// CHECK-NEXT:     v9 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v8, v9
// CHECK-NEXT:     v10 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(2) }, None): bool <- ByVal(v10): bool
// CHECK-NEXT:     v11 = stack_addr.i16 ss1+2
// CHECK-NEXT:     store notrap v10, v11  ; v10 = 0
// CHECK-NEXT: ; _7 = Sub(const core::num::<impl usize>::MAX, const 1_usize)
// CHECK-NEXT:     v12 = iconst.i16 -1
// CHECK-NEXT:     v13 = iconst.i16 1
// CHECK-NEXT:     v14 = isub v12, v13  ; v12 = -1, v13 = 1
// CHECK-NEXT: ; write_cvalue: Var(_7, var5): usize <- ByVal(v14): usize
// CHECK-NEXT: ; _6 = (move _7, const true)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): usize <- ByVal(v14): usize
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v14, v15
// CHECK-NEXT:     v16 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(2) }, None): bool <- ByVal(v16): bool
// CHECK-NEXT:     v17 = stack_addr.i16 ss2+2
// CHECK-NEXT:     store notrap v16, v17  ; v16 = 1
// CHECK-NEXT: ; _68 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v20 = iadd.i16 v18, v19
// CHECK-NEXT:     v21 = icmp ult v20, v18
// CHECK-NEXT: ; write_cvalue: VarPair(_68, var60, var61): (usize, bool) <- ByValPair(v20, v21): (usize, bool)
// CHECK-NEXT: ; _67 = copy (_68.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_67, var59): bool <- ByVal(v21): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _67)
// CHECK-NEXT:     brif v21, block18, block20
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _19 = move _20 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_19, var16): u8 <- ByVal(v22): u8
// CHECK-NEXT: ; _18 = Shl(move _19, const 1_i32)
// CHECK-NEXT:     v23 = iconst.i32 1
// CHECK-NEXT:     v24 = ishl.i8 v22, v23  ; v23 = 1
// CHECK-NEXT: ; write_cvalue: Var(_18, var15): u8 <- ByVal(v24): u8
// CHECK-NEXT: ; _13 = BitOr(move _14, move _18)
// CHECK-NEXT:     v26 = bor.i8 v25, v24
// CHECK-NEXT:     v189 -> v26
// CHECK-NEXT:     v190 -> v26
// CHECK-NEXT: ; write_cvalue: Var(_13, var11): u8 <- ByVal(v26): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _28 = core::num::<impl usize>::checked_mul(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<usize>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u16, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u16, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u16 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(2 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(4 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u16, b_offset: Size(2 bytes) }, field_offsets: [Size(2 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8189316934619006962 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v29, v30 = call fn0(v27, v28)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): core::option::Option<usize> <- ByValPair(v29, v30): core::option::Option<usize>
// CHECK-NEXT:     v31 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v29, v31
// CHECK-NEXT:     v32 = stack_addr.i16 ss5+2
// CHECK-NEXT:     store notrap v30, v32
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _27 = &_28
// CHECK-NEXT:     v33 = stack_addr.i16 ss5
// CHECK-NEXT: ; write_cvalue: Var(_27, var23): &'{erased} core::option::Option<usize> <- ByVal(v33): &'{erased} core::option::Option<usize>
// CHECK-NEXT: ; _76 = discriminant((*_27))
// CHECK-NEXT:     v34 = load.i16 notrap v33
// CHECK-NEXT: ; write_cvalue: Var(_76, var68): isize <- ByVal(v34): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _76)
// CHECK-NEXT:     v35 = uextend.i32 v34
// CHECK-NEXT:     br_table v35, block24, [block29, block30]
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _47)
// CHECK-NEXT:     brif.i8 v36, block5, block8
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _86 = copy _1 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_86, var80): u16 <- ByVal(v37): u16
// CHECK-NEXT: ; _87 = copy _2 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_87, var81): u16 <- ByVal(v38): u16
// CHECK-NEXT: ; _85 = SubWithOverflow(move _86, move _87)
// CHECK-NEXT:     v39 = isub.i16 v37, v38
// CHECK-NEXT:     v40 = icmp ugt v39, v37
// CHECK-NEXT: ; write_cvalue: VarPair(_85, var78, var79): (u16, bool) <- ByValPair(v39, v40): (u16, bool)
// CHECK-NEXT: ; _83 = copy (_85.0: u16)
// CHECK-NEXT: ; write_cvalue: Var(_83, var76): u16 <- ByVal(v39): u16
// CHECK-NEXT: ; _84 = copy (_85.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_84, var77): bool <- ByVal(v40): bool
// CHECK-NEXT: ; _88 = copy _83 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_88, var82): usize <- ByVal(v39): usize
// CHECK-NEXT: ; _53 = (move _88, copy _84)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): usize <- ByVal(v39): usize
// CHECK-NEXT:     v41 = stack_addr.i16 ss7
// CHECK-NEXT:     store notrap v39, v41
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(2) }, None): bool <- ByVal(v40): bool
// CHECK-NEXT:     v42 = stack_addr.i16 ss7+2
// CHECK-NEXT:     store notrap v40, v42
// CHECK-NEXT: ; _52 = &_53
// CHECK-NEXT:     v43 = stack_addr.i16 ss7
// CHECK-NEXT: ; write_cvalue: Var(_52, var46): &'{erased} (usize, bool) <- ByVal(v43): &'{erased} (usize, bool)
// CHECK-NEXT: ; _54 = &_4
// CHECK-NEXT:     v44 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_54, var47): &'{erased} (usize, bool) <- ByVal(v44): &'{erased} (usize, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _51 = <(usize, bool) as core::cmp::PartialEq>::eq(move _52, move _54)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v45 = call fn1(v43, v44)
// CHECK-NEXT:     v46 -> v45
// CHECK-NEXT: ; write_cvalue: Var(_51, var45): bool <- ByVal(v45): bool
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _51)
// CHECK-NEXT:     brif.i8 v46, block7, block8
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _92 = copy _1 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_92, var87): u16 <- ByVal(v47): u16
// CHECK-NEXT: ; _93 = copy _2 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_93, var88): u16 <- ByVal(v48): u16
// CHECK-NEXT: ; _91 = MulWithOverflow(move _92, move _93)
// CHECK-NEXT:     v49 = uextend.i32 v47
// CHECK-NEXT:     v50 = uextend.i32 v48
// CHECK-NEXT:     v51 = imul v49, v50
// CHECK-NEXT:     v52 = iconst.i32 0xffff
// CHECK-NEXT:     v53 = icmp ugt v51, v52  ; v52 = 0xffff
// CHECK-NEXT:     v54 = ireduce.i16 v51
// CHECK-NEXT: ; write_cvalue: VarPair(_91, var85, var86): (u16, bool) <- ByValPair(v54, v53): (u16, bool)
// CHECK-NEXT: ; _89 = copy (_91.0: u16)
// CHECK-NEXT: ; write_cvalue: Var(_89, var83): u16 <- ByVal(v54): u16
// CHECK-NEXT: ; _90 = copy (_91.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_90, var84): bool <- ByVal(v53): bool
// CHECK-NEXT: ; _94 = copy _89 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_94, var89): usize <- ByVal(v54): usize
// CHECK-NEXT: ; _56 = (move _94, copy _90)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): usize <- ByVal(v54): usize
// CHECK-NEXT:     v55 = stack_addr.i16 ss8
// CHECK-NEXT:     store notrap v54, v55
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(2) }, None): bool <- ByVal(v53): bool
// CHECK-NEXT:     v56 = stack_addr.i16 ss8+2
// CHECK-NEXT:     store notrap v53, v56
// CHECK-NEXT: ; _55 = &_56
// CHECK-NEXT:     v57 = stack_addr.i16 ss8
// CHECK-NEXT: ; write_cvalue: Var(_55, var48): &'{erased} (usize, bool) <- ByVal(v57): &'{erased} (usize, bool)
// CHECK-NEXT: ; _57 = &_6
// CHECK-NEXT:     v58 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_57, var49): &'{erased} (usize, bool) <- ByVal(v58): &'{erased} (usize, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _46 = <(usize, bool) as core::cmp::PartialEq>::eq(move _55, move _57)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v59 = call fn2(v57, v58)
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v59): bool
// CHECK-NEXT:     jump block9(v59)
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _46 = const false
// CHECK-NEXT:     v60 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v60): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block9(v60)  ; v60 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block9(v61: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _45 = move _46 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_45, var40): u8 <- ByVal(v61): u8
// CHECK-NEXT: ; _44 = Shl(move _45, const 6_i32)
// CHECK-NEXT:     v62 = iconst.i32 6
// CHECK-NEXT:     v63 = ishl v61, v62  ; v62 = 6
// CHECK-NEXT: ; write_cvalue: Var(_44, var39): u8 <- ByVal(v63): u8
// CHECK-NEXT: ; _8 = BitOr(move _9, move _44)
// CHECK-NEXT:     v65 = bor.i8 v64, v63
// CHECK-NEXT:     v170 -> v65
// CHECK-NEXT:     v171 -> v65
// CHECK-NEXT:     v172 -> v65
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): u8 <- ByVal(v65): u8
// CHECK-NEXT: ; _95 = copy _1 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_95, var90): u16 <- ByVal(v66): u16
// CHECK-NEXT: ;
// CHECK-NEXT: ; _62 = core::intrinsics::ctlz::<u16>(move _95)
// CHECK-NEXT:     v67 = clz.i16 v66
// CHECK-NEXT:     v68 = uextend.i32 v67
// CHECK-NEXT:     v153 -> v68
// CHECK-NEXT: ; write_cvalue: Var(_62, var54): u32 <- ByVal(v68): u32
// CHECK-NEXT:     jump block31
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _64 = core::intrinsics::cttz::<usize>(copy _1)
// CHECK-NEXT:     v70 = ctz.i16 v69
// CHECK-NEXT:     v71 = uextend.i32 v70
// CHECK-NEXT:     v156 -> v71
// CHECK-NEXT: ; write_cvalue: Var(_64, var56): u32 <- ByVal(v71): u32
// CHECK-NEXT:     jump block32
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _65 = core::intrinsics::ctpop::<usize>(copy _1)
// CHECK-NEXT:     v73 = popcnt.i16 v72
// CHECK-NEXT:     v74 = uextend.i32 v73
// CHECK-NEXT:     v159 -> v74
// CHECK-NEXT: ; write_cvalue: Var(_65, var57): u32 <- ByVal(v74): u32
// CHECK-NEXT:     jump block33
// CHECK-EMPTY:
// CHECK-NEXT: block12(v169: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = const false
// CHECK-NEXT:     v75 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v75): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block13(v75, v169)  ; v75 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block13(v76: i8, v79: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _59 = move _60 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_59, var51): u8 <- ByVal(v76): u8
// CHECK-NEXT: ; _58 = Shl(move _59, const 7_i32)
// CHECK-NEXT:     v77 = iconst.i32 7
// CHECK-NEXT:     v78 = ishl v76, v77  ; v77 = 7
// CHECK-NEXT: ; write_cvalue: Var(_58, var50): u8 <- ByVal(v78): u8
// CHECK-NEXT: ; _0 = BitOr(move _8, move _58)
// CHECK-NEXT:     v80 = bor v79, v78
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v80): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v80
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = &_17
// CHECK-NEXT:     v81 = stack_addr.i16 ss3
// CHECK-NEXT: ; write_cvalue: Var(_16, var14): &'{erased} core::option::Option<usize> <- ByVal(v81): &'{erased} core::option::Option<usize>
// CHECK-NEXT: ; _72 = discriminant((*_16))
// CHECK-NEXT:     v82 = load.i16 notrap v81
// CHECK-NEXT: ; write_cvalue: Var(_72, var64): isize <- ByVal(v82): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _72)
// CHECK-NEXT:     v83 = uextend.i32 v82
// CHECK-NEXT:     br_table v83, block24, [block22, block23]
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _17 = core::option::Option::<usize>::None
// CHECK-NEXT:     v84 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u16 <- ByVal(v84): u16
// CHECK-NEXT:     v85 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v84, v85  ; v84 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block14
// CHECK-EMPTY:
// CHECK-NEXT: block16:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _69 = AddUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v88 = iadd.i16 v86, v87
// CHECK-NEXT: ; write_cvalue: Var(_69, var62): usize <- ByVal(v88): usize
// CHECK-NEXT: ; _17 = core::option::Option::<usize>::Some(move _69)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(2) }, None): usize <- ByVal(v88): usize
// CHECK-NEXT:     v89 = stack_addr.i16 ss3+2
// CHECK-NEXT:     store notrap v88, v89
// CHECK-NEXT:     v90 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u16 <- ByVal(v90): u16
// CHECK-NEXT:     v91 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v90, v91  ; v90 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block14
// CHECK-EMPTY:
// CHECK-NEXT: block17(v92: i8, v173: i16, v174: i16):
// CHECK-NEXT:     v86 -> v173
// CHECK-NEXT:     v179 -> v173
// CHECK-NEXT:     v180 -> v173
// CHECK-NEXT:     v183 -> v173
// CHECK-NEXT:     v87 -> v174
// CHECK-NEXT:     v181 -> v174
// CHECK-NEXT:     v182 -> v174
// CHECK-NEXT:     v184 -> v174
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _66)
// CHECK-NEXT:     brif v92, block15, block16
// CHECK-EMPTY:
// CHECK-NEXT: block18 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _70 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block19
// CHECK-EMPTY:
// CHECK-NEXT: block19:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _66 = const true
// CHECK-NEXT:     v93 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_66, var58): bool <- ByVal(v93): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block17(v93, v175, v177)  ; v93 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block20:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _66 = const false
// CHECK-NEXT:     v94 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_66, var58): bool <- ByVal(v94): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block17(v94, v176, v178)  ; v94 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block21(v95: i8, v98: i16, v99: i16):
// CHECK-NEXT:     v108 -> v98
// CHECK-NEXT:     v187 -> v98
// CHECK-NEXT:     v109 -> v99
// CHECK-NEXT:     v188 -> v99
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = Not(move _71)
// CHECK-NEXT:     v96 = iconst.i8 0
// CHECK-NEXT:     v97 = icmp eq v95, v96  ; v96 = 0
// CHECK-NEXT:     v185 -> v97
// CHECK-NEXT:     v186 -> v97
// CHECK-NEXT: ; write_cvalue: Var(_15, var13): bool <- ByVal(v97): bool
// CHECK-NEXT: ; _14 = move _15 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_14, var12): u8 <- ByVal(v97): u8
// CHECK-NEXT: ; _73 = Lt(copy _1, copy _2)
// CHECK-NEXT:     v100 = icmp ult v98, v99
// CHECK-NEXT: ; write_cvalue: Var(_73, var65): bool <- ByVal(v100): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _73)
// CHECK-NEXT:     brif v100, block26, block27
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _71 = const false
// CHECK-NEXT:     v101 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_71, var63): bool <- ByVal(v101): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block21(v101, v179, v181)  ; v101 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block23:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _71 = const true
// CHECK-NEXT:     v102 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_71, var63): bool <- ByVal(v102): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block21(v102, v180, v182)  ; v102 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block24 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; unreachable
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block25(v162: i8, v163: i16, v164: i16):
// CHECK-NEXT:     v25 -> v162
// CHECK-NEXT:     v27 -> v163
// CHECK-NEXT:     v191 -> v163
// CHECK-NEXT:     v192 -> v163
// CHECK-NEXT:     v28 -> v164
// CHECK-NEXT:     v193 -> v164
// CHECK-NEXT:     v194 -> v164
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _21 = &_22
// CHECK-NEXT:     v103 = stack_addr.i16 ss4
// CHECK-NEXT: ; write_cvalue: Var(_21, var18): &'{erased} core::option::Option<usize> <- ByVal(v103): &'{erased} core::option::Option<usize>
// CHECK-NEXT: ; _23 = const integer_methods::promoted[0]
// CHECK-NEXT:     v104 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_23, var19): &'{erased} core::option::Option<usize> <- ByVal(v104): &'{erased} core::option::Option<usize>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _20 = <core::option::Option<usize> as core::cmp::PartialEq>::eq(move _21, move _23)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<usize>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &core::option::Option<usize>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v105 = call fn3(v103, v104)
// CHECK-NEXT:     v22 -> v105
// CHECK-NEXT: ; write_cvalue: Var(_20, var17): bool <- ByVal(v105): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block26:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = core::option::Option::<usize>::None
// CHECK-NEXT:     v106 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u16 <- ByVal(v106): u16
// CHECK-NEXT:     v107 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v106, v107  ; v106 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block25(v185, v187, v188)
// CHECK-EMPTY:
// CHECK-NEXT: block27:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _74 = SubUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v110 = isub.i16 v108, v109
// CHECK-NEXT: ; write_cvalue: Var(_74, var66): usize <- ByVal(v110): usize
// CHECK-NEXT: ; _22 = core::option::Option::<usize>::Some(move _74)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(2) }, None): usize <- ByVal(v110): usize
// CHECK-NEXT:     v111 = stack_addr.i16 ss4+2
// CHECK-NEXT:     store notrap v110, v111
// CHECK-NEXT:     v112 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u16 <- ByVal(v112): u16
// CHECK-NEXT:     v113 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v112, v113  ; v112 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block25(v186, v108, v109)
// CHECK-EMPTY:
// CHECK-NEXT: block28(v114: i8, v119: i8, v121: i16, v122: i16):
// CHECK-NEXT:     v37 -> v121
// CHECK-NEXT:     v47 -> v121
// CHECK-NEXT:     v66 -> v121
// CHECK-NEXT:     v166 -> v121
// CHECK-NEXT:     v167 -> v121
// CHECK-NEXT:     v168 -> v121
// CHECK-NEXT:     v72 -> v168
// CHECK-NEXT:     v69 -> v167
// CHECK-NEXT:     v38 -> v122
// CHECK-NEXT:     v48 -> v122
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _26 = Not(move _75)
// CHECK-NEXT:     v115 = iconst.i8 0
// CHECK-NEXT:     v116 = icmp eq v114, v115  ; v115 = 0
// CHECK-NEXT: ; write_cvalue: Var(_26, var22): bool <- ByVal(v116): bool
// CHECK-NEXT: ; _25 = move _26 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_25, var21): u8 <- ByVal(v116): u8
// CHECK-NEXT: ; _24 = Shl(move _25, const 2_i32)
// CHECK-NEXT:     v117 = iconst.i32 2
// CHECK-NEXT:     v118 = ishl v116, v117  ; v117 = 2
// CHECK-NEXT: ; write_cvalue: Var(_24, var20): u8 <- ByVal(v118): u8
// CHECK-NEXT: ; _12 = BitOr(move _13, move _24)
// CHECK-NEXT:     v120 = bor v119, v118
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): u8 <- ByVal(v120): u8
// CHECK-NEXT: ; _32 = Add(copy _1, copy _2)
// CHECK-NEXT:     v123 = iadd v121, v122
// CHECK-NEXT: ; write_cvalue: Var(_32, var27): usize <- ByVal(v123): usize
// CHECK-NEXT: ; _33 = copy (_3.0: usize)
// CHECK-NEXT: ; write_cvalue: Var(_33, var28): usize <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): usize
// CHECK-NEXT:     v124 = stack_addr.i16 ss0
// CHECK-NEXT:     v125 = load.i16 notrap v124
// CHECK-NEXT: ; _31 = Eq(move _32, move _33)
// CHECK-NEXT:     v126 = icmp eq v123, v125
// CHECK-NEXT: ; write_cvalue: Var(_31, var26): bool <- ByVal(v126): bool
// CHECK-NEXT: ; _30 = move _31 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_30, var25): u8 <- ByVal(v126): u8
// CHECK-NEXT: ; _29 = Shl(move _30, const 3_i32)
// CHECK-NEXT:     v127 = iconst.i32 3
// CHECK-NEXT:     v128 = ishl v126, v127  ; v127 = 3
// CHECK-NEXT: ; write_cvalue: Var(_29, var24): u8 <- ByVal(v128): u8
// CHECK-NEXT: ; _11 = BitOr(move _12, move _29)
// CHECK-NEXT:     v129 = bor v120, v128
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): u8 <- ByVal(v129): u8
// CHECK-NEXT: ; _37 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v130 = isub v121, v122
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): usize <- ByVal(v130): usize
// CHECK-NEXT: ; _38 = copy (_4.0: usize)
// CHECK-NEXT: ; write_cvalue: Var(_38, var33): usize <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): usize
// CHECK-NEXT:     v131 = stack_addr.i16 ss1
// CHECK-NEXT:     v132 = load.i16 notrap v131
// CHECK-NEXT: ; _36 = Eq(move _37, move _38)
// CHECK-NEXT:     v133 = icmp eq v130, v132
// CHECK-NEXT: ; write_cvalue: Var(_36, var31): bool <- ByVal(v133): bool
// CHECK-NEXT: ; _35 = move _36 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_35, var30): u8 <- ByVal(v133): u8
// CHECK-NEXT: ; _34 = Shl(move _35, const 4_i32)
// CHECK-NEXT:     v134 = iconst.i32 4
// CHECK-NEXT:     v135 = ishl v133, v134  ; v134 = 4
// CHECK-NEXT: ; write_cvalue: Var(_34, var29): u8 <- ByVal(v135): u8
// CHECK-NEXT: ; _10 = BitOr(move _11, move _34)
// CHECK-NEXT:     v136 = bor v129, v135
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): u8 <- ByVal(v136): u8
// CHECK-NEXT: ; _42 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v137 = imul v121, v122
// CHECK-NEXT: ; write_cvalue: Var(_42, var37): usize <- ByVal(v137): usize
// CHECK-NEXT: ; _43 = copy (_6.0: usize)
// CHECK-NEXT: ; write_cvalue: Var(_43, var38): usize <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): usize
// CHECK-NEXT:     v138 = stack_addr.i16 ss2
// CHECK-NEXT:     v139 = load.i16 notrap v138
// CHECK-NEXT: ; _41 = Eq(move _42, move _43)
// CHECK-NEXT:     v140 = icmp eq v137, v139
// CHECK-NEXT: ; write_cvalue: Var(_41, var36): bool <- ByVal(v140): bool
// CHECK-NEXT: ; _40 = move _41 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_40, var35): u8 <- ByVal(v140): u8
// CHECK-NEXT: ; _39 = Shl(move _40, const 5_i32)
// CHECK-NEXT:     v141 = iconst.i32 5
// CHECK-NEXT:     v142 = ishl v140, v141  ; v141 = 5
// CHECK-NEXT: ; write_cvalue: Var(_39, var34): u8 <- ByVal(v142): u8
// CHECK-NEXT: ; _9 = BitOr(move _10, move _39)
// CHECK-NEXT:     v143 = bor v136, v142
// CHECK-NEXT:     v64 -> v143
// CHECK-NEXT:     v165 -> v143
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): u8 <- ByVal(v143): u8
// CHECK-NEXT: ; _80 = copy _1 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_80, var73): u16 <- ByVal(v121): u16
// CHECK-NEXT: ; _81 = copy _2 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_81, var74): u16 <- ByVal(v122): u16
// CHECK-NEXT: ; _79 = AddWithOverflow(move _80, move _81)
// CHECK-NEXT:     v144 = iadd v121, v122
// CHECK-NEXT:     v145 = icmp ult v144, v121
// CHECK-NEXT: ; write_cvalue: VarPair(_79, var71, var72): (u16, bool) <- ByValPair(v144, v145): (u16, bool)
// CHECK-NEXT: ; _77 = copy (_79.0: u16)
// CHECK-NEXT: ; write_cvalue: Var(_77, var69): u16 <- ByVal(v144): u16
// CHECK-NEXT: ; _78 = copy (_79.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_78, var70): bool <- ByVal(v145): bool
// CHECK-NEXT: ; _82 = copy _77 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_82, var75): usize <- ByVal(v144): usize
// CHECK-NEXT: ; _49 = (move _82, copy _78)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): usize <- ByVal(v144): usize
// CHECK-NEXT:     v146 = stack_addr.i16 ss6
// CHECK-NEXT:     store notrap v144, v146
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(2) }, None): bool <- ByVal(v145): bool
// CHECK-NEXT:     v147 = stack_addr.i16 ss6+2
// CHECK-NEXT:     store notrap v145, v147
// CHECK-NEXT: ; _48 = &_49
// CHECK-NEXT:     v148 = stack_addr.i16 ss6
// CHECK-NEXT: ; write_cvalue: Var(_48, var43): &'{erased} (usize, bool) <- ByVal(v148): &'{erased} (usize, bool)
// CHECK-NEXT: ; _50 = &_3
// CHECK-NEXT:     v149 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_50, var44): &'{erased} (usize, bool) <- ByVal(v149): &'{erased} (usize, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _47 = <(usize, bool) as core::cmp::PartialEq>::eq(move _48, move _50)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(usize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v150 = call fn4(v148, v149)
// CHECK-NEXT:     v36 -> v150
// CHECK-NEXT: ; write_cvalue: Var(_47, var42): bool <- ByVal(v150): bool
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block29:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _75 = const false
// CHECK-NEXT:     v151 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_75, var67): bool <- ByVal(v151): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block28(v151, v189, v191, v193)  ; v151 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block30:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _75 = const true
// CHECK-NEXT:     v152 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_75, var67): bool <- ByVal(v152): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block28(v152, v190, v192, v194)  ; v152 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block31:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _61 = Eq(move _62, const 0_u32)
// CHECK-NEXT:     v154 = iconst.i32 0
// CHECK-NEXT:     v155 = icmp.i32 eq v153, v154  ; v154 = 0
// CHECK-NEXT: ; write_cvalue: Var(_61, var53): bool <- ByVal(v155): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _61)
// CHECK-NEXT:     brif.i32 v153, block12(v170), block10
// CHECK-EMPTY:
// CHECK-NEXT: block32:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _63 = Eq(move _64, const 0_u32)
// CHECK-NEXT:     v157 = iconst.i32 0
// CHECK-NEXT:     v158 = icmp.i32 eq v156, v157  ; v157 = 0
// CHECK-NEXT: ; write_cvalue: Var(_63, var55): bool <- ByVal(v158): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _63)
// CHECK-NEXT:     brif.i32 v156, block12(v171), block11
// CHECK-EMPTY:
// CHECK-NEXT: block33:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = Eq(move _65, const 16_u32)
// CHECK-NEXT:     v160 = iconst.i32 16
// CHECK-NEXT:     v161 = icmp.i32 eq v159, v160  ; v160 = 16
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v161): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block13(v161, v172)
// CHECK-NEXT: }
