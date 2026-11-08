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
// CHECK-NEXT:     fn0 = colocated u0:3 sig0 ; Instance { def: Item(DefId(1:29990 ~ core[..]::cmp::impls::{impl#28}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:2 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
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
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCs4jUVW15fqGy_10integer_i8
// CHECK-NEXT: function u0:2(i16, i16) -> i8 uext system_v {
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
// CHECK-NEXT: function u0:3(i16, i16) -> i8 uext system_v {
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
// CHECK-NEXT:     fn0 = colocated u0:5 sig0 ; Instance { def: Item(DefId(0:3 ~ integer_i8[..]::integer_methods)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = integer_methods(const i8::MAX, const 2_i8)
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
// CHECK-NEXT: function u0:5(i8, i8) -> i8 system_v {
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
// CHECK-NEXT: ; stack _4    (i8, bool)                        2b 1                storage=ss1
// CHECK-NEXT: ; stack _5    (i8, bool)                        2b 1                storage=ss2
// CHECK-NEXT: ; ssa   _6    u8                                1b 1                var=4
// CHECK-NEXT: ; ssa   _7    u8                                1b 1                var=5
// CHECK-NEXT: ; ssa   _8    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=7
// CHECK-NEXT: ; ssa   _10   u8                                1b 1                var=8
// CHECK-NEXT: ; ssa   _11   u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=10
// CHECK-NEXT: ; ssa   _13   bool                              1b 1                var=11
// CHECK-NEXT: ; stack _14   core::option::Option<i8>          2b 1                storage=ss3
// CHECK-NEXT: ; ssa   _15   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _16   u8                                1b 1                var=13
// CHECK-NEXT: ; ssa   _17   bool                              1b 1                var=14
// CHECK-NEXT: ; stack _18   core::option::Option<i8>          2b 1                storage=ss4
// CHECK-NEXT: ; ssa   _19   u8                                1b 1                var=15
// CHECK-NEXT: ; ssa   _20   u8                                1b 1                var=16
// CHECK-NEXT: ; ssa   _21   bool                              1b 1                var=17
// CHECK-NEXT: ; stack _22   core::option::Option<i8>          2b 1                storage=ss5
// CHECK-NEXT: ; ssa   _23   u8                                1b 1                var=18
// CHECK-NEXT: ; ssa   _24   u8                                1b 1                var=19
// CHECK-NEXT: ; ssa   _25   bool                              1b 1                var=20
// CHECK-NEXT: ; ssa   _26   i8                                1b 1                var=21
// CHECK-NEXT: ; ssa   _27   u8                                1b 1                var=22
// CHECK-NEXT: ; ssa   _28   u8                                1b 1                var=23
// CHECK-NEXT: ; ssa   _29   bool                              1b 1                var=24
// CHECK-NEXT: ; ssa   _30   i8                                1b 1                var=25
// CHECK-NEXT: ; ssa   _31   u8                                1b 1                var=26
// CHECK-NEXT: ; ssa   _32   u8                                1b 1                var=27
// CHECK-NEXT: ; ssa   _33   bool                              1b 1                var=28
// CHECK-NEXT: ; ssa   _34   i8                                1b 1                var=29
// CHECK-NEXT: ; ssa   _35   u8                                1b 1                var=30
// CHECK-NEXT: ; ssa   _36   u8                                1b 1                var=31
// CHECK-NEXT: ; ssa   _37   bool                              1b 1                var=32
// CHECK-NEXT: ; ssa   _38   bool                              1b 1                var=33
// CHECK-NEXT: ; ssa   _39   &'{erased} (i8, bool)             2b 2                var=34
// CHECK-NEXT: ; stack _40   (i8, bool)                        2b 1                storage=ss6
// CHECK-NEXT: ; ssa   _41   &'{erased} (i8, bool)             2b 2                var=35
// CHECK-NEXT: ; ssa   _42   bool                              1b 1                var=36
// CHECK-NEXT: ; ssa   _43   &'{erased} (i8, bool)             2b 2                var=37
// CHECK-NEXT: ; stack _44   (i8, bool)                        2b 1                storage=ss7
// CHECK-NEXT: ; ssa   _45   &'{erased} (i8, bool)             2b 2                var=38
// CHECK-NEXT: ; ssa   _46   &'{erased} (i8, bool)             2b 2                var=39
// CHECK-NEXT: ; stack _47   (i8, bool)                        2b 1                storage=ss8
// CHECK-NEXT: ; ssa   _48   &'{erased} (i8, bool)             2b 2                var=40
// CHECK-NEXT: ; ssa   _49   u8                                1b 1                var=41
// CHECK-NEXT: ; ssa   _50   u8                                1b 1                var=42
// CHECK-NEXT: ; ssa   _51   bool                              1b 1                var=43
// CHECK-NEXT: ; ssa   _52   u32                               4b 2                var=44
// CHECK-NEXT: ; ssa   _53   u32                               4b 2                var=45
// CHECK-NEXT: ; ssa   _54   u32                               4b 2                var=46
// CHECK-NEXT: ; ssa   _55   i8                                1b 1                var=47
// CHECK-NEXT: ; ssa   _56   bool                              1b 1                var=48
// CHECK-NEXT: ; ssa   _57   (i8, bool)                        2b 1                var=(49, 50)
// CHECK-NEXT: ; zst   _58   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _59   bool                              1b 1                var=51
// CHECK-NEXT: ; ssa   _60   isize                             2b 2                var=52
// CHECK-NEXT: ; ssa   _61   i8                                1b 1                var=53
// CHECK-NEXT: ; ssa   _62   bool                              1b 1                var=54
// CHECK-NEXT: ; ssa   _63   (i8, bool)                        2b 1                var=(55, 56)
// CHECK-NEXT: ; zst   _64   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _65   i8                                1b 1                var=57
// CHECK-NEXT: ; ssa   _66   i8                                1b 1                var=58
// CHECK-NEXT: ; ssa   _67   bool                              1b 1                var=59
// CHECK-NEXT: ; ssa   _68   (i8, bool)                        2b 1                var=(60, 61)
// CHECK-NEXT: ; zst   _69   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _70   bool                              1b 1                var=62
// CHECK-NEXT: ; ssa   _71   isize                             2b 2                var=63
// CHECK-NEXT: ; ssa   _72   u8                                1b 1                var=64
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
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc9
// CHECK-NEXT:     gv1 = symbol colocated userextname1 ; alloc10
// CHECK-NEXT:     gv2 = symbol colocated userextname2 ; alloc11
// CHECK-NEXT:     gv3 = symbol colocated userextname4 ; alloc6
// CHECK-NEXT:     gv4 = symbol colocated userextname5 ; alloc8
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [i8, bool] }
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [i8, bool] }
// CHECK-NEXT:     fn2 = colocated u0:1 sig2 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [i8, bool] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v17 -> v0
// CHECK-NEXT:     v175 -> v0
// CHECK-NEXT:     v176 -> v0
// CHECK-NEXT:     v18 -> v1
// CHECK-NEXT:     v177 -> v1
// CHECK-NEXT:     v178 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): i8 <- ByVal(v0): i8
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): i8 <- ByVal(v1): i8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = const (-127_i8, true)
// CHECK-NEXT:     v2 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): (i8, bool) <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): (i8, bool)
// CHECK-NEXT:     v3 = load.i8 notrap v2
// CHECK-NEXT:     v4 = load.i8 notrap v2+1
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v3, v5
// CHECK-NEXT:     v6 = stack_addr.i16 ss0+1
// CHECK-NEXT:     store notrap v4, v6
// CHECK-NEXT: ; _4 = const (125_i8, false)
// CHECK-NEXT:     v7 = symbol_value.i16 gv1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): (i8, bool) <- ByRef(Pointer { base: Addr(v7), offset: Offset32(0) }, None): (i8, bool)
// CHECK-NEXT:     v8 = load.i8 notrap v7
// CHECK-NEXT:     v9 = load.i8 notrap v7+1
// CHECK-NEXT:     v10 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v8, v10
// CHECK-NEXT:     v11 = stack_addr.i16 ss1+1
// CHECK-NEXT:     store notrap v9, v11
// CHECK-NEXT: ; _5 = const (-2_i8, true)
// CHECK-NEXT:     v12 = symbol_value.i16 gv2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): (i8, bool) <- ByRef(Pointer { base: Addr(v12), offset: Offset32(0) }, None): (i8, bool)
// CHECK-NEXT:     v13 = load.i8 notrap v12
// CHECK-NEXT:     v14 = load.i8 notrap v12+1
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v13, v15
// CHECK-NEXT:     v16 = stack_addr.i16 ss2+1
// CHECK-NEXT:     store notrap v14, v16
// CHECK-NEXT: ; _57 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v19 = iadd.i8 v17, v18
// CHECK-NEXT:     v80 -> v19
// CHECK-NEXT:     v185 -> v19
// CHECK-NEXT:     v186 -> v19
// CHECK-NEXT:     v197 -> v19
// CHECK-NEXT:     v198 -> v19
// CHECK-NEXT:     v20 = iconst.i8 0
// CHECK-NEXT:     v21 = icmp.i8 slt v18, v20  ; v20 = 0
// CHECK-NEXT:     v22 = icmp slt v19, v17
// CHECK-NEXT:     v23 = bxor v21, v22
// CHECK-NEXT:     v187 -> v23
// CHECK-NEXT:     v188 -> v23
// CHECK-NEXT:     v199 -> v23
// CHECK-NEXT:     v200 -> v23
// CHECK-NEXT: ; write_cvalue: VarPair(_57, var49, var50): (i8, bool) <- ByValPair(v19, v23): (i8, bool)
// CHECK-NEXT: ; _55 = copy (_57.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_55, var47): i8 <- ByVal(v19): i8
// CHECK-NEXT: ; _56 = copy (_57.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_56, var48): bool <- ByVal(v23): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _56)
// CHECK-NEXT:     brif v23, block13, block15
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _38)
// CHECK-NEXT:     brif.i8 v24, block3, block6
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _44 = copy _63
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): (i8, bool) <- ByValPair(v25, v26): (i8, bool)
// CHECK-NEXT:     v27 = stack_addr.i16 ss7
// CHECK-NEXT:     store.i8 notrap v25, v27
// CHECK-NEXT:     v28 = stack_addr.i16 ss7+1
// CHECK-NEXT:     store.i8 notrap v26, v28
// CHECK-NEXT: ; _43 = &_44
// CHECK-NEXT:     v29 = stack_addr.i16 ss7
// CHECK-NEXT: ; write_cvalue: Var(_43, var37): &'{erased} (i8, bool) <- ByVal(v29): &'{erased} (i8, bool)
// CHECK-NEXT: ; _45 = &_4
// CHECK-NEXT:     v30 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_45, var38): &'{erased} (i8, bool) <- ByVal(v30): &'{erased} (i8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _42 = <(i8, bool) as core::cmp::PartialEq>::eq(move _43, move _45)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v31 = call fn0(v29, v30)
// CHECK-NEXT:     v32 -> v31
// CHECK-NEXT: ; write_cvalue: Var(_42, var36): bool <- ByVal(v31): bool
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _42)
// CHECK-NEXT:     brif.i8 v32, block5, block6
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _47 = copy _68
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): (i8, bool) <- ByValPair(v33, v34): (i8, bool)
// CHECK-NEXT:     v35 = stack_addr.i16 ss8
// CHECK-NEXT:     store.i8 notrap v33, v35
// CHECK-NEXT:     v36 = stack_addr.i16 ss8+1
// CHECK-NEXT:     store.i8 notrap v34, v36
// CHECK-NEXT: ; _46 = &_47
// CHECK-NEXT:     v37 = stack_addr.i16 ss8
// CHECK-NEXT: ; write_cvalue: Var(_46, var39): &'{erased} (i8, bool) <- ByVal(v37): &'{erased} (i8, bool)
// CHECK-NEXT: ; _48 = &_5
// CHECK-NEXT:     v38 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_48, var40): &'{erased} (i8, bool) <- ByVal(v38): &'{erased} (i8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _37 = <(i8, bool) as core::cmp::PartialEq>::eq(move _46, move _48)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v39 = call fn1(v37, v38)
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): bool <- ByVal(v39): bool
// CHECK-NEXT:     jump block7(v39)
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _37 = const false
// CHECK-NEXT:     v40 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): bool <- ByVal(v40): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block7(v40)  ; v40 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block7(v41: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _36 = move _37 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_36, var31): u8 <- ByVal(v41): u8
// CHECK-NEXT: ; _35 = Shl(move _36, const 6_i32)
// CHECK-NEXT:     v42 = iconst.i32 6
// CHECK-NEXT:     v43 = ishl v41, v42  ; v42 = 6
// CHECK-NEXT: ; write_cvalue: Var(_35, var30): u8 <- ByVal(v43): u8
// CHECK-NEXT: ; _6 = BitOr(move _7, move _35)
// CHECK-NEXT:     v45 = bor.i8 v44, v43
// CHECK-NEXT:     v172 -> v45
// CHECK-NEXT:     v173 -> v45
// CHECK-NEXT:     v174 -> v45
// CHECK-NEXT: ; write_cvalue: Var(_6, var4): u8 <- ByVal(v45): u8
// CHECK-NEXT: ; _72 = copy _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_72, var64): u8 <- ByVal(v46): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _52 = core::intrinsics::ctlz::<u8>(copy _72)
// CHECK-NEXT:     v47 = clz.i8 v46
// CHECK-NEXT:     v48 = uextend.i32 v47
// CHECK-NEXT:     v156 -> v48
// CHECK-NEXT: ; write_cvalue: Var(_52, var44): u32 <- ByVal(v48): u32
// CHECK-NEXT:     jump block24
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _53 = core::intrinsics::cttz::<u8>(copy _72)
// CHECK-NEXT:     v50 = ctz.i8 v49
// CHECK-NEXT:     v51 = uextend.i32 v50
// CHECK-NEXT:     v159 -> v51
// CHECK-NEXT: ; write_cvalue: Var(_53, var45): u32 <- ByVal(v51): u32
// CHECK-NEXT:     jump block25
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _54 = core::intrinsics::ctpop::<u8>(move _72)
// CHECK-NEXT:     v53 = popcnt.i8 v52
// CHECK-NEXT:     v54 = uextend.i32 v53
// CHECK-NEXT:     v160 -> v54
// CHECK-NEXT: ; write_cvalue: Var(_54, var46): u32 <- ByVal(v54): u32
// CHECK-NEXT:     jump block26
// CHECK-EMPTY:
// CHECK-NEXT: block10(v171: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _51 = const false
// CHECK-NEXT:     v55 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_51, var43): bool <- ByVal(v55): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block11(v55, v171)  ; v55 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block11(v56: i8, v59: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _50 = move _51 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_50, var42): u8 <- ByVal(v56): u8
// CHECK-NEXT: ; _49 = Shl(move _50, const 7_i32)
// CHECK-NEXT:     v57 = iconst.i32 7
// CHECK-NEXT:     v58 = ishl v56, v57  ; v57 = 7
// CHECK-NEXT: ; write_cvalue: Var(_49, var41): u8 <- ByVal(v58): u8
// CHECK-NEXT: ; _0 = BitOr(move _6, move _49)
// CHECK-NEXT:     v60 = bor v59, v58
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v60): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v60
// CHECK-EMPTY:
// CHECK-NEXT: block12(v68: i8, v69: i8):
// CHECK-NEXT:     v98 -> v68
// CHECK-NEXT:     v181 -> v68
// CHECK-NEXT:     v182 -> v68
// CHECK-NEXT:     v99 -> v69
// CHECK-NEXT:     v183 -> v69
// CHECK-NEXT:     v184 -> v69
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = discriminant(_14)
// CHECK-NEXT:     v61 = stack_addr.i16 ss3
// CHECK-NEXT:     v62 = load.i8 notrap v61
// CHECK-NEXT:     v63 = uextend.i16 v62
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): isize <- ByVal(v63): isize
// CHECK-NEXT: ; _59 = Eq(copy _60, const 1_isize)
// CHECK-NEXT:     v64 = iconst.i16 1
// CHECK-NEXT:     v65 = icmp eq v63, v64  ; v64 = 1
// CHECK-NEXT: ; write_cvalue: Var(_59, var51): bool <- ByVal(v65): bool
// CHECK-NEXT: ; _13 = Not(move _59)
// CHECK-NEXT:     v66 = iconst.i8 0
// CHECK-NEXT:     v67 = icmp eq v65, v66  ; v66 = 0
// CHECK-NEXT:     v96 -> v67
// CHECK-NEXT: ; write_cvalue: Var(_13, var11): bool <- ByVal(v67): bool
// CHECK-NEXT: ; _12 = move _13 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): u8 <- ByVal(v67): u8
// CHECK-NEXT: ; _63 = SubWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v70 = isub v68, v69
// CHECK-NEXT:     v85 -> v70
// CHECK-NEXT:     v189 -> v70
// CHECK-NEXT:     v190 -> v70
// CHECK-NEXT:     v201 -> v70
// CHECK-NEXT:     v71 = iconst.i8 0
// CHECK-NEXT:     v72 = icmp slt v69, v71  ; v71 = 0
// CHECK-NEXT:     v73 = icmp sgt v70, v68
// CHECK-NEXT:     v74 = bxor v72, v73
// CHECK-NEXT:     v191 -> v74
// CHECK-NEXT:     v192 -> v74
// CHECK-NEXT:     v202 -> v74
// CHECK-NEXT: ; write_cvalue: VarPair(_63, var55, var56): (i8, bool) <- ByValPair(v70, v74): (i8, bool)
// CHECK-NEXT: ; _61 = copy (_63.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_61, var53): i8 <- ByVal(v70): i8
// CHECK-NEXT: ; _62 = copy (_63.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_62, var54): bool <- ByVal(v74): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _62)
// CHECK-NEXT:     brif v74, block16, block18
// CHECK-EMPTY:
// CHECK-NEXT: block13 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _58 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block14
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _14 = const core::option::Option::<i8>::None
// CHECK-NEXT:     v75 = symbol_value.i16 gv3
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): core::option::Option<i8> <- ByRef(Pointer { base: Addr(v75), offset: Offset32(0) }, None): core::option::Option<i8>
// CHECK-NEXT:     v76 = load.i8 notrap v75
// CHECK-NEXT:     v77 = load.i8 notrap v75+1
// CHECK-NEXT:     v78 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v76, v78
// CHECK-NEXT:     v79 = stack_addr.i16 ss3+1
// CHECK-NEXT:     store notrap v77, v79
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v175, v177)
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _14 = core::option::Option::<i8>::Some(copy _55)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(1) }, None): i8 <- ByVal(v80): i8
// CHECK-NEXT:     v81 = stack_addr.i16 ss3+1
// CHECK-NEXT:     store.i8 notrap v80, v81
// CHECK-NEXT:     v82 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): u8 <- ByVal(v82): u8
// CHECK-NEXT:     v83 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v82, v83  ; v82 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v176, v178)
// CHECK-EMPTY:
// CHECK-NEXT: block16 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _64 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block17
// CHECK-EMPTY:
// CHECK-NEXT: block17:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _17 = const false
// CHECK-NEXT:     v84 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_17, var14): bool <- ByVal(v84): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block19(v84)  ; v84 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block18:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _18 = core::option::Option::<i8>::Some(copy _61)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(1) }, None): i8 <- ByVal(v85): i8
// CHECK-NEXT:     v86 = stack_addr.i16 ss4+1
// CHECK-NEXT:     store.i8 notrap v85, v86
// CHECK-NEXT:     v87 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): u8 <- ByVal(v87): u8
// CHECK-NEXT:     v88 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v87, v88  ; v87 = 1
// CHECK-NEXT: ; _65 = copy ((_18 as Some).0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_65, var57): i8 <- ByRef(Pointer { base: Stack(ss4), offset: Offset32(1) }, None): i8
// CHECK-NEXT:     v89 = stack_addr.i16 ss4+1
// CHECK-NEXT:     v90 = load.i8 notrap v89
// CHECK-NEXT: ; _17 = Eq(move _65, const 125_i8)
// CHECK-NEXT:     v91 = iconst.i8 125
// CHECK-NEXT:     v92 = icmp eq v90, v91  ; v91 = 125
// CHECK-NEXT: ; write_cvalue: Var(_17, var14): bool <- ByVal(v92): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block19(v92)
// CHECK-EMPTY:
// CHECK-NEXT: block19(v93: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = move _17 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_16, var13): u8 <- ByVal(v93): u8
// CHECK-NEXT: ; _15 = Shl(move _16, const 1_i32)
// CHECK-NEXT:     v94 = iconst.i32 1
// CHECK-NEXT:     v95 = ishl v93, v94  ; v94 = 1
// CHECK-NEXT: ; write_cvalue: Var(_15, var12): u8 <- ByVal(v95): u8
// CHECK-NEXT: ; _11 = BitOr(move _12, move _15)
// CHECK-NEXT:     v97 = bor.i8 v96, v95
// CHECK-NEXT:     v179 -> v97
// CHECK-NEXT:     v180 -> v97
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): u8 <- ByVal(v97): u8
// CHECK-NEXT: ; _68 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v100 = sextend.i16 v98
// CHECK-NEXT:     v101 = sextend.i16 v99
// CHECK-NEXT:     v102 = imul v100, v101
// CHECK-NEXT:     v103 = iconst.i16 -128
// CHECK-NEXT:     v104 = icmp slt v102, v103  ; v103 = -128
// CHECK-NEXT:     v105 = iconst.i16 127
// CHECK-NEXT:     v106 = icmp sgt v102, v105  ; v105 = 127
// CHECK-NEXT:     v107 = ireduce.i8 v102
// CHECK-NEXT:     v152 -> v107
// CHECK-NEXT:     v193 -> v107
// CHECK-NEXT:     v194 -> v107
// CHECK-NEXT:     v108 = bor v104, v106
// CHECK-NEXT:     v195 -> v108
// CHECK-NEXT:     v196 -> v108
// CHECK-NEXT: ; write_cvalue: VarPair(_68, var60, var61): (i8, bool) <- ByValPair(v107, v108): (i8, bool)
// CHECK-NEXT: ; _66 = copy (_68.0: i8)
// CHECK-NEXT: ; write_cvalue: Var(_66, var58): i8 <- ByVal(v107): i8
// CHECK-NEXT: ; _67 = copy (_68.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_67, var59): bool <- ByVal(v108): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _67)
// CHECK-NEXT:     brif v108, block21, block23
// CHECK-EMPTY:
// CHECK-NEXT: block20(v118: i8, v120: i8, v121: i8, v140: i8, v141: i8, v163: i8, v164: i8, v165: i8, v166: i8):
// CHECK-NEXT:     v46 -> v120
// CHECK-NEXT:     v168 -> v120
// CHECK-NEXT:     v169 -> v120
// CHECK-NEXT:     v170 -> v120
// CHECK-NEXT:     v52 -> v170
// CHECK-NEXT:     v49 -> v169
// CHECK-NEXT:     v25 -> v163
// CHECK-NEXT:     v26 -> v164
// CHECK-NEXT:     v33 -> v165
// CHECK-NEXT:     v34 -> v166
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _71 = discriminant(_22)
// CHECK-NEXT:     v109 = stack_addr.i16 ss5
// CHECK-NEXT:     v110 = load.i8 notrap v109
// CHECK-NEXT:     v111 = uextend.i16 v110
// CHECK-NEXT: ; write_cvalue: Var(_71, var63): isize <- ByVal(v111): isize
// CHECK-NEXT: ; _70 = Eq(copy _71, const 1_isize)
// CHECK-NEXT:     v112 = iconst.i16 1
// CHECK-NEXT:     v113 = icmp eq v111, v112  ; v112 = 1
// CHECK-NEXT: ; write_cvalue: Var(_70, var62): bool <- ByVal(v113): bool
// CHECK-NEXT: ; _21 = Not(move _70)
// CHECK-NEXT:     v114 = iconst.i8 0
// CHECK-NEXT:     v115 = icmp eq v113, v114  ; v114 = 0
// CHECK-NEXT: ; write_cvalue: Var(_21, var17): bool <- ByVal(v115): bool
// CHECK-NEXT: ; _20 = move _21 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_20, var16): u8 <- ByVal(v115): u8
// CHECK-NEXT: ; _19 = Shl(move _20, const 2_i32)
// CHECK-NEXT:     v116 = iconst.i32 2
// CHECK-NEXT:     v117 = ishl v115, v116  ; v116 = 2
// CHECK-NEXT: ; write_cvalue: Var(_19, var15): u8 <- ByVal(v117): u8
// CHECK-NEXT: ; _10 = BitOr(move _11, move _19)
// CHECK-NEXT:     v119 = bor v118, v117
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): u8 <- ByVal(v119): u8
// CHECK-NEXT: ; _26 = Add(copy _1, copy _2)
// CHECK-NEXT:     v122 = iadd v120, v121
// CHECK-NEXT: ; write_cvalue: Var(_26, var21): i8 <- ByVal(v122): i8
// CHECK-NEXT: ; _25 = Eq(move _26, const -127_i8)
// CHECK-NEXT:     v123 = iconst.i8 -127
// CHECK-NEXT:     v124 = icmp eq v122, v123  ; v123 = -127
// CHECK-NEXT: ; write_cvalue: Var(_25, var20): bool <- ByVal(v124): bool
// CHECK-NEXT: ; _24 = move _25 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_24, var19): u8 <- ByVal(v124): u8
// CHECK-NEXT: ; _23 = Shl(move _24, const 3_i32)
// CHECK-NEXT:     v125 = iconst.i32 3
// CHECK-NEXT:     v126 = ishl v124, v125  ; v125 = 3
// CHECK-NEXT: ; write_cvalue: Var(_23, var18): u8 <- ByVal(v126): u8
// CHECK-NEXT: ; _9 = BitOr(move _10, move _23)
// CHECK-NEXT:     v127 = bor v119, v126
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): u8 <- ByVal(v127): u8
// CHECK-NEXT: ; _30 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v128 = isub v120, v121
// CHECK-NEXT: ; write_cvalue: Var(_30, var25): i8 <- ByVal(v128): i8
// CHECK-NEXT: ; _29 = Eq(move _30, const 125_i8)
// CHECK-NEXT:     v129 = iconst.i8 125
// CHECK-NEXT:     v130 = icmp eq v128, v129  ; v129 = 125
// CHECK-NEXT: ; write_cvalue: Var(_29, var24): bool <- ByVal(v130): bool
// CHECK-NEXT: ; _28 = move _29 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_28, var23): u8 <- ByVal(v130): u8
// CHECK-NEXT: ; _27 = Shl(move _28, const 4_i32)
// CHECK-NEXT:     v131 = iconst.i32 4
// CHECK-NEXT:     v132 = ishl v130, v131  ; v131 = 4
// CHECK-NEXT: ; write_cvalue: Var(_27, var22): u8 <- ByVal(v132): u8
// CHECK-NEXT: ; _8 = BitOr(move _9, move _27)
// CHECK-NEXT:     v133 = bor v127, v132
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): u8 <- ByVal(v133): u8
// CHECK-NEXT: ; _34 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v134 = imul v120, v121
// CHECK-NEXT: ; write_cvalue: Var(_34, var29): i8 <- ByVal(v134): i8
// CHECK-NEXT: ; _33 = Eq(move _34, const -2_i8)
// CHECK-NEXT:     v135 = iconst.i8 -2
// CHECK-NEXT:     v136 = icmp eq v134, v135  ; v135 = -2
// CHECK-NEXT: ; write_cvalue: Var(_33, var28): bool <- ByVal(v136): bool
// CHECK-NEXT: ; _32 = move _33 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_32, var27): u8 <- ByVal(v136): u8
// CHECK-NEXT: ; _31 = Shl(move _32, const 5_i32)
// CHECK-NEXT:     v137 = iconst.i32 5
// CHECK-NEXT:     v138 = ishl v136, v137  ; v137 = 5
// CHECK-NEXT: ; write_cvalue: Var(_31, var26): u8 <- ByVal(v138): u8
// CHECK-NEXT: ; _7 = BitOr(move _8, move _31)
// CHECK-NEXT:     v139 = bor v133, v138
// CHECK-NEXT:     v44 -> v139
// CHECK-NEXT:     v167 -> v139
// CHECK-NEXT: ; write_cvalue: Var(_7, var5): u8 <- ByVal(v139): u8
// CHECK-NEXT: ; _40 = copy _57
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): (i8, bool) <- ByValPair(v140, v141): (i8, bool)
// CHECK-NEXT:     v142 = stack_addr.i16 ss6
// CHECK-NEXT:     store notrap v140, v142
// CHECK-NEXT:     v143 = stack_addr.i16 ss6+1
// CHECK-NEXT:     store notrap v141, v143
// CHECK-NEXT: ; _39 = &_40
// CHECK-NEXT:     v144 = stack_addr.i16 ss6
// CHECK-NEXT: ; write_cvalue: Var(_39, var34): &'{erased} (i8, bool) <- ByVal(v144): &'{erased} (i8, bool)
// CHECK-NEXT: ; _41 = &_3
// CHECK-NEXT:     v145 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_41, var35): &'{erased} (i8, bool) <- ByVal(v145): &'{erased} (i8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _38 = <(i8, bool) as core::cmp::PartialEq>::eq(move _39, move _41)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(i8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v146 = call fn2(v144, v145)
// CHECK-NEXT:     v24 -> v146
// CHECK-NEXT: ; write_cvalue: Var(_38, var33): bool <- ByVal(v146): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block21 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _69 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block22
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = const core::option::Option::<i8>::None
// CHECK-NEXT:     v147 = symbol_value.i16 gv4
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): core::option::Option<i8> <- ByRef(Pointer { base: Addr(v147), offset: Offset32(0) }, None): core::option::Option<i8>
// CHECK-NEXT:     v148 = load.i8 notrap v147
// CHECK-NEXT:     v149 = load.i8 notrap v147+1
// CHECK-NEXT:     v150 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v148, v150
// CHECK-NEXT:     v151 = stack_addr.i16 ss5+1
// CHECK-NEXT:     store notrap v149, v151
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block20(v179, v181, v183, v185, v187, v189, v191, v193, v195)
// CHECK-EMPTY:
// CHECK-NEXT: block23:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = core::option::Option::<i8>::Some(copy _66)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(1) }, None): i8 <- ByVal(v152): i8
// CHECK-NEXT:     v153 = stack_addr.i16 ss5+1
// CHECK-NEXT:     store.i8 notrap v152, v153
// CHECK-NEXT:     v154 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): u8 <- ByVal(v154): u8
// CHECK-NEXT:     v155 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v154, v155  ; v154 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block20(v180, v182, v184, v186, v188, v190, v192, v194, v196)
// CHECK-EMPTY:
// CHECK-NEXT: block24:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _52)
// CHECK-NEXT:     v157 = iconst.i32 1
// CHECK-NEXT:     v158 = icmp.i32 eq v156, v157  ; v157 = 1
// CHECK-NEXT:     brif v158, block8, block10(v172)
// CHECK-EMPTY:
// CHECK-NEXT: block25:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _53)
// CHECK-NEXT:     brif.i32 v159, block10(v173), block9
// CHECK-EMPTY:
// CHECK-NEXT: block26:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _51 = Eq(move _54, const 7_u32)
// CHECK-NEXT:     v161 = iconst.i32 7
// CHECK-NEXT:     v162 = icmp.i32 eq v160, v161  ; v161 = 7
// CHECK-NEXT: ; write_cvalue: Var(_51, var43): bool <- ByVal(v162): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block11(v162, v174)
// CHECK-NEXT: }
