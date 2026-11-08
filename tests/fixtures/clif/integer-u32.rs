#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn integer_methods(x: u32, y: u32) -> u8 {
    let add = (1, true);
    let sub = (u32::MAX - 2, false);
    let mul = (u32::MAX - 1, true);
    (x.checked_add(y).is_none() as u8)
        | ((x.checked_sub(y) == Some(u32::MAX - 2)) as u8) << 1
        | (x.checked_mul(y).is_none() as u8) << 2
        | ((x.wrapping_add(y) == add.0) as u8) << 3
        | ((x.wrapping_sub(y) == sub.0) as u8) << 4
        | ((x.wrapping_mul(y) == mul.0) as u8) << 5
        | ((x.overflowing_add(y) == add
            && x.overflowing_sub(y) == sub
            && x.overflowing_mul(y) == mul) as u8) << 6
        | ((x.leading_zeros() == 0
            && x.trailing_zeros() == 0
            && x.count_ones() == 32) as u8) << 7
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    (integer_methods(u32::MAX, 2) == 255) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCsj0cxGwO4mzt_11integer_u32
// CHECK-NEXT: function u0:0() system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCsj0cxGwO4mzt_11integer_u32
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
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTmbENtNtB7_3cmp9PartialEq2eqCsj0cxGwO4mzt_11integer_u32
// CHECK-NEXT: function u0:1(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTmbENtNtB7_3cmp9PartialEq2eqCsj0cxGwO4mzt_11integer_u32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (u32, bool)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (u32, bool)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} (u32, bool)            2b 2                storage=ss0
// CHECK-NEXT: ; stack _2    &'{erased} (u32, bool)            2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _4    &'{erased} u32                    2b 2                var=3
// CHECK-NEXT: ; ssa   _5    &'{erased} u32                    2b 2                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} bool                   2b 2                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} bool                   2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(1:29978 ~ core[..]::cmp::impls::{impl#24}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (u32, bool) <- ByVal(v0): &'{erased} (u32, bool)
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u32, bool) <- ByVal(v1): &'{erased} (u32, bool)
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = &((*_1).0: U)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): &'{erased} u32 <- ByVal(v5): &'{erased} u32
// CHECK-NEXT: ; _5 = &((*_2).0: U)
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): &'{erased} u32 <- ByVal(v7): &'{erased} u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = <U as core::cmp::PartialEq>::eq(move _4, move _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &u32, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &u32, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT:     v12 = iconst.i16 4
// CHECK-NEXT:     v13 = iadd v11, v12  ; v12 = 4
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): &'{erased} bool <- ByVal(v13): &'{erased} bool
// CHECK-NEXT: ; _7 = &((*_2).1: T)
// CHECK-NEXT:     v14 = stack_addr.i16 ss1
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = iconst.i16 4
// CHECK-NEXT:     v17 = iadd v15, v16  ; v16 = 4
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
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptionmENtNtB7_3cmp9PartialEq2eqCsj0cxGwO4mzt_11integer_u32
// CHECK-NEXT: function u0:2(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptionmENtNtB7_3cmp9PartialEq2eqCsj0cxGwO4mzt_11integer_u32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::option::Option<u32>
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::option::Option<u32>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} core::option::Option<u32>    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} core::option::Option<u32>    2b 2                var=3
// CHECK-NEXT: ; ssa   _3    (&'{erased} core::option::Option<u32>, &'{erased} core::option::Option<u32>)    4b 2                var=(4, 5)
// CHECK-NEXT: ; ssa   _4    isize                             2b 2                var=6
// CHECK-NEXT: ; ssa   _5    isize                             2b 2                var=7
// CHECK-NEXT: ; ssa   _6    isize                             2b 2                var=8
// CHECK-NEXT: ; ssa   _7    &'{erased} u32                    2b 2                var=9
// CHECK-NEXT: ; ssa   _8    &'{erased} u32                    2b 2                var=10
// CHECK-NEXT: ; ssa   _9    &'{erased} core::option::Option<u32>    2b 2                var=11
// CHECK-NEXT: ; ssa   _10   &'{erased} core::option::Option<u32>    2b 2                var=12
// CHECK-NEXT: ; ssa   _11   &'{erased} core::option::Option<u32>    2b 2                var=13
// CHECK-NEXT: ; stack _12   &'{erased} core::option::Option<u32>    2b 2                storage=ss0
// CHECK-NEXT: ; stack _13   &'{erased} core::option::Option<u32>    2b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(1:29978 ~ core[..]::cmp::impls::{impl#24}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v15 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v6 -> v1
// CHECK-NEXT:     v9 -> v1
// CHECK-NEXT:     v21 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} core::option::Option<u32> <- ByVal(v0): &'{erased} core::option::Option<u32>
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} core::option::Option<u32> <- ByVal(v1): &'{erased} core::option::Option<u32>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): &'{erased} core::option::Option<u32> <- ByVal(v2): &'{erased} core::option::Option<u32>
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): &'{erased} core::option::Option<u32> <- ByVal(v3): &'{erased} core::option::Option<u32>
// CHECK-NEXT: ; _9 = no_retag copy (_3.0: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Var(_9, var11): &'{erased} core::option::Option<u32> <- ByVal(v2): &'{erased} core::option::Option<u32>
// CHECK-NEXT: ; _6 = discriminant((*_9))
// CHECK-NEXT:     v4 = load.i16 notrap v2
// CHECK-NEXT: ; write_cvalue: Var(_6, var8): isize <- ByVal(v4): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     v5 = uextend.i32 v4
// CHECK-NEXT:     br_table v5, block2, [block4, block3]
// CHECK-EMPTY:
// CHECK-NEXT: block2 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; unreachable
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = no_retag copy (_3.1: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): &'{erased} core::option::Option<u32> <- ByVal(v6): &'{erased} core::option::Option<u32>
// CHECK-NEXT: ; _4 = discriminant((*_10))
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: Var(_4, var6): isize <- ByVal(v7): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _4)
// CHECK-NEXT:     v8 = uextend.i32 v7
// CHECK-NEXT:     br_table v8, block2, [block7, block8]
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _11 = no_retag copy (_3.1: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): &'{erased} core::option::Option<u32> <- ByVal(v9): &'{erased} core::option::Option<u32>
// CHECK-NEXT: ; _5 = discriminant((*_11))
// CHECK-NEXT:     v10 = load.i16 notrap v9
// CHECK-NEXT: ; write_cvalue: Var(_5, var7): isize <- ByVal(v10): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _5)
// CHECK-NEXT:     v11 = uextend.i32 v10
// CHECK-NEXT:     br_table v11, block2, [block5, block6]
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const true
// CHECK-NEXT:     v12 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v12): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v12  ; v12 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const false
// CHECK-NEXT:     v13 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v13): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v13  ; v13 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const false
// CHECK-NEXT:     v14 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v14): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v14  ; v14 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _12 = no_retag copy (_3.0: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} core::option::Option<u32> <- ByVal(v15): &'{erased} core::option::Option<u32>
// CHECK-NEXT:     v16 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v15, v16
// CHECK-NEXT: ; _7 = &(((*_12) as Some).0: T)
// CHECK-NEXT:     v17 = stack_addr.i16 ss0
// CHECK-NEXT:     v18 = load.i16 notrap v17
// CHECK-NEXT:     v19 = iconst.i16 2
// CHECK-NEXT:     v20 = iadd v18, v19  ; v19 = 2
// CHECK-NEXT: ; write_cvalue: Var(_7, var9): &'{erased} u32 <- ByVal(v20): &'{erased} u32
// CHECK-NEXT: ; _13 = no_retag copy (_3.1: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} core::option::Option<u32> <- ByVal(v21): &'{erased} core::option::Option<u32>
// CHECK-NEXT:     v22 = stack_addr.i16 ss1
// CHECK-NEXT:     store.i16 notrap v21, v22
// CHECK-NEXT: ; _8 = &(((*_13) as Some).0: T)
// CHECK-NEXT:     v23 = stack_addr.i16 ss1
// CHECK-NEXT:     v24 = load.i16 notrap v23
// CHECK-NEXT:     v25 = iconst.i16 2
// CHECK-NEXT:     v26 = iadd v24, v25  ; v25 = 2
// CHECK-NEXT: ; write_cvalue: Var(_8, var10): &'{erased} u32 <- ByVal(v26): &'{erased} u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::cmp::PartialEq>::eq(copy _7, copy _8)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &u32, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &u32, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v27 = call fn0(v20, v26)
// CHECK-NEXT:     v28 -> v27
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v27): bool
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v28
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCsj0cxGwO4mzt_11integer_u32
// CHECK-NEXT: function u0:3(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCsj0cxGwO4mzt_11integer_u32
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
// CHECK-NEXT: ; symbol _RNvXsm_NtNtCs40R3J0iFVxM_4core3cmp5implsmNtB7_9PartialEq2eqCsj0cxGwO4mzt_11integer_u32
// CHECK-NEXT: function u0:4(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsm_NtNtCs40R3J0iFVxM_4core3cmp5implsmNtB7_9PartialEq2eqCsj0cxGwO4mzt_11integer_u32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} u32
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} u32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} u32                    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} u32                    2b 2                var=3
// CHECK-NEXT: ; ssa   _3    u32                               4b 2                var=4
// CHECK-NEXT: ; ssa   _4    u32                               4b 2                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} u32 <- ByVal(v0): &'{erased} u32
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} u32 <- ByVal(v1): &'{erased} u32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): u32 <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): u32
// CHECK-NEXT:     v3 = load.i32 notrap v2
// CHECK-NEXT: ; _4 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): u32 <- ByRef(Pointer { base: Addr(v4), offset: Offset32(0) }, None): u32
// CHECK-NEXT:     v5 = load.i32 notrap v4
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
// CHECK-NEXT:     sig0 = (i32, i32) -> i8 system_v
// CHECK-NEXT:     fn0 = colocated u0:6 sig0 ; Instance { def: Item(DefId(0:3 ~ integer_u32[..]::integer_methods)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = integer_methods(const core::num::<impl u32>::MAX, const 2_u32)
// CHECK-NEXT:     v0 = iconst.i32 -1
// CHECK-NEXT:     v1 = iconst.i32 2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: function u0:6(i32, i32) -> i8 system_v {
// CHECK-NEXT: ; symbol integer_methods
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u32                               4b 2                var=2
// CHECK-NEXT: ; ssa   _2    u32                               4b 2                var=3
// CHECK-NEXT: ; stack _3    (u32, bool)                       6b 2                storage=ss0
// CHECK-NEXT: ; stack _4    (u32, bool)                       6b 2                storage=ss1
// CHECK-NEXT: ; ssa   _5    u32                               4b 2                var=4
// CHECK-NEXT: ; stack _6    (u32, bool)                       6b 2                storage=ss2
// CHECK-NEXT: ; ssa   _7    u32                               4b 2                var=5
// CHECK-NEXT: ; ssa   _8    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=7
// CHECK-NEXT: ; ssa   _10   u8                                1b 1                var=8
// CHECK-NEXT: ; ssa   _11   u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=10
// CHECK-NEXT: ; ssa   _13   u8                                1b 1                var=11
// CHECK-NEXT: ; ssa   _14   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _15   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _16   &'{erased} core::option::Option<u32>    2b 2                var=14
// CHECK-NEXT: ; stack _17   core::option::Option<u32>         6b 2                storage=ss3
// CHECK-NEXT: ; ssa   _18   u8                                1b 1                var=15
// CHECK-NEXT: ; ssa   _19   u8                                1b 1                var=16
// CHECK-NEXT: ; ssa   _20   bool                              1b 1                var=17
// CHECK-NEXT: ; ssa   _21   &'{erased} core::option::Option<u32>    2b 2                var=18
// CHECK-NEXT: ; stack _22   core::option::Option<u32>         6b 2                storage=ss4
// CHECK-NEXT: ; ssa   _23   &'{erased} core::option::Option<u32>    2b 2                var=19
// CHECK-NEXT: ; ssa   _24   u8                                1b 1                var=20
// CHECK-NEXT: ; ssa   _25   u8                                1b 1                var=21
// CHECK-NEXT: ; ssa   _26   bool                              1b 1                var=22
// CHECK-NEXT: ; ssa   _27   &'{erased} core::option::Option<u32>    2b 2                var=23
// CHECK-NEXT: ; stack _28   core::option::Option<u32>         6b 2                storage=ss5
// CHECK-NEXT: ; ssa   _29   u8                                1b 1                var=24
// CHECK-NEXT: ; ssa   _30   u8                                1b 1                var=25
// CHECK-NEXT: ; ssa   _31   bool                              1b 1                var=26
// CHECK-NEXT: ; ssa   _32   u32                               4b 2                var=27
// CHECK-NEXT: ; ssa   _33   u32                               4b 2                var=28
// CHECK-NEXT: ; ssa   _34   u8                                1b 1                var=29
// CHECK-NEXT: ; ssa   _35   u8                                1b 1                var=30
// CHECK-NEXT: ; ssa   _36   bool                              1b 1                var=31
// CHECK-NEXT: ; ssa   _37   u32                               4b 2                var=32
// CHECK-NEXT: ; ssa   _38   u32                               4b 2                var=33
// CHECK-NEXT: ; ssa   _39   u8                                1b 1                var=34
// CHECK-NEXT: ; ssa   _40   u8                                1b 1                var=35
// CHECK-NEXT: ; ssa   _41   bool                              1b 1                var=36
// CHECK-NEXT: ; ssa   _42   u32                               4b 2                var=37
// CHECK-NEXT: ; ssa   _43   u32                               4b 2                var=38
// CHECK-NEXT: ; ssa   _44   u8                                1b 1                var=39
// CHECK-NEXT: ; ssa   _45   u8                                1b 1                var=40
// CHECK-NEXT: ; ssa   _46   bool                              1b 1                var=41
// CHECK-NEXT: ; ssa   _47   bool                              1b 1                var=42
// CHECK-NEXT: ; ssa   _48   &'{erased} (u32, bool)            2b 2                var=43
// CHECK-NEXT: ; stack _49   (u32, bool)                       6b 2                storage=ss6
// CHECK-NEXT: ; ssa   _50   &'{erased} (u32, bool)            2b 2                var=44
// CHECK-NEXT: ; ssa   _51   bool                              1b 1                var=45
// CHECK-NEXT: ; ssa   _52   &'{erased} (u32, bool)            2b 2                var=46
// CHECK-NEXT: ; stack _53   (u32, bool)                       6b 2                storage=ss7
// CHECK-NEXT: ; ssa   _54   &'{erased} (u32, bool)            2b 2                var=47
// CHECK-NEXT: ; ssa   _55   &'{erased} (u32, bool)            2b 2                var=48
// CHECK-NEXT: ; stack _56   (u32, bool)                       6b 2                storage=ss8
// CHECK-NEXT: ; ssa   _57   &'{erased} (u32, bool)            2b 2                var=49
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
// CHECK-NEXT: ; ssa   _68   (u32, bool)                       6b 2                var=(60, 61)
// CHECK-NEXT: ; ssa   _69   u32                               4b 2                var=62
// CHECK-NEXT: ; zst   _70   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _71   bool                              1b 1                var=63
// CHECK-NEXT: ; ssa   _72   isize                             2b 2                var=64
// CHECK-NEXT: ; ssa   _73   bool                              1b 1                var=65
// CHECK-NEXT: ; ssa   _74   u32                               4b 2                var=66
// CHECK-NEXT: ; ssa   _75   u32                               4b 2                var=67
// CHECK-NEXT: ; ssa   _76   bool                              1b 1                var=68
// CHECK-NEXT: ; ssa   _77   (u32, bool)                       6b 2                var=(69, 70)
// CHECK-NEXT: ; ssa   _78   bool                              1b 1                var=71
// CHECK-NEXT: ; ssa   _79   u32                               4b 2                var=72
// CHECK-NEXT: ; ssa   _80   bool                              1b 1                var=73
// CHECK-NEXT: ; ssa   _81   (u32, bool)                       6b 2                var=(74, 75)
// CHECK-NEXT: ; zst   _82   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _83   bool                              1b 1                var=76
// CHECK-NEXT: ; ssa   _84   isize                             2b 2                var=77
// CHECK-NEXT: ; ssa   _85   u32                               4b 2                var=78
// CHECK-NEXT: ; ssa   _86   bool                              1b 1                var=79
// CHECK-NEXT: ; ssa   _87   (u32, bool)                       6b 2                var=(80, 81)
// CHECK-NEXT: ; ssa   _88   u32                               4b 2                var=82
// CHECK-NEXT: ; ssa   _89   bool                              1b 1                var=83
// CHECK-NEXT: ; ssa   _90   (u32, bool)                       6b 2                var=(84, 85)
// CHECK-NEXT: ; ssa   _91   u32                               4b 2                var=86
// CHECK-NEXT: ; ssa   _92   bool                              1b 1                var=87
// CHECK-NEXT: ; ssa   _93   (u32, bool)                       6b 2                var=(88, 89)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 6, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 6, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 6, align = 2
// CHECK-NEXT:     ss3 = explicit_slot 6, align = 2
// CHECK-NEXT:     ss4 = explicit_slot 6, align = 2
// CHECK-NEXT:     ss5 = explicit_slot 6, align = 2
// CHECK-NEXT:     ss6 = explicit_slot 6, align = 2
// CHECK-NEXT:     ss7 = explicit_slot 6, align = 2
// CHECK-NEXT:     ss8 = explicit_slot 6, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname1 ; alloc3
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u32, bool] }
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u32, bool] }
// CHECK-NEXT:     fn2 = colocated u0:2 sig2 ; Instance { def: Item(DefId(1:11950 ~ core[..]::option::{impl#17}::eq)), args: [u32] }
// CHECK-NEXT:     fn3 = colocated u0:1 sig3 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u32, bool] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32, v1: i32):
// CHECK-NEXT:     v18 -> v0
// CHECK-NEXT:     v183 -> v0
// CHECK-NEXT:     v184 -> v0
// CHECK-NEXT:     v19 -> v1
// CHECK-NEXT:     v185 -> v1
// CHECK-NEXT:     v186 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u32 <- ByVal(v0): u32
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u32 <- ByVal(v1): u32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (const 1_u32, const true)
// CHECK-NEXT:     v2 = iconst.i32 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u32 <- ByVal(v2): u32
// CHECK-NEXT:     v3 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v2, v3  ; v2 = 1
// CHECK-NEXT:     v4 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(4) }, None): bool <- ByVal(v4): bool
// CHECK-NEXT:     v5 = stack_addr.i16 ss0+4
// CHECK-NEXT:     store notrap v4, v5  ; v4 = 1
// CHECK-NEXT: ; _5 = Sub(const core::num::<impl u32>::MAX, const 2_u32)
// CHECK-NEXT:     v6 = iconst.i32 -1
// CHECK-NEXT:     v7 = iconst.i32 2
// CHECK-NEXT:     v8 = isub v6, v7  ; v6 = -1, v7 = 2
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): u32 <- ByVal(v8): u32
// CHECK-NEXT: ; _4 = (move _5, const false)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u32 <- ByVal(v8): u32
// CHECK-NEXT:     v9 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v8, v9
// CHECK-NEXT:     v10 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(4) }, None): bool <- ByVal(v10): bool
// CHECK-NEXT:     v11 = stack_addr.i16 ss1+4
// CHECK-NEXT:     store notrap v10, v11  ; v10 = 0
// CHECK-NEXT: ; _7 = Sub(const core::num::<impl u32>::MAX, const 1_u32)
// CHECK-NEXT:     v12 = iconst.i32 -1
// CHECK-NEXT:     v13 = iconst.i32 1
// CHECK-NEXT:     v14 = isub v12, v13  ; v12 = -1, v13 = 1
// CHECK-NEXT: ; write_cvalue: Var(_7, var5): u32 <- ByVal(v14): u32
// CHECK-NEXT: ; _6 = (move _7, const true)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u32 <- ByVal(v14): u32
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v14, v15
// CHECK-NEXT:     v16 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(4) }, None): bool <- ByVal(v16): bool
// CHECK-NEXT:     v17 = stack_addr.i16 ss2+4
// CHECK-NEXT:     store notrap v16, v17  ; v16 = 1
// CHECK-NEXT: ; _68 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v20 = iadd.i32 v18, v19
// CHECK-NEXT:     v21 = icmp ult v20, v18
// CHECK-NEXT: ; write_cvalue: VarPair(_68, var60, var61): (u32, bool) <- ByValPair(v20, v21): (u32, bool)
// CHECK-NEXT: ; _67 = copy (_68.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_67, var59): bool <- ByVal(v21): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _67)
// CHECK-NEXT:     brif v21, block17, block19
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
// CHECK-NEXT:     v200 -> v26
// CHECK-NEXT:     v201 -> v26
// CHECK-NEXT:     v206 -> v26
// CHECK-NEXT:     v207 -> v26
// CHECK-NEXT: ; write_cvalue: Var(_13, var11): u8 <- ByVal(v26): u8
// CHECK-NEXT: ; _81 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v29 = uextend.i64 v27
// CHECK-NEXT:     v30 = uextend.i64 v28
// CHECK-NEXT:     v31 = imul v29, v30
// CHECK-NEXT:     v32 = iconst.i64 0xffff_ffff
// CHECK-NEXT:     v33 = icmp ugt v31, v32  ; v32 = 0xffff_ffff
// CHECK-NEXT:     v34 = ireduce.i32 v31
// CHECK-NEXT:     v198 -> v34
// CHECK-NEXT:     v199 -> v34
// CHECK-NEXT: ; write_cvalue: VarPair(_81, var74, var75): (u32, bool) <- ByValPair(v34, v33): (u32, bool)
// CHECK-NEXT: ; _79 = copy (_81.0: u32)
// CHECK-NEXT: ; write_cvalue: Var(_79, var72): u32 <- ByVal(v34): u32
// CHECK-NEXT: ; _80 = copy (_81.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_80, var73): bool <- ByVal(v33): bool
// CHECK-NEXT: ; _77 = (copy _79, copy _80)
// CHECK-NEXT: ; write_cvalue: Var(_77, var69): u32 <- ByVal(v34): u32
// CHECK-NEXT: ; write_cvalue: Var(_77, var70): bool <- ByVal(v33): bool
// CHECK-NEXT: ; _75 = copy (_77.0: u32)
// CHECK-NEXT: ; write_cvalue: Var(_75, var67): u32 <- ByVal(v34): u32
// CHECK-NEXT: ; _76 = copy (_77.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_76, var68): bool <- ByVal(v33): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _76)
// CHECK-NEXT:     brif v33, block31, block33
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _47)
// CHECK-NEXT:     brif.i8 v35, block4, block7
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _90 = SubWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v38 = isub.i32 v36, v37
// CHECK-NEXT:     v39 = icmp ugt v38, v36
// CHECK-NEXT: ; write_cvalue: VarPair(_90, var84, var85): (u32, bool) <- ByValPair(v38, v39): (u32, bool)
// CHECK-NEXT: ; _88 = copy (_90.0: u32)
// CHECK-NEXT: ; write_cvalue: Var(_88, var82): u32 <- ByVal(v38): u32
// CHECK-NEXT: ; _89 = copy (_90.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_89, var83): bool <- ByVal(v39): bool
// CHECK-NEXT: ; _53 = (copy _88, copy _89)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): u32 <- ByVal(v38): u32
// CHECK-NEXT:     v40 = stack_addr.i16 ss7
// CHECK-NEXT:     store notrap v38, v40
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(4) }, None): bool <- ByVal(v39): bool
// CHECK-NEXT:     v41 = stack_addr.i16 ss7+4
// CHECK-NEXT:     store notrap v39, v41
// CHECK-NEXT: ; _52 = &_53
// CHECK-NEXT:     v42 = stack_addr.i16 ss7
// CHECK-NEXT: ; write_cvalue: Var(_52, var46): &'{erased} (u32, bool) <- ByVal(v42): &'{erased} (u32, bool)
// CHECK-NEXT: ; _54 = &_4
// CHECK-NEXT:     v43 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_54, var47): &'{erased} (u32, bool) <- ByVal(v43): &'{erased} (u32, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _51 = <(u32, bool) as core::cmp::PartialEq>::eq(move _52, move _54)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u32, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u32, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v44 = call fn0(v42, v43)
// CHECK-NEXT:     v45 -> v44
// CHECK-NEXT: ; write_cvalue: Var(_51, var45): bool <- ByVal(v44): bool
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _51)
// CHECK-NEXT:     brif.i8 v45, block6, block7
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _93 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v48 = uextend.i64 v46
// CHECK-NEXT:     v49 = uextend.i64 v47
// CHECK-NEXT:     v50 = imul v48, v49
// CHECK-NEXT:     v51 = iconst.i64 0xffff_ffff
// CHECK-NEXT:     v52 = icmp ugt v50, v51  ; v51 = 0xffff_ffff
// CHECK-NEXT:     v53 = ireduce.i32 v50
// CHECK-NEXT: ; write_cvalue: VarPair(_93, var88, var89): (u32, bool) <- ByValPair(v53, v52): (u32, bool)
// CHECK-NEXT: ; _91 = copy (_93.0: u32)
// CHECK-NEXT: ; write_cvalue: Var(_91, var86): u32 <- ByVal(v53): u32
// CHECK-NEXT: ; _92 = copy (_93.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_92, var87): bool <- ByVal(v52): bool
// CHECK-NEXT: ; _56 = (copy _91, copy _92)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): u32 <- ByVal(v53): u32
// CHECK-NEXT:     v54 = stack_addr.i16 ss8
// CHECK-NEXT:     store notrap v53, v54
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(4) }, None): bool <- ByVal(v52): bool
// CHECK-NEXT:     v55 = stack_addr.i16 ss8+4
// CHECK-NEXT:     store notrap v52, v55
// CHECK-NEXT: ; _55 = &_56
// CHECK-NEXT:     v56 = stack_addr.i16 ss8
// CHECK-NEXT: ; write_cvalue: Var(_55, var48): &'{erased} (u32, bool) <- ByVal(v56): &'{erased} (u32, bool)
// CHECK-NEXT: ; _57 = &_6
// CHECK-NEXT:     v57 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_57, var49): &'{erased} (u32, bool) <- ByVal(v57): &'{erased} (u32, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _46 = <(u32, bool) as core::cmp::PartialEq>::eq(move _55, move _57)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u32, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u32, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v58 = call fn1(v56, v57)
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v58): bool
// CHECK-NEXT:     jump block8(v58)
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _46 = const false
// CHECK-NEXT:     v59 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v59): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block8(v59)  ; v59 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block8(v60: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _45 = move _46 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_45, var40): u8 <- ByVal(v60): u8
// CHECK-NEXT: ; _44 = Shl(move _45, const 6_i32)
// CHECK-NEXT:     v61 = iconst.i32 6
// CHECK-NEXT:     v62 = ishl v60, v61  ; v61 = 6
// CHECK-NEXT: ; write_cvalue: Var(_44, var39): u8 <- ByVal(v62): u8
// CHECK-NEXT: ; _8 = BitOr(move _9, move _44)
// CHECK-NEXT:     v64 = bor.i8 v63, v62
// CHECK-NEXT:     v178 -> v64
// CHECK-NEXT:     v179 -> v64
// CHECK-NEXT:     v180 -> v64
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): u8 <- ByVal(v64): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _62 = core::intrinsics::ctlz::<u32>(copy _1)
// CHECK-NEXT:     v66 = clz.i32 v65
// CHECK-NEXT:     v161 -> v66
// CHECK-NEXT: ; write_cvalue: Var(_62, var54): u32 <- ByVal(v66): u32
// CHECK-NEXT:     jump block37
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _64 = core::intrinsics::cttz::<u32>(copy _1)
// CHECK-NEXT:     v68 = ctz.i32 v67
// CHECK-NEXT:     v164 -> v68
// CHECK-NEXT: ; write_cvalue: Var(_64, var56): u32 <- ByVal(v68): u32
// CHECK-NEXT:     jump block38
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _65 = core::intrinsics::ctpop::<u32>(copy _1)
// CHECK-NEXT:     v70 = popcnt.i32 v69
// CHECK-NEXT:     v167 -> v70
// CHECK-NEXT: ; write_cvalue: Var(_65, var57): u32 <- ByVal(v70): u32
// CHECK-NEXT:     jump block39
// CHECK-EMPTY:
// CHECK-NEXT: block11(v177: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = const false
// CHECK-NEXT:     v71 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v71): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v71, v177)  ; v71 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block12(v72: i8, v75: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _59 = move _60 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_59, var51): u8 <- ByVal(v72): u8
// CHECK-NEXT: ; _58 = Shl(move _59, const 7_i32)
// CHECK-NEXT:     v73 = iconst.i32 7
// CHECK-NEXT:     v74 = ishl v72, v73  ; v73 = 7
// CHECK-NEXT: ; write_cvalue: Var(_58, var50): u8 <- ByVal(v74): u8
// CHECK-NEXT: ; _0 = BitOr(move _8, move _58)
// CHECK-NEXT:     v76 = bor v75, v74
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v76): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v76
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = &_17
// CHECK-NEXT:     v77 = stack_addr.i16 ss3
// CHECK-NEXT: ; write_cvalue: Var(_16, var14): &'{erased} core::option::Option<u32> <- ByVal(v77): &'{erased} core::option::Option<u32>
// CHECK-NEXT: ; _72 = discriminant((*_16))
// CHECK-NEXT:     v78 = load.i16 notrap v77
// CHECK-NEXT: ; write_cvalue: Var(_72, var64): isize <- ByVal(v78): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _72)
// CHECK-NEXT:     v79 = uextend.i32 v78
// CHECK-NEXT:     br_table v79, block23, [block21, block22]
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _17 = core::option::Option::<u32>::None
// CHECK-NEXT:     v80 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u16 <- ByVal(v80): u16
// CHECK-NEXT:     v81 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v80, v81  ; v80 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _69 = AddUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v84 = iadd.i32 v82, v83
// CHECK-NEXT: ; write_cvalue: Var(_69, var62): u32 <- ByVal(v84): u32
// CHECK-NEXT: ; _17 = core::option::Option::<u32>::Some(move _69)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(2) }, None): u32 <- ByVal(v84): u32
// CHECK-NEXT:     v85 = stack_addr.i16 ss3+2
// CHECK-NEXT:     store notrap v84, v85
// CHECK-NEXT:     v86 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u16 <- ByVal(v86): u16
// CHECK-NEXT:     v87 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v86, v87  ; v86 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block16(v88: i8, v181: i32, v182: i32):
// CHECK-NEXT:     v82 -> v181
// CHECK-NEXT:     v187 -> v181
// CHECK-NEXT:     v188 -> v181
// CHECK-NEXT:     v191 -> v181
// CHECK-NEXT:     v83 -> v182
// CHECK-NEXT:     v189 -> v182
// CHECK-NEXT:     v190 -> v182
// CHECK-NEXT:     v192 -> v182
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _66)
// CHECK-NEXT:     brif v88, block14, block15
// CHECK-EMPTY:
// CHECK-NEXT: block17 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _70 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block18
// CHECK-EMPTY:
// CHECK-NEXT: block18:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _66 = const true
// CHECK-NEXT:     v89 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_66, var58): bool <- ByVal(v89): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block16(v89, v183, v185)  ; v89 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block19:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _66 = const false
// CHECK-NEXT:     v90 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_66, var58): bool <- ByVal(v90): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block16(v90, v184, v186)  ; v90 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block20(v91: i8, v94: i32, v95: i32):
// CHECK-NEXT:     v104 -> v94
// CHECK-NEXT:     v195 -> v94
// CHECK-NEXT:     v105 -> v95
// CHECK-NEXT:     v196 -> v95
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = Not(move _71)
// CHECK-NEXT:     v92 = iconst.i8 0
// CHECK-NEXT:     v93 = icmp eq v91, v92  ; v92 = 0
// CHECK-NEXT:     v193 -> v93
// CHECK-NEXT:     v194 -> v93
// CHECK-NEXT: ; write_cvalue: Var(_15, var13): bool <- ByVal(v93): bool
// CHECK-NEXT: ; _14 = move _15 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_14, var12): u8 <- ByVal(v93): u8
// CHECK-NEXT: ; _73 = Lt(copy _1, copy _2)
// CHECK-NEXT:     v96 = icmp ult v94, v95
// CHECK-NEXT: ; write_cvalue: Var(_73, var65): bool <- ByVal(v96): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _73)
// CHECK-NEXT:     brif v96, block25, block26
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _71 = const false
// CHECK-NEXT:     v97 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_71, var63): bool <- ByVal(v97): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block20(v97, v187, v189)  ; v97 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _71 = const true
// CHECK-NEXT:     v98 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_71, var63): bool <- ByVal(v98): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block20(v98, v188, v190)  ; v98 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block23 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; unreachable
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block24(v170: i8, v171: i32, v172: i32):
// CHECK-NEXT:     v25 -> v170
// CHECK-NEXT:     v27 -> v171
// CHECK-NEXT:     v202 -> v171
// CHECK-NEXT:     v203 -> v171
// CHECK-NEXT:     v208 -> v171
// CHECK-NEXT:     v209 -> v171
// CHECK-NEXT:     v28 -> v172
// CHECK-NEXT:     v204 -> v172
// CHECK-NEXT:     v205 -> v172
// CHECK-NEXT:     v210 -> v172
// CHECK-NEXT:     v211 -> v172
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _21 = &_22
// CHECK-NEXT:     v99 = stack_addr.i16 ss4
// CHECK-NEXT: ; write_cvalue: Var(_21, var18): &'{erased} core::option::Option<u32> <- ByVal(v99): &'{erased} core::option::Option<u32>
// CHECK-NEXT: ; _23 = const integer_methods::promoted[0]
// CHECK-NEXT:     v100 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_23, var19): &'{erased} core::option::Option<u32> <- ByVal(v100): &'{erased} core::option::Option<u32>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _20 = <core::option::Option<u32> as core::cmp::PartialEq>::eq(move _21, move _23)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u32>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u32>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v101 = call fn2(v99, v100)
// CHECK-NEXT:     v22 -> v101
// CHECK-NEXT: ; write_cvalue: Var(_20, var17): bool <- ByVal(v101): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block25:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = core::option::Option::<u32>::None
// CHECK-NEXT:     v102 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u16 <- ByVal(v102): u16
// CHECK-NEXT:     v103 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v102, v103  ; v102 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block24(v193, v195, v196)
// CHECK-EMPTY:
// CHECK-NEXT: block26:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _74 = SubUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v106 = isub.i32 v104, v105
// CHECK-NEXT: ; write_cvalue: Var(_74, var66): u32 <- ByVal(v106): u32
// CHECK-NEXT: ; _22 = core::option::Option::<u32>::Some(move _74)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(2) }, None): u32 <- ByVal(v106): u32
// CHECK-NEXT:     v107 = stack_addr.i16 ss4+2
// CHECK-NEXT:     store notrap v106, v107
// CHECK-NEXT:     v108 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u16 <- ByVal(v108): u16
// CHECK-NEXT:     v109 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v108, v109  ; v108 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block24(v194, v104, v105)
// CHECK-EMPTY:
// CHECK-NEXT: block27:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _27 = &_28
// CHECK-NEXT:     v110 = stack_addr.i16 ss5
// CHECK-NEXT: ; write_cvalue: Var(_27, var23): &'{erased} core::option::Option<u32> <- ByVal(v110): &'{erased} core::option::Option<u32>
// CHECK-NEXT: ; _84 = discriminant((*_27))
// CHECK-NEXT:     v111 = load.i16 notrap v110
// CHECK-NEXT: ; write_cvalue: Var(_84, var77): isize <- ByVal(v111): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _84)
// CHECK-NEXT:     v112 = uextend.i32 v111
// CHECK-NEXT:     br_table v112, block23, [block35, block36]
// CHECK-EMPTY:
// CHECK-NEXT: block28:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _28 = core::option::Option::<u32>::None
// CHECK-NEXT:     v113 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): u16 <- ByVal(v113): u16
// CHECK-NEXT:     v114 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v113, v114  ; v113 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block27
// CHECK-EMPTY:
// CHECK-NEXT: block29:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _28 = core::option::Option::<u32>::Some(copy _75)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(2) }, None): u32 <- ByVal(v115): u32
// CHECK-NEXT:     v116 = stack_addr.i16 ss5+2
// CHECK-NEXT:     store.i32 notrap v115, v116
// CHECK-NEXT:     v117 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): u16 <- ByVal(v117): u16
// CHECK-NEXT:     v118 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v117, v118  ; v117 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block27
// CHECK-EMPTY:
// CHECK-NEXT: block30(v119: i8, v197: i32):
// CHECK-NEXT:     v115 -> v197
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _78)
// CHECK-NEXT:     brif v119, block28, block29
// CHECK-EMPTY:
// CHECK-NEXT: block31 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _82 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block32
// CHECK-EMPTY:
// CHECK-NEXT: block32:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _78 = const true
// CHECK-NEXT:     v120 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_78, var71): bool <- ByVal(v120): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block30(v120, v198)  ; v120 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block33:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _78 = const false
// CHECK-NEXT:     v121 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_78, var71): bool <- ByVal(v121): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block30(v121, v199)  ; v121 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block34(v122: i8, v127: i8, v129: i32, v130: i32):
// CHECK-NEXT:     v36 -> v129
// CHECK-NEXT:     v46 -> v129
// CHECK-NEXT:     v65 -> v129
// CHECK-NEXT:     v174 -> v129
// CHECK-NEXT:     v175 -> v129
// CHECK-NEXT:     v176 -> v129
// CHECK-NEXT:     v69 -> v176
// CHECK-NEXT:     v67 -> v175
// CHECK-NEXT:     v37 -> v130
// CHECK-NEXT:     v47 -> v130
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _26 = Not(move _83)
// CHECK-NEXT:     v123 = iconst.i8 0
// CHECK-NEXT:     v124 = icmp eq v122, v123  ; v123 = 0
// CHECK-NEXT: ; write_cvalue: Var(_26, var22): bool <- ByVal(v124): bool
// CHECK-NEXT: ; _25 = move _26 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_25, var21): u8 <- ByVal(v124): u8
// CHECK-NEXT: ; _24 = Shl(move _25, const 2_i32)
// CHECK-NEXT:     v125 = iconst.i32 2
// CHECK-NEXT:     v126 = ishl v124, v125  ; v125 = 2
// CHECK-NEXT: ; write_cvalue: Var(_24, var20): u8 <- ByVal(v126): u8
// CHECK-NEXT: ; _12 = BitOr(move _13, move _24)
// CHECK-NEXT:     v128 = bor v127, v126
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): u8 <- ByVal(v128): u8
// CHECK-NEXT: ; _32 = Add(copy _1, copy _2)
// CHECK-NEXT:     v131 = iadd v129, v130
// CHECK-NEXT: ; write_cvalue: Var(_32, var27): u32 <- ByVal(v131): u32
// CHECK-NEXT: ; _33 = copy (_3.0: u32)
// CHECK-NEXT: ; write_cvalue: Var(_33, var28): u32 <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u32
// CHECK-NEXT:     v132 = stack_addr.i16 ss0
// CHECK-NEXT:     v133 = load.i32 notrap v132
// CHECK-NEXT: ; _31 = Eq(move _32, move _33)
// CHECK-NEXT:     v134 = icmp eq v131, v133
// CHECK-NEXT: ; write_cvalue: Var(_31, var26): bool <- ByVal(v134): bool
// CHECK-NEXT: ; _30 = move _31 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_30, var25): u8 <- ByVal(v134): u8
// CHECK-NEXT: ; _29 = Shl(move _30, const 3_i32)
// CHECK-NEXT:     v135 = iconst.i32 3
// CHECK-NEXT:     v136 = ishl v134, v135  ; v135 = 3
// CHECK-NEXT: ; write_cvalue: Var(_29, var24): u8 <- ByVal(v136): u8
// CHECK-NEXT: ; _11 = BitOr(move _12, move _29)
// CHECK-NEXT:     v137 = bor v128, v136
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): u8 <- ByVal(v137): u8
// CHECK-NEXT: ; _37 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v138 = isub v129, v130
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): u32 <- ByVal(v138): u32
// CHECK-NEXT: ; _38 = copy (_4.0: u32)
// CHECK-NEXT: ; write_cvalue: Var(_38, var33): u32 <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u32
// CHECK-NEXT:     v139 = stack_addr.i16 ss1
// CHECK-NEXT:     v140 = load.i32 notrap v139
// CHECK-NEXT: ; _36 = Eq(move _37, move _38)
// CHECK-NEXT:     v141 = icmp eq v138, v140
// CHECK-NEXT: ; write_cvalue: Var(_36, var31): bool <- ByVal(v141): bool
// CHECK-NEXT: ; _35 = move _36 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_35, var30): u8 <- ByVal(v141): u8
// CHECK-NEXT: ; _34 = Shl(move _35, const 4_i32)
// CHECK-NEXT:     v142 = iconst.i32 4
// CHECK-NEXT:     v143 = ishl v141, v142  ; v142 = 4
// CHECK-NEXT: ; write_cvalue: Var(_34, var29): u8 <- ByVal(v143): u8
// CHECK-NEXT: ; _10 = BitOr(move _11, move _34)
// CHECK-NEXT:     v144 = bor v137, v143
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): u8 <- ByVal(v144): u8
// CHECK-NEXT: ; _42 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v145 = imul v129, v130
// CHECK-NEXT: ; write_cvalue: Var(_42, var37): u32 <- ByVal(v145): u32
// CHECK-NEXT: ; _43 = copy (_6.0: u32)
// CHECK-NEXT: ; write_cvalue: Var(_43, var38): u32 <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u32
// CHECK-NEXT:     v146 = stack_addr.i16 ss2
// CHECK-NEXT:     v147 = load.i32 notrap v146
// CHECK-NEXT: ; _41 = Eq(move _42, move _43)
// CHECK-NEXT:     v148 = icmp eq v145, v147
// CHECK-NEXT: ; write_cvalue: Var(_41, var36): bool <- ByVal(v148): bool
// CHECK-NEXT: ; _40 = move _41 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_40, var35): u8 <- ByVal(v148): u8
// CHECK-NEXT: ; _39 = Shl(move _40, const 5_i32)
// CHECK-NEXT:     v149 = iconst.i32 5
// CHECK-NEXT:     v150 = ishl v148, v149  ; v149 = 5
// CHECK-NEXT: ; write_cvalue: Var(_39, var34): u8 <- ByVal(v150): u8
// CHECK-NEXT: ; _9 = BitOr(move _10, move _39)
// CHECK-NEXT:     v151 = bor v144, v150
// CHECK-NEXT:     v63 -> v151
// CHECK-NEXT:     v173 -> v151
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): u8 <- ByVal(v151): u8
// CHECK-NEXT: ; _87 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v152 = iadd v129, v130
// CHECK-NEXT:     v153 = icmp ult v152, v129
// CHECK-NEXT: ; write_cvalue: VarPair(_87, var80, var81): (u32, bool) <- ByValPair(v152, v153): (u32, bool)
// CHECK-NEXT: ; _85 = copy (_87.0: u32)
// CHECK-NEXT: ; write_cvalue: Var(_85, var78): u32 <- ByVal(v152): u32
// CHECK-NEXT: ; _86 = copy (_87.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_86, var79): bool <- ByVal(v153): bool
// CHECK-NEXT: ; _49 = (copy _85, copy _86)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): u32 <- ByVal(v152): u32
// CHECK-NEXT:     v154 = stack_addr.i16 ss6
// CHECK-NEXT:     store notrap v152, v154
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(4) }, None): bool <- ByVal(v153): bool
// CHECK-NEXT:     v155 = stack_addr.i16 ss6+4
// CHECK-NEXT:     store notrap v153, v155
// CHECK-NEXT: ; _48 = &_49
// CHECK-NEXT:     v156 = stack_addr.i16 ss6
// CHECK-NEXT: ; write_cvalue: Var(_48, var43): &'{erased} (u32, bool) <- ByVal(v156): &'{erased} (u32, bool)
// CHECK-NEXT: ; _50 = &_3
// CHECK-NEXT:     v157 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_50, var44): &'{erased} (u32, bool) <- ByVal(v157): &'{erased} (u32, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _47 = <(u32, bool) as core::cmp::PartialEq>::eq(move _48, move _50)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u32, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u32, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v158 = call fn3(v156, v157)
// CHECK-NEXT:     v35 -> v158
// CHECK-NEXT: ; write_cvalue: Var(_47, var42): bool <- ByVal(v158): bool
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block35:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _83 = const false
// CHECK-NEXT:     v159 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_83, var76): bool <- ByVal(v159): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block34(v159, v200, v202, v204)  ; v159 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block36:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _83 = const true
// CHECK-NEXT:     v160 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_83, var76): bool <- ByVal(v160): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block34(v160, v201, v203, v205)  ; v160 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block37:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _61 = Eq(move _62, const 0_u32)
// CHECK-NEXT:     v162 = iconst.i32 0
// CHECK-NEXT:     v163 = icmp.i32 eq v161, v162  ; v162 = 0
// CHECK-NEXT: ; write_cvalue: Var(_61, var53): bool <- ByVal(v163): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _61)
// CHECK-NEXT:     brif.i32 v161, block11(v178), block9
// CHECK-EMPTY:
// CHECK-NEXT: block38:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _63 = Eq(move _64, const 0_u32)
// CHECK-NEXT:     v165 = iconst.i32 0
// CHECK-NEXT:     v166 = icmp.i32 eq v164, v165  ; v165 = 0
// CHECK-NEXT: ; write_cvalue: Var(_63, var55): bool <- ByVal(v166): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _63)
// CHECK-NEXT:     brif.i32 v164, block11(v179), block10
// CHECK-EMPTY:
// CHECK-NEXT: block39:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = Eq(move _65, const 32_u32)
// CHECK-NEXT:     v168 = iconst.i32 32
// CHECK-NEXT:     v169 = icmp.i32 eq v167, v168  ; v168 = 32
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v169): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v169, v180)
// CHECK-NEXT: }
