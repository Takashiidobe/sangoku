#![no_std]

#[unsafe(no_mangle)]
pub fn bits(x: u32, n: u32) -> u32 { x.count_ones() + x.leading_zeros() + x.trailing_zeros() + x.swap_bytes() + x.rotate_left(n) + x.reverse_bits() }
#[unsafe(no_mangle)]
pub fn saturating(x: u16, y: u16) -> (u16, u16) { (x.saturating_add(y), x.saturating_sub(y)) }
#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 { (bits(1, 1) == 0x81000022 && saturating(65535, 1) == (65535, 65534)) as u8 }

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core10intrinsics11rotate_leftmECs4FamA2b6zOK_15core_intrinsics
// CHECK-NEXT: function u0:0(i32, i32) -> i32 system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core10intrinsics11rotate_leftmECs4FamA2b6zOK_15core_intrinsics
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u32                               4b 2                var=2
// CHECK-NEXT: ; ssa   _2    u32                               4b 2                var=3
// CHECK-NEXT: ; ssa   _3    u32                               4b 2                var=4
// CHECK-NEXT: ; ssa   _4    u32                               4b 2                var=5
// CHECK-NEXT: ; ssa   _5    u32                               4b 2                var=6
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=7
// CHECK-EMPTY:
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc7
// CHECK-NEXT:     sig0 = (i16) system_v
// CHECK-NEXT:     sig1 = (i32, i32, i32) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:3 sig0 ; "_RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_rem_by_zeroCs4FamA2b6zOK_15core_intrinsics"
// CHECK-NEXT:     fn1 = colocated u0:2 sig1 ; Instance { def: Item(DefId(1:2463 ~ core[..]::intrinsics::unchecked_funnel_shl)), args: [u32] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32, v1: i32):
// CHECK-NEXT:     v12 -> v0
// CHECK-NEXT:     v9 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u32 <- ByVal(v0): u32
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u32 <- ByVal(v1): u32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = const <T as core::mem::SizedTypeProperties>::SIZE as u32 (IntToInt)
// CHECK-NEXT:     v2 = iconst.i16 4
// CHECK-NEXT:     v3 = uextend.i32 v2  ; v2 = 4
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): u32 <- ByVal(v3): u32
// CHECK-NEXT: ; _4 = Mul(move _5, const 8_u32)
// CHECK-NEXT:     v4 = iconst.i32 8
// CHECK-NEXT:     v5 = imul v3, v4  ; v4 = 8
// CHECK-NEXT:     v10 -> v5
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): u32 <- ByVal(v5): u32
// CHECK-NEXT: ; _6 = Eq(copy _4, const 0_u32)
// CHECK-NEXT:     v6 = iconst.i32 0
// CHECK-NEXT:     v7 = icmp eq v5, v6  ; v6 = 0
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v7): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(!move _6, "attempt to calculate the remainder of `{}` with a divisor of zero", copy _2)
// CHECK-NEXT:     brif v7, block4, block2
// CHECK-EMPTY:
// CHECK-NEXT: block4 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v8 = symbol_value.i16 gv0
// CHECK-NEXT: ; panic _RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_rem_by_zeroCs4FamA2b6zOK_15core_intrinsics
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn0(v8)
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = Rem(copy _2, move _4)
// CHECK-NEXT:     v11 = urem.i32 v9, v10
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): u32 <- ByVal(v11): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::unchecked_funnel_shl::<T>(copy _1, move _1, move _3)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v13 = call fn1(v12, v12, v11)
// CHECK-NEXT:     v14 -> v13
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v13): u32
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v14
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core10intrinsics14disjoint_bitormECs4FamA2b6zOK_15core_intrinsics
// CHECK-NEXT: function u0:1(i32, i32, i16) -> i32 system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core10intrinsics14disjoint_bitormECs4FamA2b6zOK_15core_intrinsics
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg  ???      = v2       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u32                               4b 2                var=2
// CHECK-NEXT: ; ssa   _2    u32                               4b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i32, i32) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:5 sig0 ; Instance { def: Item(DefId(1:28849 ~ core[..]::intrinsics::fallback::{impl#15}::disjoint_bitor)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32, v1: i32, v2: i16):
// CHECK-NEXT:     v3 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u32 <- ByVal(v0): u32
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u32 <- ByVal(v1): u32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::intrinsics::fallback::DisjointBitOr>::disjoint_bitor(move _1, move _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v5 = call fn0(v3, v4)
// CHECK-NEXT:     v6 -> v5
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v5): u32
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v6
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core10intrinsics20unchecked_funnel_shlmECs4FamA2b6zOK_15core_intrinsics
// CHECK-NEXT: function u0:2(i32, i32, i32) -> i32 system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core10intrinsics20unchecked_funnel_shlmECs4FamA2b6zOK_15core_intrinsics
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u32                               4b 2                var=2
// CHECK-NEXT: ; ssa   _2    u32                               4b 2                var=3
// CHECK-NEXT: ; ssa   _3    u32                               4b 2                var=4
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i32, i32, i32) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:7 sig0 ; Instance { def: Item(DefId(1:28875 ~ core[..]::intrinsics::fallback::{impl#27}::unchecked_funnel_shl)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32, v1: i32, v2: i32):
// CHECK-NEXT:     v3 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     v5 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u32 <- ByVal(v0): u32
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u32 <- ByVal(v1): u32
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): u32 <- ByVal(v2): u32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::intrinsics::fallback::FunnelShift>::unchecked_funnel_shl(move _1, move _2, move _3)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v6 = call fn0(v3, v4, v5)
// CHECK-NEXT:     v7 -> v6
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v6): u32
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_rem_by_zeroCs4FamA2b6zOK_15core_intrinsics
// CHECK-NEXT: function u0:3(i16) system_v {
// CHECK-NEXT: ; symbol _RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_rem_by_zeroCs4FamA2b6zOK_15core_intrinsics
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               !
// CHECK-NEXT: ; arg  ???      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; zst   _1    !                                 0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _1 = core::intrinsics::abort_immediate()
// CHECK-NEXT:     trap user2
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTttENtNtB7_3cmp9PartialEq2eqCs4FamA2b6zOK_15core_intrinsics
// CHECK-NEXT: function u0:4(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTttENtNtB7_3cmp9PartialEq2eqCs4FamA2b6zOK_15core_intrinsics
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (u16, u16)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (u16, u16)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} (u16, u16)             2b 2                storage=ss0
// CHECK-NEXT: ; stack _2    &'{erased} (u16, u16)             2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _4    &'{erased} u16                    2b 2                var=3
// CHECK-NEXT: ; ssa   _5    &'{erased} u16                    2b 2                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} u16                    2b 2                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} u16                    2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:6 sig0 ; Instance { def: Item(DefId(1:29975 ~ core[..]::cmp::impls::{impl#23}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:6 sig1 ; Instance { def: Item(DefId(1:29975 ~ core[..]::cmp::impls::{impl#23}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (u16, u16) <- ByVal(v0): &'{erased} (u16, u16)
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u16, u16) <- ByVal(v1): &'{erased} (u16, u16)
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = &((*_1).0: U)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): &'{erased} u16 <- ByVal(v5): &'{erased} u16
// CHECK-NEXT: ; _5 = &((*_2).0: U)
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): &'{erased} u16 <- ByVal(v7): &'{erased} u16
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = <U as core::cmp::PartialEq>::eq(move _4, move _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): &'{erased} u16 <- ByVal(v13): &'{erased} u16
// CHECK-NEXT: ; _7 = &((*_2).1: T)
// CHECK-NEXT:     v14 = stack_addr.i16 ss1
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = iconst.i16 2
// CHECK-NEXT:     v17 = iadd v15, v16  ; v16 = 2
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): &'{erased} u16 <- ByVal(v17): &'{erased} u16
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::cmp::PartialEq>::eq(move _6, move _7)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; symbol _RNvXsd_NtNtCs40R3J0iFVxM_4core10intrinsics8fallbackmNtB5_13DisjointBitOr14disjoint_bitorCs4FamA2b6zOK_15core_intrinsics
// CHECK-NEXT: function u0:5(i32, i32) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvXsd_NtNtCs40R3J0iFVxM_4core10intrinsics8fallbackmNtB5_13DisjointBitOr14disjoint_bitorCs4FamA2b6zOK_15core_intrinsics
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u32                               4b 2                var=2
// CHECK-NEXT: ; ssa   _2    u32                               4b 2                var=3
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _4    u32                               4b 2                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32, v1: i32):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u32 <- ByVal(v0): u32
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u32 <- ByVal(v1): u32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = BitAnd(copy _1, copy _2)
// CHECK-NEXT:     v4 = band.i32 v2, v3
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): u32 <- ByVal(v4): u32
// CHECK-NEXT: ; _3 = Eq(move _4, const 0_u32)
// CHECK-NEXT:     v5 = iconst.i32 0
// CHECK-NEXT:     v6 = icmp eq v4, v5  ; v5 = 0
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): bool <- ByVal(v6): bool
// CHECK-NEXT: ; assume(move _3)
// CHECK-NEXT: ; _0 = BitOr(copy _1, copy _2)
// CHECK-NEXT:     v7 = bor.i32 v2, v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v7): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXsl_NtNtCs40R3J0iFVxM_4core3cmp5implstNtB7_9PartialEq2eqCs4FamA2b6zOK_15core_intrinsics
// CHECK-NEXT: function u0:6(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsl_NtNtCs40R3J0iFVxM_4core3cmp5implstNtB7_9PartialEq2eqCs4FamA2b6zOK_15core_intrinsics
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} u16
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} u16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} u16                    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} u16                    2b 2                var=3
// CHECK-NEXT: ; ssa   _3    u16                               2b 2                var=4
// CHECK-NEXT: ; ssa   _4    u16                               2b 2                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} u16 <- ByVal(v0): &'{erased} u16
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} u16 <- ByVal(v1): &'{erased} u16
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): u16 <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): u16
// CHECK-NEXT:     v3 = load.i16 notrap v2
// CHECK-NEXT: ; _4 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): u16 <- ByRef(Pointer { base: Addr(v4), offset: Offset32(0) }, None): u16
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
// CHECK-NEXT: ; symbol _RNvXsp_NtNtCs40R3J0iFVxM_4core10intrinsics8fallbackmNtB5_11FunnelShift20unchecked_funnel_shlCs4FamA2b6zOK_15core_intrinsics
// CHECK-NEXT: function u0:7(i32, i32, i32) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvXsp_NtNtCs40R3J0iFVxM_4core10intrinsics8fallbackmNtB5_11FunnelShift20unchecked_funnel_shlCs4FamA2b6zOK_15core_intrinsics
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u32                               4b 2                var=2
// CHECK-NEXT: ; ssa   _2    u32                               4b 2                var=3
// CHECK-NEXT: ; ssa   _3    u32                               4b 2                var=4
// CHECK-NEXT: ; ssa   _4    u32                               4b 2                var=5
// CHECK-NEXT: ; ssa   _5    u32                               4b 2                var=6
// CHECK-NEXT: ; ssa   _6    u32                               4b 2                var=7
// CHECK-EMPTY:
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc9
// CHECK-NEXT:     sig0 = (i32, i32, i16) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(1:29129 ~ core[..]::intrinsics::disjoint_bitor)), args: [u32] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32, v1: i32, v2: i32):
// CHECK-NEXT:     v4 -> v0
// CHECK-NEXT:     v5 -> v0
// CHECK-NEXT:     v10 -> v1
// CHECK-NEXT:     v3 -> v2
// CHECK-NEXT:     v6 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u32 <- ByVal(v0): u32
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u32 <- ByVal(v1): u32
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): u32 <- ByVal(v2): u32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _3)
// CHECK-NEXT:     brif.i32 v3, block3, block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = copy _1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v4): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v4
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = ShlUnchecked(copy _1, copy _3)
// CHECK-NEXT:     v7 = ishl.i32 v5, v6
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): u32 <- ByVal(v7): u32
// CHECK-NEXT: ; _6 = Sub(const 32_u32, copy _3)
// CHECK-NEXT:     v8 = iconst.i32 32
// CHECK-NEXT:     v9 = isub v8, v6  ; v8 = 32
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): u32 <- ByVal(v9): u32
// CHECK-NEXT: ; _5 = ShrUnchecked(copy _2, move _6)
// CHECK-NEXT:     v11 = ushr.i32 v10, v9
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): u32 <- ByVal(v11): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::disjoint_bitor::<u32>(move _4, move _5)
// CHECK-NEXT:     v12 = symbol_value.i16 gv0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v13 = call fn0(v7, v11, v12)
// CHECK-NEXT:     v14 -> v13
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v13): u32
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v14
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol bits
// CHECK-NEXT: function u0:8(i32, i32) -> i32 system_v {
// CHECK-NEXT: ; symbol bits
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u32                               4b 2                var=2
// CHECK-NEXT: ; ssa   _2    u32                               4b 2                var=3
// CHECK-NEXT: ; ssa   _3    u32                               4b 2                var=4
// CHECK-NEXT: ; ssa   _4    u32                               4b 2                var=5
// CHECK-NEXT: ; ssa   _5    u32                               4b 2                var=6
// CHECK-NEXT: ; ssa   _6    u32                               4b 2                var=7
// CHECK-NEXT: ; ssa   _7    u32                               4b 2                var=8
// CHECK-NEXT: ; ssa   _8    u32                               4b 2                var=9
// CHECK-NEXT: ; ssa   _9    u32                               4b 2                var=10
// CHECK-NEXT: ; ssa   _10   u32                               4b 2                var=11
// CHECK-NEXT: ; ssa   _11   u32                               4b 2                var=12
// CHECK-NEXT: ; ssa   _12   u32                               4b 2                var=13
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32, v1: i32):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v0
// CHECK-NEXT:     v9 -> v0
// CHECK-NEXT:     v14 -> v0
// CHECK-NEXT:     v19 -> v0
// CHECK-NEXT:     v25 -> v0
// CHECK-NEXT:     v20 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u32 <- ByVal(v0): u32
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u32 <- ByVal(v1): u32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = core::intrinsics::ctpop::<u32>(copy _1)
// CHECK-NEXT:     v3 = popcnt.i32 v2
// CHECK-NEXT:     v6 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): u32 <- ByVal(v3): u32
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = core::intrinsics::ctlz::<u32>(copy _1)
// CHECK-NEXT:     v5 = clz.i32 v4
// CHECK-NEXT:     v7 -> v5
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): u32 <- ByVal(v5): u32
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = Add(move _7, move _8)
// CHECK-NEXT:     v8 = iadd.i32 v6, v7
// CHECK-NEXT:     v11 -> v8
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): u32 <- ByVal(v8): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; _9 = core::intrinsics::cttz::<u32>(copy _1)
// CHECK-NEXT:     v10 = ctz.i32 v9
// CHECK-NEXT:     v12 -> v10
// CHECK-NEXT: ; write_cvalue: Var(_9, var10): u32 <- ByVal(v10): u32
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = Add(move _6, move _9)
// CHECK-NEXT:     v13 = iadd.i32 v11, v12
// CHECK-NEXT:     v16 -> v13
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): u32 <- ByVal(v13): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; _10 = core::intrinsics::bswap::<u32>(copy _1)
// CHECK-NEXT:     v15 = bswap.i32 v14
// CHECK-NEXT:     v17 -> v15
// CHECK-NEXT: ; write_cvalue: Var(_10, var11): u32 <- ByVal(v15): u32
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = Add(move _5, move _10)
// CHECK-NEXT:     v18 = iadd.i32 v16, v17
// CHECK-NEXT:     v22 -> v18
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): u32 <- ByVal(v18): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; _11 = core::intrinsics::rotate_left::<u32>(copy _1, move _2)
// CHECK-NEXT:     v21 = rotl.i32 v19, v20
// CHECK-NEXT:     v23 -> v21
// CHECK-NEXT: ; write_cvalue: Var(_11, var12): u32 <- ByVal(v21): u32
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = Add(move _4, move _11)
// CHECK-NEXT:     v24 = iadd.i32 v22, v23
// CHECK-NEXT:     v27 -> v24
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): u32 <- ByVal(v24): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; _12 = core::intrinsics::bitreverse::<u32>(move _1)
// CHECK-NEXT:     v26 = bitrev.i32 v25
// CHECK-NEXT:     v28 -> v26
// CHECK-NEXT: ; write_cvalue: Var(_12, var13): u32 <- ByVal(v26): u32
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Add(move _3, move _12)
// CHECK-NEXT:     v29 = iadd.i32 v27, v28
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v29): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v29
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:9() -> i8 system_v {
// CHECK-NEXT: ; symbol gbc_main
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _2    u32                               4b 2                var=3
// CHECK-NEXT: ; ssa   _3    &'{erased} (u16, u16)             2b 2                var=4
// CHECK-NEXT: ; stack _4    (u16, u16)                        4b 2                storage=ss0
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname2 ; alloc2
// CHECK-NEXT:     sig0 = (i32, i32) -> i32 system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:8 sig0 ; Instance { def: Item(DefId(0:3 ~ core_intrinsics[..]::bits)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:10 sig1 ; Instance { def: Item(DefId(0:4 ~ core_intrinsics[..]::saturating)), args: [] }
// CHECK-NEXT:     fn2 = colocated u0:4 sig2 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u16, u16] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = bits(const 1_u32, const 1_u32)
// CHECK-NEXT:     v0 = iconst.i32 1
// CHECK-NEXT:     v1 = iconst.i32 1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0(v0, v1)  ; v0 = 1, v1 = 1
// CHECK-NEXT:     v3 -> v2
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u32 <- ByVal(v2): u32
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _2)
// CHECK-NEXT:     v4 = iconst.i32 -2130706398
// CHECK-NEXT:     v5 = icmp.i32 eq v3, v4  ; v4 = -2130706398
// CHECK-NEXT:     brif v5, block3, block4
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = saturating(const u16::MAX, const 1_u16)
// CHECK-NEXT:     v6 = iconst.i16 -1
// CHECK-NEXT:     v7 = iconst.i16 1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u16, u16), layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is .., b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v8, v9 = call fn1(v6, v7)  ; v6 = -1, v7 = 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): (u16, u16) <- ByValPair(v8, v9): (u16, u16)
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v8, v10
// CHECK-NEXT:     v11 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v9, v11
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = const false
// CHECK-NEXT:     v12 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v12): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block6(v12)  ; v12 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = &_4
// CHECK-NEXT:     v13 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): &'{erased} (u16, u16) <- ByVal(v13): &'{erased} (u16, u16)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _1 = <(u16, u16) as core::cmp::PartialEq>::eq(move _3, const gbc_main::promoted[0])
// CHECK-NEXT:     v14 = symbol_value.i16 gv0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u16, u16), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u16, u16), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v15 = call fn2(v13, v14)
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v15): bool
// CHECK-NEXT:     jump block6(v15)
// CHECK-EMPTY:
// CHECK-NEXT: block6(v16: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v16): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v16
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol saturating
// CHECK-NEXT: function u0:10(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol saturating
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    (u16, u16)                        4b 2                var=(1, 2)
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) (u16, u16)
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u16
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u16                               2b 2                var=3
// CHECK-NEXT: ; ssa   _2    u16                               2b 2                var=4
// CHECK-NEXT: ; ssa   _3    u16                               2b 2                var=5
// CHECK-NEXT: ; ssa   _4    u16                               2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v9 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v10 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var3): u16 <- ByVal(v0): u16
// CHECK-NEXT: ; write_cvalue: Var(_2, var4): u16 <- ByVal(v1): u16
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::intrinsics::saturating_add::<u16>(copy _1, copy _2)
// CHECK-NEXT:     v4 = iconst.i16 0
// CHECK-NEXT:     v5 = iconst.i16 -1
// CHECK-NEXT:     v6 = iadd.i16 v2, v3
// CHECK-NEXT:     v7 = icmp ult v6, v2
// CHECK-NEXT:     v8 = select v7, v5, v6  ; v5 = -1
// CHECK-NEXT:     v16 -> v8
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): u16 <- ByVal(v8): u16
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::intrinsics::saturating_sub::<u16>(move _1, move _2)
// CHECK-NEXT:     v11 = iconst.i16 0
// CHECK-NEXT:     v12 = iconst.i16 -1
// CHECK-NEXT:     v13 = isub.i16 v9, v10
// CHECK-NEXT:     v14 = icmp ugt v13, v9
// CHECK-NEXT:     v15 = select v14, v11, v13  ; v11 = 0
// CHECK-NEXT:     v17 -> v15
// CHECK-NEXT: ; write_cvalue: Var(_4, var6): u16 <- ByVal(v15): u16
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = (move _3, move _4)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u16 <- ByVal(v16): u16
// CHECK-NEXT: ; write_cvalue: Var(_0, var2): u16 <- ByVal(v17): u16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v16, v17
// CHECK-NEXT: }
