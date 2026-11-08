#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn integer_methods(x: u64, y: u64) -> u8 {
    let add = (1, true);
    let sub = (u64::MAX - 2, false);
    let mul = (u64::MAX - 1, true);
    (x.checked_add(y).is_none() as u8)
        | ((x.checked_sub(y) == Some(u64::MAX - 2)) as u8) << 1
        | (x.checked_mul(y).is_none() as u8) << 2
        | ((x.wrapping_add(y) == add.0) as u8) << 3
        | ((x.wrapping_sub(y) == sub.0) as u8) << 4
        | ((x.wrapping_mul(y) == mul.0) as u8) << 5
        | ((x.overflowing_add(y) == add
            && x.overflowing_sub(y) == sub
            && x.overflowing_mul(y) == mul) as u8) << 6
        | ((x.leading_zeros() == 0
            && x.trailing_zeros() == 0
            && x.count_ones() == 64) as u8) << 7
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    (integer_methods(u64::MAX, 2) == 255) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCsi7blA9Y4n5i_11integer_u64
// CHECK-NEXT: function u0:0() system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCsi7blA9Y4n5i_11integer_u64
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
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTybENtNtB7_3cmp9PartialEq2eqCsi7blA9Y4n5i_11integer_u64
// CHECK-NEXT: function u0:1(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTybENtNtB7_3cmp9PartialEq2eqCsi7blA9Y4n5i_11integer_u64
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
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(1:29981 ~ core[..]::cmp::impls::{impl#25}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
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
// CHECK-NEXT: ; _3 = <U as core::cmp::PartialEq>::eq(move _4, move _5)
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
// CHECK-NEXT: ; _6 = &((*_1).1: T)
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     v11 = load.i16 notrap v10
// CHECK-NEXT:     v12 = iconst.i16 8
// CHECK-NEXT:     v13 = iadd v11, v12  ; v12 = 8
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): &'{erased} bool <- ByVal(v13): &'{erased} bool
// CHECK-NEXT: ; _7 = &((*_2).1: T)
// CHECK-NEXT:     v14 = stack_addr.i16 ss1
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = iconst.i16 8
// CHECK-NEXT:     v17 = iadd v15, v16  ; v16 = 8
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
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptionyENtNtB7_3cmp9PartialEq2eqCsi7blA9Y4n5i_11integer_u64
// CHECK-NEXT: function u0:2(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptionyENtNtB7_3cmp9PartialEq2eqCsi7blA9Y4n5i_11integer_u64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::option::Option<u64>
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::option::Option<u64>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} core::option::Option<u64>    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} core::option::Option<u64>    2b 2                var=3
// CHECK-NEXT: ; ssa   _3    (&'{erased} core::option::Option<u64>, &'{erased} core::option::Option<u64>)    4b 2                var=(4, 5)
// CHECK-NEXT: ; ssa   _4    isize                             2b 2                var=6
// CHECK-NEXT: ; ssa   _5    isize                             2b 2                var=7
// CHECK-NEXT: ; ssa   _6    isize                             2b 2                var=8
// CHECK-NEXT: ; ssa   _7    &'{erased} u64                    2b 2                var=9
// CHECK-NEXT: ; ssa   _8    &'{erased} u64                    2b 2                var=10
// CHECK-NEXT: ; ssa   _9    &'{erased} core::option::Option<u64>    2b 2                var=11
// CHECK-NEXT: ; ssa   _10   &'{erased} core::option::Option<u64>    2b 2                var=12
// CHECK-NEXT: ; ssa   _11   &'{erased} core::option::Option<u64>    2b 2                var=13
// CHECK-NEXT: ; stack _12   &'{erased} core::option::Option<u64>    2b 2                storage=ss0
// CHECK-NEXT: ; stack _13   &'{erased} core::option::Option<u64>    2b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(1:29981 ~ core[..]::cmp::impls::{impl#25}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v15 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v6 -> v1
// CHECK-NEXT:     v9 -> v1
// CHECK-NEXT:     v21 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} core::option::Option<u64> <- ByVal(v0): &'{erased} core::option::Option<u64>
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} core::option::Option<u64> <- ByVal(v1): &'{erased} core::option::Option<u64>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): &'{erased} core::option::Option<u64> <- ByVal(v2): &'{erased} core::option::Option<u64>
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): &'{erased} core::option::Option<u64> <- ByVal(v3): &'{erased} core::option::Option<u64>
// CHECK-NEXT: ; _9 = no_retag copy (_3.0: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Var(_9, var11): &'{erased} core::option::Option<u64> <- ByVal(v2): &'{erased} core::option::Option<u64>
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
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): &'{erased} core::option::Option<u64> <- ByVal(v6): &'{erased} core::option::Option<u64>
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
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): &'{erased} core::option::Option<u64> <- ByVal(v9): &'{erased} core::option::Option<u64>
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
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} core::option::Option<u64> <- ByVal(v15): &'{erased} core::option::Option<u64>
// CHECK-NEXT:     v16 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v15, v16
// CHECK-NEXT: ; _7 = &(((*_12) as Some).0: T)
// CHECK-NEXT:     v17 = stack_addr.i16 ss0
// CHECK-NEXT:     v18 = load.i16 notrap v17
// CHECK-NEXT:     v19 = iconst.i16 2
// CHECK-NEXT:     v20 = iadd v18, v19  ; v19 = 2
// CHECK-NEXT: ; write_cvalue: Var(_7, var9): &'{erased} u64 <- ByVal(v20): &'{erased} u64
// CHECK-NEXT: ; _13 = no_retag copy (_3.1: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} core::option::Option<u64> <- ByVal(v21): &'{erased} core::option::Option<u64>
// CHECK-NEXT:     v22 = stack_addr.i16 ss1
// CHECK-NEXT:     store.i16 notrap v21, v22
// CHECK-NEXT: ; _8 = &(((*_13) as Some).0: T)
// CHECK-NEXT:     v23 = stack_addr.i16 ss1
// CHECK-NEXT:     v24 = load.i16 notrap v23
// CHECK-NEXT:     v25 = iconst.i16 2
// CHECK-NEXT:     v26 = iadd v24, v25  ; v25 = 2
// CHECK-NEXT: ; write_cvalue: Var(_8, var10): &'{erased} u64 <- ByVal(v26): &'{erased} u64
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::cmp::PartialEq>::eq(copy _7, copy _8)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &u64, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &u64, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCsi7blA9Y4n5i_11integer_u64
// CHECK-NEXT: function u0:3(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCsi7blA9Y4n5i_11integer_u64
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
// CHECK-NEXT: ; symbol _RNvXsn_NtNtCs40R3J0iFVxM_4core3cmp5implsyNtB7_9PartialEq2eqCsi7blA9Y4n5i_11integer_u64
// CHECK-NEXT: function u0:4(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsn_NtNtCs40R3J0iFVxM_4core3cmp5implsyNtB7_9PartialEq2eqCsi7blA9Y4n5i_11integer_u64
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
// CHECK-NEXT:     sig0 = (i64, i64) -> i8 system_v
// CHECK-NEXT:     fn0 = colocated u0:6 sig0 ; Instance { def: Item(DefId(0:3 ~ integer_u64[..]::integer_methods)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = integer_methods(const core::num::<impl u64>::MAX, const 2_u64)
// CHECK-NEXT:     v0 = iconst.i64 -1
// CHECK-NEXT:     v1 = iconst.i64 2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u64, layout: Layout { size: Size(8 bytes), align: Align(2 bytes), backend_repr: Scalar(u64 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934599 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u64, layout: Layout { size: Size(8 bytes), align: Align(2 bytes), backend_repr: Scalar(u64 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934599 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: function u0:6(i64, i64) -> i8 system_v {
// CHECK-NEXT: ; symbol integer_methods
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u64
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u64                               8b 2                var=2
// CHECK-NEXT: ; ssa   _2    u64                               8b 2                var=3
// CHECK-NEXT: ; stack _3    (u64, bool)                      10b 2                storage=ss0
// CHECK-NEXT: ; stack _4    (u64, bool)                      10b 2                storage=ss1
// CHECK-NEXT: ; ssa   _5    u64                               8b 2                var=4
// CHECK-NEXT: ; stack _6    (u64, bool)                      10b 2                storage=ss2
// CHECK-NEXT: ; ssa   _7    u64                               8b 2                var=5
// CHECK-NEXT: ; ssa   _8    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=7
// CHECK-NEXT: ; ssa   _10   u8                                1b 1                var=8
// CHECK-NEXT: ; ssa   _11   u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=10
// CHECK-NEXT: ; ssa   _13   u8                                1b 1                var=11
// CHECK-NEXT: ; ssa   _14   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _15   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _16   &'{erased} core::option::Option<u64>    2b 2                var=14
// CHECK-NEXT: ; stack _17   core::option::Option<u64>        10b 2                storage=ss3
// CHECK-NEXT: ; ssa   _18   u8                                1b 1                var=15
// CHECK-NEXT: ; ssa   _19   u8                                1b 1                var=16
// CHECK-NEXT: ; ssa   _20   bool                              1b 1                var=17
// CHECK-NEXT: ; ssa   _21   &'{erased} core::option::Option<u64>    2b 2                var=18
// CHECK-NEXT: ; stack _22   core::option::Option<u64>        10b 2                storage=ss4
// CHECK-NEXT: ; ssa   _23   &'{erased} core::option::Option<u64>    2b 2                var=19
// CHECK-NEXT: ; ssa   _24   u8                                1b 1                var=20
// CHECK-NEXT: ; ssa   _25   u8                                1b 1                var=21
// CHECK-NEXT: ; ssa   _26   bool                              1b 1                var=22
// CHECK-NEXT: ; ssa   _27   &'{erased} core::option::Option<u64>    2b 2                var=23
// CHECK-NEXT: ; stack _28   core::option::Option<u64>        10b 2                storage=ss5
// CHECK-NEXT: ; ssa   _29   u8                                1b 1                var=24
// CHECK-NEXT: ; ssa   _30   u8                                1b 1                var=25
// CHECK-NEXT: ; ssa   _31   bool                              1b 1                var=26
// CHECK-NEXT: ; ssa   _32   u64                               8b 2                var=27
// CHECK-NEXT: ; ssa   _33   u64                               8b 2                var=28
// CHECK-NEXT: ; ssa   _34   u8                                1b 1                var=29
// CHECK-NEXT: ; ssa   _35   u8                                1b 1                var=30
// CHECK-NEXT: ; ssa   _36   bool                              1b 1                var=31
// CHECK-NEXT: ; ssa   _37   u64                               8b 2                var=32
// CHECK-NEXT: ; ssa   _38   u64                               8b 2                var=33
// CHECK-NEXT: ; ssa   _39   u8                                1b 1                var=34
// CHECK-NEXT: ; ssa   _40   u8                                1b 1                var=35
// CHECK-NEXT: ; ssa   _41   bool                              1b 1                var=36
// CHECK-NEXT: ; ssa   _42   u64                               8b 2                var=37
// CHECK-NEXT: ; ssa   _43   u64                               8b 2                var=38
// CHECK-NEXT: ; ssa   _44   u8                                1b 1                var=39
// CHECK-NEXT: ; ssa   _45   u8                                1b 1                var=40
// CHECK-NEXT: ; ssa   _46   bool                              1b 1                var=41
// CHECK-NEXT: ; ssa   _47   bool                              1b 1                var=42
// CHECK-NEXT: ; ssa   _48   &'{erased} (u64, bool)            2b 2                var=43
// CHECK-NEXT: ; stack _49   (u64, bool)                      10b 2                storage=ss6
// CHECK-NEXT: ; ssa   _50   &'{erased} (u64, bool)            2b 2                var=44
// CHECK-NEXT: ; ssa   _51   bool                              1b 1                var=45
// CHECK-NEXT: ; ssa   _52   &'{erased} (u64, bool)            2b 2                var=46
// CHECK-NEXT: ; stack _53   (u64, bool)                      10b 2                storage=ss7
// CHECK-NEXT: ; ssa   _54   &'{erased} (u64, bool)            2b 2                var=47
// CHECK-NEXT: ; ssa   _55   &'{erased} (u64, bool)            2b 2                var=48
// CHECK-NEXT: ; stack _56   (u64, bool)                      10b 2                storage=ss8
// CHECK-NEXT: ; ssa   _57   &'{erased} (u64, bool)            2b 2                var=49
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
// CHECK-NEXT: ; ssa   _68   (u64, bool)                      10b 2                var=(60, 61)
// CHECK-NEXT: ; ssa   _69   u64                               8b 2                var=62
// CHECK-NEXT: ; zst   _70   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _71   bool                              1b 1                var=63
// CHECK-NEXT: ; ssa   _72   isize                             2b 2                var=64
// CHECK-NEXT: ; ssa   _73   bool                              1b 1                var=65
// CHECK-NEXT: ; ssa   _74   u64                               8b 2                var=66
// CHECK-NEXT: ; ssa   _75   u64                               8b 2                var=67
// CHECK-NEXT: ; ssa   _76   bool                              1b 1                var=68
// CHECK-NEXT: ; ssa   _77   (u64, bool)                      10b 2                var=(69, 70)
// CHECK-NEXT: ; ssa   _78   bool                              1b 1                var=71
// CHECK-NEXT: ; ssa   _79   u64                               8b 2                var=72
// CHECK-NEXT: ; ssa   _80   bool                              1b 1                var=73
// CHECK-NEXT: ; ssa   _81   (u64, bool)                      10b 2                var=(74, 75)
// CHECK-NEXT: ; zst   _82   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _83   bool                              1b 1                var=76
// CHECK-NEXT: ; ssa   _84   isize                             2b 2                var=77
// CHECK-NEXT: ; ssa   _85   u64                               8b 2                var=78
// CHECK-NEXT: ; ssa   _86   bool                              1b 1                var=79
// CHECK-NEXT: ; ssa   _87   (u64, bool)                      10b 2                var=(80, 81)
// CHECK-NEXT: ; ssa   _88   u64                               8b 2                var=82
// CHECK-NEXT: ; ssa   _89   bool                              1b 1                var=83
// CHECK-NEXT: ; ssa   _90   (u64, bool)                      10b 2                var=(84, 85)
// CHECK-NEXT: ; ssa   _91   u64                               8b 2                var=86
// CHECK-NEXT: ; ssa   _92   bool                              1b 1                var=87
// CHECK-NEXT: ; ssa   _93   (u64, bool)                      10b 2                var=(88, 89)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss3 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss4 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss5 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss6 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss7 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss8 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss9 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss10 = explicit_slot 16, align = 2
// CHECK-NEXT:     ss11 = explicit_slot 16, align = 2
// CHECK-NEXT:     ss12 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss13 = explicit_slot 16, align = 2
// CHECK-NEXT:     ss14 = explicit_slot 16, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname2 ; alloc3
// CHECK-NEXT:     sig0 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig4 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig5 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = u0:7 sig0 ; "__gbc_u128_mulo"
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u64, bool] }
// CHECK-NEXT:     fn2 = u0:7 sig2 ; "__gbc_u128_mulo"
// CHECK-NEXT:     fn3 = colocated u0:1 sig3 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u64, bool] }
// CHECK-NEXT:     fn4 = colocated u0:2 sig4 ; Instance { def: Item(DefId(1:11950 ~ core[..]::option::{impl#17}::eq)), args: [u64] }
// CHECK-NEXT:     fn5 = colocated u0:1 sig5 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u64, bool] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i64, v1: i64):
// CHECK-NEXT:     v18 -> v0
// CHECK-NEXT:     v212 -> v0
// CHECK-NEXT:     v213 -> v0
// CHECK-NEXT:     v19 -> v1
// CHECK-NEXT:     v214 -> v1
// CHECK-NEXT:     v215 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u64 <- ByVal(v0): u64
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u64 <- ByVal(v1): u64
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (const 1_u64, const true)
// CHECK-NEXT:     v2 = iconst.i64 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u64 <- ByVal(v2): u64
// CHECK-NEXT:     v3 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v2, v3  ; v2 = 1
// CHECK-NEXT:     v4 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(8) }, None): bool <- ByVal(v4): bool
// CHECK-NEXT:     v5 = stack_addr.i16 ss0+8
// CHECK-NEXT:     store notrap v4, v5  ; v4 = 1
// CHECK-NEXT: ; _5 = Sub(const core::num::<impl u64>::MAX, const 2_u64)
// CHECK-NEXT:     v6 = iconst.i64 -1
// CHECK-NEXT:     v7 = iconst.i64 2
// CHECK-NEXT:     v8 = isub v6, v7  ; v6 = -1, v7 = 2
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): u64 <- ByVal(v8): u64
// CHECK-NEXT: ; _4 = (move _5, const false)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u64 <- ByVal(v8): u64
// CHECK-NEXT:     v9 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v8, v9
// CHECK-NEXT:     v10 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(8) }, None): bool <- ByVal(v10): bool
// CHECK-NEXT:     v11 = stack_addr.i16 ss1+8
// CHECK-NEXT:     store notrap v10, v11  ; v10 = 0
// CHECK-NEXT: ; _7 = Sub(const core::num::<impl u64>::MAX, const 1_u64)
// CHECK-NEXT:     v12 = iconst.i64 -1
// CHECK-NEXT:     v13 = iconst.i64 1
// CHECK-NEXT:     v14 = isub v12, v13  ; v12 = -1, v13 = 1
// CHECK-NEXT: ; write_cvalue: Var(_7, var5): u64 <- ByVal(v14): u64
// CHECK-NEXT: ; _6 = (move _7, const true)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u64 <- ByVal(v14): u64
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v14, v15
// CHECK-NEXT:     v16 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(8) }, None): bool <- ByVal(v16): bool
// CHECK-NEXT:     v17 = stack_addr.i16 ss2+8
// CHECK-NEXT:     store notrap v16, v17  ; v16 = 1
// CHECK-NEXT: ; _68 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v20 = iadd.i64 v18, v19
// CHECK-NEXT:     v21 = icmp ult v20, v18
// CHECK-NEXT: ; write_cvalue: VarPair(_68, var60, var61): (u64, bool) <- ByValPair(v20, v21): (u64, bool)
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
// CHECK-NEXT:     v229 -> v26
// CHECK-NEXT:     v230 -> v26
// CHECK-NEXT:     v235 -> v26
// CHECK-NEXT:     v236 -> v26
// CHECK-NEXT: ; write_cvalue: Var(_13, var11): u8 <- ByVal(v26): u8
// CHECK-NEXT: ; _81 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v29 = uextend.i128 v27
// CHECK-NEXT:     v30 = uextend.i128 v28
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss10), offset: Offset32(0) }, None): u128 <- ByVal(v29): u128
// CHECK-NEXT:     v31 = stack_addr.i16 ss10
// CHECK-NEXT:     store notrap v29, v31
// CHECK-NEXT:     v32 = stack_addr.i16 ss10
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss11), offset: Offset32(0) }, None): u128 <- ByVal(v30): u128
// CHECK-NEXT:     v33 = stack_addr.i16 ss11
// CHECK-NEXT:     store notrap v30, v33
// CHECK-NEXT:     v34 = stack_addr.i16 ss11
// CHECK-NEXT:     v35 = stack_addr.i16 ss9
// CHECK-NEXT: ; lib_call __gbc_u128_mulo
// CHECK-NEXT:     call fn0(v35, v32, v34)
// CHECK-NEXT:     v36 = stack_addr.i16 ss9
// CHECK-NEXT:     v37 = load.i128 notrap v36
// CHECK-NEXT:     v38 = stack_addr.i16 ss9+16
// CHECK-NEXT:     v39 = load.i8 notrap v38
// CHECK-NEXT:     v40 = iconst.i64 64
// CHECK-NEXT:     v41 = uextend.i128 v40  ; v40 = 64
// CHECK-NEXT:     v42 = ushr v37, v41
// CHECK-NEXT:     v43 = ireduce.i64 v42
// CHECK-NEXT:     v44 = iconst.i64 0
// CHECK-NEXT:     v45 = icmp ne v43, v44  ; v44 = 0
// CHECK-NEXT:     v46 = bor v39, v45
// CHECK-NEXT:     v47 = ireduce.i64 v37
// CHECK-NEXT:     v227 -> v47
// CHECK-NEXT:     v228 -> v47
// CHECK-NEXT: ; write_cvalue: VarPair(_81, var74, var75): (u64, bool) <- ByValPair(v47, v46): (u64, bool)
// CHECK-NEXT: ; _79 = copy (_81.0: u64)
// CHECK-NEXT: ; write_cvalue: Var(_79, var72): u64 <- ByVal(v47): u64
// CHECK-NEXT: ; _80 = copy (_81.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_80, var73): bool <- ByVal(v46): bool
// CHECK-NEXT: ; _77 = (copy _79, copy _80)
// CHECK-NEXT: ; write_cvalue: Var(_77, var69): u64 <- ByVal(v47): u64
// CHECK-NEXT: ; write_cvalue: Var(_77, var70): bool <- ByVal(v46): bool
// CHECK-NEXT: ; _75 = copy (_77.0: u64)
// CHECK-NEXT: ; write_cvalue: Var(_75, var67): u64 <- ByVal(v47): u64
// CHECK-NEXT: ; _76 = copy (_77.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_76, var68): bool <- ByVal(v46): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _76)
// CHECK-NEXT:     brif v46, block31, block33
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _47)
// CHECK-NEXT:     brif.i8 v48, block4, block7
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _90 = SubWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v51 = isub.i64 v49, v50
// CHECK-NEXT:     v52 = icmp ugt v51, v49
// CHECK-NEXT: ; write_cvalue: VarPair(_90, var84, var85): (u64, bool) <- ByValPair(v51, v52): (u64, bool)
// CHECK-NEXT: ; _88 = copy (_90.0: u64)
// CHECK-NEXT: ; write_cvalue: Var(_88, var82): u64 <- ByVal(v51): u64
// CHECK-NEXT: ; _89 = copy (_90.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_89, var83): bool <- ByVal(v52): bool
// CHECK-NEXT: ; _53 = (copy _88, copy _89)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): u64 <- ByVal(v51): u64
// CHECK-NEXT:     v53 = stack_addr.i16 ss7
// CHECK-NEXT:     store notrap v51, v53
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(8) }, None): bool <- ByVal(v52): bool
// CHECK-NEXT:     v54 = stack_addr.i16 ss7+8
// CHECK-NEXT:     store notrap v52, v54
// CHECK-NEXT: ; _52 = &_53
// CHECK-NEXT:     v55 = stack_addr.i16 ss7
// CHECK-NEXT: ; write_cvalue: Var(_52, var46): &'{erased} (u64, bool) <- ByVal(v55): &'{erased} (u64, bool)
// CHECK-NEXT: ; _54 = &_4
// CHECK-NEXT:     v56 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_54, var47): &'{erased} (u64, bool) <- ByVal(v56): &'{erased} (u64, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _51 = <(u64, bool) as core::cmp::PartialEq>::eq(move _52, move _54)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u64, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u64, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v57 = call fn1(v55, v56)
// CHECK-NEXT:     v58 -> v57
// CHECK-NEXT: ; write_cvalue: Var(_51, var45): bool <- ByVal(v57): bool
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _51)
// CHECK-NEXT:     brif.i8 v58, block6, block7
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _93 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v61 = uextend.i128 v59
// CHECK-NEXT:     v62 = uextend.i128 v60
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss13), offset: Offset32(0) }, None): u128 <- ByVal(v61): u128
// CHECK-NEXT:     v63 = stack_addr.i16 ss13
// CHECK-NEXT:     store notrap v61, v63
// CHECK-NEXT:     v64 = stack_addr.i16 ss13
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss14), offset: Offset32(0) }, None): u128 <- ByVal(v62): u128
// CHECK-NEXT:     v65 = stack_addr.i16 ss14
// CHECK-NEXT:     store notrap v62, v65
// CHECK-NEXT:     v66 = stack_addr.i16 ss14
// CHECK-NEXT:     v67 = stack_addr.i16 ss12
// CHECK-NEXT: ; lib_call __gbc_u128_mulo
// CHECK-NEXT:     call fn2(v67, v64, v66)
// CHECK-NEXT:     v68 = stack_addr.i16 ss12
// CHECK-NEXT:     v69 = load.i128 notrap v68
// CHECK-NEXT:     v70 = stack_addr.i16 ss12+16
// CHECK-NEXT:     v71 = load.i8 notrap v70
// CHECK-NEXT:     v72 = iconst.i64 64
// CHECK-NEXT:     v73 = uextend.i128 v72  ; v72 = 64
// CHECK-NEXT:     v74 = ushr v69, v73
// CHECK-NEXT:     v75 = ireduce.i64 v74
// CHECK-NEXT:     v76 = iconst.i64 0
// CHECK-NEXT:     v77 = icmp ne v75, v76  ; v76 = 0
// CHECK-NEXT:     v78 = bor v71, v77
// CHECK-NEXT:     v79 = ireduce.i64 v69
// CHECK-NEXT: ; write_cvalue: VarPair(_93, var88, var89): (u64, bool) <- ByValPair(v79, v78): (u64, bool)
// CHECK-NEXT: ; _91 = copy (_93.0: u64)
// CHECK-NEXT: ; write_cvalue: Var(_91, var86): u64 <- ByVal(v79): u64
// CHECK-NEXT: ; _92 = copy (_93.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_92, var87): bool <- ByVal(v78): bool
// CHECK-NEXT: ; _56 = (copy _91, copy _92)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): u64 <- ByVal(v79): u64
// CHECK-NEXT:     v80 = stack_addr.i16 ss8
// CHECK-NEXT:     store notrap v79, v80
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(8) }, None): bool <- ByVal(v78): bool
// CHECK-NEXT:     v81 = stack_addr.i16 ss8+8
// CHECK-NEXT:     store notrap v78, v81
// CHECK-NEXT: ; _55 = &_56
// CHECK-NEXT:     v82 = stack_addr.i16 ss8
// CHECK-NEXT: ; write_cvalue: Var(_55, var48): &'{erased} (u64, bool) <- ByVal(v82): &'{erased} (u64, bool)
// CHECK-NEXT: ; _57 = &_6
// CHECK-NEXT:     v83 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_57, var49): &'{erased} (u64, bool) <- ByVal(v83): &'{erased} (u64, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _46 = <(u64, bool) as core::cmp::PartialEq>::eq(move _55, move _57)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u64, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u64, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v84 = call fn3(v82, v83)
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v84): bool
// CHECK-NEXT:     jump block8(v84)
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _46 = const false
// CHECK-NEXT:     v85 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v85): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block8(v85)  ; v85 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block8(v86: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _45 = move _46 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_45, var40): u8 <- ByVal(v86): u8
// CHECK-NEXT: ; _44 = Shl(move _45, const 6_i32)
// CHECK-NEXT:     v87 = iconst.i32 6
// CHECK-NEXT:     v88 = ishl v86, v87  ; v87 = 6
// CHECK-NEXT: ; write_cvalue: Var(_44, var39): u8 <- ByVal(v88): u8
// CHECK-NEXT: ; _8 = BitOr(move _9, move _44)
// CHECK-NEXT:     v90 = bor.i8 v89, v88
// CHECK-NEXT:     v207 -> v90
// CHECK-NEXT:     v208 -> v90
// CHECK-NEXT:     v209 -> v90
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): u8 <- ByVal(v90): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _62 = core::intrinsics::ctlz::<u64>(copy _1)
// CHECK-NEXT:     v92 = clz.i64 v91
// CHECK-NEXT:     v93 = ireduce.i32 v92
// CHECK-NEXT:     v190 -> v93
// CHECK-NEXT: ; write_cvalue: Var(_62, var54): u32 <- ByVal(v93): u32
// CHECK-NEXT:     jump block37
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _64 = core::intrinsics::cttz::<u64>(copy _1)
// CHECK-NEXT:     v95 = ctz.i64 v94
// CHECK-NEXT:     v96 = ireduce.i32 v95
// CHECK-NEXT:     v193 -> v96
// CHECK-NEXT: ; write_cvalue: Var(_64, var56): u32 <- ByVal(v96): u32
// CHECK-NEXT:     jump block38
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _65 = core::intrinsics::ctpop::<u64>(copy _1)
// CHECK-NEXT:     v98 = popcnt.i64 v97
// CHECK-NEXT:     v99 = ireduce.i32 v98
// CHECK-NEXT:     v196 -> v99
// CHECK-NEXT: ; write_cvalue: Var(_65, var57): u32 <- ByVal(v99): u32
// CHECK-NEXT:     jump block39
// CHECK-EMPTY:
// CHECK-NEXT: block11(v206: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = const false
// CHECK-NEXT:     v100 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v100): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v100, v206)  ; v100 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block12(v101: i8, v104: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _59 = move _60 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_59, var51): u8 <- ByVal(v101): u8
// CHECK-NEXT: ; _58 = Shl(move _59, const 7_i32)
// CHECK-NEXT:     v102 = iconst.i32 7
// CHECK-NEXT:     v103 = ishl v101, v102  ; v102 = 7
// CHECK-NEXT: ; write_cvalue: Var(_58, var50): u8 <- ByVal(v103): u8
// CHECK-NEXT: ; _0 = BitOr(move _8, move _58)
// CHECK-NEXT:     v105 = bor v104, v103
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v105): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v105
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = &_17
// CHECK-NEXT:     v106 = stack_addr.i16 ss3
// CHECK-NEXT: ; write_cvalue: Var(_16, var14): &'{erased} core::option::Option<u64> <- ByVal(v106): &'{erased} core::option::Option<u64>
// CHECK-NEXT: ; _72 = discriminant((*_16))
// CHECK-NEXT:     v107 = load.i16 notrap v106
// CHECK-NEXT: ; write_cvalue: Var(_72, var64): isize <- ByVal(v107): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _72)
// CHECK-NEXT:     v108 = uextend.i32 v107
// CHECK-NEXT:     br_table v108, block23, [block21, block22]
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _17 = core::option::Option::<u64>::None
// CHECK-NEXT:     v109 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u16 <- ByVal(v109): u16
// CHECK-NEXT:     v110 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v109, v110  ; v109 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _69 = AddUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v113 = iadd.i64 v111, v112
// CHECK-NEXT: ; write_cvalue: Var(_69, var62): u64 <- ByVal(v113): u64
// CHECK-NEXT: ; _17 = core::option::Option::<u64>::Some(move _69)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(2) }, None): u64 <- ByVal(v113): u64
// CHECK-NEXT:     v114 = stack_addr.i16 ss3+2
// CHECK-NEXT:     store notrap v113, v114
// CHECK-NEXT:     v115 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u16 <- ByVal(v115): u16
// CHECK-NEXT:     v116 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v115, v116  ; v115 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block16(v117: i8, v210: i64, v211: i64):
// CHECK-NEXT:     v111 -> v210
// CHECK-NEXT:     v216 -> v210
// CHECK-NEXT:     v217 -> v210
// CHECK-NEXT:     v220 -> v210
// CHECK-NEXT:     v112 -> v211
// CHECK-NEXT:     v218 -> v211
// CHECK-NEXT:     v219 -> v211
// CHECK-NEXT:     v221 -> v211
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _66)
// CHECK-NEXT:     brif v117, block14, block15
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
// CHECK-NEXT:     v118 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_66, var58): bool <- ByVal(v118): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block16(v118, v212, v214)  ; v118 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block19:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _66 = const false
// CHECK-NEXT:     v119 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_66, var58): bool <- ByVal(v119): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block16(v119, v213, v215)  ; v119 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block20(v120: i8, v123: i64, v124: i64):
// CHECK-NEXT:     v133 -> v123
// CHECK-NEXT:     v224 -> v123
// CHECK-NEXT:     v134 -> v124
// CHECK-NEXT:     v225 -> v124
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = Not(move _71)
// CHECK-NEXT:     v121 = iconst.i8 0
// CHECK-NEXT:     v122 = icmp eq v120, v121  ; v121 = 0
// CHECK-NEXT:     v222 -> v122
// CHECK-NEXT:     v223 -> v122
// CHECK-NEXT: ; write_cvalue: Var(_15, var13): bool <- ByVal(v122): bool
// CHECK-NEXT: ; _14 = move _15 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_14, var12): u8 <- ByVal(v122): u8
// CHECK-NEXT: ; _73 = Lt(copy _1, copy _2)
// CHECK-NEXT:     v125 = icmp ult v123, v124
// CHECK-NEXT: ; write_cvalue: Var(_73, var65): bool <- ByVal(v125): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _73)
// CHECK-NEXT:     brif v125, block25, block26
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _71 = const false
// CHECK-NEXT:     v126 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_71, var63): bool <- ByVal(v126): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block20(v126, v216, v218)  ; v126 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _71 = const true
// CHECK-NEXT:     v127 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_71, var63): bool <- ByVal(v127): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block20(v127, v217, v219)  ; v127 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block23 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; unreachable
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block24(v199: i8, v200: i64, v201: i64):
// CHECK-NEXT:     v25 -> v199
// CHECK-NEXT:     v27 -> v200
// CHECK-NEXT:     v231 -> v200
// CHECK-NEXT:     v232 -> v200
// CHECK-NEXT:     v237 -> v200
// CHECK-NEXT:     v238 -> v200
// CHECK-NEXT:     v28 -> v201
// CHECK-NEXT:     v233 -> v201
// CHECK-NEXT:     v234 -> v201
// CHECK-NEXT:     v239 -> v201
// CHECK-NEXT:     v240 -> v201
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _21 = &_22
// CHECK-NEXT:     v128 = stack_addr.i16 ss4
// CHECK-NEXT: ; write_cvalue: Var(_21, var18): &'{erased} core::option::Option<u64> <- ByVal(v128): &'{erased} core::option::Option<u64>
// CHECK-NEXT: ; _23 = const integer_methods::promoted[0]
// CHECK-NEXT:     v129 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_23, var19): &'{erased} core::option::Option<u64> <- ByVal(v129): &'{erased} core::option::Option<u64>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _20 = <core::option::Option<u64> as core::cmp::PartialEq>::eq(move _21, move _23)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u64>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u64>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v130 = call fn4(v128, v129)
// CHECK-NEXT:     v22 -> v130
// CHECK-NEXT: ; write_cvalue: Var(_20, var17): bool <- ByVal(v130): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block25:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = core::option::Option::<u64>::None
// CHECK-NEXT:     v131 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u16 <- ByVal(v131): u16
// CHECK-NEXT:     v132 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v131, v132  ; v131 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block24(v222, v224, v225)
// CHECK-EMPTY:
// CHECK-NEXT: block26:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _74 = SubUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v135 = isub.i64 v133, v134
// CHECK-NEXT: ; write_cvalue: Var(_74, var66): u64 <- ByVal(v135): u64
// CHECK-NEXT: ; _22 = core::option::Option::<u64>::Some(move _74)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(2) }, None): u64 <- ByVal(v135): u64
// CHECK-NEXT:     v136 = stack_addr.i16 ss4+2
// CHECK-NEXT:     store notrap v135, v136
// CHECK-NEXT:     v137 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u16 <- ByVal(v137): u16
// CHECK-NEXT:     v138 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v137, v138  ; v137 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block24(v223, v133, v134)
// CHECK-EMPTY:
// CHECK-NEXT: block27:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _27 = &_28
// CHECK-NEXT:     v139 = stack_addr.i16 ss5
// CHECK-NEXT: ; write_cvalue: Var(_27, var23): &'{erased} core::option::Option<u64> <- ByVal(v139): &'{erased} core::option::Option<u64>
// CHECK-NEXT: ; _84 = discriminant((*_27))
// CHECK-NEXT:     v140 = load.i16 notrap v139
// CHECK-NEXT: ; write_cvalue: Var(_84, var77): isize <- ByVal(v140): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _84)
// CHECK-NEXT:     v141 = uextend.i32 v140
// CHECK-NEXT:     br_table v141, block23, [block35, block36]
// CHECK-EMPTY:
// CHECK-NEXT: block28:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _28 = core::option::Option::<u64>::None
// CHECK-NEXT:     v142 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): u16 <- ByVal(v142): u16
// CHECK-NEXT:     v143 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v142, v143  ; v142 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block27
// CHECK-EMPTY:
// CHECK-NEXT: block29:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _28 = core::option::Option::<u64>::Some(copy _75)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(2) }, None): u64 <- ByVal(v144): u64
// CHECK-NEXT:     v145 = stack_addr.i16 ss5+2
// CHECK-NEXT:     store.i64 notrap v144, v145
// CHECK-NEXT:     v146 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): u16 <- ByVal(v146): u16
// CHECK-NEXT:     v147 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v146, v147  ; v146 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block27
// CHECK-EMPTY:
// CHECK-NEXT: block30(v148: i8, v226: i64):
// CHECK-NEXT:     v144 -> v226
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _78)
// CHECK-NEXT:     brif v148, block28, block29
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
// CHECK-NEXT:     v149 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_78, var71): bool <- ByVal(v149): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block30(v149, v227)  ; v149 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block33:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _78 = const false
// CHECK-NEXT:     v150 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_78, var71): bool <- ByVal(v150): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block30(v150, v228)  ; v150 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block34(v151: i8, v156: i8, v158: i64, v159: i64):
// CHECK-NEXT:     v49 -> v158
// CHECK-NEXT:     v59 -> v158
// CHECK-NEXT:     v91 -> v158
// CHECK-NEXT:     v203 -> v158
// CHECK-NEXT:     v204 -> v158
// CHECK-NEXT:     v205 -> v158
// CHECK-NEXT:     v97 -> v205
// CHECK-NEXT:     v94 -> v204
// CHECK-NEXT:     v50 -> v159
// CHECK-NEXT:     v60 -> v159
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _26 = Not(move _83)
// CHECK-NEXT:     v152 = iconst.i8 0
// CHECK-NEXT:     v153 = icmp eq v151, v152  ; v152 = 0
// CHECK-NEXT: ; write_cvalue: Var(_26, var22): bool <- ByVal(v153): bool
// CHECK-NEXT: ; _25 = move _26 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_25, var21): u8 <- ByVal(v153): u8
// CHECK-NEXT: ; _24 = Shl(move _25, const 2_i32)
// CHECK-NEXT:     v154 = iconst.i32 2
// CHECK-NEXT:     v155 = ishl v153, v154  ; v154 = 2
// CHECK-NEXT: ; write_cvalue: Var(_24, var20): u8 <- ByVal(v155): u8
// CHECK-NEXT: ; _12 = BitOr(move _13, move _24)
// CHECK-NEXT:     v157 = bor v156, v155
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): u8 <- ByVal(v157): u8
// CHECK-NEXT: ; _32 = Add(copy _1, copy _2)
// CHECK-NEXT:     v160 = iadd v158, v159
// CHECK-NEXT: ; write_cvalue: Var(_32, var27): u64 <- ByVal(v160): u64
// CHECK-NEXT: ; _33 = copy (_3.0: u64)
// CHECK-NEXT: ; write_cvalue: Var(_33, var28): u64 <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u64
// CHECK-NEXT:     v161 = stack_addr.i16 ss0
// CHECK-NEXT:     v162 = load.i64 notrap v161
// CHECK-NEXT: ; _31 = Eq(move _32, move _33)
// CHECK-NEXT:     v163 = icmp eq v160, v162
// CHECK-NEXT: ; write_cvalue: Var(_31, var26): bool <- ByVal(v163): bool
// CHECK-NEXT: ; _30 = move _31 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_30, var25): u8 <- ByVal(v163): u8
// CHECK-NEXT: ; _29 = Shl(move _30, const 3_i32)
// CHECK-NEXT:     v164 = iconst.i32 3
// CHECK-NEXT:     v165 = ishl v163, v164  ; v164 = 3
// CHECK-NEXT: ; write_cvalue: Var(_29, var24): u8 <- ByVal(v165): u8
// CHECK-NEXT: ; _11 = BitOr(move _12, move _29)
// CHECK-NEXT:     v166 = bor v157, v165
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): u8 <- ByVal(v166): u8
// CHECK-NEXT: ; _37 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v167 = isub v158, v159
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): u64 <- ByVal(v167): u64
// CHECK-NEXT: ; _38 = copy (_4.0: u64)
// CHECK-NEXT: ; write_cvalue: Var(_38, var33): u64 <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u64
// CHECK-NEXT:     v168 = stack_addr.i16 ss1
// CHECK-NEXT:     v169 = load.i64 notrap v168
// CHECK-NEXT: ; _36 = Eq(move _37, move _38)
// CHECK-NEXT:     v170 = icmp eq v167, v169
// CHECK-NEXT: ; write_cvalue: Var(_36, var31): bool <- ByVal(v170): bool
// CHECK-NEXT: ; _35 = move _36 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_35, var30): u8 <- ByVal(v170): u8
// CHECK-NEXT: ; _34 = Shl(move _35, const 4_i32)
// CHECK-NEXT:     v171 = iconst.i32 4
// CHECK-NEXT:     v172 = ishl v170, v171  ; v171 = 4
// CHECK-NEXT: ; write_cvalue: Var(_34, var29): u8 <- ByVal(v172): u8
// CHECK-NEXT: ; _10 = BitOr(move _11, move _34)
// CHECK-NEXT:     v173 = bor v166, v172
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): u8 <- ByVal(v173): u8
// CHECK-NEXT: ; _42 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v174 = imul v158, v159
// CHECK-NEXT: ; write_cvalue: Var(_42, var37): u64 <- ByVal(v174): u64
// CHECK-NEXT: ; _43 = copy (_6.0: u64)
// CHECK-NEXT: ; write_cvalue: Var(_43, var38): u64 <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u64
// CHECK-NEXT:     v175 = stack_addr.i16 ss2
// CHECK-NEXT:     v176 = load.i64 notrap v175
// CHECK-NEXT: ; _41 = Eq(move _42, move _43)
// CHECK-NEXT:     v177 = icmp eq v174, v176
// CHECK-NEXT: ; write_cvalue: Var(_41, var36): bool <- ByVal(v177): bool
// CHECK-NEXT: ; _40 = move _41 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_40, var35): u8 <- ByVal(v177): u8
// CHECK-NEXT: ; _39 = Shl(move _40, const 5_i32)
// CHECK-NEXT:     v178 = iconst.i32 5
// CHECK-NEXT:     v179 = ishl v177, v178  ; v178 = 5
// CHECK-NEXT: ; write_cvalue: Var(_39, var34): u8 <- ByVal(v179): u8
// CHECK-NEXT: ; _9 = BitOr(move _10, move _39)
// CHECK-NEXT:     v180 = bor v173, v179
// CHECK-NEXT:     v89 -> v180
// CHECK-NEXT:     v202 -> v180
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): u8 <- ByVal(v180): u8
// CHECK-NEXT: ; _87 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v181 = iadd v158, v159
// CHECK-NEXT:     v182 = icmp ult v181, v158
// CHECK-NEXT: ; write_cvalue: VarPair(_87, var80, var81): (u64, bool) <- ByValPair(v181, v182): (u64, bool)
// CHECK-NEXT: ; _85 = copy (_87.0: u64)
// CHECK-NEXT: ; write_cvalue: Var(_85, var78): u64 <- ByVal(v181): u64
// CHECK-NEXT: ; _86 = copy (_87.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_86, var79): bool <- ByVal(v182): bool
// CHECK-NEXT: ; _49 = (copy _85, copy _86)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): u64 <- ByVal(v181): u64
// CHECK-NEXT:     v183 = stack_addr.i16 ss6
// CHECK-NEXT:     store notrap v181, v183
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(8) }, None): bool <- ByVal(v182): bool
// CHECK-NEXT:     v184 = stack_addr.i16 ss6+8
// CHECK-NEXT:     store notrap v182, v184
// CHECK-NEXT: ; _48 = &_49
// CHECK-NEXT:     v185 = stack_addr.i16 ss6
// CHECK-NEXT: ; write_cvalue: Var(_48, var43): &'{erased} (u64, bool) <- ByVal(v185): &'{erased} (u64, bool)
// CHECK-NEXT: ; _50 = &_3
// CHECK-NEXT:     v186 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_50, var44): &'{erased} (u64, bool) <- ByVal(v186): &'{erased} (u64, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _47 = <(u64, bool) as core::cmp::PartialEq>::eq(move _48, move _50)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u64, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u64, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v187 = call fn5(v185, v186)
// CHECK-NEXT:     v48 -> v187
// CHECK-NEXT: ; write_cvalue: Var(_47, var42): bool <- ByVal(v187): bool
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block35:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _83 = const false
// CHECK-NEXT:     v188 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_83, var76): bool <- ByVal(v188): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block34(v188, v229, v231, v233)  ; v188 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block36:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _83 = const true
// CHECK-NEXT:     v189 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_83, var76): bool <- ByVal(v189): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block34(v189, v230, v232, v234)  ; v189 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block37:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _61 = Eq(move _62, const 0_u32)
// CHECK-NEXT:     v191 = iconst.i32 0
// CHECK-NEXT:     v192 = icmp.i32 eq v190, v191  ; v191 = 0
// CHECK-NEXT: ; write_cvalue: Var(_61, var53): bool <- ByVal(v192): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _61)
// CHECK-NEXT:     brif.i32 v190, block11(v207), block9
// CHECK-EMPTY:
// CHECK-NEXT: block38:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _63 = Eq(move _64, const 0_u32)
// CHECK-NEXT:     v194 = iconst.i32 0
// CHECK-NEXT:     v195 = icmp.i32 eq v193, v194  ; v194 = 0
// CHECK-NEXT: ; write_cvalue: Var(_63, var55): bool <- ByVal(v195): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _63)
// CHECK-NEXT:     brif.i32 v193, block11(v208), block10
// CHECK-EMPTY:
// CHECK-NEXT: block39:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = Eq(move _65, const 64_u32)
// CHECK-NEXT:     v197 = iconst.i32 64
// CHECK-NEXT:     v198 = icmp.i32 eq v196, v197  ; v197 = 64
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v198): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v198, v209)
// CHECK-NEXT: }
