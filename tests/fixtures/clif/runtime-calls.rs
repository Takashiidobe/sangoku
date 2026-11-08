#![no_std]

#[unsafe(no_mangle)]
pub unsafe fn memory(dst: *mut u8, src: *const u8, n: usize) {
    unsafe { core::ptr::copy_nonoverlapping(src, dst, n); core::ptr::copy(dst, dst.add(1), n); core::ptr::write_bytes(dst, 7, n); }
}
#[unsafe(no_mangle)]
pub fn divide(x: u32, y: u32) -> u32 { x / y }
#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut bytes = [0; 5];
    unsafe { memory(bytes.as_mut_ptr(), [1, 2, 3].as_ptr(), 3); }
    (bytes == [7, 7, 7, 3, 0] && divide(1000, 7) == 142) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_div_by_zeroCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:0(i16) system_v {
// CHECK-NEXT: ; symbol _RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_div_by_zeroCs7YLftNnbcrY_13runtime_calls
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
// CHECK-NEXT: ; symbol divide
// CHECK-NEXT: function u0:1(i32, i32) -> i32 system_v {
// CHECK-NEXT: ; symbol divide
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
// CHECK-EMPTY:
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc13
// CHECK-NEXT:     sig0 = (i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:0 sig0 ; "_RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_div_by_zeroCs7YLftNnbcrY_13runtime_calls"
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32, v1: i32):
// CHECK-NEXT:     v6 -> v0
// CHECK-NEXT:     v2 -> v1
// CHECK-NEXT:     v7 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u32 <- ByVal(v0): u32
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u32 <- ByVal(v1): u32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = Eq(copy _2, const 0_u32)
// CHECK-NEXT:     v3 = iconst.i32 0
// CHECK-NEXT:     v4 = icmp.i32 eq v2, v3  ; v3 = 0
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): bool <- ByVal(v4): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(!move _3, "attempt to divide `{}` by zero", copy _1)
// CHECK-NEXT:     brif v4, block3, block2
// CHECK-EMPTY:
// CHECK-NEXT: block3 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v5 = symbol_value.i16 gv0
// CHECK-NEXT: ; panic _RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_div_by_zeroCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn0(v5)
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Div(copy _1, copy _2)
// CHECK-NEXT:     v8 = udiv.i32 v6, v7
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v8): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v8
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:2() -> i8 system_v {
// CHECK-NEXT: ; symbol gbc_main
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    [u8; 5_usize]                     5b 1                storage=ss0
// CHECK-NEXT: ; zst   _2    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _3    *mut u8                           2b 2                var=2
// CHECK-NEXT: ; stack _4    &'{erased} mut [u8]               4b 2                storage=ss1
// CHECK-NEXT: ; ssa   _5    &'{erased} mut [u8; 5_usize]      2b 2                var=3
// CHECK-NEXT: ; ssa   _6    *const u8                         2b 2                var=4
// CHECK-NEXT: ; stack _7    &'{erased} [u8]                   4b 2                storage=ss2
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=5
// CHECK-NEXT: ; ssa   _9    bool                              1b 1                var=6
// CHECK-NEXT: ; ssa   _10   &'{erased} [u8; 5_usize]          2b 2                var=7
// CHECK-NEXT: ; ssa   _11   u32                               4b 2                var=8
// CHECK-NEXT: ; ssa   _12   *mut [u8]                         4b 2                var=(9, 10)
// CHECK-NEXT: ; ssa   _13   *const [u8]                       4b 2                var=(11, 12)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 5
// CHECK-NEXT:     ss1 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc2
// CHECK-NEXT:     gv1 = symbol colocated userextname2 ; alloc4
// CHECK-NEXT:     sig0 = (i16, i32, i16) -> i16 system_v
// CHECK-NEXT:     sig1 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16) -> i32 system_v
// CHECK-NEXT:     sig3 = (i32, i32) -> i32 system_v
// CHECK-NEXT:     fn0 = %Memset sig0
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(0:3 ~ runtime_calls[..]::memory)), args: [] }
// CHECK-NEXT:     fn2 = u0:4 sig2 ; "memcmp"
// CHECK-NEXT:     fn3 = colocated u0:1 sig3 ; Instance { def: Item(DefId(0:4 ~ runtime_calls[..]::divide)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = [const 0_u8; 5]
// CHECK-NEXT:     v0 = iconst.i8 0
// CHECK-NEXT:     v1 = iconst.i16 5
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     v3 = uextend.i32 v0  ; v0 = 0
// CHECK-NEXT:     v4 = call fn0(v2, v3, v1)  ; v1 = 5
// CHECK-NEXT: ; _5 = &mut _1
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_5, var3): &'{erased} mut [u8; 5_usize] <- ByVal(v5): &'{erased} mut [u8; 5_usize]
// CHECK-NEXT: ; _4 = copy _5 as &mut [u8] (PointerCoercion(Unsize, Implicit))
// CHECK-NEXT:     v6 = iconst.i16 5
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} mut [u8] <- ByValPair(v5, v6): &'{erased} mut [u8]
// CHECK-NEXT:     v7 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v5, v7
// CHECK-NEXT:     v8 = stack_addr.i16 ss1+2
// CHECK-NEXT:     store notrap v6, v8  ; v6 = 5
// CHECK-NEXT: ; _12 = &raw mut (*_4)
// CHECK-NEXT:     v9 = stack_addr.i16 ss1
// CHECK-NEXT:     v10 = load.i16 notrap v9
// CHECK-NEXT:     v11 = stack_addr.i16 ss1+2
// CHECK-NEXT:     v12 = load.i16 notrap v11
// CHECK-NEXT: ; write_cvalue: VarPair(_12, var9, var10): *mut [u8] <- ByValPair(v10, v12): *mut [u8]
// CHECK-NEXT: ; _3 = copy _12 as *mut u8 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var2): *mut u8 <- ByVal(v10): *mut u8
// CHECK-NEXT: ; _7 = const gbc_main::promoted[1] as &[u8] (PointerCoercion(Unsize, Implicit))
// CHECK-NEXT:     v13 = symbol_value.i16 gv0
// CHECK-NEXT:     v14 = iconst.i16 3
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): &'{erased} [u8] <- ByValPair(v13, v14): &'{erased} [u8]
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v13, v15
// CHECK-NEXT:     v16 = stack_addr.i16 ss2+2
// CHECK-NEXT:     store notrap v14, v16  ; v14 = 3
// CHECK-NEXT: ; _13 = &raw const (*_7)
// CHECK-NEXT:     v17 = stack_addr.i16 ss2
// CHECK-NEXT:     v18 = load.i16 notrap v17
// CHECK-NEXT:     v19 = stack_addr.i16 ss2+2
// CHECK-NEXT:     v20 = load.i16 notrap v19
// CHECK-NEXT: ; write_cvalue: VarPair(_13, var11, var12): *const [u8] <- ByValPair(v18, v20): *const [u8]
// CHECK-NEXT: ; _6 = copy _13 as *const u8 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_6, var4): *const u8 <- ByVal(v18): *const u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = memory(move _3, move _6, const 3_usize)
// CHECK-NEXT:     v21 = iconst.i16 3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *mut u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *const u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn1(v10, v18, v21)  ; v21 = 3
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = &_1
// CHECK-NEXT:     v22 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_10, var7): &'{erased} [u8; 5_usize] <- ByVal(v22): &'{erased} [u8; 5_usize]
// CHECK-NEXT: ;
// CHECK-NEXT: ; _9 = core::intrinsics::raw_eq::<[u8; 5]>(move _10, const gbc_main::promoted[0])
// CHECK-NEXT:     v23 = symbol_value.i16 gv1
// CHECK-NEXT:     v24 = iconst.i16 5
// CHECK-NEXT: ; lib_call memcmp
// CHECK-NEXT:     v25 = call fn2(v22, v23, v24)  ; v24 = 5
// CHECK-NEXT:     v26 = iconst.i32 0
// CHECK-NEXT:     v27 = icmp eq v25, v26  ; v26 = 0
// CHECK-NEXT:     v36 -> v27
// CHECK-NEXT: ; write_cvalue: Var(_9, var6): bool <- ByVal(v27): bool
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _11 = divide(const 1000_u32, const 7_u32)
// CHECK-NEXT:     v28 = iconst.i32 1000
// CHECK-NEXT:     v29 = iconst.i32 7
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v30 = call fn3(v28, v29)  ; v28 = 1000, v29 = 7
// CHECK-NEXT:     v32 -> v30
// CHECK-NEXT: ; write_cvalue: Var(_11, var8): u32 <- ByVal(v30): u32
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = const false
// CHECK-NEXT:     v31 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_8, var5): bool <- ByVal(v31): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block6(v31)  ; v31 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = Eq(move _11, const 142_u32)
// CHECK-NEXT:     v33 = iconst.i32 142
// CHECK-NEXT:     v34 = icmp.i32 eq v32, v33  ; v33 = 142
// CHECK-NEXT: ; write_cvalue: Var(_8, var5): bool <- ByVal(v34): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block6(v34)
// CHECK-EMPTY:
// CHECK-NEXT: block6(v35: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _8 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v35): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v35
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _9)
// CHECK-NEXT:     brif.i8 v36, block3, block4
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol memory
// CHECK-NEXT: function u0:3(i16, i16, i16) system_v {
// CHECK-NEXT: ; symbol memory
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const u8
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut u8                           2b 2                var=1
// CHECK-NEXT: ; ssa   _2    *const u8                         2b 2                var=2
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=3
// CHECK-NEXT: ; zst   _4    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _5    *const u8                         2b 2                var=4
// CHECK-NEXT: ; ssa   _6    *mut u8                           2b 2                var=5
// CHECK-NEXT: ; zst   _7    ()                                0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16, i16, i16) -> i16 system_v
// CHECK-NEXT:     sig1 = (i16, i16, i16) -> i16 system_v
// CHECK-NEXT:     sig2 = (i16, i32, i16) -> i16 system_v
// CHECK-NEXT:     fn0 = %Memcpy sig0
// CHECK-NEXT:     fn1 = %Memmove sig1
// CHECK-NEXT:     fn2 = %Memset sig2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     v3 -> v0
// CHECK-NEXT:     v12 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     v5 -> v2
// CHECK-NEXT:     v14 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut u8 <- ByVal(v0): *mut u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): *const u8 <- ByVal(v1): *const u8
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v2): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; copy_nonoverlapping(dst = copy _1, src = copy _2, count = copy _3)
// CHECK-NEXT:     v6 = call fn0(v3, v4, v5)
// CHECK-NEXT: ; _5 = copy _1 as *const u8 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): *const u8 <- ByVal(v3): *const u8
// CHECK-NEXT: ; _6 = Offset(copy _1, const 1_usize)
// CHECK-NEXT:     v7 = iconst.i16 1
// CHECK-NEXT:     v8 = iconst.i16 1
// CHECK-NEXT:     v9 = imul v7, v8  ; v7 = 1, v8 = 1
// CHECK-NEXT:     v10 = iadd.i16 v3, v9
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): *mut u8 <- ByVal(v10): *mut u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::intrinsics::copy::<u8>(move _5, move _6, copy _3)
// CHECK-NEXT:     v11 = call fn1(v10, v3, v5)
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = core::intrinsics::write_bytes::<u8>(move _1, const 7_u8, move _3)
// CHECK-NEXT:     v13 = iconst.i8 7
// CHECK-NEXT:     v15 = uextend.i32 v13  ; v13 = 7
// CHECK-NEXT:     v16 = call fn2(v12, v15, v14)
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
