#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn scalar(x: u16, _: ()) -> u16 { x.wrapping_add(1) }

#[inline(never)]
#[unsafe(no_mangle)]
pub fn pair(x: (u8, u16)) -> (u8, u16) { (x.0.wrapping_add(1), x.1.wrapping_add(2)) }

#[inline(never)]
#[unsafe(no_mangle)]
pub fn aggregate(mut x: [u16; 5]) -> [u16; 5] { x[4] = x[0].wrapping_add(x[4]); x }

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let a = scalar(10, ());
    let b = pair((3, 20));
    let c = aggregate([1, 2, 3, 4, 5]);
    (a == 11 && b == (4, 22) && c == [1, 2, 3, 4, 6]) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCs1vI5IyamSN6_14abi_pass_modes
// CHECK-NEXT: function u0:0(i16, i16, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCs1vI5IyamSN6_14abi_pass_modes
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               !
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg  ???      = v2       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    usize                             2b 2                var=1
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=2
// CHECK-NEXT: ; zst   _3    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _4    (&'{erased} usize, &'{erased} usize)    4b 2                var=(3, 4)
// CHECK-NEXT: ; stack _5    [core::fmt::rt::Argument<'{erased}>; 2_usize]    8b 2                storage=ss0
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 8, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): usize <- ByVal(v0): usize
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::intrinsics::abort_immediate()
// CHECK-NEXT:     trap user2
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleThtENtNtB7_3cmp9PartialEq2eqCs1vI5IyamSN6_14abi_pass_modes
// CHECK-NEXT: function u0:1(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleThtENtNtB7_3cmp9PartialEq2eqCs1vI5IyamSN6_14abi_pass_modes
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (u8, u16)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (u8, u16)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} (u8, u16)              2b 2                storage=ss0
// CHECK-NEXT: ; stack _2    &'{erased} (u8, u16)              2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _4    &'{erased} u8                     2b 2                var=3
// CHECK-NEXT: ; ssa   _5    &'{erased} u8                     2b 2                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} u16                    2b 2                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} u16                    2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:2 sig0 ; Instance { def: Item(DefId(1:29972 ~ core[..]::cmp::impls::{impl#22}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(1:29975 ~ core[..]::cmp::impls::{impl#23}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (u8, u16) <- ByVal(v0): &'{erased} (u8, u16)
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (u8, u16) <- ByVal(v1): &'{erased} (u8, u16)
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
// CHECK-NEXT: ; symbol _RNvXsk_NtNtCs40R3J0iFVxM_4core3cmp5implshNtB7_9PartialEq2eqCs1vI5IyamSN6_14abi_pass_modes
// CHECK-NEXT: function u0:2(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsk_NtNtCs40R3J0iFVxM_4core3cmp5implshNtB7_9PartialEq2eqCs1vI5IyamSN6_14abi_pass_modes
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
// CHECK-NEXT: ; symbol _RNvXsl_NtNtCs40R3J0iFVxM_4core3cmp5implstNtB7_9PartialEq2eqCs1vI5IyamSN6_14abi_pass_modes
// CHECK-NEXT: function u0:3(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsl_NtNtCs40R3J0iFVxM_4core3cmp5implstNtB7_9PartialEq2eqCs1vI5IyamSN6_14abi_pass_modes
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
// CHECK-NEXT: ; symbol aggregate
// CHECK-NEXT: function u0:4(i16 sret, i16) system_v {
// CHECK-NEXT: ; symbol aggregate
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(10 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } [u16; 5_usize]
// CHECK-NEXT: ; arg   _1      = v1       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(10 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } [u16; 5_usize]
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; reuse _1    [u16; 5_usize]                   10b 2                storage=v1
// CHECK-NEXT: ; ssa   _2    u16                               2b 2                var=1
// CHECK-NEXT: ; ssa   _3    u16                               2b 2                var=2
// CHECK-NEXT: ; ssa   _4    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _5    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _6    u16                               2b 2                var=5
// CHECK-NEXT: ; ssa   _7    usize                             2b 2                var=6
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=7
// CHECK-NEXT: ; ssa   _9    usize                             2b 2                var=8
// CHECK-NEXT: ; ssa   _10   bool                              1b 1                var=9
// CHECK-EMPTY:
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc6
// CHECK-NEXT:     gv1 = symbol colocated userextname2 ; alloc7
// CHECK-NEXT:     gv2 = symbol colocated userextname3 ; alloc8
// CHECK-NEXT:     sig0 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig1 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:0 sig0 ; "_RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCs1vI5IyamSN6_14abi_pass_modes"
// CHECK-NEXT:     fn1 = colocated u0:0 sig1 ; "_RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCs1vI5IyamSN6_14abi_pass_modes"
// CHECK-NEXT:     fn2 = colocated u0:0 sig2 ; "_RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCs1vI5IyamSN6_14abi_pass_modes"
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = const 0_usize
// CHECK-NEXT:     v2 = iconst.i16 0
// CHECK-NEXT:     v6 -> v2
// CHECK-NEXT:     v8 -> v2
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): usize <- ByVal(v2): usize
// CHECK-NEXT: ; _5 = Lt(copy _4, const 5_usize)
// CHECK-NEXT:     v3 = iconst.i16 5
// CHECK-NEXT:     v4 = icmp ult v2, v3  ; v2 = 0, v3 = 5
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): bool <- ByVal(v4): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(move _5, "index out of bounds: the length is {} but the index is {}", const 5_usize, copy _4)
// CHECK-NEXT:     brif v4, block2, block5
// CHECK-EMPTY:
// CHECK-NEXT: block5 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v5 = iconst.i16 5
// CHECK-NEXT:     v7 = symbol_value.i16 gv0
// CHECK-NEXT: ; panic _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCs1vI5IyamSN6_14abi_pass_modes
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn0(v6, v5, v7)  ; v6 = 0, v5 = 5
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy _1[_4]
// CHECK-NEXT:     v9 = iconst.i16 2
// CHECK-NEXT:     v10 = imul.i16 v8, v9  ; v8 = 0, v9 = 2
// CHECK-NEXT:     v11 = iadd.i16 v1, v10
// CHECK-NEXT: ; write_cvalue: Var(_3, var2): u16 <- ByRef(Pointer { base: Addr(v11), offset: Offset32(0) }, None): u16
// CHECK-NEXT:     v12 = load.i16 notrap v11
// CHECK-NEXT:     v24 -> v12
// CHECK-NEXT: ; _7 = const 4_usize
// CHECK-NEXT:     v13 = iconst.i16 4
// CHECK-NEXT:     v17 -> v13
// CHECK-NEXT:     v19 -> v13
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): usize <- ByVal(v13): usize
// CHECK-NEXT: ; _8 = Lt(copy _7, const 5_usize)
// CHECK-NEXT:     v14 = iconst.i16 5
// CHECK-NEXT:     v15 = icmp ult v13, v14  ; v13 = 4, v14 = 5
// CHECK-NEXT: ; write_cvalue: Var(_8, var7): bool <- ByVal(v15): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(move _8, "index out of bounds: the length is {} but the index is {}", const 5_usize, copy _7)
// CHECK-NEXT:     brif v15, block3, block6
// CHECK-EMPTY:
// CHECK-NEXT: block6 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v16 = iconst.i16 5
// CHECK-NEXT:     v18 = symbol_value.i16 gv1
// CHECK-NEXT: ; panic _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCs1vI5IyamSN6_14abi_pass_modes
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn1(v17, v16, v18)  ; v17 = 4, v16 = 5
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy _1[_7]
// CHECK-NEXT:     v20 = iconst.i16 2
// CHECK-NEXT:     v21 = imul.i16 v19, v20  ; v19 = 4, v20 = 2
// CHECK-NEXT:     v22 = iadd.i16 v1, v21
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): u16 <- ByRef(Pointer { base: Addr(v22), offset: Offset32(0) }, None): u16
// CHECK-NEXT:     v23 = load.i16 notrap v22
// CHECK-NEXT: ; _2 = Add(copy _3, copy _6)
// CHECK-NEXT:     v25 = iadd.i16 v24, v23
// CHECK-NEXT:     v36 -> v25
// CHECK-NEXT: ; write_cvalue: Var(_2, var1): u16 <- ByVal(v25): u16
// CHECK-NEXT: ; _9 = const 4_usize
// CHECK-NEXT:     v26 = iconst.i16 4
// CHECK-NEXT:     v30 -> v26
// CHECK-NEXT:     v32 -> v26
// CHECK-NEXT: ; write_cvalue: Var(_9, var8): usize <- ByVal(v26): usize
// CHECK-NEXT: ; _10 = Lt(copy _9, const 5_usize)
// CHECK-NEXT:     v27 = iconst.i16 5
// CHECK-NEXT:     v28 = icmp ult v26, v27  ; v26 = 4, v27 = 5
// CHECK-NEXT: ; write_cvalue: Var(_10, var9): bool <- ByVal(v28): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(move _10, "index out of bounds: the length is {} but the index is {}", const 5_usize, copy _9)
// CHECK-NEXT:     brif v28, block4, block7
// CHECK-EMPTY:
// CHECK-NEXT: block7 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v29 = iconst.i16 5
// CHECK-NEXT:     v31 = symbol_value.i16 gv2
// CHECK-NEXT: ; panic _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCs1vI5IyamSN6_14abi_pass_modes
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn2(v30, v29, v31)  ; v30 = 4, v29 = 5
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1[_9] = move _2
// CHECK-NEXT:     v33 = iconst.i16 2
// CHECK-NEXT:     v34 = imul.i16 v32, v33  ; v32 = 4, v33 = 2
// CHECK-NEXT:     v35 = iadd.i16 v1, v34
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v35), offset: Offset32(0) }, None): u16 <- ByVal(v36): u16
// CHECK-NEXT:     store.i16 notrap v36, v35
// CHECK-NEXT: ; _0 = copy _1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): [u16; 5_usize] <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): [u16; 5_usize]
// CHECK-NEXT:     v37 = load.i16 notrap aligned v1
// CHECK-NEXT:     v38 = load.i16 notrap aligned v1+2
// CHECK-NEXT:     v39 = load.i16 notrap aligned v1+4
// CHECK-NEXT:     v40 = load.i16 notrap aligned v1+6
// CHECK-NEXT:     v41 = load.i16 notrap aligned v1+8
// CHECK-NEXT:     store notrap aligned v37, v0
// CHECK-NEXT:     store notrap aligned v38, v0+2
// CHECK-NEXT:     store notrap aligned v39, v0+4
// CHECK-NEXT:     store notrap aligned v40, v0+6
// CHECK-NEXT:     store notrap aligned v41, v0+8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
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
// CHECK-NEXT: ; ssa   _1    u16                               2b 2                var=2
// CHECK-NEXT: ; stack _2    (u8, u16)                         4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _3    (u8, u16)                         4b 2                var=(3, 4)
// CHECK-NEXT: ; stack _4    [u16; 5_usize]                   10b 2                storage=ss1
// CHECK-NEXT: ; stack _5    [u16; 5_usize]                   10b 2                storage=ss2
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=5
// CHECK-NEXT: ; ssa   _7    bool                              1b 1                var=6
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=7
// CHECK-NEXT: ; ssa   _9    &'{erased} (u8, u16)              2b 2                var=8
// CHECK-NEXT: ; ssa   _10   &'{erased} (u8, u16)              2b 2                var=9
// CHECK-NEXT: ; ssa   _11   &'{erased} [u16; 5_usize]         2b 2                var=10
// CHECK-NEXT: ; ssa   _12   &'{erased} [u16; 5_usize]         2b 2                var=11
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 10, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname3 ; alloc2
// CHECK-NEXT:     gv1 = symbol colocated userextname5 ; alloc4
// CHECK-NEXT:     sig0 = (i16) -> i16 system_v
// CHECK-NEXT:     sig1 = (i8, i16) -> i8, i16 system_v
// CHECK-NEXT:     sig2 = (i16 sret, i16) system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig4 = (i16, i16, i16) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:7 sig0 ; Instance { def: Item(DefId(0:3 ~ abi_pass_modes[..]::scalar)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:6 sig1 ; Instance { def: Item(DefId(0:4 ~ abi_pass_modes[..]::pair)), args: [] }
// CHECK-NEXT:     fn2 = colocated u0:4 sig2 ; Instance { def: Item(DefId(0:5 ~ abi_pass_modes[..]::aggregate)), args: [] }
// CHECK-NEXT:     fn3 = colocated u0:1 sig3 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [u8, u16] }
// CHECK-NEXT:     fn4 = u0:8 sig4 ; "memcmp"
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _1 = scalar(const 10_u16, const ())
// CHECK-NEXT:     v0 = iconst.i16 10
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }], ret: ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v1 = call fn0(v0)  ; v0 = 10
// CHECK-NEXT:     v40 -> v1
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u16 <- ByVal(v1): u16
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (const 3_u8, const 20_u16)
// CHECK-NEXT:     v2 = iconst.i8 3
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): u8 <- ByVal(v2): u8
// CHECK-NEXT:     v3 = iconst.i16 20
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): u16 <- ByVal(v3): u16
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = pair(move _3)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: (u8, u16), layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u8 is .., b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (u8, u16), layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u8 is .., b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4, v5 = call fn1(v2, v3)  ; v2 = 3, v3 = 20
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): (u8, u16) <- ByValPair(v4, v5): (u8, u16)
// CHECK-NEXT:     v6 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v4, v6
// CHECK-NEXT:     v7 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v5, v7
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = [const 1_u16, const 2_u16, const 3_u16, const 4_u16, const 5_u16]
// CHECK-NEXT:     v8 = iconst.i16 1
// CHECK-NEXT:     v9 = iconst.i16 0
// CHECK-NEXT:     v10 = iconst.i16 2
// CHECK-NEXT:     v11 = imul v9, v10  ; v9 = 0, v10 = 2
// CHECK-NEXT:     v12 = stack_addr.i16 ss2
// CHECK-NEXT:     v13 = iadd v12, v11
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v13), offset: Offset32(0) }, None): u16 <- ByVal(v8): u16
// CHECK-NEXT:     store notrap v8, v13  ; v8 = 1
// CHECK-NEXT:     v14 = iconst.i16 2
// CHECK-NEXT:     v15 = iconst.i16 1
// CHECK-NEXT:     v16 = iconst.i16 2
// CHECK-NEXT:     v17 = imul v15, v16  ; v15 = 1, v16 = 2
// CHECK-NEXT:     v18 = stack_addr.i16 ss2
// CHECK-NEXT:     v19 = iadd v18, v17
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v19), offset: Offset32(0) }, None): u16 <- ByVal(v14): u16
// CHECK-NEXT:     store notrap v14, v19  ; v14 = 2
// CHECK-NEXT:     v20 = iconst.i16 3
// CHECK-NEXT:     v21 = iconst.i16 2
// CHECK-NEXT:     v22 = iconst.i16 2
// CHECK-NEXT:     v23 = imul v21, v22  ; v21 = 2, v22 = 2
// CHECK-NEXT:     v24 = stack_addr.i16 ss2
// CHECK-NEXT:     v25 = iadd v24, v23
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v25), offset: Offset32(0) }, None): u16 <- ByVal(v20): u16
// CHECK-NEXT:     store notrap v20, v25  ; v20 = 3
// CHECK-NEXT:     v26 = iconst.i16 4
// CHECK-NEXT:     v27 = iconst.i16 3
// CHECK-NEXT:     v28 = iconst.i16 2
// CHECK-NEXT:     v29 = imul v27, v28  ; v27 = 3, v28 = 2
// CHECK-NEXT:     v30 = stack_addr.i16 ss2
// CHECK-NEXT:     v31 = iadd v30, v29
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v31), offset: Offset32(0) }, None): u16 <- ByVal(v26): u16
// CHECK-NEXT:     store notrap v26, v31  ; v26 = 4
// CHECK-NEXT:     v32 = iconst.i16 5
// CHECK-NEXT:     v33 = iconst.i16 4
// CHECK-NEXT:     v34 = iconst.i16 2
// CHECK-NEXT:     v35 = imul v33, v34  ; v33 = 4, v34 = 2
// CHECK-NEXT:     v36 = stack_addr.i16 ss2
// CHECK-NEXT:     v37 = iadd v36, v35
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v37), offset: Offset32(0) }, None): u16 <- ByVal(v32): u16
// CHECK-NEXT:     store notrap v32, v37  ; v32 = 5
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = aggregate(move _5)
// CHECK-NEXT:     v38 = stack_addr.i16 ss1
// CHECK-NEXT:     v39 = stack_addr.i16 ss2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: [u16; 5], layout: Layout { size: Size(10 bytes), align: Align(2 bytes), backend_repr: Memory { sized: true }, fields: Array { stride: Size(2 bytes), count: 5 }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775559 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(10 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }], ret: ArgAbi { layout: TyAndLayout { ty: [u16; 5], layout: Layout { size: Size(10 bytes), align: Align(2 bytes), backend_repr: Memory { sized: true }, fields: Array { stride: Size(2 bytes), count: 5 }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775559 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(10 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v38, v39)
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _7 = Eq(copy _1, const 11_u16)
// CHECK-NEXT:     v41 = iconst.i16 11
// CHECK-NEXT:     v42 = icmp.i16 eq v40, v41  ; v41 = 11
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): bool <- ByVal(v42): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _7)
// CHECK-NEXT:     brif v42, block5, block8
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = &_2
// CHECK-NEXT:     v43 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_9, var8): &'{erased} (u8, u16) <- ByVal(v43): &'{erased} (u8, u16)
// CHECK-NEXT: ; _10 = const gbc_main::promoted[1]
// CHECK-NEXT:     v44 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_10, var9): &'{erased} (u8, u16) <- ByVal(v44): &'{erased} (u8, u16)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = <(u8, u16) as core::cmp::PartialEq>::eq(move _9, move _10)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(u8, u16), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(u8, u16), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v45 = call fn3(v43, v44)
// CHECK-NEXT:     v46 -> v45
// CHECK-NEXT: ; write_cvalue: Var(_8, var7): bool <- ByVal(v45): bool
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _8)
// CHECK-NEXT:     brif.i8 v46, block7, block8
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _11 = &_4
// CHECK-NEXT:     v47 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_11, var10): &'{erased} [u16; 5_usize] <- ByVal(v47): &'{erased} [u16; 5_usize]
// CHECK-NEXT: ; _12 = const gbc_main::promoted[0]
// CHECK-NEXT:     v48 = symbol_value.i16 gv1
// CHECK-NEXT: ; write_cvalue: Var(_12, var11): &'{erased} [u16; 5_usize] <- ByVal(v48): &'{erased} [u16; 5_usize]
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::intrinsics::raw_eq::<[u16; 5]>(copy _11, copy _12)
// CHECK-NEXT:     v49 = iconst.i16 10
// CHECK-NEXT: ; lib_call memcmp
// CHECK-NEXT:     v50 = call fn4(v47, v48, v49)  ; v49 = 10
// CHECK-NEXT:     v51 = iconst.i32 0
// CHECK-NEXT:     v52 = icmp eq v50, v51  ; v51 = 0
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): bool <- ByVal(v52): bool
// CHECK-NEXT:     jump block9(v52)
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = const false
// CHECK-NEXT:     v53 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): bool <- ByVal(v53): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block9(v53)  ; v53 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block9(v54: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _6 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v54): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v54
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol pair
// CHECK-NEXT: function u0:6(i8, i16) -> i8, i16 system_v {
// CHECK-NEXT: ; symbol pair
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    (u8, u16)                         4b 2                var=(1, 2)
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) (u8, u16)
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) (u8, u16)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    (u8, u16)                         4b 2                var=(3, 4)
// CHECK-NEXT: ; ssa   _2    u8                                1b 1                var=5
// CHECK-NEXT: ; ssa   _3    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _4    u16                               2b 2                var=7
// CHECK-NEXT: ; ssa   _5    u16                               2b 2                var=8
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i8, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v5 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: VarPair(_1, var3, var4): (u8, u16) <- ByValPair(v0, v1): (u8, u16)
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (_1.0: u8)
// CHECK-NEXT: ; write_cvalue: Var(_3, var6): u8 <- ByVal(v2): u8
// CHECK-NEXT: ; _2 = Add(copy _3, const 1_u8)
// CHECK-NEXT:     v3 = iconst.i8 1
// CHECK-NEXT:     v4 = iadd.i8 v2, v3  ; v3 = 1
// CHECK-NEXT: ; write_cvalue: Var(_2, var5): u8 <- ByVal(v4): u8
// CHECK-NEXT: ; _5 = copy (_1.1: u16)
// CHECK-NEXT: ; write_cvalue: Var(_5, var8): u16 <- ByVal(v5): u16
// CHECK-NEXT: ; _4 = Add(copy _5, const 2_u16)
// CHECK-NEXT:     v6 = iconst.i16 2
// CHECK-NEXT:     v7 = iadd.i16 v5, v6  ; v6 = 2
// CHECK-NEXT: ; write_cvalue: Var(_4, var7): u16 <- ByVal(v7): u16
// CHECK-NEXT: ; _0 = (move _2, move _4)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v4): u8
// CHECK-NEXT: ; write_cvalue: Var(_0, var2): u16 <- ByVal(v7): u16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4, v7
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol scalar
// CHECK-NEXT: function u0:7(i16) -> i16 system_v {
// CHECK-NEXT: ; symbol scalar
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u16                               2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u16
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u16
// CHECK-NEXT: ; arg   _2      -          Ignore                               ()
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u16                               2b 2                var=2
// CHECK-NEXT: ; zst   _2    ()                                0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u16 <- ByVal(v0): u16
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Add(copy _1, const 1_u16)
// CHECK-NEXT:     v2 = iconst.i16 1
// CHECK-NEXT:     v3 = iadd.i16 v1, v2  ; v2 = 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u16 <- ByVal(v3): u16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v3
// CHECK-NEXT: }
