#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn integer_methods(x: i8, y: i8) -> u8 {
    let add = (i8::MIN + 1, true);
    let sub = (i8::MAX - 2, false);
    let mul = (-2, true);
    (x.checked_add(y).is_none() as u8)
        | ((x.checked_sub(y) == Some(i8::MAX - 2)) as u8) << 1
        | (x.checked_mul(y).is_none() as u8) << 2
        | ((x.wrapping_add(y) == add.0) as u8) << 3
        | ((x.wrapping_sub(y) == sub.0) as u8) << 4
        | ((x.wrapping_mul(y) == mul.0) as u8) << 5
        | ((x.overflowing_add(y) == add
            && x.overflowing_sub(y) == sub
            && x.overflowing_mul(y) == mul) as u8) << 6
        | ((x.leading_zeros() == 1
            && x.trailing_zeros() == 0
            && x.count_ones() == 7) as u8) << 7
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    (integer_methods(i8::MAX, 2) == 255) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCs4jUVW15fqGy_10integer_i8
// CHECK-NEXT: function u0:0() system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCs4jUVW15fqGy_10integer_i8
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
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTabENtNtB7_3cmp9PartialEq2eqCs4jUVW15fqGy_10integer_i8
// CHECK-NEXT: function u0:1(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTabENtNtB7_3cmp9PartialEq2eqCs4jUVW15fqGy_10integer_i8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} (i8, bool)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} (i8, bool)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} (i8, bool)             2b 2                storage=ss0
// CHECK-NEXT: ; stack _2    &'{erased} (i8, bool)             2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _4    &'{erased} i8                     2b 2                var=3
// CHECK-NEXT: ; ssa   _5    &'{erased} i8                     2b 2                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} bool                   2b 2                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} bool                   2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(1:29990 ~ core[..]::cmp::impls::{impl#28}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (i8, bool) <- ByVal(v0): &'{erased} (i8, bool)
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (i8, bool) <- ByVal(v1): &'{erased} (i8, bool)
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = &((*_1).0: U)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): &'{erased} i8 <- ByVal(v5): &'{erased} i8
// CHECK-NEXT: ; _5 = &((*_2).0: U)
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): &'{erased} i8 <- ByVal(v7): &'{erased} i8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = <U as core::cmp::PartialEq>::eq(move _4, move _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &i8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &i8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptionaENtNtB7_3cmp9PartialEq2eqCs4jUVW15fqGy_10integer_i8
// CHECK-NEXT: function u0:2(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptionaENtNtB7_3cmp9PartialEq2eqCs4jUVW15fqGy_10integer_i8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} core::option::Option<i8>
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} core::option::Option<i8>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} core::option::Option<i8>    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} core::option::Option<i8>    2b 2                var=3
// CHECK-NEXT: ; ssa   _3    (&'{erased} core::option::Option<i8>, &'{erased} core::option::Option<i8>)    4b 2                var=(4, 5)
// CHECK-NEXT: ; ssa   _4    isize                             2b 2                var=6
// CHECK-NEXT: ; ssa   _5    isize                             2b 2                var=7
// CHECK-NEXT: ; ssa   _6    isize                             2b 2                var=8
// CHECK-NEXT: ; ssa   _7    &'{erased} i8                     2b 2                var=9
// CHECK-NEXT: ; ssa   _8    &'{erased} i8                     2b 2                var=10
// CHECK-NEXT: ; ssa   _9    &'{erased} core::option::Option<i8>    2b 2                var=11
// CHECK-NEXT: ; ssa   _10   &'{erased} core::option::Option<i8>    2b 2                var=12
// CHECK-NEXT: ; ssa   _11   &'{erased} core::option::Option<i8>    2b 2                var=13
// CHECK-NEXT: ; stack _12   &'{erased} core::option::Option<i8>    2b 2                storage=ss0
// CHECK-NEXT: ; stack _13   &'{erased} core::option::Option<i8>    2b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(1:29990 ~ core[..]::cmp::impls::{impl#28}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v18 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v7 -> v1
// CHECK-NEXT:     v11 -> v1
// CHECK-NEXT:     v24 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} core::option::Option<i8> <- ByVal(v0): &'{erased} core::option::Option<i8>
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} core::option::Option<i8> <- ByVal(v1): &'{erased} core::option::Option<i8>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): &'{erased} core::option::Option<i8> <- ByVal(v2): &'{erased} core::option::Option<i8>
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): &'{erased} core::option::Option<i8> <- ByVal(v3): &'{erased} core::option::Option<i8>
// CHECK-NEXT: ; _9 = no_retag copy (_3.0: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Var(_9, var11): &'{erased} core::option::Option<i8> <- ByVal(v2): &'{erased} core::option::Option<i8>
// CHECK-NEXT: ; _6 = discriminant((*_9))
// CHECK-NEXT:     v4 = load.i8 notrap v2
// CHECK-NEXT:     v5 = uextend.i16 v4
// CHECK-NEXT: ; write_cvalue: Var(_6, var8): isize <- ByVal(v5): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     v6 = uextend.i32 v5
// CHECK-NEXT:     br_table v6, block2, [block4, block3]
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
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): &'{erased} core::option::Option<i8> <- ByVal(v7): &'{erased} core::option::Option<i8>
// CHECK-NEXT: ; _4 = discriminant((*_10))
// CHECK-NEXT:     v8 = load.i8 notrap v7
// CHECK-NEXT:     v9 = uextend.i16 v8
// CHECK-NEXT: ; write_cvalue: Var(_4, var6): isize <- ByVal(v9): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _4)
// CHECK-NEXT:     v10 = uextend.i32 v9
// CHECK-NEXT:     br_table v10, block2, [block7, block8]
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _11 = no_retag copy (_3.1: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): &'{erased} core::option::Option<i8> <- ByVal(v11): &'{erased} core::option::Option<i8>
// CHECK-NEXT: ; _5 = discriminant((*_11))
// CHECK-NEXT:     v12 = load.i8 notrap v11
// CHECK-NEXT:     v13 = uextend.i16 v12
// CHECK-NEXT: ; write_cvalue: Var(_5, var7): isize <- ByVal(v13): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _5)
// CHECK-NEXT:     v14 = uextend.i32 v13
// CHECK-NEXT:     br_table v14, block2, [block5, block6]
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const true
// CHECK-NEXT:     v15 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v15): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v15  ; v15 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const false
// CHECK-NEXT:     v16 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v16): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v16  ; v16 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const false
// CHECK-NEXT:     v17 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v17): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v17  ; v17 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _12 = no_retag copy (_3.0: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} core::option::Option<i8> <- ByVal(v18): &'{erased} core::option::Option<i8>
// CHECK-NEXT:     v19 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v18, v19
// CHECK-NEXT: ; _7 = &(((*_12) as Some).0: T)
// CHECK-NEXT:     v20 = stack_addr.i16 ss0
// CHECK-NEXT:     v21 = load.i16 notrap v20
// CHECK-NEXT:     v22 = iconst.i16 1
// CHECK-NEXT:     v23 = iadd v21, v22  ; v22 = 1
// CHECK-NEXT: ; write_cvalue: Var(_7, var9): &'{erased} i8 <- ByVal(v23): &'{erased} i8
// CHECK-NEXT: ; _13 = no_retag copy (_3.1: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} core::option::Option<i8> <- ByVal(v24): &'{erased} core::option::Option<i8>
// CHECK-NEXT:     v25 = stack_addr.i16 ss1
// CHECK-NEXT:     store.i16 notrap v24, v25
// CHECK-NEXT: ; _8 = &(((*_13) as Some).0: T)
// CHECK-NEXT:     v26 = stack_addr.i16 ss1
// CHECK-NEXT:     v27 = load.i16 notrap v26
// CHECK-NEXT:     v28 = iconst.i16 1
// CHECK-NEXT:     v29 = iadd v27, v28  ; v28 = 1
// CHECK-NEXT: ; write_cvalue: Var(_8, var10): &'{erased} i8 <- ByVal(v29): &'{erased} i8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::cmp::PartialEq>::eq(copy _7, copy _8)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &i8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &i8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v30 = call fn0(v23, v29)
// CHECK-NEXT:     v31 -> v30
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v30): bool
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v31
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCs4jUVW15fqGy_10integer_i8
// CHECK-NEXT: function u0:3(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCs4jUVW15fqGy_10integer_i8
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
// CHECK-NEXT: ; symbol _RNvXsq_NtNtCs40R3J0iFVxM_4core3cmp5implsaNtB7_9PartialEq2eqCs4jUVW15fqGy_10integer_i8
// CHECK-NEXT: function u0:4(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsq_NtNtCs40R3J0iFVxM_4core3cmp5implsaNtB7_9PartialEq2eqCs4jUVW15fqGy_10integer_i8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} i8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} i8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} i8                     2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} i8                     2b 2                var=3
// CHECK-NEXT: ; ssa   _3    i8                                1b 1                var=4
// CHECK-NEXT: ; ssa   _4    i8                                1b 1                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} i8 <- ByVal(v0): &'{erased} i8
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} i8 <- ByVal(v1): &'{erased} i8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): i8 <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): i8
// CHECK-NEXT:     v3 = load.i8 notrap v2
// CHECK-NEXT: ; _4 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): i8 <- ByRef(Pointer { base: Addr(v4), offset: Offset32(0) }, None): i8
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
// CHECK-NEXT:     sig0 = (i8, i8) -> i8 system_v
// CHECK-NEXT:     fn0 = colocated u0:6 sig0 ; Instance { def: Item(DefId(0:3 ~ integer_i8[..]::integer_methods)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = integer_methods(const core::num::<impl i8>::MAX, const 2_i8)
// CHECK-NEXT:     v0 = iconst.i8 127
// CHECK-NEXT:     v1 = iconst.i8 2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: i8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(i8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776123356184577 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: i8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(i8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776123356184577 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0(v0, v1)  ; v0 = 127, v1 = 2
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
// CHECK-NEXT: function u0:6(i8, i8) -> i8 system_v {
// CHECK-NEXT: ; symbol integer_methods
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    i8                                1b 1                var=2
// CHECK-NEXT: ; ssa   _2    i8                                1b 1                var=3
// CHECK-NEXT: ; stack _3    (i8, bool)                        2b 1                storage=ss0
// CHECK-NEXT: ; ssa   _4    i8                                1b 1                var=4
// CHECK-NEXT: ; stack _5    (i8, bool)                        2b 1                storage=ss1
// CHECK-NEXT: ; ssa   _6    i8                                1b 1                var=5
// CHECK-NEXT: ; stack _7    (i8, bool)                        2b 1                storage=ss2
// CHECK-NEXT: ; ssa   _8    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=7
// CHECK-NEXT: ; ssa   _10   u8                                1b 1                var=8
// CHECK-NEXT: ; ssa   _11   u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=10
// CHECK-NEXT: ; ssa   _13   u8                                1b 1                var=11
// CHECK-NEXT: ; ssa   _14   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _15   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _16   &'{erased} core::option::Option<i8>    2b 2                var=14
// CHECK-NEXT: ; stack _17   core::option::Option<i8>          2b 1                storage=ss3
// CHECK-NEXT: ; ssa   _18   u8                                1b 1                var=15
// CHECK-NEXT: ; ssa   _19   u8                                1b 1                var=16
// CHECK-NEXT: ; ssa   _20   bool                              1b 1                var=17
// CHECK-NEXT: ; ssa   _21   &'{erased} core::option::Option<i8>    2b 2                var=18
// CHECK-NEXT: ; stack _22   core::option::Option<i8>          2b 1                storage=ss4
// CHECK-NEXT: ; ssa   _23   &'{erased} core::option::Option<i8>    2b 2                var=19
// CHECK-NEXT: ; ssa   _24   u8                                1b 1                var=20
// CHECK-NEXT: ; ssa   _25   u8                                1b 1                var=21
// CHECK-NEXT: ; ssa   _26   bool                              1b 1                var=22
// CHECK-NEXT: ; ssa   _27   &'{erased} core::option::Option<i8>    2b 2                var=23
// CHECK-NEXT: ; stack _28   core::option::Option<i8>          2b 1                storage=ss5
// CHECK-NEXT: ; ssa   _29   u8                                1b 1                var=24
// CHECK-NEXT: ; ssa   _30   u8                                1b 1                var=25
// CHECK-NEXT: ; ssa   _31   bool                              1b 1                var=26
// CHECK-NEXT: ; ssa   _32   i8                                1b 1                var=27
// CHECK-NEXT: ; ssa   _33   i8                                1b 1                var=28
// CHECK-NEXT: ; ssa   _34   u8                                1b 1                var=29
// CHECK-NEXT: ; ssa   _35   u8                                1b 1                var=30
// CHECK-NEXT: ; ssa   _36   bool                              1b 1                var=31
// CHECK-NEXT: ; ssa   _37   i8                                1b 1                var=32
// CHECK-NEXT: ; ssa   _38   i8                                1b 1                var=33
// CHECK-NEXT: ; ssa   _39   u8                                1b 1                var=34
// CHECK-NEXT: ; ssa   _40   u8                                1b 1                var=35
// CHECK-NEXT: ; ssa   _41   bool                              1b 1                var=36
// CHECK-NEXT: ; ssa   _42   i8                                1b 1                var=37
// CHECK-NEXT: ; ssa   _43   i8                                1b 1                var=38
// CHECK-NEXT: ; ssa   _44   u8                                1b 1                var=39
// CHECK-NEXT: ; ssa   _45   u8                                1b 1                var=40
// CHECK-NEXT: ; ssa   _46   bool                              1b 1                var=41
// CHECK-NEXT: ; ssa   _47   bool                              1b 1                var=42
// CHECK-NEXT: ; ssa   _48   &'{erased} (i8, bool)             2b 2                var=43
// CHECK-NEXT: ; stack _49   (i8, bool)                        2b 1                storage=ss6
// CHECK-NEXT: ; ssa   _50   &'{erased} (i8, bool)             2b 2                var=44
// CHECK-NEXT: ; ssa   _51   bool                              1b 1                var=45
// CHECK-NEXT: ; ssa   _52   &'{erased} (i8, bool)             2b 2                var=46
// CHECK-NEXT: ; stack _53   (i8, bool)                        2b 1                storage=ss7
// CHECK-NEXT: ; ssa   _54   &'{erased} (i8, bool)             2b 2                var=47
// CHECK-NEXT: ; ssa   _55   &'{erased} (i8, bool)             2b 2                var=48
// CHECK-NEXT: ; stack _56   (i8, bool)                        2b 1                storage=ss8
// CHECK-NEXT: ; ssa   _57   &'{erased} (i8, bool)             2b 2                var=49
// CHECK-NEXT: ; ssa   _58   u8                                1b 1                var=50
// CHECK-NEXT: ; ssa   _59   u8                                1b 1                var=51
// CHECK-NEXT: ; ssa   _60   bool                              1b 1                var=52
// CHECK-NEXT: ; ssa   _61   bool                              1b 1                var=53
// CHECK-NEXT: ; ssa   _62   u32                               4b 2                var=54
// CHECK-NEXT: ; ssa   _63   bool                              1b 1                var=55
// CHECK-NEXT: ; ssa   _64   u32                               4b 2                var=56
// CHECK-NEXT: ; ssa   _65   u32                               4b 2                var=57
// CHECK-NEXT: ; ssa   _66   i8                                1b 1                var=58
// CHECK-NEXT: ; ssa   _67   bool                              1b 1                var=59
// CHECK-NEXT: ; ssa   _68   (i8, bool)                        2b 1                var=(60, 61)
// CHECK-NEXT: ; ssa   _69   bool                              1b 1                var=62
// CHECK-NEXT: ; ssa   _70   i8                                1b 1                var=63
// CHECK-NEXT: ; ssa   _71   bool                              1b 1                var=64
// CHECK-NEXT: ; ssa   _72   (i8, bool)                        2b 1                var=(65, 66)
// CHECK-NEXT: ; zst   _73   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _74   bool                              1b 1                var=67
// CHECK-NEXT: ; ssa   _75   isize                             2b 2                var=68
// CHECK-NEXT: ; ssa   _76   i8                                1b 1                var=69
// CHECK-NEXT: ; ssa   _77   bool                              1b 1                var=70
// CHECK-NEXT: ; ssa   _78   (i8, bool)                        2b 1                var=(71, 72)
// CHECK-NEXT: ; ssa   _79   bool                              1b 1                var=73
// CHECK-NEXT: ; ssa   _80   i8                                1b 1                var=74
// CHECK-NEXT: ; ssa   _81   bool                              1b 1                var=75
// CHECK-NEXT: ; ssa   _82   (i8, bool)                        2b 1                var=(76, 77)
// CHECK-NEXT: ; zst   _83   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _84   i8                                1b 1                var=78
// CHECK-NEXT: ; ssa   _85   bool                              1b 1                var=79
// CHECK-NEXT: ; ssa   _86   (i8, bool)                        2b 1                var=(80, 81)
// CHECK-NEXT: ; ssa   _87   bool                              1b 1                var=82
// CHECK-NEXT: ; ssa   _88   i8                                1b 1                var=83
// CHECK-NEXT: ; ssa   _89   bool                              1b 1                var=84
// CHECK-NEXT: ; ssa   _90   (i8, bool)                        2b 1                var=(85, 86)
// CHECK-NEXT: ; zst   _91   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _92   bool                              1b 1                var=87
// CHECK-NEXT: ; ssa   _93   isize                             2b 2                var=88
// CHECK-NEXT: ; ssa   _94   i8                                1b 1                var=89
// CHECK-NEXT: ; ssa   _95   bool                              1b 1                var=90
// CHECK-NEXT: ; ssa   _96   (i8, bool)                        2b 1                var=(91, 92)
// CHECK-NEXT: ; ssa   _97   i8                                1b 1                var=93
// CHECK-NEXT: ; ssa   _98   bool                              1b 1                var=94
// CHECK-NEXT: ; ssa   _99   (i8, bool)                        2b 1                var=(95, 96)
// CHECK-NEXT: ; ssa   _100  i8                                1b 1                var=97
// CHECK-NEXT: ; ssa   _101  bool                              1b 1                var=98
// CHECK-NEXT: ; ssa   _102  (i8, bool)                        2b 1                var=(99, 100)
// CHECK-NEXT: ; ssa   _103  u8                                1b 1                var=101
// CHECK-NEXT: ; ssa   _104  u8                                1b 1                var=102
// CHECK-NEXT: ; ssa   _105  u8                                1b 1                var=103
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
// CHECK-NEXT:     gv0 = symbol colocated userextname1 ; alloc5
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [i8, bool] }
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [i8, bool] }
// CHECK-NEXT:     fn2 = colocated u0:2 sig2 ; Instance { def: Item(DefId(1:11950 ~ core[..]::option::{impl#17}::eq)), args: [i8] }
// CHECK-NEXT:     fn3 = colocated u0:1 sig3 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [i8, bool] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v18 -> v0
// CHECK-NEXT:     v207 -> v0
// CHECK-NEXT:     v208 -> v0
// CHECK-NEXT:     v211 -> v0
// CHECK-NEXT:     v212 -> v0
// CHECK-NEXT:     v19 -> v1
// CHECK-NEXT:     v209 -> v1
// CHECK-NEXT:     v210 -> v1
// CHECK-NEXT:     v213 -> v1
// CHECK-NEXT:     v214 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): i8 <- ByVal(v0): i8
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): i8 <- ByVal(v1): i8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = Add(const core::num::<impl i8>::MIN, const 1_i8)
// CHECK-NEXT:     v2 = iconst.i8 -128
// CHECK-NEXT:     v3 = iconst.i8 1
// CHECK-NEXT:     v4 = iadd v2, v3  ; v2 = -128, v3 = 1
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): i8 <- ByVal(v4): i8
// CHECK-NEXT: ; _3 = (move _4, const true)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): i8 <- ByVal(v4): i8
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v4, v5
// CHECK-NEXT:     v6 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(1) }, None): bool <- ByVal(v6): bool
// CHECK-NEXT:     v7 = stack_addr.i16 ss0+1
// CHECK-NEXT:     store notrap v6, v7  ; v6 = 1
// CHECK-NEXT: ; _6 = Sub(const core::num::<impl i8>::MAX, const 2_i8)
// CHECK-NEXT:     v8 = iconst.i8 127
// CHECK-NEXT:     v9 = iconst.i8 2
// CHECK-NEXT:     v10 = isub v8, v9  ; v8 = 127, v9 = 2
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): i8 <- ByVal(v10): i8
// CHECK-NEXT: ; _5 = (move _6, const false)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): i8 <- ByVal(v10): i8
// CHECK-NEXT:     v11 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v10, v11
// CHECK-NEXT:     v12 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(1) }, None): bool <- ByVal(v12): bool
// CHECK-NEXT:     v13 = stack_addr.i16 ss1+1
// CHECK-NEXT:     store notrap v12, v13  ; v12 = 0
// CHECK-NEXT: ; _7 = (const -2_i8, const true)
// CHECK-NEXT:     v14 = iconst.i8 -2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): i8 <- ByVal(v14): i8
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v14, v15  ; v14 = -2
// CHECK-NEXT:     v16 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(1) }, None): bool <- ByVal(v16): bool
// CHECK-NEXT:     v17 = stack_addr.i16 ss2+1
// CHECK-NEXT:     store notrap v16, v17  ; v16 = 1
// CHECK-NEXT: ; _72 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v20 = iadd.i8 v18, v19
// CHECK-NEXT:     v205 -> v20
// CHECK-NEXT:     v206 -> v20
// CHECK-NEXT:     v21 = iconst.i8 0
// CHECK-NEXT:     v22 = icmp.i8 slt v19, v21  ; v21 = 0
// CHECK-NEXT:     v23 = icmp slt v20, v18
// CHECK-NEXT:     v24 = bxor v22, v23
// CHECK-NEXT: ; write_cvalue: VarPair(_72, var65, var66): (i8, bool) <- ByValPair(v20, v24): (i8, bool)
// CHECK-NEXT: ; _70 = copy (_72.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_70, var63): i8 <- ByVal(v20): i8
// CHECK-NEXT: ; _71 = copy (_72.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_71, var64): bool <- ByVal(v24): bool
// CHECK-NEXT: ; _68 = (copy _70, copy _71)
// CHECK-NEXT: ; write_cvalue: Var(_68, var60): i8 <- ByVal(v20): i8
// CHECK-NEXT: ; write_cvalue: Var(_68, var61): bool <- ByVal(v24): bool
// CHECK-NEXT: ; _66 = copy (_68.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_66, var58): i8 <- ByVal(v20): i8
// CHECK-NEXT: ; _67 = copy (_68.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_67, var59): bool <- ByVal(v24): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _67)
// CHECK-NEXT:     brif v24, block17, block19
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _19 = move _20 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_19, var16): u8 <- ByVal(v25): u8
// CHECK-NEXT: ; _18 = Shl(move _19, const 1_i32)
// CHECK-NEXT:     v26 = iconst.i32 1
// CHECK-NEXT:     v27 = ishl.i8 v25, v26  ; v26 = 1
// CHECK-NEXT: ; write_cvalue: Var(_18, var15): u8 <- ByVal(v27): u8
// CHECK-NEXT: ; _13 = BitOr(move _14, move _18)
// CHECK-NEXT:     v29 = bor.i8 v28, v27
// CHECK-NEXT:     v236 -> v29
// CHECK-NEXT:     v237 -> v29
// CHECK-NEXT:     v242 -> v29
// CHECK-NEXT:     v243 -> v29
// CHECK-NEXT: ; write_cvalue: Var(_13, var11): u8 <- ByVal(v29): u8
// CHECK-NEXT: ; _90 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v32 = sextend.i16 v30
// CHECK-NEXT:     v33 = sextend.i16 v31
// CHECK-NEXT:     v34 = imul v32, v33
// CHECK-NEXT:     v35 = iconst.i16 -128
// CHECK-NEXT:     v36 = icmp slt v34, v35  ; v35 = -128
// CHECK-NEXT:     v37 = iconst.i16 127
// CHECK-NEXT:     v38 = icmp sgt v34, v37  ; v37 = 127
// CHECK-NEXT:     v39 = ireduce.i8 v34
// CHECK-NEXT:     v234 -> v39
// CHECK-NEXT:     v235 -> v39
// CHECK-NEXT:     v40 = bor v36, v38
// CHECK-NEXT: ; write_cvalue: VarPair(_90, var85, var86): (i8, bool) <- ByValPair(v39, v40): (i8, bool)
// CHECK-NEXT: ; _88 = copy (_90.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_88, var83): i8 <- ByVal(v39): i8
// CHECK-NEXT: ; _89 = copy (_90.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_89, var84): bool <- ByVal(v40): bool
// CHECK-NEXT: ; _86 = (copy _88, copy _89)
// CHECK-NEXT: ; write_cvalue: Var(_86, var80): i8 <- ByVal(v39): i8
// CHECK-NEXT: ; write_cvalue: Var(_86, var81): bool <- ByVal(v40): bool
// CHECK-NEXT: ; _84 = copy (_86.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_84, var78): i8 <- ByVal(v39): i8
// CHECK-NEXT: ; _85 = copy (_86.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_85, var79): bool <- ByVal(v40): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _85)
// CHECK-NEXT:     brif v40, block35, block37
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _47)
// CHECK-NEXT:     brif.i8 v41, block4, block7
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _99 = SubWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v44 = isub.i8 v42, v43
// CHECK-NEXT:     v45 = iconst.i8 0
// CHECK-NEXT:     v46 = icmp.i8 slt v43, v45  ; v45 = 0
// CHECK-NEXT:     v47 = icmp sgt v44, v42
// CHECK-NEXT:     v48 = bxor v46, v47
// CHECK-NEXT: ; write_cvalue: VarPair(_99, var95, var96): (i8, bool) <- ByValPair(v44, v48): (i8, bool)
// CHECK-NEXT: ; _97 = copy (_99.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_97, var93): i8 <- ByVal(v44): i8
// CHECK-NEXT: ; _98 = copy (_99.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_98, var94): bool <- ByVal(v48): bool
// CHECK-NEXT: ; _53 = (copy _97, copy _98)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): i8 <- ByVal(v44): i8
// CHECK-NEXT:     v49 = stack_addr.i16 ss7
// CHECK-NEXT:     store notrap v44, v49
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(1) }, None): bool <- ByVal(v48): bool
// CHECK-NEXT:     v50 = stack_addr.i16 ss7+1
// CHECK-NEXT:     store notrap v48, v50
// CHECK-NEXT: ; _52 = &_53
// CHECK-NEXT:     v51 = stack_addr.i16 ss7
// CHECK-NEXT: ; write_cvalue: Var(_52, var46): &'{erased} (i8, bool) <- ByVal(v51): &'{erased} (i8, bool)
// CHECK-NEXT: ; _54 = &_5
// CHECK-NEXT:     v52 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_54, var47): &'{erased} (i8, bool) <- ByVal(v52): &'{erased} (i8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _51 = <(i8, bool) as core::cmp::PartialEq>::eq(move _52, move _54)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v53 = call fn0(v51, v52)
// CHECK-NEXT:     v54 -> v53
// CHECK-NEXT: ; write_cvalue: Var(_51, var45): bool <- ByVal(v53): bool
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _51)
// CHECK-NEXT:     brif.i8 v54, block6, block7
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _102 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v57 = sextend.i16 v55
// CHECK-NEXT:     v58 = sextend.i16 v56
// CHECK-NEXT:     v59 = imul v57, v58
// CHECK-NEXT:     v60 = iconst.i16 -128
// CHECK-NEXT:     v61 = icmp slt v59, v60  ; v60 = -128
// CHECK-NEXT:     v62 = iconst.i16 127
// CHECK-NEXT:     v63 = icmp sgt v59, v62  ; v62 = 127
// CHECK-NEXT:     v64 = ireduce.i8 v59
// CHECK-NEXT:     v65 = bor v61, v63
// CHECK-NEXT: ; write_cvalue: VarPair(_102, var99, var100): (i8, bool) <- ByValPair(v64, v65): (i8, bool)
// CHECK-NEXT: ; _100 = copy (_102.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_100, var97): i8 <- ByVal(v64): i8
// CHECK-NEXT: ; _101 = copy (_102.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_101, var98): bool <- ByVal(v65): bool
// CHECK-NEXT: ; _56 = (copy _100, copy _101)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): i8 <- ByVal(v64): i8
// CHECK-NEXT:     v66 = stack_addr.i16 ss8
// CHECK-NEXT:     store notrap v64, v66
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(1) }, None): bool <- ByVal(v65): bool
// CHECK-NEXT:     v67 = stack_addr.i16 ss8+1
// CHECK-NEXT:     store notrap v65, v67
// CHECK-NEXT: ; _55 = &_56
// CHECK-NEXT:     v68 = stack_addr.i16 ss8
// CHECK-NEXT: ; write_cvalue: Var(_55, var48): &'{erased} (i8, bool) <- ByVal(v68): &'{erased} (i8, bool)
// CHECK-NEXT: ; _57 = &_7
// CHECK-NEXT:     v69 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_57, var49): &'{erased} (i8, bool) <- ByVal(v69): &'{erased} (i8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _46 = <(i8, bool) as core::cmp::PartialEq>::eq(move _55, move _57)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v70 = call fn1(v68, v69)
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v70): bool
// CHECK-NEXT:     jump block8(v70)
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _46 = const false
// CHECK-NEXT:     v71 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v71): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block8(v71)  ; v71 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block8(v72: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _45 = move _46 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_45, var40): u8 <- ByVal(v72): u8
// CHECK-NEXT: ; _44 = Shl(move _45, const 6_i32)
// CHECK-NEXT:     v73 = iconst.i32 6
// CHECK-NEXT:     v74 = ishl v72, v73  ; v73 = 6
// CHECK-NEXT: ; write_cvalue: Var(_44, var39): u8 <- ByVal(v74): u8
// CHECK-NEXT: ; _8 = BitOr(move _9, move _44)
// CHECK-NEXT:     v76 = bor.i8 v75, v74
// CHECK-NEXT:     v201 -> v76
// CHECK-NEXT:     v202 -> v76
// CHECK-NEXT:     v203 -> v76
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): u8 <- ByVal(v76): u8
// CHECK-NEXT: ; _103 = copy _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_103, var101): u8 <- ByVal(v77): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _62 = core::intrinsics::ctlz::<u8>(copy _103)
// CHECK-NEXT:     v78 = clz.i8 v77
// CHECK-NEXT:     v79 = uextend.i32 v78
// CHECK-NEXT:     v184 -> v79
// CHECK-NEXT: ; write_cvalue: Var(_62, var54): u32 <- ByVal(v79): u32
// CHECK-NEXT:     jump block41
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _104 = copy _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_104, var102): u8 <- ByVal(v80): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _64 = core::intrinsics::cttz::<u8>(copy _104)
// CHECK-NEXT:     v81 = ctz.i8 v80
// CHECK-NEXT:     v82 = uextend.i32 v81
// CHECK-NEXT:     v187 -> v82
// CHECK-NEXT: ; write_cvalue: Var(_64, var56): u32 <- ByVal(v82): u32
// CHECK-NEXT:     jump block42
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _105 = copy _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_105, var103): u8 <- ByVal(v83): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _65 = core::intrinsics::ctpop::<u8>(copy _105)
// CHECK-NEXT:     v84 = popcnt.i8 v83
// CHECK-NEXT:     v85 = uextend.i32 v84
// CHECK-NEXT:     v190 -> v85
// CHECK-NEXT: ; write_cvalue: Var(_65, var57): u32 <- ByVal(v85): u32
// CHECK-NEXT:     jump block43
// CHECK-EMPTY:
// CHECK-NEXT: block11(v200: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = const false
// CHECK-NEXT:     v86 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v86): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v86, v200)  ; v86 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block12(v87: i8, v90: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _59 = move _60 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_59, var51): u8 <- ByVal(v87): u8
// CHECK-NEXT: ; _58 = Shl(move _59, const 7_i32)
// CHECK-NEXT:     v88 = iconst.i32 7
// CHECK-NEXT:     v89 = ishl v87, v88  ; v88 = 7
// CHECK-NEXT: ; write_cvalue: Var(_58, var50): u8 <- ByVal(v89): u8
// CHECK-NEXT: ; _0 = BitOr(move _8, move _58)
// CHECK-NEXT:     v91 = bor v90, v89
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v91): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v91
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = &_17
// CHECK-NEXT:     v92 = stack_addr.i16 ss3
// CHECK-NEXT: ; write_cvalue: Var(_16, var14): &'{erased} core::option::Option<i8> <- ByVal(v92): &'{erased} core::option::Option<i8>
// CHECK-NEXT: ; _75 = discriminant((*_16))
// CHECK-NEXT:     v93 = load.i8 notrap v92
// CHECK-NEXT:     v94 = uextend.i16 v93
// CHECK-NEXT: ; write_cvalue: Var(_75, var68): isize <- ByVal(v94): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _75)
// CHECK-NEXT:     v95 = uextend.i32 v94
// CHECK-NEXT:     br_table v95, block23, [block21, block22]
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _17 = core::option::Option::<i8>::None
// CHECK-NEXT:     v96 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u8 <- ByVal(v96): u8
// CHECK-NEXT:     v97 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v96, v97  ; v96 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _17 = core::option::Option::<i8>::Some(copy _66)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(1) }, None): i8 <- ByVal(v98): i8
// CHECK-NEXT:     v99 = stack_addr.i16 ss3+1
// CHECK-NEXT:     store.i8 notrap v98, v99
// CHECK-NEXT:     v100 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u8 <- ByVal(v100): u8
// CHECK-NEXT:     v101 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v100, v101  ; v100 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block16(v102: i8, v204: i8):
// CHECK-NEXT:     v98 -> v204
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _69)
// CHECK-NEXT:     brif v102, block14, block15
// CHECK-EMPTY:
// CHECK-NEXT: block17 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _73 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block18
// CHECK-EMPTY:
// CHECK-NEXT: block18:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _69 = const true
// CHECK-NEXT:     v103 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_69, var62): bool <- ByVal(v103): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block16(v103, v205)  ; v103 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block19:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _69 = const false
// CHECK-NEXT:     v104 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_69, var62): bool <- ByVal(v104): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block16(v104, v206)  ; v104 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block20(v105: i8, v108: i8, v109: i8):
// CHECK-NEXT:     v227 -> v108
// CHECK-NEXT:     v228 -> v108
// CHECK-NEXT:     v229 -> v109
// CHECK-NEXT:     v230 -> v109
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = Not(move _74)
// CHECK-NEXT:     v106 = iconst.i8 0
// CHECK-NEXT:     v107 = icmp eq v105, v106  ; v106 = 0
// CHECK-NEXT:     v225 -> v107
// CHECK-NEXT:     v226 -> v107
// CHECK-NEXT: ; write_cvalue: Var(_15, var13): bool <- ByVal(v107): bool
// CHECK-NEXT: ; _14 = move _15 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_14, var12): u8 <- ByVal(v107): u8
// CHECK-NEXT: ; _82 = SubWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v110 = isub v108, v109
// CHECK-NEXT:     v231 -> v110
// CHECK-NEXT:     v232 -> v110
// CHECK-NEXT:     v111 = iconst.i8 0
// CHECK-NEXT:     v112 = icmp slt v109, v111  ; v111 = 0
// CHECK-NEXT:     v113 = icmp sgt v110, v108
// CHECK-NEXT:     v114 = bxor v112, v113
// CHECK-NEXT: ; write_cvalue: VarPair(_82, var76, var77): (i8, bool) <- ByValPair(v110, v114): (i8, bool)
// CHECK-NEXT: ; _80 = copy (_82.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_80, var74): i8 <- ByVal(v110): i8
// CHECK-NEXT: ; _81 = copy (_82.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_81, var75): bool <- ByVal(v114): bool
// CHECK-NEXT: ; _78 = (copy _80, copy _81)
// CHECK-NEXT: ; write_cvalue: Var(_78, var71): i8 <- ByVal(v110): i8
// CHECK-NEXT: ; write_cvalue: Var(_78, var72): bool <- ByVal(v114): bool
// CHECK-NEXT: ; _76 = copy (_78.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_76, var69): i8 <- ByVal(v110): i8
// CHECK-NEXT: ; _77 = copy (_78.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_77, var70): bool <- ByVal(v114): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _77)
// CHECK-NEXT:     brif v114, block28, block30
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _74 = const false
// CHECK-NEXT:     v115 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_74, var67): bool <- ByVal(v115): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block20(v115, v207, v209)  ; v115 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _74 = const true
// CHECK-NEXT:     v116 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_74, var67): bool <- ByVal(v116): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block20(v116, v208, v210)  ; v116 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block23 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; unreachable
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block24(v193: i8, v194: i8, v195: i8):
// CHECK-NEXT:     v28 -> v193
// CHECK-NEXT:     v30 -> v194
// CHECK-NEXT:     v238 -> v194
// CHECK-NEXT:     v239 -> v194
// CHECK-NEXT:     v244 -> v194
// CHECK-NEXT:     v245 -> v194
// CHECK-NEXT:     v31 -> v195
// CHECK-NEXT:     v240 -> v195
// CHECK-NEXT:     v241 -> v195
// CHECK-NEXT:     v246 -> v195
// CHECK-NEXT:     v247 -> v195
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _21 = &_22
// CHECK-NEXT:     v117 = stack_addr.i16 ss4
// CHECK-NEXT: ; write_cvalue: Var(_21, var18): &'{erased} core::option::Option<i8> <- ByVal(v117): &'{erased} core::option::Option<i8>
// CHECK-NEXT: ; _23 = const integer_methods::promoted[0]
// CHECK-NEXT:     v118 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_23, var19): &'{erased} core::option::Option<i8> <- ByVal(v118): &'{erased} core::option::Option<i8>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _20 = <core::option::Option<i8> as core::cmp::PartialEq>::eq(move _21, move _23)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<i8>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::option::Option<i8>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v119 = call fn2(v117, v118)
// CHECK-NEXT:     v25 -> v119
// CHECK-NEXT: ; write_cvalue: Var(_20, var17): bool <- ByVal(v119): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block25:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = core::option::Option::<i8>::None
// CHECK-NEXT:     v120 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u8 <- ByVal(v120): u8
// CHECK-NEXT:     v121 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v120, v121  ; v120 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block24(v215, v217, v219)
// CHECK-EMPTY:
// CHECK-NEXT: block26:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = core::option::Option::<i8>::Some(copy _76)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(1) }, None): i8 <- ByVal(v122): i8
// CHECK-NEXT:     v123 = stack_addr.i16 ss4+1
// CHECK-NEXT:     store.i8 notrap v122, v123
// CHECK-NEXT:     v124 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u8 <- ByVal(v124): u8
// CHECK-NEXT:     v125 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v124, v125  ; v124 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block24(v216, v218, v220)
// CHECK-EMPTY:
// CHECK-NEXT: block27(v126: i8, v221: i8, v222: i8, v223: i8, v224: i8):
// CHECK-NEXT:     v215 -> v221
// CHECK-NEXT:     v216 -> v221
// CHECK-NEXT:     v217 -> v222
// CHECK-NEXT:     v218 -> v222
// CHECK-NEXT:     v219 -> v223
// CHECK-NEXT:     v220 -> v223
// CHECK-NEXT:     v122 -> v224
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _79)
// CHECK-NEXT:     brif v126, block25, block26
// CHECK-EMPTY:
// CHECK-NEXT: block28 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _83 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block29
// CHECK-EMPTY:
// CHECK-NEXT: block29:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _79 = const true
// CHECK-NEXT:     v127 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_79, var73): bool <- ByVal(v127): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block27(v127, v225, v227, v229, v231)  ; v127 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block30:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _79 = const false
// CHECK-NEXT:     v128 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_79, var73): bool <- ByVal(v128): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block27(v128, v226, v228, v230, v232)  ; v128 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block31:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _27 = &_28
// CHECK-NEXT:     v129 = stack_addr.i16 ss5
// CHECK-NEXT: ; write_cvalue: Var(_27, var23): &'{erased} core::option::Option<i8> <- ByVal(v129): &'{erased} core::option::Option<i8>
// CHECK-NEXT: ; _93 = discriminant((*_27))
// CHECK-NEXT:     v130 = load.i8 notrap v129
// CHECK-NEXT:     v131 = uextend.i16 v130
// CHECK-NEXT: ; write_cvalue: Var(_93, var88): isize <- ByVal(v131): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _93)
// CHECK-NEXT:     v132 = uextend.i32 v131
// CHECK-NEXT:     br_table v132, block23, [block39, block40]
// CHECK-EMPTY:
// CHECK-NEXT: block32:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _28 = core::option::Option::<i8>::None
// CHECK-NEXT:     v133 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): u8 <- ByVal(v133): u8
// CHECK-NEXT:     v134 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v133, v134  ; v133 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block31
// CHECK-EMPTY:
// CHECK-NEXT: block33:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _28 = core::option::Option::<i8>::Some(copy _84)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(1) }, None): i8 <- ByVal(v135): i8
// CHECK-NEXT:     v136 = stack_addr.i16 ss5+1
// CHECK-NEXT:     store.i8 notrap v135, v136
// CHECK-NEXT:     v137 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): u8 <- ByVal(v137): u8
// CHECK-NEXT:     v138 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v137, v138  ; v137 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block31
// CHECK-EMPTY:
// CHECK-NEXT: block34(v139: i8, v233: i8):
// CHECK-NEXT:     v135 -> v233
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _87)
// CHECK-NEXT:     brif v139, block32, block33
// CHECK-EMPTY:
// CHECK-NEXT: block35 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _91 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block36
// CHECK-EMPTY:
// CHECK-NEXT: block36:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _87 = const true
// CHECK-NEXT:     v140 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_87, var82): bool <- ByVal(v140): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block34(v140, v234)  ; v140 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block37:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _87 = const false
// CHECK-NEXT:     v141 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_87, var82): bool <- ByVal(v141): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block34(v141, v235)  ; v141 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block38(v142: i8, v147: i8, v149: i8, v150: i8):
// CHECK-NEXT:     v42 -> v149
// CHECK-NEXT:     v55 -> v149
// CHECK-NEXT:     v77 -> v149
// CHECK-NEXT:     v197 -> v149
// CHECK-NEXT:     v198 -> v149
// CHECK-NEXT:     v199 -> v149
// CHECK-NEXT:     v83 -> v199
// CHECK-NEXT:     v80 -> v198
// CHECK-NEXT:     v43 -> v150
// CHECK-NEXT:     v56 -> v150
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _26 = Not(move _92)
// CHECK-NEXT:     v143 = iconst.i8 0
// CHECK-NEXT:     v144 = icmp eq v142, v143  ; v143 = 0
// CHECK-NEXT: ; write_cvalue: Var(_26, var22): bool <- ByVal(v144): bool
// CHECK-NEXT: ; _25 = move _26 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_25, var21): u8 <- ByVal(v144): u8
// CHECK-NEXT: ; _24 = Shl(move _25, const 2_i32)
// CHECK-NEXT:     v145 = iconst.i32 2
// CHECK-NEXT:     v146 = ishl v144, v145  ; v145 = 2
// CHECK-NEXT: ; write_cvalue: Var(_24, var20): u8 <- ByVal(v146): u8
// CHECK-NEXT: ; _12 = BitOr(move _13, move _24)
// CHECK-NEXT:     v148 = bor v147, v146
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): u8 <- ByVal(v148): u8
// CHECK-NEXT: ; _32 = Add(copy _1, copy _2)
// CHECK-NEXT:     v151 = iadd v149, v150
// CHECK-NEXT: ; write_cvalue: Var(_32, var27): i8 <- ByVal(v151): i8
// CHECK-NEXT: ; _33 = copy (_3.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_33, var28): i8 <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): i8
// CHECK-NEXT:     v152 = stack_addr.i16 ss0
// CHECK-NEXT:     v153 = load.i8 notrap v152
// CHECK-NEXT: ; _31 = Eq(move _32, move _33)
// CHECK-NEXT:     v154 = icmp eq v151, v153
// CHECK-NEXT: ; write_cvalue: Var(_31, var26): bool <- ByVal(v154): bool
// CHECK-NEXT: ; _30 = move _31 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_30, var25): u8 <- ByVal(v154): u8
// CHECK-NEXT: ; _29 = Shl(move _30, const 3_i32)
// CHECK-NEXT:     v155 = iconst.i32 3
// CHECK-NEXT:     v156 = ishl v154, v155  ; v155 = 3
// CHECK-NEXT: ; write_cvalue: Var(_29, var24): u8 <- ByVal(v156): u8
// CHECK-NEXT: ; _11 = BitOr(move _12, move _29)
// CHECK-NEXT:     v157 = bor v148, v156
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): u8 <- ByVal(v157): u8
// CHECK-NEXT: ; _37 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v158 = isub v149, v150
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): i8 <- ByVal(v158): i8
// CHECK-NEXT: ; _38 = copy (_5.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_38, var33): i8 <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): i8
// CHECK-NEXT:     v159 = stack_addr.i16 ss1
// CHECK-NEXT:     v160 = load.i8 notrap v159
// CHECK-NEXT: ; _36 = Eq(move _37, move _38)
// CHECK-NEXT:     v161 = icmp eq v158, v160
// CHECK-NEXT: ; write_cvalue: Var(_36, var31): bool <- ByVal(v161): bool
// CHECK-NEXT: ; _35 = move _36 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_35, var30): u8 <- ByVal(v161): u8
// CHECK-NEXT: ; _34 = Shl(move _35, const 4_i32)
// CHECK-NEXT:     v162 = iconst.i32 4
// CHECK-NEXT:     v163 = ishl v161, v162  ; v162 = 4
// CHECK-NEXT: ; write_cvalue: Var(_34, var29): u8 <- ByVal(v163): u8
// CHECK-NEXT: ; _10 = BitOr(move _11, move _34)
// CHECK-NEXT:     v164 = bor v157, v163
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): u8 <- ByVal(v164): u8
// CHECK-NEXT: ; _42 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v165 = imul v149, v150
// CHECK-NEXT: ; write_cvalue: Var(_42, var37): i8 <- ByVal(v165): i8
// CHECK-NEXT: ; _43 = copy (_7.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_43, var38): i8 <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): i8
// CHECK-NEXT:     v166 = stack_addr.i16 ss2
// CHECK-NEXT:     v167 = load.i8 notrap v166
// CHECK-NEXT: ; _41 = Eq(move _42, move _43)
// CHECK-NEXT:     v168 = icmp eq v165, v167
// CHECK-NEXT: ; write_cvalue: Var(_41, var36): bool <- ByVal(v168): bool
// CHECK-NEXT: ; _40 = move _41 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_40, var35): u8 <- ByVal(v168): u8
// CHECK-NEXT: ; _39 = Shl(move _40, const 5_i32)
// CHECK-NEXT:     v169 = iconst.i32 5
// CHECK-NEXT:     v170 = ishl v168, v169  ; v169 = 5
// CHECK-NEXT: ; write_cvalue: Var(_39, var34): u8 <- ByVal(v170): u8
// CHECK-NEXT: ; _9 = BitOr(move _10, move _39)
// CHECK-NEXT:     v171 = bor v164, v170
// CHECK-NEXT:     v75 -> v171
// CHECK-NEXT:     v196 -> v171
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): u8 <- ByVal(v171): u8
// CHECK-NEXT: ; _96 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v172 = iadd v149, v150
// CHECK-NEXT:     v173 = iconst.i8 0
// CHECK-NEXT:     v174 = icmp slt v150, v173  ; v173 = 0
// CHECK-NEXT:     v175 = icmp slt v172, v149
// CHECK-NEXT:     v176 = bxor v174, v175
// CHECK-NEXT: ; write_cvalue: VarPair(_96, var91, var92): (i8, bool) <- ByValPair(v172, v176): (i8, bool)
// CHECK-NEXT: ; _94 = copy (_96.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_94, var89): i8 <- ByVal(v172): i8
// CHECK-NEXT: ; _95 = copy (_96.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_95, var90): bool <- ByVal(v176): bool
// CHECK-NEXT: ; _49 = (copy _94, copy _95)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): i8 <- ByVal(v172): i8
// CHECK-NEXT:     v177 = stack_addr.i16 ss6
// CHECK-NEXT:     store notrap v172, v177
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(1) }, None): bool <- ByVal(v176): bool
// CHECK-NEXT:     v178 = stack_addr.i16 ss6+1
// CHECK-NEXT:     store notrap v176, v178
// CHECK-NEXT: ; _48 = &_49
// CHECK-NEXT:     v179 = stack_addr.i16 ss6
// CHECK-NEXT: ; write_cvalue: Var(_48, var43): &'{erased} (i8, bool) <- ByVal(v179): &'{erased} (i8, bool)
// CHECK-NEXT: ; _50 = &_3
// CHECK-NEXT:     v180 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_50, var44): &'{erased} (i8, bool) <- ByVal(v180): &'{erased} (i8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _47 = <(i8, bool) as core::cmp::PartialEq>::eq(move _48, move _50)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v181 = call fn3(v179, v180)
// CHECK-NEXT:     v41 -> v181
// CHECK-NEXT: ; write_cvalue: Var(_47, var42): bool <- ByVal(v181): bool
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block39:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _92 = const false
// CHECK-NEXT:     v182 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_92, var87): bool <- ByVal(v182): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block38(v182, v236, v238, v240)  ; v182 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block40:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _92 = const true
// CHECK-NEXT:     v183 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_92, var87): bool <- ByVal(v183): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block38(v183, v237, v239, v241)  ; v183 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block41:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _61 = Eq(move _62, const 1_u32)
// CHECK-NEXT:     v185 = iconst.i32 1
// CHECK-NEXT:     v186 = icmp.i32 eq v184, v185  ; v185 = 1
// CHECK-NEXT: ; write_cvalue: Var(_61, var53): bool <- ByVal(v186): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _61)
// CHECK-NEXT:     brif v186, block9, block11(v201)
// CHECK-EMPTY:
// CHECK-NEXT: block42:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _63 = Eq(move _64, const 0_u32)
// CHECK-NEXT:     v188 = iconst.i32 0
// CHECK-NEXT:     v189 = icmp.i32 eq v187, v188  ; v188 = 0
// CHECK-NEXT: ; write_cvalue: Var(_63, var55): bool <- ByVal(v189): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _63)
// CHECK-NEXT:     brif.i32 v187, block11(v202), block10
// CHECK-EMPTY:
// CHECK-NEXT: block43:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = Eq(move _65, const 7_u32)
// CHECK-NEXT:     v191 = iconst.i32 7
// CHECK-NEXT:     v192 = icmp.i32 eq v190, v191  ; v191 = 7
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v192): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v192, v203)
// CHECK-NEXT: }
