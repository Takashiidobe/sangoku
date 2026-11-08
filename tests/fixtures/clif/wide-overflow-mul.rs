#![no_std]

#[unsafe(no_mangle)]
pub fn checked64(x: u64, y: u64) -> (u64, bool) { x.overflowing_add(y) }
#[inline(never)]
#[unsafe(no_mangle)]
pub fn checked128(x: u128, y: u128) -> (u128, bool) { x.overflowing_mul(y) }
#[unsafe(no_mangle)]
pub fn signed128(x: i128, y: i128) -> i128 { x.wrapping_add(y) }
#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    (checked64(u64::MAX, 1) == (0, true)
        && checked128(u128::MAX, 2) == (u128::MAX - 1, true)
        && checked128(3, 5) == (15, false)
        && checked128(0, u128::MAX) == (0, false)
        && checked128(u128::MAX, 1) == (u128::MAX, false)
        && checked128(1 << 127, 2) == (0, true)
        && signed128(-3, 5) == 2) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTobENtNtB7_3cmp9PartialEq2eqCs8MiQZvKHqG6_17wide_overflow_mul
// CHECK-NEXT: function u0:0(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTobENtNtB7_3cmp9PartialEq2eqCs8MiQZvKHqG6_17wide_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (u128, bool)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (u128, bool)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} (u128, bool)           2b 2                storage=ss0
// CHECK-NEXT: ; stack _2    &'{erased} (u128, bool)           2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _4    &'{erased} u128                   2b 2                var=3
// CHECK-NEXT: ; ssa   _5    &'{erased} u128                   2b 2                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} bool                   2b 2                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} bool                   2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(1:29984 ~ core[..]::cmp::impls::{impl#26}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:2 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (u128, bool) <- ByVal(v0): &'{erased} (u128, bool)
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u128, bool) <- ByVal(v1): &'{erased} (u128, bool)
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = &((*_1).0: U)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): &'{erased} u128 <- ByVal(v5): &'{erased} u128
// CHECK-NEXT: ; _5 = &((*_2).0: U)
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): &'{erased} u128 <- ByVal(v7): &'{erased} u128
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = <U as core::cmp::PartialEq>::eq(move _4, move _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &u128, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &u128, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT:     v12 = iconst.i16 16
// CHECK-NEXT:     v13 = iadd v11, v12  ; v12 = 16
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): &'{erased} bool <- ByVal(v13): &'{erased} bool
// CHECK-NEXT: ; _7 = &((*_2).1: T)
// CHECK-NEXT:     v14 = stack_addr.i16 ss1
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = iconst.i16 16
// CHECK-NEXT:     v17 = iadd v15, v16  ; v16 = 16
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
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTybENtNtB7_3cmp9PartialEq2eqCs8MiQZvKHqG6_17wide_overflow_mul
// CHECK-NEXT: function u0:1(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTybENtNtB7_3cmp9PartialEq2eqCs8MiQZvKHqG6_17wide_overflow_mul
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
// CHECK-NEXT:     fn0 = colocated u0:3 sig0 ; Instance { def: Item(DefId(1:29981 ~ core[..]::cmp::impls::{impl#25}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:2 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
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
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCs8MiQZvKHqG6_17wide_overflow_mul
// CHECK-NEXT: function u0:2(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCs8MiQZvKHqG6_17wide_overflow_mul
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
// CHECK-NEXT: ; symbol _RNvXsn_NtNtCs40R3J0iFVxM_4core3cmp5implsyNtB7_9PartialEq2eqCs8MiQZvKHqG6_17wide_overflow_mul
// CHECK-NEXT: function u0:3(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsn_NtNtCs40R3J0iFVxM_4core3cmp5implsyNtB7_9PartialEq2eqCs8MiQZvKHqG6_17wide_overflow_mul
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
// CHECK-NEXT: ; symbol _RNvXso_NtNtCs40R3J0iFVxM_4core3cmp5implsoNtB7_9PartialEq2eqCs8MiQZvKHqG6_17wide_overflow_mul
// CHECK-NEXT: function u0:4(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXso_NtNtCs40R3J0iFVxM_4core3cmp5implsoNtB7_9PartialEq2eqCs8MiQZvKHqG6_17wide_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} u128
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} u128                   2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} u128                   2b 2                var=3
// CHECK-NEXT: ; ssa   _3    u128                             16b 2                var=4
// CHECK-NEXT: ; ssa   _4    u128                             16b 2                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} u128 <- ByVal(v0): &'{erased} u128
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} u128 <- ByVal(v1): &'{erased} u128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): u128 <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): u128
// CHECK-NEXT:     v3 = load.i128 notrap v2
// CHECK-NEXT: ; _4 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): u128 <- ByRef(Pointer { base: Addr(v4), offset: Offset32(0) }, None): u128
// CHECK-NEXT:     v5 = load.i128 notrap v4
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
// CHECK-NEXT: ; symbol checked128
// CHECK-NEXT: function u0:5(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol checked128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } (u128, bool)
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 16, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 16, align = 2
// CHECK-NEXT:     sig0 = (i16, i16, i16) system_v
// CHECK-NEXT:     fn0 = u0:9 sig0 ; "__gbc_u128_mulo"
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i128, v2: i128):
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v4 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): u128 <- ByVal(v1): u128
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): u128 <- ByVal(v2): u128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u128 <- ByVal(v3): u128
// CHECK-NEXT:     v5 = stack_addr.i16 ss1
// CHECK-NEXT:     store.i128 notrap v3, v5
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u128 <- ByVal(v4): u128
// CHECK-NEXT:     v7 = stack_addr.i16 ss2
// CHECK-NEXT:     store.i128 notrap v4, v7
// CHECK-NEXT:     v8 = stack_addr.i16 ss2
// CHECK-NEXT:     v9 = stack_addr.i16 ss0
// CHECK-NEXT: ; lib_call __gbc_u128_mulo
// CHECK-NEXT:     call fn0(v9, v6, v8)
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     v11 = load.i128 notrap v10
// CHECK-NEXT:     v12 = stack_addr.i16 ss0+16
// CHECK-NEXT:     v13 = load.i8 notrap v12
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): (u128, bool) <- ByValPair(v11, v13): (u128, bool)
// CHECK-NEXT:     store notrap v11, v0
// CHECK-NEXT:     store notrap v13, v0+16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol checked64
// CHECK-NEXT: function u0:6(i16 sret, i64, i64) system_v {
// CHECK-NEXT: ; symbol checked64
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
// CHECK-NEXT: ; _0 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v5 = iadd.i64 v3, v4
// CHECK-NEXT:     v6 = icmp ult v5, v3
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): (u64, bool) <- ByValPair(v5, v6): (u64, bool)
// CHECK-NEXT:     store notrap v5, v0
// CHECK-NEXT:     store notrap v6, v0+8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:7() -> i8 system_v {
// CHECK-NEXT: ; symbol gbc_main
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _2    bool                              1b 1                var=3
// CHECK-NEXT: ; ssa   _3    &'{erased} (u64, bool)            2b 2                var=4
// CHECK-NEXT: ; stack _4    (u64, bool)                      10b 2                storage=ss0
// CHECK-NEXT: ; ssa   _5    bool                              1b 1                var=5
// CHECK-NEXT: ; ssa   _6    &'{erased} (u128, bool)           2b 2                var=6
// CHECK-NEXT: ; stack _7    (u128, bool)                     18b 2                storage=ss1
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=7
// CHECK-NEXT: ; ssa   _9    &'{erased} (u128, bool)           2b 2                var=8
// CHECK-NEXT: ; stack _10   (u128, bool)                     18b 2                storage=ss2
// CHECK-NEXT: ; ssa   _11   bool                              1b 1                var=9
// CHECK-NEXT: ; ssa   _12   &'{erased} (u128, bool)           2b 2                var=10
// CHECK-NEXT: ; stack _13   (u128, bool)                     18b 2                storage=ss3
// CHECK-NEXT: ; ssa   _14   bool                              1b 1                var=11
// CHECK-NEXT: ; ssa   _15   &'{erased} (u128, bool)           2b 2                var=12
// CHECK-NEXT: ; stack _16   (u128, bool)                     18b 2                storage=ss4
// CHECK-NEXT: ; ssa   _17   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _18   &'{erased} (u128, bool)           2b 2                var=14
// CHECK-NEXT: ; stack _19   (u128, bool)                     18b 2                storage=ss5
// CHECK-NEXT: ; ssa   _20   i128                             16b 2                var=15
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss3 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss4 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss5 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss6 = explicit_slot 16, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname1 ; alloc4
// CHECK-NEXT:     gv1 = symbol colocated userextname4 ; alloc6
// CHECK-NEXT:     gv2 = symbol colocated userextname6 ; alloc8
// CHECK-NEXT:     gv3 = symbol colocated userextname7 ; alloc10
// CHECK-NEXT:     gv4 = symbol colocated userextname8 ; alloc12
// CHECK-NEXT:     gv5 = symbol colocated userextname9 ; alloc14
// CHECK-NEXT:     sig0 = (i16 sret, i64, i64) system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig4 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig5 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig6 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig7 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig8 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig9 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig10 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig11 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig12 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     fn0 = colocated u0:6 sig0 ; Instance { def: Item(DefId(0:3 ~ wide_overflow_mul[..]::checked64)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u64, bool] }
// CHECK-NEXT:     fn2 = colocated u0:5 sig2 ; Instance { def: Item(DefId(0:4 ~ wide_overflow_mul[..]::checked128)), args: [] }
// CHECK-NEXT:     fn3 = colocated u0:0 sig3 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u128, bool] }
// CHECK-NEXT:     fn4 = colocated u0:5 sig4 ; Instance { def: Item(DefId(0:4 ~ wide_overflow_mul[..]::checked128)), args: [] }
// CHECK-NEXT:     fn5 = colocated u0:0 sig5 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u128, bool] }
// CHECK-NEXT:     fn6 = colocated u0:5 sig6 ; Instance { def: Item(DefId(0:4 ~ wide_overflow_mul[..]::checked128)), args: [] }
// CHECK-NEXT:     fn7 = colocated u0:0 sig7 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u128, bool] }
// CHECK-NEXT:     fn8 = colocated u0:5 sig8 ; Instance { def: Item(DefId(0:4 ~ wide_overflow_mul[..]::checked128)), args: [] }
// CHECK-NEXT:     fn9 = colocated u0:0 sig9 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u128, bool] }
// CHECK-NEXT:     fn10 = colocated u0:5 sig10 ; Instance { def: Item(DefId(0:4 ~ wide_overflow_mul[..]::checked128)), args: [] }
// CHECK-NEXT:     fn11 = colocated u0:0 sig11 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u128, bool] }
// CHECK-NEXT:     fn12 = colocated u0:8 sig12 ; Instance { def: Item(DefId(0:5 ~ wide_overflow_mul[..]::signed128)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = checked64(const u64::MAX, const 1_u64)
// CHECK-NEXT:     v0 = iconst.i64 -1
// CHECK-NEXT:     v1 = iconst.i64 1
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u64, layout: Layout { size: Size(8 bytes), align: Align(2 bytes), backend_repr: Scalar(u64 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934599 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u64, layout: Layout { size: Size(8 bytes), align: Align(2 bytes), backend_repr: Scalar(u64 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934599 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u64, bool), layout: Layout { size: Size(10 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u64 is .., b: u8 is 0..=1, b_offset: Size(8 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(8 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(8 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934599 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(10 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn0(v2, v0, v1)  ; v0 = -1, v1 = 1
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = &_4
// CHECK-NEXT:     v3 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): &'{erased} (u64, bool) <- ByVal(v3): &'{erased} (u64, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = <(u64, bool) as core::cmp::PartialEq>::eq(move _3, const gbc_main::promoted[5])
// CHECK-NEXT:     v4 = symbol_value.i16 gv0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u64, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u64, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v5 = call fn1(v3, v4)
// CHECK-NEXT:     v6 -> v5
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): bool <- ByVal(v5): bool
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _2)
// CHECK-NEXT:     brif.i8 v6, block4, block20
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = checked128(const u128::MAX, const 2_u128)
// CHECK-NEXT:     v7 = iconst.i64 -1
// CHECK-NEXT:     v8 = iconst.i64 -1
// CHECK-NEXT:     v9 = iconcat v7, v8  ; v7 = -1, v8 = -1
// CHECK-NEXT:     v10 = iconst.i64 2
// CHECK-NEXT:     v11 = iconst.i64 0
// CHECK-NEXT:     v12 = iconcat v10, v11  ; v10 = 2, v11 = 0
// CHECK-NEXT:     v13 = stack_addr.i16 ss1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v13, v9, v12)
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = &_7
// CHECK-NEXT:     v14 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_6, var6): &'{erased} (u128, bool) <- ByVal(v14): &'{erased} (u128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = <(u128, bool) as core::cmp::PartialEq>::eq(move _6, const gbc_main::promoted[4])
// CHECK-NEXT:     v15 = symbol_value.i16 gv1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v16 = call fn3(v14, v15)
// CHECK-NEXT:     v17 -> v16
// CHECK-NEXT: ; write_cvalue: Var(_5, var5): bool <- ByVal(v16): bool
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _5)
// CHECK-NEXT:     brif.i8 v17, block7, block20
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _10 = checked128(const 3_u128, const 5_u128)
// CHECK-NEXT:     v18 = iconst.i64 3
// CHECK-NEXT:     v19 = iconst.i64 0
// CHECK-NEXT:     v20 = iconcat v18, v19  ; v18 = 3, v19 = 0
// CHECK-NEXT:     v21 = iconst.i64 5
// CHECK-NEXT:     v22 = iconst.i64 0
// CHECK-NEXT:     v23 = iconcat v21, v22  ; v21 = 5, v22 = 0
// CHECK-NEXT:     v24 = stack_addr.i16 ss2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn4(v24, v20, v23)
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = &_10
// CHECK-NEXT:     v25 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_9, var8): &'{erased} (u128, bool) <- ByVal(v25): &'{erased} (u128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = <(u128, bool) as core::cmp::PartialEq>::eq(move _9, const gbc_main::promoted[3])
// CHECK-NEXT:     v26 = symbol_value.i16 gv2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v27 = call fn5(v25, v26)
// CHECK-NEXT:     v28 -> v27
// CHECK-NEXT: ; write_cvalue: Var(_8, var7): bool <- ByVal(v27): bool
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _8)
// CHECK-NEXT:     brif.i8 v28, block10, block20
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = checked128(const 0_u128, const u128::MAX)
// CHECK-NEXT:     v29 = iconst.i64 0
// CHECK-NEXT:     v30 = iconst.i64 0
// CHECK-NEXT:     v31 = iconcat v29, v30  ; v29 = 0, v30 = 0
// CHECK-NEXT:     v32 = iconst.i64 -1
// CHECK-NEXT:     v33 = iconst.i64 -1
// CHECK-NEXT:     v34 = iconcat v32, v33  ; v32 = -1, v33 = -1
// CHECK-NEXT:     v35 = stack_addr.i16 ss3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn6(v35, v31, v34)
// CHECK-NEXT:     jump block11
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _12 = &_13
// CHECK-NEXT:     v36 = stack_addr.i16 ss3
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): &'{erased} (u128, bool) <- ByVal(v36): &'{erased} (u128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _11 = <(u128, bool) as core::cmp::PartialEq>::eq(move _12, const gbc_main::promoted[2])
// CHECK-NEXT:     v37 = symbol_value.i16 gv3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v38 = call fn7(v36, v37)
// CHECK-NEXT:     v39 -> v38
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): bool <- ByVal(v38): bool
// CHECK-NEXT:     jump block12
// CHECK-EMPTY:
// CHECK-NEXT: block12:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _11)
// CHECK-NEXT:     brif.i8 v39, block13, block20
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _16 = checked128(const u128::MAX, const 1_u128)
// CHECK-NEXT:     v40 = iconst.i64 -1
// CHECK-NEXT:     v41 = iconst.i64 -1
// CHECK-NEXT:     v42 = iconcat v40, v41  ; v40 = -1, v41 = -1
// CHECK-NEXT:     v43 = iconst.i64 1
// CHECK-NEXT:     v44 = iconst.i64 0
// CHECK-NEXT:     v45 = iconcat v43, v44  ; v43 = 1, v44 = 0
// CHECK-NEXT:     v46 = stack_addr.i16 ss4
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn8(v46, v42, v45)
// CHECK-NEXT:     jump block14
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = &_16
// CHECK-NEXT:     v47 = stack_addr.i16 ss4
// CHECK-NEXT: ; write_cvalue: Var(_15, var12): &'{erased} (u128, bool) <- ByVal(v47): &'{erased} (u128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _14 = <(u128, bool) as core::cmp::PartialEq>::eq(move _15, const gbc_main::promoted[1])
// CHECK-NEXT:     v48 = symbol_value.i16 gv4
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v49 = call fn9(v47, v48)
// CHECK-NEXT:     v50 -> v49
// CHECK-NEXT: ; write_cvalue: Var(_14, var11): bool <- ByVal(v49): bool
// CHECK-NEXT:     jump block15
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _14)
// CHECK-NEXT:     brif.i8 v50, block16, block20
// CHECK-EMPTY:
// CHECK-NEXT: block16:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _19 = checked128(const 170141183460469231731687303715884105728_u128, const 2_u128)
// CHECK-NEXT:     v51 = iconst.i64 0
// CHECK-NEXT:     v52 = iconst.i64 -9223372036854775808
// CHECK-NEXT:     v53 = iconcat v51, v52  ; v51 = 0, v52 = -9223372036854775808
// CHECK-NEXT:     v54 = iconst.i64 2
// CHECK-NEXT:     v55 = iconst.i64 0
// CHECK-NEXT:     v56 = iconcat v54, v55  ; v54 = 2, v55 = 0
// CHECK-NEXT:     v57 = stack_addr.i16 ss5
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn10(v57, v53, v56)
// CHECK-NEXT:     jump block17
// CHECK-EMPTY:
// CHECK-NEXT: block17:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _18 = &_19
// CHECK-NEXT:     v58 = stack_addr.i16 ss5
// CHECK-NEXT: ; write_cvalue: Var(_18, var14): &'{erased} (u128, bool) <- ByVal(v58): &'{erased} (u128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _17 = <(u128, bool) as core::cmp::PartialEq>::eq(move _18, const gbc_main::promoted[0])
// CHECK-NEXT:     v59 = symbol_value.i16 gv5
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v60 = call fn11(v58, v59)
// CHECK-NEXT:     v61 -> v60
// CHECK-NEXT: ; write_cvalue: Var(_17, var13): bool <- ByVal(v60): bool
// CHECK-NEXT:     jump block18
// CHECK-EMPTY:
// CHECK-NEXT: block18:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _17)
// CHECK-NEXT:     brif.i8 v61, block19, block20
// CHECK-EMPTY:
// CHECK-NEXT: block19:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _20 = signed128(const -3_i128, const 5_i128)
// CHECK-NEXT:     v62 = iconst.i64 -3
// CHECK-NEXT:     v63 = iconst.i64 -1
// CHECK-NEXT:     v64 = iconcat v62, v63  ; v62 = -3, v63 = -1
// CHECK-NEXT:     v65 = iconst.i64 5
// CHECK-NEXT:     v66 = iconst.i64 0
// CHECK-NEXT:     v67 = iconcat v65, v66  ; v65 = 5, v66 = 0
// CHECK-NEXT:     v68 = stack_addr.i16 ss6
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: i128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(i128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4294967311 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: i128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(i128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4294967311 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: i128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(i128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4294967311 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(16 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn12(v68, v64, v67)
// CHECK-NEXT: ; write_cvalue: Var(_20, var15): i128 <- ByRef(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): i128
// CHECK-NEXT:     v69 = stack_addr.i16 ss6
// CHECK-NEXT:     v70 = load.i128 notrap v69
// CHECK-NEXT:     v72 -> v70
// CHECK-NEXT:     jump block21
// CHECK-EMPTY:
// CHECK-NEXT: block20:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = const false
// CHECK-NEXT:     v71 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v71): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block22(v71)  ; v71 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = Eq(move _20, const 2_i128)
// CHECK-NEXT:     v73 = iconst.i64 2
// CHECK-NEXT:     v74 = iconst.i64 0
// CHECK-NEXT:     v75 = iconcat v73, v74  ; v73 = 2, v74 = 0
// CHECK-NEXT:     v76 = icmp.i128 eq v72, v75
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v76): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block22(v76)
// CHECK-EMPTY:
// CHECK-NEXT: block22(v77: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v77): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v77
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol signed128
// CHECK-NEXT: function u0:8(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol signed128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(16 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } i128
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    i128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    i128                             16b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i128, v2: i128):
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v4 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): i128 <- ByVal(v1): i128
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): i128 <- ByVal(v2): i128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Add(copy _1, copy _2)
// CHECK-NEXT:     v5 = iadd.i128 v3, v4
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): i128 <- ByVal(v5): i128
// CHECK-NEXT:     store notrap v5, v0
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
