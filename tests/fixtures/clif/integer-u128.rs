#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn integer_methods(x: u128, y: u128) -> u8 {
    let add = (1, true);
    let sub = (u128::MAX - 2, false);
    let mul = (u128::MAX - 1, true);
    (x.checked_add(y).is_none() as u8)
        | ((x.checked_sub(y) == Some(u128::MAX - 2)) as u8) << 1
        | (x.checked_mul(y).is_none() as u8) << 2
        | ((x.wrapping_add(y) == add.0) as u8) << 3
        | ((x.wrapping_sub(y) == sub.0) as u8) << 4
        | ((x.wrapping_mul(y) == mul.0) as u8) << 5
        | ((x.overflowing_add(y) == add
            && x.overflowing_sub(y) == sub
            && x.overflowing_mul(y) == mul) as u8) << 6
        | ((x.leading_zeros() == 0
            && x.trailing_zeros() == 0
            && x.count_ones() == 128) as u8) << 7
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    (integer_methods(u128::MAX, 2) == 255) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core6optionINtB2_6OptionoE7is_noneCs98nLHvXPTjv_12integer_u128
// CHECK-NEXT: function u0:0(i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core6optionINtB2_6OptionoE7is_noneCs98nLHvXPTjv_12integer_u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::option::Option<u128>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} core::option::Option<u128>    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    bool                              1b 1                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(1:11807 ~ core[..]::option::{impl#0}::is_some)), args: [u128] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} core::option::Option<u128> <- ByVal(v0): &'{erased} core::option::Option<u128>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::option::Option::<T>::is_some(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u128>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0(v1)
// CHECK-NEXT:     v3 -> v2
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): bool <- ByVal(v2): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Not(move _2)
// CHECK-NEXT:     v4 = iconst.i8 0
// CHECK-NEXT:     v5 = icmp.i8 eq v3, v4  ; v4 = 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v5): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core6optionINtB2_6OptionoE7is_someCs98nLHvXPTjv_12integer_u128
// CHECK-NEXT: function u0:1(i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core6optionINtB2_6OptionoE7is_someCs98nLHvXPTjv_12integer_u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::option::Option<u128>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} core::option::Option<u128>    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    isize                             2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} core::option::Option<u128> <- ByVal(v0): &'{erased} core::option::Option<u128>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = discriminant((*_1))
// CHECK-NEXT:     v2 = load.i16 notrap v1
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): isize <- ByVal(v2): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _2)
// CHECK-NEXT:     v3 = uextend.i32 v2
// CHECK-NEXT:     br_table v3, block5, [block2, block3]
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const false
// CHECK-NEXT:     v4 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v4): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v4  ; v4 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const true
// CHECK-NEXT:     v5 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v5): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v5  ; v5 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     v7 = iconst.i8 0
// CHECK-NEXT:     v6 -> v7
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v6  ; v6 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block5 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; unreachable
// CHECK-NEXT:     trap user1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo10count_ones
// CHECK-NEXT: function u0:2(i128) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo10count_ones
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i128):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u128 <- ByVal(v0): u128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::ctpop::<u128>(copy _1)
// CHECK-NEXT:     v2 = popcnt.i128 v1
// CHECK-NEXT:     v3 = ireduce.i32 v2
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
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo11checked_addCs98nLHvXPTjv_12integer_u128
// CHECK-NEXT: function u0:3(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo11checked_addCs98nLHvXPTjv_12integer_u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } core::option::Option<u128>
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (u128, bool)                     18b 2                var=(5, 6)
// CHECK-NEXT: ; ssa   _6    u128                             16b 2                var=7
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i8 uext) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:14 sig0 ; Instance { def: Item(DefId(1:2345 ~ core[..]::intrinsics::unlikely)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i128, v2: i128):
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v10 -> v1
// CHECK-NEXT:     v4 -> v2
// CHECK-NEXT:     v11 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): u128 <- ByVal(v1): u128
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): u128 <- ByVal(v2): u128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v5 = iadd.i128 v3, v4
// CHECK-NEXT:     v6 = icmp ult v5, v3
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var5, var6): (u128, bool) <- ByValPair(v5, v6): (u128, bool)
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v6): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::intrinsics::unlikely(move _4)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v7 = call fn0(v6)
// CHECK-NEXT:     v8 -> v7
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): bool <- ByVal(v7): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _3)
// CHECK-NEXT:     brif.i8 v8, block3, block4
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<u128>::None
// CHECK-NEXT:     v9 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u16 <- ByVal(v9): u16
// CHECK-NEXT:     store notrap v9, v0  ; v9 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = AddUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v12 = iadd.i128 v10, v11
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): u128 <- ByVal(v12): u128
// CHECK-NEXT: ; _0 = core::option::Option::<u128>::Some(move _6)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(2) }, None): u128 <- ByVal(v12): u128
// CHECK-NEXT:     store notrap v12, v0+2
// CHECK-NEXT:     v13 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u16 <- ByVal(v13): u16
// CHECK-NEXT:     store notrap v13, v0  ; v13 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo11checked_mulCs98nLHvXPTjv_12integer_u128
// CHECK-NEXT: function u0:4(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo11checked_mulCs98nLHvXPTjv_12integer_u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } core::option::Option<u128>
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    u128                             16b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (u128, bool)                     18b 2                var=(5, 6)
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=7
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 18, align = 2
// CHECK-NEXT:     sig0 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig1 = (i8 uext) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:12 sig0 ; Instance { def: Item(DefId(1:28223 ~ core[..]::num::{impl#10}::overflowing_mul)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:14 sig1 ; Instance { def: Item(DefId(1:2345 ~ core[..]::intrinsics::unlikely)), args: [] }
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
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::num::<impl u128>::overflowing_mul(copy _1, copy _2)
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn0(v5, v3, v4)
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var5, var6): (u128, bool) <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): (u128, bool)
// CHECK-NEXT:     v6 = stack_addr.i16 ss0
// CHECK-NEXT:     v7 = load.i128 notrap v6
// CHECK-NEXT:     v10 -> v7
// CHECK-NEXT:     v15 -> v7
// CHECK-NEXT:     v8 = stack_addr.i16 ss0+16
// CHECK-NEXT:     v9 = load.i8 notrap v8
// CHECK-NEXT:     v11 -> v9
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (_5.0: u128)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): u128 <- ByVal(v10): u128
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v11): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::intrinsics::unlikely(copy _4)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v12 = call fn1(v11)
// CHECK-NEXT:     v13 -> v12
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v12): bool
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     brif.i8 v13, block4, block5
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<u128>::None
// CHECK-NEXT:     v14 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u16 <- ByVal(v14): u16
// CHECK-NEXT:     store notrap v14, v0  ; v14 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<u128>::Some(copy _3)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(2) }, None): u128 <- ByVal(v15): u128
// CHECK-NEXT:     store.i128 notrap v15, v0+2
// CHECK-NEXT:     v16 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u16 <- ByVal(v16): u16
// CHECK-NEXT:     store notrap v16, v0  ; v16 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo11checked_subCs98nLHvXPTjv_12integer_u128
// CHECK-NEXT: function u0:5(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo11checked_subCs98nLHvXPTjv_12integer_u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } core::option::Option<u128>
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=3
// CHECK-NEXT: ; ssa   _4    u128                             16b 2                var=4
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i128, v2: i128):
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v7 -> v1
// CHECK-NEXT:     v4 -> v2
// CHECK-NEXT:     v8 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): u128 <- ByVal(v1): u128
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): u128 <- ByVal(v2): u128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = Lt(copy _1, copy _2)
// CHECK-NEXT:     v5 = icmp.i128 ult v3, v4
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): bool <- ByVal(v5): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _3)
// CHECK-NEXT:     brif v5, block2, block3
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<u128>::None
// CHECK-NEXT:     v6 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u16 <- ByVal(v6): u16
// CHECK-NEXT:     store notrap v6, v0  ; v6 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = SubUnchecked(copy _1, copy _2)
// CHECK-NEXT:     v9 = isub.i128 v7, v8
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): u128 <- ByVal(v9): u128
// CHECK-NEXT: ; _0 = core::option::Option::<u128>::Some(move _4)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(2) }, None): u128 <- ByVal(v9): u128
// CHECK-NEXT:     store notrap v9, v0+2
// CHECK-NEXT:     v10 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u16 <- ByVal(v10): u16
// CHECK-NEXT:     store notrap v10, v0  ; v10 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo12wrapping_add
// CHECK-NEXT: function u0:6(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo12wrapping_add
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(16 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } u128
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
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
// CHECK-NEXT: ; _0 = Add(copy _1, copy _2)
// CHECK-NEXT:     v5 = iadd.i128 v3, v4
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u128 <- ByVal(v5): u128
// CHECK-NEXT:     store notrap v5, v0
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo12wrapping_mul
// CHECK-NEXT: function u0:7(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo12wrapping_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(16 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } u128
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
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
// CHECK-NEXT: ; _0 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v5 = imul.i128 v3, v4
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u128 <- ByVal(v5): u128
// CHECK-NEXT:     store notrap v5, v0
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo12wrapping_sub
// CHECK-NEXT: function u0:8(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo12wrapping_sub
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(16 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } u128
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
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
// CHECK-NEXT: ; _0 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v5 = isub.i128 v3, v4
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u128 <- ByVal(v5): u128
// CHECK-NEXT:     store notrap v5, v0
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo13leading_zeros
// CHECK-NEXT: function u0:9(i128) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo13leading_zeros
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i128):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u128 <- ByVal(v0): u128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::ctlz::<u128>(copy _1)
// CHECK-NEXT:     v2 = clz.i128 v1
// CHECK-NEXT:     v3 = ireduce.i32 v2
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
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo14trailing_zeros
// CHECK-NEXT: function u0:10(i128) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo14trailing_zeros
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i128):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u128 <- ByVal(v0): u128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::cttz::<u128>(copy _1)
// CHECK-NEXT:     v2 = ctz.i128 v1
// CHECK-NEXT:     v3 = ireduce.i32 v2
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
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo15overflowing_add
// CHECK-NEXT: function u0:11(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo15overflowing_add
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } (u128, bool)
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    u128                             16b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (u128, bool)                     18b 2                var=(5, 6)
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
// CHECK-NEXT: ; _5 = AddWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v5 = iadd.i128 v3, v4
// CHECK-NEXT:     v6 = icmp ult v5, v3
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var5, var6): (u128, bool) <- ByValPair(v5, v6): (u128, bool)
// CHECK-NEXT: ; _3 = copy (_5.0: u128)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): u128 <- ByVal(v5): u128
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v6): bool
// CHECK-NEXT: ; _0 = (copy _3, copy _4)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u128 <- ByVal(v5): u128
// CHECK-NEXT:     store notrap v5, v0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(16) }, None): bool <- ByVal(v6): bool
// CHECK-NEXT:     store notrap v6, v0+16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo15overflowing_mul
// CHECK-NEXT: function u0:12(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo15overflowing_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } (u128, bool)
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    u128                             16b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (u128, bool)                     18b 2                var=(5, 6)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 16, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 16, align = 2
// CHECK-NEXT:     sig0 = (i16, i16, i16) system_v
// CHECK-NEXT:     fn0 = u0:22 sig0 ; "__gbc_u128_mulo"
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
// CHECK-NEXT: ; _5 = MulWithOverflow(copy _1, copy _2)
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
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var5, var6): (u128, bool) <- ByValPair(v11, v13): (u128, bool)
// CHECK-NEXT: ; _3 = copy (_5.0: u128)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): u128 <- ByVal(v11): u128
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v13): bool
// CHECK-NEXT: ; _0 = (copy _3, copy _4)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u128 <- ByVal(v11): u128
// CHECK-NEXT:     store notrap v11, v0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(16) }, None): bool <- ByVal(v13): bool
// CHECK-NEXT:     store notrap v13, v0+16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo15overflowing_sub
// CHECK-NEXT: function u0:13(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol _RNvMs8_NtCs40R3J0iFVxM_4core3numo15overflowing_sub
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } (u128, bool)
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    u128                             16b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (u128, bool)                     18b 2                var=(5, 6)
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
// CHECK-NEXT: ; _5 = SubWithOverflow(copy _1, copy _2)
// CHECK-NEXT:     v5 = isub.i128 v3, v4
// CHECK-NEXT:     v6 = icmp ugt v5, v3
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var5, var6): (u128, bool) <- ByValPair(v5, v6): (u128, bool)
// CHECK-NEXT: ; _3 = copy (_5.0: u128)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): u128 <- ByVal(v5): u128
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v6): bool
// CHECK-NEXT: ; _0 = (copy _3, copy _4)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): u128 <- ByVal(v5): u128
// CHECK-NEXT:     store notrap v5, v0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(16) }, None): bool <- ByVal(v6): bool
// CHECK-NEXT:     store notrap v6, v0+16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics8unlikely
// CHECK-NEXT: function u0:14(i8 uext) -> i8 uext system_v {
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
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCs98nLHvXPTjv_12integer_u128
// CHECK-NEXT: function u0:15() system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCs98nLHvXPTjv_12integer_u128
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
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTobENtNtB7_3cmp9PartialEq2eqCs98nLHvXPTjv_12integer_u128
// CHECK-NEXT: function u0:16(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTobENtNtB7_3cmp9PartialEq2eqCs98nLHvXPTjv_12integer_u128
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
// CHECK-NEXT:     fn0 = colocated u0:19 sig0 ; Instance { def: Item(DefId(1:29984 ~ core[..]::cmp::impls::{impl#26}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:18 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
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
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptionoENtNtB7_3cmp9PartialEq2eqCs98nLHvXPTjv_12integer_u128
// CHECK-NEXT: function u0:17(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptionoENtNtB7_3cmp9PartialEq2eqCs98nLHvXPTjv_12integer_u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::option::Option<u128>
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::option::Option<u128>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} core::option::Option<u128>    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} core::option::Option<u128>    2b 2                var=3
// CHECK-NEXT: ; ssa   _3    (&'{erased} core::option::Option<u128>, &'{erased} core::option::Option<u128>)    4b 2                var=(4, 5)
// CHECK-NEXT: ; ssa   _4    isize                             2b 2                var=6
// CHECK-NEXT: ; ssa   _5    isize                             2b 2                var=7
// CHECK-NEXT: ; ssa   _6    isize                             2b 2                var=8
// CHECK-NEXT: ; ssa   _7    &'{erased} u128                   2b 2                var=9
// CHECK-NEXT: ; ssa   _8    &'{erased} u128                   2b 2                var=10
// CHECK-NEXT: ; ssa   _9    &'{erased} core::option::Option<u128>    2b 2                var=11
// CHECK-NEXT: ; ssa   _10   &'{erased} core::option::Option<u128>    2b 2                var=12
// CHECK-NEXT: ; ssa   _11   &'{erased} core::option::Option<u128>    2b 2                var=13
// CHECK-NEXT: ; stack _12   &'{erased} core::option::Option<u128>    2b 2                storage=ss0
// CHECK-NEXT: ; stack _13   &'{erased} core::option::Option<u128>    2b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:19 sig0 ; Instance { def: Item(DefId(1:29984 ~ core[..]::cmp::impls::{impl#26}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v15 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v6 -> v1
// CHECK-NEXT:     v9 -> v1
// CHECK-NEXT:     v21 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} core::option::Option<u128> <- ByVal(v0): &'{erased} core::option::Option<u128>
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} core::option::Option<u128> <- ByVal(v1): &'{erased} core::option::Option<u128>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): &'{erased} core::option::Option<u128> <- ByVal(v2): &'{erased} core::option::Option<u128>
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): &'{erased} core::option::Option<u128> <- ByVal(v3): &'{erased} core::option::Option<u128>
// CHECK-NEXT: ; _9 = no_retag copy (_3.0: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Var(_9, var11): &'{erased} core::option::Option<u128> <- ByVal(v2): &'{erased} core::option::Option<u128>
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
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): &'{erased} core::option::Option<u128> <- ByVal(v6): &'{erased} core::option::Option<u128>
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
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): &'{erased} core::option::Option<u128> <- ByVal(v9): &'{erased} core::option::Option<u128>
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
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} core::option::Option<u128> <- ByVal(v15): &'{erased} core::option::Option<u128>
// CHECK-NEXT:     v16 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v15, v16
// CHECK-NEXT: ; _7 = &(((*_12) as Some).0: T)
// CHECK-NEXT:     v17 = stack_addr.i16 ss0
// CHECK-NEXT:     v18 = load.i16 notrap v17
// CHECK-NEXT:     v19 = iconst.i16 2
// CHECK-NEXT:     v20 = iadd v18, v19  ; v19 = 2
// CHECK-NEXT: ; write_cvalue: Var(_7, var9): &'{erased} u128 <- ByVal(v20): &'{erased} u128
// CHECK-NEXT: ; _13 = no_retag copy (_3.1: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} core::option::Option<u128> <- ByVal(v21): &'{erased} core::option::Option<u128>
// CHECK-NEXT:     v22 = stack_addr.i16 ss1
// CHECK-NEXT:     store.i16 notrap v21, v22
// CHECK-NEXT: ; _8 = &(((*_13) as Some).0: T)
// CHECK-NEXT:     v23 = stack_addr.i16 ss1
// CHECK-NEXT:     v24 = load.i16 notrap v23
// CHECK-NEXT:     v25 = iconst.i16 2
// CHECK-NEXT:     v26 = iadd v24, v25  ; v25 = 2
// CHECK-NEXT: ; write_cvalue: Var(_8, var10): &'{erased} u128 <- ByVal(v26): &'{erased} u128
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::cmp::PartialEq>::eq(copy _7, copy _8)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &u128, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &u128, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCs98nLHvXPTjv_12integer_u128
// CHECK-NEXT: function u0:18(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCs98nLHvXPTjv_12integer_u128
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
// CHECK-NEXT: ; symbol _RNvXso_NtNtCs40R3J0iFVxM_4core3cmp5implsoNtB7_9PartialEq2eqCs98nLHvXPTjv_12integer_u128
// CHECK-NEXT: function u0:19(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXso_NtNtCs40R3J0iFVxM_4core3cmp5implsoNtB7_9PartialEq2eqCs98nLHvXPTjv_12integer_u128
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
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:20() -> i8 system_v {
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
// CHECK-NEXT:     sig0 = (i128, i128) -> i8 system_v
// CHECK-NEXT:     fn0 = colocated u0:21 sig0 ; Instance { def: Item(DefId(0:3 ~ integer_u128[..]::integer_methods)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = integer_methods(const core::num::<impl u128>::MAX, const 2_u128)
// CHECK-NEXT:     v0 = iconst.i64 -1
// CHECK-NEXT:     v1 = iconst.i64 -1
// CHECK-NEXT:     v2 = iconcat v0, v1  ; v0 = -1, v1 = -1
// CHECK-NEXT:     v3 = iconst.i64 2
// CHECK-NEXT:     v4 = iconst.i64 0
// CHECK-NEXT:     v5 = iconcat v3, v4  ; v3 = 2, v4 = 0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v6 = call fn0(v2, v5)
// CHECK-NEXT:     v7 -> v6
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u8 <- ByVal(v6): u8
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = Eq(move _2, const u8::MAX)
// CHECK-NEXT:     v8 = iconst.i8 -1
// CHECK-NEXT:     v9 = icmp.i8 eq v7, v8  ; v8 = -1
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v9): bool
// CHECK-NEXT: ; _0 = move _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v9): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v9
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol integer_methods
// CHECK-NEXT: function u0:21(i128, i128) -> i8 system_v {
// CHECK-NEXT: ; symbol integer_methods
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=3
// CHECK-NEXT: ; stack _3    (u128, bool)                     18b 2                storage=ss0
// CHECK-NEXT: ; stack _4    (u128, bool)                     18b 2                storage=ss1
// CHECK-NEXT: ; ssa   _5    u128                             16b 2                var=4
// CHECK-NEXT: ; stack _6    (u128, bool)                     18b 2                storage=ss2
// CHECK-NEXT: ; ssa   _7    u128                             16b 2                var=5
// CHECK-NEXT: ; ssa   _8    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=7
// CHECK-NEXT: ; ssa   _10   u8                                1b 1                var=8
// CHECK-NEXT: ; ssa   _11   u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=10
// CHECK-NEXT: ; ssa   _13   u8                                1b 1                var=11
// CHECK-NEXT: ; ssa   _14   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _15   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _16   &'{erased} core::option::Option<u128>    2b 2                var=14
// CHECK-NEXT: ; stack _17   core::option::Option<u128>       18b 2                storage=ss3
// CHECK-NEXT: ; ssa   _18   u8                                1b 1                var=15
// CHECK-NEXT: ; ssa   _19   u8                                1b 1                var=16
// CHECK-NEXT: ; ssa   _20   bool                              1b 1                var=17
// CHECK-NEXT: ; ssa   _21   &'{erased} core::option::Option<u128>    2b 2                var=18
// CHECK-NEXT: ; stack _22   core::option::Option<u128>       18b 2                storage=ss4
// CHECK-NEXT: ; ssa   _23   &'{erased} core::option::Option<u128>    2b 2                var=19
// CHECK-NEXT: ; ssa   _24   u8                                1b 1                var=20
// CHECK-NEXT: ; ssa   _25   u8                                1b 1                var=21
// CHECK-NEXT: ; ssa   _26   bool                              1b 1                var=22
// CHECK-NEXT: ; ssa   _27   &'{erased} core::option::Option<u128>    2b 2                var=23
// CHECK-NEXT: ; stack _28   core::option::Option<u128>       18b 2                storage=ss5
// CHECK-NEXT: ; ssa   _29   u8                                1b 1                var=24
// CHECK-NEXT: ; ssa   _30   u8                                1b 1                var=25
// CHECK-NEXT: ; ssa   _31   bool                              1b 1                var=26
// CHECK-NEXT: ; ssa   _32   u128                             16b 2                var=27
// CHECK-NEXT: ; ssa   _33   u128                             16b 2                var=28
// CHECK-NEXT: ; ssa   _34   u8                                1b 1                var=29
// CHECK-NEXT: ; ssa   _35   u8                                1b 1                var=30
// CHECK-NEXT: ; ssa   _36   bool                              1b 1                var=31
// CHECK-NEXT: ; ssa   _37   u128                             16b 2                var=32
// CHECK-NEXT: ; ssa   _38   u128                             16b 2                var=33
// CHECK-NEXT: ; ssa   _39   u8                                1b 1                var=34
// CHECK-NEXT: ; ssa   _40   u8                                1b 1                var=35
// CHECK-NEXT: ; ssa   _41   bool                              1b 1                var=36
// CHECK-NEXT: ; ssa   _42   u128                             16b 2                var=37
// CHECK-NEXT: ; ssa   _43   u128                             16b 2                var=38
// CHECK-NEXT: ; ssa   _44   u8                                1b 1                var=39
// CHECK-NEXT: ; ssa   _45   u8                                1b 1                var=40
// CHECK-NEXT: ; ssa   _46   bool                              1b 1                var=41
// CHECK-NEXT: ; ssa   _47   bool                              1b 1                var=42
// CHECK-NEXT: ; ssa   _48   &'{erased} (u128, bool)           2b 2                var=43
// CHECK-NEXT: ; stack _49   (u128, bool)                     18b 2                storage=ss6
// CHECK-NEXT: ; ssa   _50   &'{erased} (u128, bool)           2b 2                var=44
// CHECK-NEXT: ; ssa   _51   bool                              1b 1                var=45
// CHECK-NEXT: ; ssa   _52   &'{erased} (u128, bool)           2b 2                var=46
// CHECK-NEXT: ; stack _53   (u128, bool)                     18b 2                storage=ss7
// CHECK-NEXT: ; ssa   _54   &'{erased} (u128, bool)           2b 2                var=47
// CHECK-NEXT: ; ssa   _55   &'{erased} (u128, bool)           2b 2                var=48
// CHECK-NEXT: ; stack _56   (u128, bool)                     18b 2                storage=ss8
// CHECK-NEXT: ; ssa   _57   &'{erased} (u128, bool)           2b 2                var=49
// CHECK-NEXT: ; ssa   _58   u8                                1b 1                var=50
// CHECK-NEXT: ; ssa   _59   u8                                1b 1                var=51
// CHECK-NEXT: ; ssa   _60   bool                              1b 1                var=52
// CHECK-NEXT: ; ssa   _61   bool                              1b 1                var=53
// CHECK-NEXT: ; ssa   _62   u32                               4b 2                var=54
// CHECK-NEXT: ; ssa   _63   bool                              1b 1                var=55
// CHECK-NEXT: ; ssa   _64   u32                               4b 2                var=56
// CHECK-NEXT: ; ssa   _65   u32                               4b 2                var=57
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss3 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss4 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss5 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss6 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss7 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss8 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss9 = explicit_slot 16, align = 2
// CHECK-NEXT:     ss10 = explicit_slot 16, align = 2
// CHECK-NEXT:     ss11 = explicit_slot 16, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname3 ; alloc3
// CHECK-NEXT:     sig0 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig1 = (i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig4 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig5 = (i16) -> i8 uext system_v
// CHECK-NEXT:     sig6 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig7 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig8 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig9 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig10 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig11 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig12 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig13 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig14 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig15 = (i128) -> i32 system_v
// CHECK-NEXT:     sig16 = (i128) -> i32 system_v
// CHECK-NEXT:     sig17 = (i128) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:3 sig0 ; Instance { def: Item(DefId(1:28148 ~ core[..]::num::{impl#10}::checked_add)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:0 sig1 ; Instance { def: Item(DefId(1:11810 ~ core[..]::option::{impl#0}::is_none)), args: [u128] }
// CHECK-NEXT:     fn2 = colocated u0:5 sig2 ; Instance { def: Item(DefId(1:28153 ~ core[..]::num::{impl#10}::checked_sub)), args: [] }
// CHECK-NEXT:     fn3 = colocated u0:17 sig3 ; Instance { def: Item(DefId(1:11950 ~ core[..]::option::{impl#17}::eq)), args: [u128] }
// CHECK-NEXT:     fn4 = colocated u0:4 sig4 ; Instance { def: Item(DefId(1:28159 ~ core[..]::num::{impl#10}::checked_mul)), args: [] }
// CHECK-NEXT:     fn5 = colocated u0:0 sig5 ; Instance { def: Item(DefId(1:11810 ~ core[..]::option::{impl#0}::is_none)), args: [u128] }
// CHECK-NEXT:     fn6 = colocated u0:6 sig6 ; Instance { def: Item(DefId(1:28203 ~ core[..]::num::{impl#10}::wrapping_add)), args: [] }
// CHECK-NEXT:     fn7 = colocated u0:8 sig7 ; Instance { def: Item(DefId(1:28205 ~ core[..]::num::{impl#10}::wrapping_sub)), args: [] }
// CHECK-NEXT:     fn8 = colocated u0:7 sig8 ; Instance { def: Item(DefId(1:28207 ~ core[..]::num::{impl#10}::wrapping_mul)), args: [] }
// CHECK-NEXT:     fn9 = colocated u0:11 sig9 ; Instance { def: Item(DefId(1:28216 ~ core[..]::num::{impl#10}::overflowing_add)), args: [] }
// CHECK-NEXT:     fn10 = colocated u0:16 sig10 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u128, bool] }
// CHECK-NEXT:     fn11 = colocated u0:13 sig11 ; Instance { def: Item(DefId(1:28219 ~ core[..]::num::{impl#10}::overflowing_sub)), args: [] }
// CHECK-NEXT:     fn12 = colocated u0:16 sig12 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u128, bool] }
// CHECK-NEXT:     fn13 = colocated u0:12 sig13 ; Instance { def: Item(DefId(1:28223 ~ core[..]::num::{impl#10}::overflowing_mul)), args: [] }
// CHECK-NEXT:     fn14 = colocated u0:16 sig14 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u128, bool] }
// CHECK-NEXT:     fn15 = colocated u0:9 sig15 ; Instance { def: Item(DefId(1:28120 ~ core[..]::num::{impl#10}::leading_zeros)), args: [] }
// CHECK-NEXT:     fn16 = colocated u0:10 sig16 ; Instance { def: Item(DefId(1:28121 ~ core[..]::num::{impl#10}::trailing_zeros)), args: [] }
// CHECK-NEXT:     fn17 = colocated u0:2 sig17 ; Instance { def: Item(DefId(1:28118 ~ core[..]::num::{impl#10}::count_ones)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i128, v1: i128):
// CHECK-NEXT:     v28 -> v0
// CHECK-NEXT:     v34 -> v0
// CHECK-NEXT:     v45 -> v0
// CHECK-NEXT:     v55 -> v0
// CHECK-NEXT:     v68 -> v0
// CHECK-NEXT:     v81 -> v0
// CHECK-NEXT:     v94 -> v0
// CHECK-NEXT:     v101 -> v0
// CHECK-NEXT:     v108 -> v0
// CHECK-NEXT:     v120 -> v0
// CHECK-NEXT:     v125 -> v0
// CHECK-NEXT:     v130 -> v0
// CHECK-NEXT:     v142 -> v0
// CHECK-NEXT:     v29 -> v1
// CHECK-NEXT:     v35 -> v1
// CHECK-NEXT:     v46 -> v1
// CHECK-NEXT:     v56 -> v1
// CHECK-NEXT:     v69 -> v1
// CHECK-NEXT:     v82 -> v1
// CHECK-NEXT:     v95 -> v1
// CHECK-NEXT:     v102 -> v1
// CHECK-NEXT:     v109 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u128 <- ByVal(v0): u128
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u128 <- ByVal(v1): u128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (const 1_u128, const true)
// CHECK-NEXT:     v2 = iconst.i64 1
// CHECK-NEXT:     v3 = iconst.i64 0
// CHECK-NEXT:     v4 = iconcat v2, v3  ; v2 = 1, v3 = 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u128 <- ByVal(v4): u128
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v4, v5
// CHECK-NEXT:     v6 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(16) }, None): bool <- ByVal(v6): bool
// CHECK-NEXT:     v7 = stack_addr.i16 ss0+16
// CHECK-NEXT:     store notrap v6, v7  ; v6 = 1
// CHECK-NEXT: ; _5 = Sub(const core::num::<impl u128>::MAX, const 2_u128)
// CHECK-NEXT:     v8 = iconst.i64 -1
// CHECK-NEXT:     v9 = iconst.i64 -1
// CHECK-NEXT:     v10 = iconcat v8, v9  ; v8 = -1, v9 = -1
// CHECK-NEXT:     v11 = iconst.i64 2
// CHECK-NEXT:     v12 = iconst.i64 0
// CHECK-NEXT:     v13 = iconcat v11, v12  ; v11 = 2, v12 = 0
// CHECK-NEXT:     v14 = isub v10, v13
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): u128 <- ByVal(v14): u128
// CHECK-NEXT: ; _4 = (move _5, const false)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u128 <- ByVal(v14): u128
// CHECK-NEXT:     v15 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v14, v15
// CHECK-NEXT:     v16 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(16) }, None): bool <- ByVal(v16): bool
// CHECK-NEXT:     v17 = stack_addr.i16 ss1+16
// CHECK-NEXT:     store notrap v16, v17  ; v16 = 0
// CHECK-NEXT: ; _7 = Sub(const core::num::<impl u128>::MAX, const 1_u128)
// CHECK-NEXT:     v18 = iconst.i64 -1
// CHECK-NEXT:     v19 = iconst.i64 -1
// CHECK-NEXT:     v20 = iconcat v18, v19  ; v18 = -1, v19 = -1
// CHECK-NEXT:     v21 = iconst.i64 1
// CHECK-NEXT:     v22 = iconst.i64 0
// CHECK-NEXT:     v23 = iconcat v21, v22  ; v21 = 1, v22 = 0
// CHECK-NEXT:     v24 = isub v20, v23
// CHECK-NEXT: ; write_cvalue: Var(_7, var5): u128 <- ByVal(v24): u128
// CHECK-NEXT: ; _6 = (move _7, const true)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u128 <- ByVal(v24): u128
// CHECK-NEXT:     v25 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v24, v25
// CHECK-NEXT:     v26 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(16) }, None): bool <- ByVal(v26): bool
// CHECK-NEXT:     v27 = stack_addr.i16 ss2+16
// CHECK-NEXT:     store notrap v26, v27  ; v26 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; _17 = core::num::<impl u128>::checked_add(copy _1, copy _2)
// CHECK-NEXT:     v30 = stack_addr.i16 ss3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<u128>, layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u128, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u16, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u16 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(2 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(18 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u128, b_offset: Size(2 bytes) }, field_offsets: [Size(2 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8189598409595717631 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn0(v30, v28, v29)
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = &_17
// CHECK-NEXT:     v31 = stack_addr.i16 ss3
// CHECK-NEXT: ; write_cvalue: Var(_16, var14): &'{erased} core::option::Option<u128> <- ByVal(v31): &'{erased} core::option::Option<u128>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _15 = core::option::Option::<u128>::is_none(move _16)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u128>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v32 = call fn1(v31)
// CHECK-NEXT:     v33 -> v32
// CHECK-NEXT:     v43 -> v32
// CHECK-NEXT: ; write_cvalue: Var(_15, var13): bool <- ByVal(v32): bool
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _14 = move _15 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_14, var12): u8 <- ByVal(v33): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _22 = core::num::<impl u128>::checked_sub(copy _1, copy _2)
// CHECK-NEXT:     v36 = stack_addr.i16 ss4
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<u128>, layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u128, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u16, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u16 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(2 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(18 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u128, b_offset: Size(2 bytes) }, field_offsets: [Size(2 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8189598409595717631 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v36, v34, v35)
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _21 = &_22
// CHECK-NEXT:     v37 = stack_addr.i16 ss4
// CHECK-NEXT: ; write_cvalue: Var(_21, var18): &'{erased} core::option::Option<u128> <- ByVal(v37): &'{erased} core::option::Option<u128>
// CHECK-NEXT: ; _23 = const integer_methods::promoted[0]
// CHECK-NEXT:     v38 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_23, var19): &'{erased} core::option::Option<u128> <- ByVal(v38): &'{erased} core::option::Option<u128>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _20 = <core::option::Option<u128> as core::cmp::PartialEq>::eq(move _21, move _23)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u128>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u128>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v39 = call fn3(v37, v38)
// CHECK-NEXT:     v40 -> v39
// CHECK-NEXT: ; write_cvalue: Var(_20, var17): bool <- ByVal(v39): bool
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _19 = move _20 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_19, var16): u8 <- ByVal(v40): u8
// CHECK-NEXT: ; _18 = Shl(move _19, const 1_i32)
// CHECK-NEXT:     v41 = iconst.i32 1
// CHECK-NEXT:     v42 = ishl.i8 v40, v41  ; v41 = 1
// CHECK-NEXT: ; write_cvalue: Var(_18, var15): u8 <- ByVal(v42): u8
// CHECK-NEXT: ; _13 = BitOr(move _14, move _18)
// CHECK-NEXT:     v44 = bor.i8 v43, v42
// CHECK-NEXT:     v53 -> v44
// CHECK-NEXT: ; write_cvalue: Var(_13, var11): u8 <- ByVal(v44): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _28 = core::num::<impl u128>::checked_mul(copy _1, copy _2)
// CHECK-NEXT:     v47 = stack_addr.i16 ss5
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<u128>, layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u128, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u16, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u16 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(2 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(18 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u128, b_offset: Size(2 bytes) }, field_offsets: [Size(2 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8189598409595717631 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn4(v47, v45, v46)
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _27 = &_28
// CHECK-NEXT:     v48 = stack_addr.i16 ss5
// CHECK-NEXT: ; write_cvalue: Var(_27, var23): &'{erased} core::option::Option<u128> <- ByVal(v48): &'{erased} core::option::Option<u128>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _26 = core::option::Option::<u128>::is_none(move _27)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<u128>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v49 = call fn5(v48)
// CHECK-NEXT:     v50 -> v49
// CHECK-NEXT: ; write_cvalue: Var(_26, var22): bool <- ByVal(v49): bool
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _25 = move _26 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_25, var21): u8 <- ByVal(v50): u8
// CHECK-NEXT: ; _24 = Shl(move _25, const 2_i32)
// CHECK-NEXT:     v51 = iconst.i32 2
// CHECK-NEXT:     v52 = ishl.i8 v50, v51  ; v51 = 2
// CHECK-NEXT: ; write_cvalue: Var(_24, var20): u8 <- ByVal(v52): u8
// CHECK-NEXT: ; _12 = BitOr(move _13, move _24)
// CHECK-NEXT:     v54 = bor.i8 v53, v52
// CHECK-NEXT:     v66 -> v54
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): u8 <- ByVal(v54): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _32 = core::num::<impl u128>::wrapping_add(copy _1, copy _2)
// CHECK-NEXT:     v57 = stack_addr.i16 ss9
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(16 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn6(v57, v55, v56)
// CHECK-NEXT: ; write_cvalue: Var(_32, var27): u128 <- ByRef(Pointer { base: Stack(ss9), offset: Offset32(0) }, None): u128
// CHECK-NEXT:     v58 = stack_addr.i16 ss9
// CHECK-NEXT:     v59 = load.i128 notrap v58
// CHECK-NEXT:     v62 -> v59
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _33 = copy (_3.0: u128)
// CHECK-NEXT: ; write_cvalue: Var(_33, var28): u128 <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u128
// CHECK-NEXT:     v60 = stack_addr.i16 ss0
// CHECK-NEXT:     v61 = load.i128 notrap v60
// CHECK-NEXT: ; _31 = Eq(move _32, move _33)
// CHECK-NEXT:     v63 = icmp.i128 eq v62, v61
// CHECK-NEXT: ; write_cvalue: Var(_31, var26): bool <- ByVal(v63): bool
// CHECK-NEXT: ; _30 = move _31 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_30, var25): u8 <- ByVal(v63): u8
// CHECK-NEXT: ; _29 = Shl(move _30, const 3_i32)
// CHECK-NEXT:     v64 = iconst.i32 3
// CHECK-NEXT:     v65 = ishl v63, v64  ; v64 = 3
// CHECK-NEXT: ; write_cvalue: Var(_29, var24): u8 <- ByVal(v65): u8
// CHECK-NEXT: ; _11 = BitOr(move _12, move _29)
// CHECK-NEXT:     v67 = bor.i8 v66, v65
// CHECK-NEXT:     v79 -> v67
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): u8 <- ByVal(v67): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _37 = core::num::<impl u128>::wrapping_sub(copy _1, copy _2)
// CHECK-NEXT:     v70 = stack_addr.i16 ss10
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(16 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn7(v70, v68, v69)
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): u128 <- ByRef(Pointer { base: Stack(ss10), offset: Offset32(0) }, None): u128
// CHECK-NEXT:     v71 = stack_addr.i16 ss10
// CHECK-NEXT:     v72 = load.i128 notrap v71
// CHECK-NEXT:     v75 -> v72
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _38 = copy (_4.0: u128)
// CHECK-NEXT: ; write_cvalue: Var(_38, var33): u128 <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u128
// CHECK-NEXT:     v73 = stack_addr.i16 ss1
// CHECK-NEXT:     v74 = load.i128 notrap v73
// CHECK-NEXT: ; _36 = Eq(move _37, move _38)
// CHECK-NEXT:     v76 = icmp.i128 eq v75, v74
// CHECK-NEXT: ; write_cvalue: Var(_36, var31): bool <- ByVal(v76): bool
// CHECK-NEXT: ; _35 = move _36 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_35, var30): u8 <- ByVal(v76): u8
// CHECK-NEXT: ; _34 = Shl(move _35, const 4_i32)
// CHECK-NEXT:     v77 = iconst.i32 4
// CHECK-NEXT:     v78 = ishl v76, v77  ; v77 = 4
// CHECK-NEXT: ; write_cvalue: Var(_34, var29): u8 <- ByVal(v78): u8
// CHECK-NEXT: ; _10 = BitOr(move _11, move _34)
// CHECK-NEXT:     v80 = bor.i8 v79, v78
// CHECK-NEXT:     v92 -> v80
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): u8 <- ByVal(v80): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _42 = core::num::<impl u128>::wrapping_mul(copy _1, copy _2)
// CHECK-NEXT:     v83 = stack_addr.i16 ss11
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(16 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn8(v83, v81, v82)
// CHECK-NEXT: ; write_cvalue: Var(_42, var37): u128 <- ByRef(Pointer { base: Stack(ss11), offset: Offset32(0) }, None): u128
// CHECK-NEXT:     v84 = stack_addr.i16 ss11
// CHECK-NEXT:     v85 = load.i128 notrap v84
// CHECK-NEXT:     v88 -> v85
// CHECK-NEXT:     jump block10
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _43 = copy (_6.0: u128)
// CHECK-NEXT: ; write_cvalue: Var(_43, var38): u128 <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u128
// CHECK-NEXT:     v86 = stack_addr.i16 ss2
// CHECK-NEXT:     v87 = load.i128 notrap v86
// CHECK-NEXT: ; _41 = Eq(move _42, move _43)
// CHECK-NEXT:     v89 = icmp.i128 eq v88, v87
// CHECK-NEXT: ; write_cvalue: Var(_41, var36): bool <- ByVal(v89): bool
// CHECK-NEXT: ; _40 = move _41 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_40, var35): u8 <- ByVal(v89): u8
// CHECK-NEXT: ; _39 = Shl(move _40, const 5_i32)
// CHECK-NEXT:     v90 = iconst.i32 5
// CHECK-NEXT:     v91 = ishl v89, v90  ; v90 = 5
// CHECK-NEXT: ; write_cvalue: Var(_39, var34): u8 <- ByVal(v91): u8
// CHECK-NEXT: ; _9 = BitOr(move _10, move _39)
// CHECK-NEXT:     v93 = bor.i8 v92, v91
// CHECK-NEXT:     v118 -> v93
// CHECK-NEXT:     v141 -> v93
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): u8 <- ByVal(v93): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _49 = core::num::<impl u128>::overflowing_add(copy _1, copy _2)
// CHECK-NEXT:     v96 = stack_addr.i16 ss6
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn9(v96, v94, v95)
// CHECK-NEXT:     jump block11
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _48 = &_49
// CHECK-NEXT:     v97 = stack_addr.i16 ss6
// CHECK-NEXT: ; write_cvalue: Var(_48, var43): &'{erased} (u128, bool) <- ByVal(v97): &'{erased} (u128, bool)
// CHECK-NEXT: ; _50 = &_3
// CHECK-NEXT:     v98 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_50, var44): &'{erased} (u128, bool) <- ByVal(v98): &'{erased} (u128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _47 = <(u128, bool) as core::cmp::PartialEq>::eq(move _48, move _50)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v99 = call fn10(v97, v98)
// CHECK-NEXT:     v100 -> v99
// CHECK-NEXT: ; write_cvalue: Var(_47, var42): bool <- ByVal(v99): bool
// CHECK-NEXT:     jump block12
// CHECK-EMPTY:
// CHECK-NEXT: block12:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _47)
// CHECK-NEXT:     brif.i8 v100, block13, block17
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _53 = core::num::<impl u128>::overflowing_sub(copy _1, copy _2)
// CHECK-NEXT:     v103 = stack_addr.i16 ss7
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn11(v103, v101, v102)
// CHECK-NEXT:     jump block14
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _52 = &_53
// CHECK-NEXT:     v104 = stack_addr.i16 ss7
// CHECK-NEXT: ; write_cvalue: Var(_52, var46): &'{erased} (u128, bool) <- ByVal(v104): &'{erased} (u128, bool)
// CHECK-NEXT: ; _54 = &_4
// CHECK-NEXT:     v105 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_54, var47): &'{erased} (u128, bool) <- ByVal(v105): &'{erased} (u128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _51 = <(u128, bool) as core::cmp::PartialEq>::eq(move _52, move _54)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v106 = call fn12(v104, v105)
// CHECK-NEXT:     v107 -> v106
// CHECK-NEXT: ; write_cvalue: Var(_51, var45): bool <- ByVal(v106): bool
// CHECK-NEXT:     jump block15
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _51)
// CHECK-NEXT:     brif.i8 v107, block16, block17
// CHECK-EMPTY:
// CHECK-NEXT: block16:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _56 = core::num::<impl u128>::overflowing_mul(copy _1, copy _2)
// CHECK-NEXT:     v110 = stack_addr.i16 ss8
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn13(v110, v108, v109)
// CHECK-NEXT:     jump block18
// CHECK-EMPTY:
// CHECK-NEXT: block17:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _46 = const false
// CHECK-NEXT:     v111 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v111): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block19(v111)  ; v111 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block18:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _55 = &_56
// CHECK-NEXT:     v112 = stack_addr.i16 ss8
// CHECK-NEXT: ; write_cvalue: Var(_55, var48): &'{erased} (u128, bool) <- ByVal(v112): &'{erased} (u128, bool)
// CHECK-NEXT: ; _57 = &_6
// CHECK-NEXT:     v113 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_57, var49): &'{erased} (u128, bool) <- ByVal(v113): &'{erased} (u128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _46 = <(u128, bool) as core::cmp::PartialEq>::eq(move _55, move _57)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v114 = call fn14(v112, v113)
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v114): bool
// CHECK-NEXT:     jump block19(v114)
// CHECK-EMPTY:
// CHECK-NEXT: block19(v115: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _45 = move _46 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_45, var40): u8 <- ByVal(v115): u8
// CHECK-NEXT: ; _44 = Shl(move _45, const 6_i32)
// CHECK-NEXT:     v116 = iconst.i32 6
// CHECK-NEXT:     v117 = ishl v115, v116  ; v116 = 6
// CHECK-NEXT: ; write_cvalue: Var(_44, var39): u8 <- ByVal(v117): u8
// CHECK-NEXT: ; _8 = BitOr(move _9, move _44)
// CHECK-NEXT:     v119 = bor.i8 v118, v117
// CHECK-NEXT:     v139 -> v119
// CHECK-NEXT:     v143 -> v119
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): u8 <- ByVal(v119): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _62 = core::num::<impl u128>::leading_zeros(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v121 = call fn15(v120)
// CHECK-NEXT:     v122 -> v121
// CHECK-NEXT: ; write_cvalue: Var(_62, var54): u32 <- ByVal(v121): u32
// CHECK-NEXT:     jump block20
// CHECK-EMPTY:
// CHECK-NEXT: block20:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _61 = Eq(move _62, const 0_u32)
// CHECK-NEXT:     v123 = iconst.i32 0
// CHECK-NEXT:     v124 = icmp.i32 eq v122, v123  ; v123 = 0
// CHECK-NEXT: ; write_cvalue: Var(_61, var53): bool <- ByVal(v124): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _61)
// CHECK-NEXT:     brif.i32 v122, block24, block21
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _64 = core::num::<impl u128>::trailing_zeros(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v126 = call fn16(v125)
// CHECK-NEXT:     v127 -> v126
// CHECK-NEXT: ; write_cvalue: Var(_64, var56): u32 <- ByVal(v126): u32
// CHECK-NEXT:     jump block22
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _63 = Eq(move _64, const 0_u32)
// CHECK-NEXT:     v128 = iconst.i32 0
// CHECK-NEXT:     v129 = icmp.i32 eq v127, v128  ; v128 = 0
// CHECK-NEXT: ; write_cvalue: Var(_63, var55): bool <- ByVal(v129): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _63)
// CHECK-NEXT:     brif.i32 v127, block24, block23
// CHECK-EMPTY:
// CHECK-NEXT: block23:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _65 = core::num::<impl u128>::count_ones(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(u128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8589934607 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v131 = call fn17(v130)
// CHECK-NEXT:     v133 -> v131
// CHECK-NEXT: ; write_cvalue: Var(_65, var57): u32 <- ByVal(v131): u32
// CHECK-NEXT:     jump block25
// CHECK-EMPTY:
// CHECK-NEXT: block24:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = const false
// CHECK-NEXT:     v132 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v132): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block26(v132)  ; v132 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block25:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = Eq(move _65, const 128_u32)
// CHECK-NEXT:     v134 = iconst.i32 128
// CHECK-NEXT:     v135 = icmp.i32 eq v133, v134  ; v134 = 128
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v135): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block26(v135)
// CHECK-EMPTY:
// CHECK-NEXT: block26(v136: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _59 = move _60 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_59, var51): u8 <- ByVal(v136): u8
// CHECK-NEXT: ; _58 = Shl(move _59, const 7_i32)
// CHECK-NEXT:     v137 = iconst.i32 7
// CHECK-NEXT:     v138 = ishl v136, v137  ; v137 = 7
// CHECK-NEXT: ; write_cvalue: Var(_58, var50): u8 <- ByVal(v138): u8
// CHECK-NEXT: ; _0 = BitOr(move _8, move _58)
// CHECK-NEXT:     v140 = bor.i8 v139, v138
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v140): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v140
// CHECK-NEXT: }
