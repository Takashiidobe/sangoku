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
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh10count_ones
// CHECK-NEXT: function u0:0(i8) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh10count_ones
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u8                                1b 1                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u8 <- ByVal(v0): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::ctpop::<u8>(copy _1)
// CHECK-NEXT:     v2 = popcnt.i8 v1
// CHECK-NEXT:     v3 = uextend.i32 v2
// CHECK-NEXT:     v4 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v3): u32
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh11checked_addCsdyHBBVKw7bQ_10integer_u8
// CHECK-NEXT: function u0:1(i8, i8) -> i8 uext, i8 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh11checked_addCsdyHBBVKw7bQ_10integer_u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::option::Option<u8>          2b 1                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::option::Option<u8>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u8                                1b 1                var=1
// CHECK-NEXT: ; ssa   _2    u8                                1b 1                var=2
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (u8, bool)                        2b 1                var=(5, 6)
// CHECK-NEXT: ; ssa   _6    u8                                1b 1                var=7
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2
// CHECK-NEXT:     sig0 = (i8 uext) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:12 sig0 ; Instance { def: Item(DefId(1:2345 ~ core[..]::intrinsics::unlikely)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v14 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v15 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v4 = iadd.i8 v2, v3
// CHECK-NEXT:     v5 = icmp ult v4, v2
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var5, var6): (u8, bool) <- ByValPair(v4, v5): (u8, bool)
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v5): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::intrinsics::unlikely(move _4)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v6 = call fn0(v5)
// CHECK-NEXT:     v7 -> v6
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): bool <- ByVal(v6): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _3)
// CHECK-NEXT:     brif.i8 v7, block3, block4
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<u8>::None
// CHECK-NEXT:     v8 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u8 <- ByVal(v8): u8
// CHECK-NEXT:     v9 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v8, v9  ; v8 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     v11 = load.i8 notrap v10
// CHECK-NEXT:     v12 = stack_addr.i16 ss0+1
// CHECK-NEXT:     v13 = load.i8 notrap v12
// CHECK-NEXT:     return v11, v13
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = AddUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v16 = iadd.i8 v14, v15
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): u8 <- ByVal(v16): u8
// CHECK-NEXT: ; _0 = core::option::Option::<u8>::Some(move _6)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(1) }, None): u8 <- ByVal(v16): u8
// CHECK-NEXT:     v17 = stack_addr.i16 ss0+1
// CHECK-NEXT:     store notrap v16, v17
// CHECK-NEXT:     v18 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u8 <- ByVal(v18): u8
// CHECK-NEXT:     v19 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v18, v19  ; v18 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v20 = stack_addr.i16 ss0
// CHECK-NEXT:     v21 = load.i8 notrap v20
// CHECK-NEXT:     v22 = stack_addr.i16 ss0+1
// CHECK-NEXT:     v23 = load.i8 notrap v22
// CHECK-NEXT:     return v21, v23
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v24 = stack_addr.i16 ss0
// CHECK-NEXT:     v25 = load.i8 notrap v24
// CHECK-NEXT:     v26 = stack_addr.i16 ss0+1
// CHECK-NEXT:     v27 = load.i8 notrap v26
// CHECK-NEXT:     return v25, v27
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh11checked_mulCsdyHBBVKw7bQ_10integer_u8
// CHECK-NEXT: function u0:2(i8, i8) -> i8 uext, i8 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh11checked_mulCsdyHBBVKw7bQ_10integer_u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::option::Option<u8>          2b 1                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::option::Option<u8>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u8                                1b 1                var=1
// CHECK-NEXT: ; ssa   _2    u8                                1b 1                var=2
// CHECK-NEXT: ; ssa   _3    u8                                1b 1                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (u8, bool)                        2b 1                var=(5, 6)
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=7
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2
// CHECK-NEXT:     sig0 = (i8, i8) -> i8, i8 uext system_v
// CHECK-NEXT:     sig1 = (i8 uext) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:10 sig0 ; Instance { def: Item(DefId(1:27498 ~ core[..]::num::{impl#6}::overflowing_mul)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:12 sig1 ; Instance { def: Item(DefId(1:2345 ~ core[..]::intrinsics::unlikely)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::num::<impl u8>::overflowing_mul(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u8, bool), layout: Layout { size: Size(2 bytes), align: Align(1 bytes), backend_repr: ScalarPair { a: u8 is .., b: u8 is 0..=1, b_offset: Size(1 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(1 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(1 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4, v5 = call fn0(v2, v3)
// CHECK-NEXT:     v6 -> v4
// CHECK-NEXT:     v16 -> v4
// CHECK-NEXT:     v7 -> v5
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var5, var6): (u8, bool) <- ByValPair(v4, v5): (u8, bool)
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (_5.0: u8)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): u8 <- ByVal(v6): u8
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v7): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::intrinsics::unlikely(copy _4)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v8 = call fn1(v7)
// CHECK-NEXT:     v9 -> v8
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v8): bool
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     brif.i8 v9, block4, block5
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<u8>::None
// CHECK-NEXT:     v10 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u8 <- ByVal(v10): u8
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v10, v11  ; v10 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v12 = stack_addr.i16 ss0
// CHECK-NEXT:     v13 = load.i8 notrap v12
// CHECK-NEXT:     v14 = stack_addr.i16 ss0+1
// CHECK-NEXT:     v15 = load.i8 notrap v14
// CHECK-NEXT:     return v13, v15
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<u8>::Some(copy _3)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(1) }, None): u8 <- ByVal(v16): u8
// CHECK-NEXT:     v17 = stack_addr.i16 ss0+1
// CHECK-NEXT:     store.i8 notrap v16, v17
// CHECK-NEXT:     v18 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u8 <- ByVal(v18): u8
// CHECK-NEXT:     v19 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v18, v19  ; v18 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v20 = stack_addr.i16 ss0
// CHECK-NEXT:     v21 = load.i8 notrap v20
// CHECK-NEXT:     v22 = stack_addr.i16 ss0+1
// CHECK-NEXT:     v23 = load.i8 notrap v22
// CHECK-NEXT:     return v21, v23
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v24 = stack_addr.i16 ss0
// CHECK-NEXT:     v25 = load.i8 notrap v24
// CHECK-NEXT:     v26 = stack_addr.i16 ss0+1
// CHECK-NEXT:     v27 = load.i8 notrap v26
// CHECK-NEXT:     return v25, v27
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh11checked_subCsdyHBBVKw7bQ_10integer_u8
// CHECK-NEXT: function u0:3(i8, i8) -> i8 uext, i8 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh11checked_subCsdyHBBVKw7bQ_10integer_u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::option::Option<u8>          2b 1                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::option::Option<u8>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u8                                1b 1                var=1
// CHECK-NEXT: ; ssa   _2    u8                                1b 1                var=2
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=3
// CHECK-NEXT: ; ssa   _4    u8                                1b 1                var=4
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v11 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v12 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = Lt(copy _1, copy _2)
// CHECK-NEXT:     v4 = icmp.i8 ult v2, v3
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): bool <- ByVal(v4): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _3)
// CHECK-NEXT:     brif v4, block2, block3
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<u8>::None
// CHECK-NEXT:     v5 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u8 <- ByVal(v5): u8
// CHECK-NEXT:     v6 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v5, v6  ; v5 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v7 = stack_addr.i16 ss0
// CHECK-NEXT:     v8 = load.i8 notrap v7
// CHECK-NEXT:     v9 = stack_addr.i16 ss0+1
// CHECK-NEXT:     v10 = load.i8 notrap v9
// CHECK-NEXT:     return v8, v10
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = SubUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v13 = isub.i8 v11, v12
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): u8 <- ByVal(v13): u8
// CHECK-NEXT: ; _0 = core::option::Option::<u8>::Some(move _4)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(1) }, None): u8 <- ByVal(v13): u8
// CHECK-NEXT:     v14 = stack_addr.i16 ss0+1
// CHECK-NEXT:     store notrap v13, v14
// CHECK-NEXT:     v15 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u8 <- ByVal(v15): u8
// CHECK-NEXT:     v16 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v15, v16  ; v15 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v17 = stack_addr.i16 ss0
// CHECK-NEXT:     v18 = load.i8 notrap v17
// CHECK-NEXT:     v19 = stack_addr.i16 ss0+1
// CHECK-NEXT:     v20 = load.i8 notrap v19
// CHECK-NEXT:     return v18, v20
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v21 = stack_addr.i16 ss0
// CHECK-NEXT:     v22 = load.i8 notrap v21
// CHECK-NEXT:     v23 = stack_addr.i16 ss0+1
// CHECK-NEXT:     v24 = load.i8 notrap v23
// CHECK-NEXT:     return v22, v24
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh12wrapping_add
// CHECK-NEXT: function u0:4(i8, i8) -> i8 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh12wrapping_add
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
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Add(copy _1, copy _2)
// CHECK-NEXT:     v4 = iadd.i8 v2, v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v4): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh12wrapping_mul
// CHECK-NEXT: function u0:5(i8, i8) -> i8 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh12wrapping_mul
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
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v4 = imul.i8 v2, v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v4): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh12wrapping_sub
// CHECK-NEXT: function u0:6(i8, i8) -> i8 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh12wrapping_sub
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
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v4 = isub.i8 v2, v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v4): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh13leading_zeros
// CHECK-NEXT: function u0:7(i8) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh13leading_zeros
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u8                                1b 1                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u8 <- ByVal(v0): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::ctlz::<u8>(copy _1)
// CHECK-NEXT:     v2 = clz.i8 v1
// CHECK-NEXT:     v3 = uextend.i32 v2
// CHECK-NEXT:     v4 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v3): u32
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh14trailing_zeros
// CHECK-NEXT: function u0:8(i8) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh14trailing_zeros
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u8                                1b 1                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u8 <- ByVal(v0): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::cttz::<u8>(copy _1)
// CHECK-NEXT:     v2 = ctz.i8 v1
// CHECK-NEXT:     v3 = uextend.i32 v2
// CHECK-NEXT:     v4 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v3): u32
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh15overflowing_add
// CHECK-NEXT: function u0:9(i8, i8) -> i8, i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh15overflowing_add
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    (u8, bool)                        2b 1                var=(1, 2)
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) (u8, bool)
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u8                                1b 1                var=3
// CHECK-NEXT: ; ssa   _2    u8                                1b 1                var=4
// CHECK-NEXT: ; ssa   _3    u8                                1b 1                var=5
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=6
// CHECK-NEXT: ; ssa   _5    (u8, bool)                        2b 1                var=(7, 8)
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var3): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var4): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v4 = iadd.i8 v2, v3
// CHECK-NEXT:     v5 = icmp ult v4, v2
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var7, var8): (u8, bool) <- ByValPair(v4, v5): (u8, bool)
// CHECK-NEXT: ; _3 = copy (_5.0: u8)
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): u8 <- ByVal(v4): u8
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var6): bool <- ByVal(v5): bool
// CHECK-NEXT: ; _0 = (copy _3, copy _4)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v4): u8
// CHECK-NEXT: ; write_cvalue: Var(_0, var2): bool <- ByVal(v5): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4, v5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh15overflowing_mul
// CHECK-NEXT: function u0:10(i8, i8) -> i8, i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh15overflowing_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    (u8, bool)                        2b 1                var=(1, 2)
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) (u8, bool)
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u8                                1b 1                var=3
// CHECK-NEXT: ; ssa   _2    u8                                1b 1                var=4
// CHECK-NEXT: ; ssa   _3    u8                                1b 1                var=5
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=6
// CHECK-NEXT: ; ssa   _5    (u8, bool)                        2b 1                var=(7, 8)
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var3): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var4): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v4 = uextend.i16 v2
// CHECK-NEXT:     v5 = uextend.i16 v3
// CHECK-NEXT:     v6 = imul v4, v5
// CHECK-NEXT:     v7 = iconst.i16 255
// CHECK-NEXT:     v8 = icmp ugt v6, v7  ; v7 = 255
// CHECK-NEXT:     v9 = ireduce.i8 v6
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var7, var8): (u8, bool) <- ByValPair(v9, v8): (u8, bool)
// CHECK-NEXT: ; _3 = copy (_5.0: u8)
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): u8 <- ByVal(v9): u8
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var6): bool <- ByVal(v8): bool
// CHECK-NEXT: ; _0 = (copy _3, copy _4)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v9): u8
// CHECK-NEXT: ; write_cvalue: Var(_0, var2): bool <- ByVal(v8): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v9, v8
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh15overflowing_sub
// CHECK-NEXT: function u0:11(i8, i8) -> i8, i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh15overflowing_sub
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    (u8, bool)                        2b 1                var=(1, 2)
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) (u8, bool)
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u8                                1b 1                var=3
// CHECK-NEXT: ; ssa   _2    u8                                1b 1                var=4
// CHECK-NEXT: ; ssa   _3    u8                                1b 1                var=5
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=6
// CHECK-NEXT: ; ssa   _5    (u8, bool)                        2b 1                var=(7, 8)
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var3): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var4): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = SubWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v4 = isub.i8 v2, v3
// CHECK-NEXT:     v5 = icmp ugt v4, v2
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var7, var8): (u8, bool) <- ByValPair(v4, v5): (u8, bool)
// CHECK-NEXT: ; _3 = copy (_5.0: u8)
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): u8 <- ByVal(v4): u8
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var6): bool <- ByVal(v5): bool
// CHECK-NEXT: ; _0 = (copy _3, copy _4)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v4): u8
// CHECK-NEXT: ; write_cvalue: Var(_0, var2): bool <- ByVal(v5): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4, v5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics8unlikely
// CHECK-NEXT: function u0:12(i8 uext) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics8unlikely
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    bool                              1b 1                var=2
// CHECK-NEXT: ; zst   _2    ()                                0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v0): bool
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _1)
// CHECK-NEXT:     brif.i8 v1, block2, block4
// CHECK-EMPTY:
// CHECK-NEXT: block2 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const true
// CHECK-NEXT:     v2 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v2): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v2  ; v2 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const false
// CHECK-NEXT:     v3 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v3): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v3  ; v3 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     v5 = iconst.i8 0
// CHECK-NEXT:     v4 -> v5
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4  ; v4 = 0
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCsdyHBBVKw7bQ_10integer_u8
// CHECK-NEXT: function u0:13() system_v {
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
// CHECK-NEXT: function u0:14(i16, i16) -> i8 uext system_v {
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
// CHECK-NEXT:     fn0 = colocated u0:17 sig0 ; Instance { def: Item(DefId(1:29972 ~ core[..]::cmp::impls::{impl#22}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:16 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
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
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptionhENtNtB7_3cmp9PartialEq2eqCsdyHBBVKw7bQ_10integer_u8
// CHECK-NEXT: function u0:15(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptionhENtNtB7_3cmp9PartialEq2eqCsdyHBBVKw7bQ_10integer_u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} core::option::Option<u8>
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} core::option::Option<u8>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} core::option::Option<u8>    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} core::option::Option<u8>    2b 2                var=3
// CHECK-NEXT: ; ssa   _3    (&'{erased} core::option::Option<u8>, &'{erased} core::option::Option<u8>)    4b 2                var=(4, 5)
// CHECK-NEXT: ; ssa   _4    isize                             2b 2                var=6
// CHECK-NEXT: ; ssa   _5    isize                             2b 2                var=7
// CHECK-NEXT: ; ssa   _6    isize                             2b 2                var=8
// CHECK-NEXT: ; ssa   _7    &'{erased} u8                     2b 2                var=9
// CHECK-NEXT: ; ssa   _8    &'{erased} u8                     2b 2                var=10
// CHECK-NEXT: ; ssa   _9    &'{erased} core::option::Option<u8>    2b 2                var=11
// CHECK-NEXT: ; ssa   _10   &'{erased} core::option::Option<u8>    2b 2                var=12
// CHECK-NEXT: ; ssa   _11   &'{erased} core::option::Option<u8>    2b 2                var=13
// CHECK-NEXT: ; stack _12   &'{erased} core::option::Option<u8>    2b 2                storage=ss0
// CHECK-NEXT: ; stack _13   &'{erased} core::option::Option<u8>    2b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:17 sig0 ; Instance { def: Item(DefId(1:29972 ~ core[..]::cmp::impls::{impl#22}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v18 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v7 -> v1
// CHECK-NEXT:     v11 -> v1
// CHECK-NEXT:     v24 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} core::option::Option<u8> <- ByVal(v0): &'{erased} core::option::Option<u8>
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} core::option::Option<u8> <- ByVal(v1): &'{erased} core::option::Option<u8>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): &'{erased} core::option::Option<u8> <- ByVal(v2): &'{erased} core::option::Option<u8>
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): &'{erased} core::option::Option<u8> <- ByVal(v3): &'{erased} core::option::Option<u8>
// CHECK-NEXT: ; _9 = no_retag copy (_3.0: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Var(_9, var11): &'{erased} core::option::Option<u8> <- ByVal(v2): &'{erased} core::option::Option<u8>
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
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): &'{erased} core::option::Option<u8> <- ByVal(v7): &'{erased} core::option::Option<u8>
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
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): &'{erased} core::option::Option<u8> <- ByVal(v11): &'{erased} core::option::Option<u8>
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
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} core::option::Option<u8> <- ByVal(v18): &'{erased} core::option::Option<u8>
// CHECK-NEXT:     v19 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v18, v19
// CHECK-NEXT: ; _7 = &(((*_12) as Some).0: T)
// CHECK-NEXT:     v20 = stack_addr.i16 ss0
// CHECK-NEXT:     v21 = load.i16 notrap v20
// CHECK-NEXT:     v22 = iconst.i16 1
// CHECK-NEXT:     v23 = iadd v21, v22  ; v22 = 1
// CHECK-NEXT: ; write_cvalue: Var(_7, var9): &'{erased} u8 <- ByVal(v23): &'{erased} u8
// CHECK-NEXT: ; _13 = no_retag copy (_3.1: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} core::option::Option<u8> <- ByVal(v24): &'{erased} core::option::Option<u8>
// CHECK-NEXT:     v25 = stack_addr.i16 ss1
// CHECK-NEXT:     store.i16 notrap v24, v25
// CHECK-NEXT: ; _8 = &(((*_13) as Some).0: T)
// CHECK-NEXT:     v26 = stack_addr.i16 ss1
// CHECK-NEXT:     v27 = load.i16 notrap v26
// CHECK-NEXT:     v28 = iconst.i16 1
// CHECK-NEXT:     v29 = iadd v27, v28  ; v28 = 1
// CHECK-NEXT: ; write_cvalue: Var(_8, var10): &'{erased} u8 <- ByVal(v29): &'{erased} u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::cmp::PartialEq>::eq(copy _7, copy _8)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCsdyHBBVKw7bQ_10integer_u8
// CHECK-NEXT: function u0:16(i16, i16) -> i8 uext system_v {
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
// CHECK-NEXT: function u0:17(i16, i16) -> i8 uext system_v {
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
// CHECK-NEXT: function u0:18() -> i8 system_v {
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
// CHECK-NEXT:     fn0 = colocated u0:19 sig0 ; Instance { def: Item(DefId(0:3 ~ integer_u8[..]::integer_methods)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = integer_methods(const core::num::<impl u8>::MAX, const 2_u8)
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
// CHECK-NEXT: function u0:19(i8, i8) -> i8 system_v {
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
// CHECK-NEXT: ; ssa   _5    u8                                1b 1                var=4
// CHECK-NEXT: ; stack _6    (u8, bool)                        2b 1                storage=ss2
// CHECK-NEXT: ; ssa   _7    u8                                1b 1                var=5
// CHECK-NEXT: ; ssa   _8    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=7
// CHECK-NEXT: ; ssa   _10   u8                                1b 1                var=8
// CHECK-NEXT: ; ssa   _11   u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=10
// CHECK-NEXT: ; ssa   _13   u8                                1b 1                var=11
// CHECK-NEXT: ; ssa   _14   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _15   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _16   &'{erased} core::option::Option<u8>    2b 2                var=14
// CHECK-NEXT: ; stack _17   core::option::Option<u8>          2b 1                storage=ss3
// CHECK-NEXT: ; ssa   _18   u8                                1b 1                var=15
// CHECK-NEXT: ; ssa   _19   u8                                1b 1                var=16
// CHECK-NEXT: ; ssa   _20   bool                              1b 1                var=17
// CHECK-NEXT: ; ssa   _21   &'{erased} core::option::Option<u8>    2b 2                var=18
// CHECK-NEXT: ; stack _22   core::option::Option<u8>          2b 1                storage=ss4
// CHECK-NEXT: ; ssa   _23   &'{erased} core::option::Option<u8>    2b 2                var=19
// CHECK-NEXT: ; ssa   _24   u8                                1b 1                var=20
// CHECK-NEXT: ; ssa   _25   u8                                1b 1                var=21
// CHECK-NEXT: ; ssa   _26   bool                              1b 1                var=22
// CHECK-NEXT: ; ssa   _27   &'{erased} core::option::Option<u8>    2b 2                var=23
// CHECK-NEXT: ; stack _28   core::option::Option<u8>          2b 1                storage=ss5
// CHECK-NEXT: ; ssa   _29   u8                                1b 1                var=24
// CHECK-NEXT: ; ssa   _30   u8                                1b 1                var=25
// CHECK-NEXT: ; ssa   _31   bool                              1b 1                var=26
// CHECK-NEXT: ; ssa   _32   u8                                1b 1                var=27
// CHECK-NEXT: ; ssa   _33   u8                                1b 1                var=28
// CHECK-NEXT: ; ssa   _34   u8                                1b 1                var=29
// CHECK-NEXT: ; ssa   _35   u8                                1b 1                var=30
// CHECK-NEXT: ; ssa   _36   bool                              1b 1                var=31
// CHECK-NEXT: ; ssa   _37   u8                                1b 1                var=32
// CHECK-NEXT: ; ssa   _38   u8                                1b 1                var=33
// CHECK-NEXT: ; ssa   _39   u8                                1b 1                var=34
// CHECK-NEXT: ; ssa   _40   u8                                1b 1                var=35
// CHECK-NEXT: ; ssa   _41   bool                              1b 1                var=36
// CHECK-NEXT: ; ssa   _42   u8                                1b 1                var=37
// CHECK-NEXT: ; ssa   _43   u8                                1b 1                var=38
// CHECK-NEXT: ; ssa   _44   u8                                1b 1                var=39
// CHECK-NEXT: ; ssa   _45   u8                                1b 1                var=40
// CHECK-NEXT: ; ssa   _46   bool                              1b 1                var=41
// CHECK-NEXT: ; ssa   _47   bool                              1b 1                var=42
// CHECK-NEXT: ; ssa   _48   &'{erased} (u8, bool)             2b 2                var=43
// CHECK-NEXT: ; stack _49   (u8, bool)                        2b 1                storage=ss6
// CHECK-NEXT: ; ssa   _50   &'{erased} (u8, bool)             2b 2                var=44
// CHECK-NEXT: ; ssa   _51   bool                              1b 1                var=45
// CHECK-NEXT: ; ssa   _52   &'{erased} (u8, bool)             2b 2                var=46
// CHECK-NEXT: ; stack _53   (u8, bool)                        2b 1                storage=ss7
// CHECK-NEXT: ; ssa   _54   &'{erased} (u8, bool)             2b 2                var=47
// CHECK-NEXT: ; ssa   _55   &'{erased} (u8, bool)             2b 2                var=48
// CHECK-NEXT: ; stack _56   (u8, bool)                        2b 1                storage=ss8
// CHECK-NEXT: ; ssa   _57   &'{erased} (u8, bool)             2b 2                var=49
// CHECK-NEXT: ; ssa   _58   u8                                1b 1                var=50
// CHECK-NEXT: ; ssa   _59   u8                                1b 1                var=51
// CHECK-NEXT: ; ssa   _60   bool                              1b 1                var=52
// CHECK-NEXT: ; ssa   _61   bool                              1b 1                var=53
// CHECK-NEXT: ; ssa   _62   u32                               4b 2                var=54
// CHECK-NEXT: ; ssa   _63   bool                              1b 1                var=55
// CHECK-NEXT: ; ssa   _64   u32                               4b 2                var=56
// CHECK-NEXT: ; ssa   _65   u32                               4b 2                var=57
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
// CHECK-NEXT:     gv0 = symbol colocated userextname3 ; alloc3
// CHECK-NEXT:     sig0 = (i8, i8) -> i8 uext, i8 system_v
// CHECK-NEXT:     sig1 = (i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i8, i8) -> i8 uext, i8 system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig4 = (i8, i8) -> i8 uext, i8 system_v
// CHECK-NEXT:     sig5 = (i16) -> i8 uext system_v
// CHECK-NEXT:     sig6 = (i8, i8) -> i8 system_v
// CHECK-NEXT:     sig7 = (i8, i8) -> i8 system_v
// CHECK-NEXT:     sig8 = (i8, i8) -> i8 system_v
// CHECK-NEXT:     sig9 = (i8, i8) -> i8, i8 uext system_v
// CHECK-NEXT:     sig10 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig11 = (i8, i8) -> i8, i8 uext system_v
// CHECK-NEXT:     sig12 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig13 = (i8, i8) -> i8, i8 uext system_v
// CHECK-NEXT:     sig14 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig15 = (i8) -> i32 system_v
// CHECK-NEXT:     sig16 = (i8) -> i32 system_v
// CHECK-NEXT:     sig17 = (i8) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(1:27423 ~ core[..]::num::{impl#6}::checked_add)), args: [] }
// CHECK-NEXT:     fn1 = u0:20 sig1 ; Instance { def: Item(DefId(1:11810 ~ core[..]::option::{impl#0}::is_none)), args: [u8] }
// CHECK-NEXT:     fn2 = colocated u0:3 sig2 ; Instance { def: Item(DefId(1:27428 ~ core[..]::num::{impl#6}::checked_sub)), args: [] }
// CHECK-NEXT:     fn3 = colocated u0:15 sig3 ; Instance { def: Item(DefId(1:11950 ~ core[..]::option::{impl#17}::eq)), args: [u8] }
// CHECK-NEXT:     fn4 = colocated u0:2 sig4 ; Instance { def: Item(DefId(1:27434 ~ core[..]::num::{impl#6}::checked_mul)), args: [] }
// CHECK-NEXT:     fn5 = u0:20 sig5 ; Instance { def: Item(DefId(1:11810 ~ core[..]::option::{impl#0}::is_none)), args: [u8] }
// CHECK-NEXT:     fn6 = colocated u0:4 sig6 ; Instance { def: Item(DefId(1:27478 ~ core[..]::num::{impl#6}::wrapping_add)), args: [] }
// CHECK-NEXT:     fn7 = colocated u0:6 sig7 ; Instance { def: Item(DefId(1:27480 ~ core[..]::num::{impl#6}::wrapping_sub)), args: [] }
// CHECK-NEXT:     fn8 = colocated u0:5 sig8 ; Instance { def: Item(DefId(1:27482 ~ core[..]::num::{impl#6}::wrapping_mul)), args: [] }
// CHECK-NEXT:     fn9 = colocated u0:9 sig9 ; Instance { def: Item(DefId(1:27491 ~ core[..]::num::{impl#6}::overflowing_add)), args: [] }
// CHECK-NEXT:     fn10 = colocated u0:14 sig10 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u8, bool] }
// CHECK-NEXT:     fn11 = colocated u0:11 sig11 ; Instance { def: Item(DefId(1:27494 ~ core[..]::num::{impl#6}::overflowing_sub)), args: [] }
// CHECK-NEXT:     fn12 = colocated u0:14 sig12 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u8, bool] }
// CHECK-NEXT:     fn13 = colocated u0:10 sig13 ; Instance { def: Item(DefId(1:27498 ~ core[..]::num::{impl#6}::overflowing_mul)), args: [] }
// CHECK-NEXT:     fn14 = colocated u0:14 sig14 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u8, bool] }
// CHECK-NEXT:     fn15 = colocated u0:7 sig15 ; Instance { def: Item(DefId(1:27395 ~ core[..]::num::{impl#6}::leading_zeros)), args: [] }
// CHECK-NEXT:     fn16 = colocated u0:8 sig16 ; Instance { def: Item(DefId(1:27396 ~ core[..]::num::{impl#6}::trailing_zeros)), args: [] }
// CHECK-NEXT:     fn17 = colocated u0:0 sig17 ; Instance { def: Item(DefId(1:27393 ~ core[..]::num::{impl#6}::count_ones)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i8):
// CHECK-NEXT:     v18 -> v0
// CHECK-NEXT:     v27 -> v0
// CHECK-NEXT:     v41 -> v0
// CHECK-NEXT:     v54 -> v0
// CHECK-NEXT:     v65 -> v0
// CHECK-NEXT:     v76 -> v0
// CHECK-NEXT:     v87 -> v0
// CHECK-NEXT:     v97 -> v0
// CHECK-NEXT:     v107 -> v0
// CHECK-NEXT:     v122 -> v0
// CHECK-NEXT:     v127 -> v0
// CHECK-NEXT:     v132 -> v0
// CHECK-NEXT:     v144 -> v0
// CHECK-NEXT:     v19 -> v1
// CHECK-NEXT:     v28 -> v1
// CHECK-NEXT:     v42 -> v1
// CHECK-NEXT:     v55 -> v1
// CHECK-NEXT:     v66 -> v1
// CHECK-NEXT:     v77 -> v1
// CHECK-NEXT:     v88 -> v1
// CHECK-NEXT:     v98 -> v1
// CHECK-NEXT:     v108 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u8 <- ByVal(v0): u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u8 <- ByVal(v1): u8
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (const 1_u8, const true)
// CHECK-NEXT:     v2 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u8 <- ByVal(v2): u8
// CHECK-NEXT:     v3 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v2, v3  ; v2 = 1
// CHECK-NEXT:     v4 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(1) }, None): bool <- ByVal(v4): bool
// CHECK-NEXT:     v5 = stack_addr.i16 ss0+1
// CHECK-NEXT:     store notrap v4, v5  ; v4 = 1
// CHECK-NEXT: ; _5 = Sub(const core::num::<impl u8>::MAX, const 2_u8)
// CHECK-NEXT:     v6 = iconst.i8 -1
// CHECK-NEXT:     v7 = iconst.i8 2
// CHECK-NEXT:     v8 = isub v6, v7  ; v6 = -1, v7 = 2
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): u8 <- ByVal(v8): u8
// CHECK-NEXT: ; _4 = (move _5, const false)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u8 <- ByVal(v8): u8
// CHECK-NEXT:     v9 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v8, v9
// CHECK-NEXT:     v10 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(1) }, None): bool <- ByVal(v10): bool
// CHECK-NEXT:     v11 = stack_addr.i16 ss1+1
// CHECK-NEXT:     store notrap v10, v11  ; v10 = 0
// CHECK-NEXT: ; _7 = Sub(const core::num::<impl u8>::MAX, const 1_u8)
// CHECK-NEXT:     v12 = iconst.i8 -1
// CHECK-NEXT:     v13 = iconst.i8 1
// CHECK-NEXT:     v14 = isub v12, v13  ; v12 = -1, v13 = 1
// CHECK-NEXT: ; write_cvalue: Var(_7, var5): u8 <- ByVal(v14): u8
// CHECK-NEXT: ; _6 = (move _7, const true)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u8 <- ByVal(v14): u8
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v14, v15
// CHECK-NEXT:     v16 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(1) }, None): bool <- ByVal(v16): bool
// CHECK-NEXT:     v17 = stack_addr.i16 ss2+1
// CHECK-NEXT:     store notrap v16, v17  ; v16 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; _17 = core::num::<impl u8>::checked_add(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<u8>, layout: Layout { size: Size(2 bytes), align: Align(1 bytes), backend_repr: ScalarPair { a: u8 is 0..=1, b: union u8, b_offset: Size(1 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u8 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(1 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(2 bytes), backend_repr: ScalarPair { a: u8 is 0..=1, b: union u8, b_offset: Size(1 bytes) }, field_offsets: [Size(1 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 8261374528656934897 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v20, v21 = call fn0(v18, v19)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): core::option::Option<u8> <- ByValPair(v20, v21): core::option::Option<u8>
// CHECK-NEXT:     v22 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v20, v22
// CHECK-NEXT:     v23 = stack_addr.i16 ss3+1
// CHECK-NEXT:     store notrap v21, v23
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = &_17
// CHECK-NEXT:     v24 = stack_addr.i16 ss3
// CHECK-NEXT: ; write_cvalue: Var(_16, var14): &'{erased} core::option::Option<u8> <- ByVal(v24): &'{erased} core::option::Option<u8>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _15 = core::option::Option::<u8>::is_none(move _16)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u8>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v25 = call fn1(v24)
// CHECK-NEXT:     v26 -> v25
// CHECK-NEXT:     v39 -> v25
// CHECK-NEXT: ; write_cvalue: Var(_15, var13): bool <- ByVal(v25): bool
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _14 = move _15 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_14, var12): u8 <- ByVal(v26): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _22 = core::num::<impl u8>::checked_sub(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<u8>, layout: Layout { size: Size(2 bytes), align: Align(1 bytes), backend_repr: ScalarPair { a: u8 is 0..=1, b: union u8, b_offset: Size(1 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u8 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(1 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(2 bytes), backend_repr: ScalarPair { a: u8 is 0..=1, b: union u8, b_offset: Size(1 bytes) }, field_offsets: [Size(1 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 8261374528656934897 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v29, v30 = call fn2(v27, v28)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): core::option::Option<u8> <- ByValPair(v29, v30): core::option::Option<u8>
// CHECK-NEXT:     v31 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v29, v31
// CHECK-NEXT:     v32 = stack_addr.i16 ss4+1
// CHECK-NEXT:     store notrap v30, v32
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _21 = &_22
// CHECK-NEXT:     v33 = stack_addr.i16 ss4
// CHECK-NEXT: ; write_cvalue: Var(_21, var18): &'{erased} core::option::Option<u8> <- ByVal(v33): &'{erased} core::option::Option<u8>
// CHECK-NEXT: ; _23 = const integer_methods::promoted[0]
// CHECK-NEXT:     v34 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_23, var19): &'{erased} core::option::Option<u8> <- ByVal(v34): &'{erased} core::option::Option<u8>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _20 = <core::option::Option<u8> as core::cmp::PartialEq>::eq(move _21, move _23)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u8>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u8>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v35 = call fn3(v33, v34)
// CHECK-NEXT:     v36 -> v35
// CHECK-NEXT: ; write_cvalue: Var(_20, var17): bool <- ByVal(v35): bool
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _19 = move _20 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_19, var16): u8 <- ByVal(v36): u8
// CHECK-NEXT: ; _18 = Shl(move _19, const 1_i32)
// CHECK-NEXT:     v37 = iconst.i32 1
// CHECK-NEXT:     v38 = ishl.i8 v36, v37  ; v37 = 1
// CHECK-NEXT: ; write_cvalue: Var(_18, var15): u8 <- ByVal(v38): u8
// CHECK-NEXT: ; _13 = BitOr(move _14, move _18)
// CHECK-NEXT:     v40 = bor.i8 v39, v38
// CHECK-NEXT:     v52 -> v40
// CHECK-NEXT: ; write_cvalue: Var(_13, var11): u8 <- ByVal(v40): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _28 = core::num::<impl u8>::checked_mul(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<u8>, layout: Layout { size: Size(2 bytes), align: Align(1 bytes), backend_repr: ScalarPair { a: u8 is 0..=1, b: union u8, b_offset: Size(1 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u8 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(1 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(2 bytes), backend_repr: ScalarPair { a: u8 is 0..=1, b: union u8, b_offset: Size(1 bytes) }, field_offsets: [Size(1 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 8261374528656934897 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v43, v44 = call fn4(v41, v42)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): core::option::Option<u8> <- ByValPair(v43, v44): core::option::Option<u8>
// CHECK-NEXT:     v45 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v43, v45
// CHECK-NEXT:     v46 = stack_addr.i16 ss5+1
// CHECK-NEXT:     store notrap v44, v46
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _27 = &_28
// CHECK-NEXT:     v47 = stack_addr.i16 ss5
// CHECK-NEXT: ; write_cvalue: Var(_27, var23): &'{erased} core::option::Option<u8> <- ByVal(v47): &'{erased} core::option::Option<u8>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _26 = core::option::Option::<u8>::is_none(move _27)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u8>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v48 = call fn5(v47)
// CHECK-NEXT:     v49 -> v48
// CHECK-NEXT: ; write_cvalue: Var(_26, var22): bool <- ByVal(v48): bool
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _25 = move _26 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_25, var21): u8 <- ByVal(v49): u8
// CHECK-NEXT: ; _24 = Shl(move _25, const 2_i32)
// CHECK-NEXT:     v50 = iconst.i32 2
// CHECK-NEXT:     v51 = ishl.i8 v49, v50  ; v50 = 2
// CHECK-NEXT: ; write_cvalue: Var(_24, var20): u8 <- ByVal(v51): u8
// CHECK-NEXT: ; _12 = BitOr(move _13, move _24)
// CHECK-NEXT:     v53 = bor.i8 v52, v51
// CHECK-NEXT:     v63 -> v53
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): u8 <- ByVal(v53): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _32 = core::num::<impl u8>::wrapping_add(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v56 = call fn6(v54, v55)
// CHECK-NEXT:     v59 -> v56
// CHECK-NEXT: ; write_cvalue: Var(_32, var27): u8 <- ByVal(v56): u8
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _33 = copy (_3.0: u8)
// CHECK-NEXT: ; write_cvalue: Var(_33, var28): u8 <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v57 = stack_addr.i16 ss0
// CHECK-NEXT:     v58 = load.i8 notrap v57
// CHECK-NEXT: ; _31 = Eq(move _32, move _33)
// CHECK-NEXT:     v60 = icmp.i8 eq v59, v58
// CHECK-NEXT: ; write_cvalue: Var(_31, var26): bool <- ByVal(v60): bool
// CHECK-NEXT: ; _30 = move _31 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_30, var25): u8 <- ByVal(v60): u8
// CHECK-NEXT: ; _29 = Shl(move _30, const 3_i32)
// CHECK-NEXT:     v61 = iconst.i32 3
// CHECK-NEXT:     v62 = ishl v60, v61  ; v61 = 3
// CHECK-NEXT: ; write_cvalue: Var(_29, var24): u8 <- ByVal(v62): u8
// CHECK-NEXT: ; _11 = BitOr(move _12, move _29)
// CHECK-NEXT:     v64 = bor.i8 v63, v62
// CHECK-NEXT:     v74 -> v64
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): u8 <- ByVal(v64): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _37 = core::num::<impl u8>::wrapping_sub(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v67 = call fn7(v65, v66)
// CHECK-NEXT:     v70 -> v67
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): u8 <- ByVal(v67): u8
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _38 = copy (_4.0: u8)
// CHECK-NEXT: ; write_cvalue: Var(_38, var33): u8 <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v68 = stack_addr.i16 ss1
// CHECK-NEXT:     v69 = load.i8 notrap v68
// CHECK-NEXT: ; _36 = Eq(move _37, move _38)
// CHECK-NEXT:     v71 = icmp.i8 eq v70, v69
// CHECK-NEXT: ; write_cvalue: Var(_36, var31): bool <- ByVal(v71): bool
// CHECK-NEXT: ; _35 = move _36 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_35, var30): u8 <- ByVal(v71): u8
// CHECK-NEXT: ; _34 = Shl(move _35, const 4_i32)
// CHECK-NEXT:     v72 = iconst.i32 4
// CHECK-NEXT:     v73 = ishl v71, v72  ; v72 = 4
// CHECK-NEXT: ; write_cvalue: Var(_34, var29): u8 <- ByVal(v73): u8
// CHECK-NEXT: ; _10 = BitOr(move _11, move _34)
// CHECK-NEXT:     v75 = bor.i8 v74, v73
// CHECK-NEXT:     v85 -> v75
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): u8 <- ByVal(v75): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _42 = core::num::<impl u8>::wrapping_mul(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v78 = call fn8(v76, v77)
// CHECK-NEXT:     v81 -> v78
// CHECK-NEXT: ; write_cvalue: Var(_42, var37): u8 <- ByVal(v78): u8
// CHECK-NEXT:     jump block10
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _43 = copy (_6.0: u8)
// CHECK-NEXT: ; write_cvalue: Var(_43, var38): u8 <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v79 = stack_addr.i16 ss2
// CHECK-NEXT:     v80 = load.i8 notrap v79
// CHECK-NEXT: ; _41 = Eq(move _42, move _43)
// CHECK-NEXT:     v82 = icmp.i8 eq v81, v80
// CHECK-NEXT: ; write_cvalue: Var(_41, var36): bool <- ByVal(v82): bool
// CHECK-NEXT: ; _40 = move _41 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_40, var35): u8 <- ByVal(v82): u8
// CHECK-NEXT: ; _39 = Shl(move _40, const 5_i32)
// CHECK-NEXT:     v83 = iconst.i32 5
// CHECK-NEXT:     v84 = ishl v82, v83  ; v83 = 5
// CHECK-NEXT: ; write_cvalue: Var(_39, var34): u8 <- ByVal(v84): u8
// CHECK-NEXT: ; _9 = BitOr(move _10, move _39)
// CHECK-NEXT:     v86 = bor.i8 v85, v84
// CHECK-NEXT:     v120 -> v86
// CHECK-NEXT:     v143 -> v86
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): u8 <- ByVal(v86): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _49 = core::num::<impl u8>::overflowing_add(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u8, bool), layout: Layout { size: Size(2 bytes), align: Align(1 bytes), backend_repr: ScalarPair { a: u8 is .., b: u8 is 0..=1, b_offset: Size(1 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(1 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(1 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v89, v90 = call fn9(v87, v88)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): (u8, bool) <- ByValPair(v89, v90): (u8, bool)
// CHECK-NEXT:     v91 = stack_addr.i16 ss6
// CHECK-NEXT:     store notrap v89, v91
// CHECK-NEXT:     v92 = stack_addr.i16 ss6+1
// CHECK-NEXT:     store notrap v90, v92
// CHECK-NEXT:     jump block11
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _48 = &_49
// CHECK-NEXT:     v93 = stack_addr.i16 ss6
// CHECK-NEXT: ; write_cvalue: Var(_48, var43): &'{erased} (u8, bool) <- ByVal(v93): &'{erased} (u8, bool)
// CHECK-NEXT: ; _50 = &_3
// CHECK-NEXT:     v94 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_50, var44): &'{erased} (u8, bool) <- ByVal(v94): &'{erased} (u8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _47 = <(u8, bool) as core::cmp::PartialEq>::eq(move _48, move _50)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v95 = call fn10(v93, v94)
// CHECK-NEXT:     v96 -> v95
// CHECK-NEXT: ; write_cvalue: Var(_47, var42): bool <- ByVal(v95): bool
// CHECK-NEXT:     jump block12
// CHECK-EMPTY:
// CHECK-NEXT: block12:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _47)
// CHECK-NEXT:     brif.i8 v96, block13, block17
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _53 = core::num::<impl u8>::overflowing_sub(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u8, bool), layout: Layout { size: Size(2 bytes), align: Align(1 bytes), backend_repr: ScalarPair { a: u8 is .., b: u8 is 0..=1, b_offset: Size(1 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(1 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(1 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v99, v100 = call fn11(v97, v98)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): (u8, bool) <- ByValPair(v99, v100): (u8, bool)
// CHECK-NEXT:     v101 = stack_addr.i16 ss7
// CHECK-NEXT:     store notrap v99, v101
// CHECK-NEXT:     v102 = stack_addr.i16 ss7+1
// CHECK-NEXT:     store notrap v100, v102
// CHECK-NEXT:     jump block14
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _52 = &_53
// CHECK-NEXT:     v103 = stack_addr.i16 ss7
// CHECK-NEXT: ; write_cvalue: Var(_52, var46): &'{erased} (u8, bool) <- ByVal(v103): &'{erased} (u8, bool)
// CHECK-NEXT: ; _54 = &_4
// CHECK-NEXT:     v104 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_54, var47): &'{erased} (u8, bool) <- ByVal(v104): &'{erased} (u8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _51 = <(u8, bool) as core::cmp::PartialEq>::eq(move _52, move _54)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v105 = call fn12(v103, v104)
// CHECK-NEXT:     v106 -> v105
// CHECK-NEXT: ; write_cvalue: Var(_51, var45): bool <- ByVal(v105): bool
// CHECK-NEXT:     jump block15
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _51)
// CHECK-NEXT:     brif.i8 v106, block16, block17
// CHECK-EMPTY:
// CHECK-NEXT: block16:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _56 = core::num::<impl u8>::overflowing_mul(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u8, bool), layout: Layout { size: Size(2 bytes), align: Align(1 bytes), backend_repr: ScalarPair { a: u8 is .., b: u8 is 0..=1, b_offset: Size(1 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(1 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(1 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v109, v110 = call fn13(v107, v108)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): (u8, bool) <- ByValPair(v109, v110): (u8, bool)
// CHECK-NEXT:     v111 = stack_addr.i16 ss8
// CHECK-NEXT:     store notrap v109, v111
// CHECK-NEXT:     v112 = stack_addr.i16 ss8+1
// CHECK-NEXT:     store notrap v110, v112
// CHECK-NEXT:     jump block18
// CHECK-EMPTY:
// CHECK-NEXT: block17:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _46 = const false
// CHECK-NEXT:     v113 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v113): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block19(v113)  ; v113 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block18:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _55 = &_56
// CHECK-NEXT:     v114 = stack_addr.i16 ss8
// CHECK-NEXT: ; write_cvalue: Var(_55, var48): &'{erased} (u8, bool) <- ByVal(v114): &'{erased} (u8, bool)
// CHECK-NEXT: ; _57 = &_6
// CHECK-NEXT:     v115 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_57, var49): &'{erased} (u8, bool) <- ByVal(v115): &'{erased} (u8, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _46 = <(u8, bool) as core::cmp::PartialEq>::eq(move _55, move _57)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &(u8, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v116 = call fn14(v114, v115)
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v116): bool
// CHECK-NEXT:     jump block19(v116)
// CHECK-EMPTY:
// CHECK-NEXT: block19(v117: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _45 = move _46 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_45, var40): u8 <- ByVal(v117): u8
// CHECK-NEXT: ; _44 = Shl(move _45, const 6_i32)
// CHECK-NEXT:     v118 = iconst.i32 6
// CHECK-NEXT:     v119 = ishl v117, v118  ; v118 = 6
// CHECK-NEXT: ; write_cvalue: Var(_44, var39): u8 <- ByVal(v119): u8
// CHECK-NEXT: ; _8 = BitOr(move _9, move _44)
// CHECK-NEXT:     v121 = bor.i8 v120, v119
// CHECK-NEXT:     v141 -> v121
// CHECK-NEXT:     v145 -> v121
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): u8 <- ByVal(v121): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _62 = core::num::<impl u8>::leading_zeros(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v123 = call fn15(v122)
// CHECK-NEXT:     v124 -> v123
// CHECK-NEXT: ; write_cvalue: Var(_62, var54): u32 <- ByVal(v123): u32
// CHECK-NEXT:     jump block20
// CHECK-EMPTY:
// CHECK-NEXT: block20:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _61 = Eq(move _62, const 0_u32)
// CHECK-NEXT:     v125 = iconst.i32 0
// CHECK-NEXT:     v126 = icmp.i32 eq v124, v125  ; v125 = 0
// CHECK-NEXT: ; write_cvalue: Var(_61, var53): bool <- ByVal(v126): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _61)
// CHECK-NEXT:     brif.i32 v124, block24, block21
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _64 = core::num::<impl u8>::trailing_zeros(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v128 = call fn16(v127)
// CHECK-NEXT:     v129 -> v128
// CHECK-NEXT: ; write_cvalue: Var(_64, var56): u32 <- ByVal(v128): u32
// CHECK-NEXT:     jump block22
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _63 = Eq(move _64, const 0_u32)
// CHECK-NEXT:     v130 = iconst.i32 0
// CHECK-NEXT:     v131 = icmp.i32 eq v129, v130  ; v130 = 0
// CHECK-NEXT: ; write_cvalue: Var(_63, var55): bool <- ByVal(v131): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _63)
// CHECK-NEXT:     brif.i32 v129, block24, block23
// CHECK-EMPTY:
// CHECK-NEXT: block23:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _65 = core::num::<impl u8>::count_ones(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v133 = call fn17(v132)
// CHECK-NEXT:     v135 -> v133
// CHECK-NEXT: ; write_cvalue: Var(_65, var57): u32 <- ByVal(v133): u32
// CHECK-NEXT:     jump block25
// CHECK-EMPTY:
// CHECK-NEXT: block24:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = const false
// CHECK-NEXT:     v134 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v134): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block26(v134)  ; v134 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block25:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = Eq(move _65, const 8_u32)
// CHECK-NEXT:     v136 = iconst.i32 8
// CHECK-NEXT:     v137 = icmp.i32 eq v135, v136  ; v136 = 8
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v137): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block26(v137)
// CHECK-EMPTY:
// CHECK-NEXT: block26(v138: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _59 = move _60 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_59, var51): u8 <- ByVal(v138): u8
// CHECK-NEXT: ; _58 = Shl(move _59, const 7_i32)
// CHECK-NEXT:     v139 = iconst.i32 7
// CHECK-NEXT:     v140 = ishl v138, v139  ; v139 = 7
// CHECK-NEXT: ; write_cvalue: Var(_58, var50): u8 <- ByVal(v140): u8
// CHECK-NEXT: ; _0 = BitOr(move _8, move _58)
// CHECK-NEXT:     v142 = bor.i8 v141, v140
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v142): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v142
// CHECK-NEXT: }
