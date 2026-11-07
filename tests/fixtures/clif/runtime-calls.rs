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
// CHECK-NEXT: ; symbol _RINvMNtNtCs40R3J0iFVxM_4core3ptr9const_ptrPu4castuECs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:0(i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RINvMNtNtCs40R3J0iFVxM_4core3ptr9const_ptrPu4castuECs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    *const ()                         2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *const () <- ByVal(v0): *const ()
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = copy _1 as *const U (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): *const () <- ByVal(v1): *const ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3mem7size_ofhECs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:1() -> i16 system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3mem7size_ofhECs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    usize                             2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const <T as core::mem::SizedTypeProperties>::SIZE
// CHECK-NEXT:     v0 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v0): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v0  ; v0 = 1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3mem7size_ofjECs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:2() -> i16 system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3mem7size_ofjECs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    usize                             2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const <T as core::mem::SizedTypeProperties>::SIZE
// CHECK-NEXT:     v0 = iconst.i16 2
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v0): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v0  ; v0 = 2
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3mem8align_ofhECs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:3() -> i16 system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3mem8align_ofhECs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    usize                             2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const <T as core::mem::SizedTypeProperties>::ALIGN
// CHECK-NEXT:     v0 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v0): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v0  ; v0 = 1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3ptr11write_byteshECs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:4(i16, i8, i16) system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3ptr11write_byteshECs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut u8                           2b 2                var=1
// CHECK-NEXT: ; ssa   _2    u8                                1b 1                var=2
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; zst   _5    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _6    *const ()                         2b 2                var=5
// CHECK-NEXT: ; ssa   _7    usize                             2b 2                var=6
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=7
// CHECK-EMPTY:
// CHECK-NEXT:     gv0 = symbol colocated userextname2 ; alloc15
// CHECK-NEXT:     sig0 = () -> i8 uext system_v
// CHECK-NEXT:     sig1 = () -> i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16, i8 uext, i16) system_v
// CHECK-NEXT:     sig3 = (i16, i32, i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:29 sig0 ; Instance { def: Item(DefId(1:3622 ~ core[..]::ub_checks::check_language_ub)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(1:2951 ~ core[..]::mem::align_of)), args: [u8] }
// CHECK-NEXT:     fn2 = colocated u0:33 sig2 ; Instance { def: Item(DefId(1:29534 ~ core[..]::ptr::write_bytes::precondition_check)), args: [] }
// CHECK-NEXT:     fn3 = %Memset sig3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i8, v2: i16):
// CHECK-NEXT:     v5 -> v0
// CHECK-NEXT:     v11 -> v0
// CHECK-NEXT:     v15 -> v0
// CHECK-NEXT:     v16 -> v1
// CHECK-NEXT:     v8 -> v2
// CHECK-NEXT:     v17 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut u8 <- ByVal(v0): *mut u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): u8 <- ByVal(v1): u8
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v2): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::ub_checks::check_language_ub()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v3 = call fn0()
// CHECK-NEXT:     v4 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v3): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _4)
// CHECK-NEXT:     brif.i8 v4, block3, block8
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy _1 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): *const () <- ByVal(v5): *const ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = core::mem::align_of::<T>()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v6 = call fn1()
// CHECK-NEXT:     v12 -> v6
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): usize <- ByVal(v6): usize
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(const <T as core::mem::SizedTypeProperties>::IS_ZST)
// CHECK-NEXT:     v7 = iconst.i8 0
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = Eq(copy _3, const 0_usize)
// CHECK-NEXT:     v9 = iconst.i16 0
// CHECK-NEXT:     v10 = icmp.i16 eq v8, v9  ; v9 = 0
// CHECK-NEXT:     v13 -> v10
// CHECK-NEXT: ; write_cvalue: Var(_8, var7): bool <- ByVal(v10): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::ptr::write_bytes::precondition_check(move _6, move _7, move _8)
// CHECK-NEXT:     v14 = symbol_value.i16 gv0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v11, v12, v13, v14)
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::write_bytes::<T>(copy _1, copy _2, copy _3)
// CHECK-NEXT:     v18 = uextend.i32 v16
// CHECK-NEXT:     v19 = call fn3(v15, v18, v17)
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3ptr19copy_nonoverlappinghECs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:5(i16, i16, i16) system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3ptr19copy_nonoverlappinghECs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut u8
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const u8                         2b 2                var=1
// CHECK-NEXT: ; ssa   _2    *mut u8                           2b 2                var=2
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; zst   _5    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _6    *const ()                         2b 2                var=5
// CHECK-NEXT: ; ssa   _7    *mut ()                           2b 2                var=6
// CHECK-NEXT: ; ssa   _8    usize                             2b 2                var=7
// CHECK-NEXT: ; ssa   _9    usize                             2b 2                var=8
// CHECK-EMPTY:
// CHECK-NEXT:     gv0 = symbol colocated userextname3 ; alloc16
// CHECK-NEXT:     sig0 = () -> i8 uext system_v
// CHECK-NEXT:     sig1 = () -> i16 system_v
// CHECK-NEXT:     sig2 = () -> i16 system_v
// CHECK-NEXT:     sig3 = (i16, i16, i16, i16, i16, i16) system_v
// CHECK-NEXT:     sig4 = (i16, i16, i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:29 sig0 ; Instance { def: Item(DefId(1:3622 ~ core[..]::ub_checks::check_language_ub)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(1:2941 ~ core[..]::mem::size_of)), args: [u8] }
// CHECK-NEXT:     fn2 = colocated u0:3 sig2 ; Instance { def: Item(DefId(1:2951 ~ core[..]::mem::align_of)), args: [u8] }
// CHECK-NEXT:     fn3 = colocated u0:34 sig3 ; Instance { def: Item(DefId(1:29532 ~ core[..]::ptr::copy_nonoverlapping::precondition_check)), args: [] }
// CHECK-NEXT:     fn4 = %Memcpy sig4
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     v5 -> v0
// CHECK-NEXT:     v9 -> v0
// CHECK-NEXT:     v16 -> v0
// CHECK-NEXT:     v6 -> v1
// CHECK-NEXT:     v10 -> v1
// CHECK-NEXT:     v15 -> v1
// CHECK-NEXT:     v13 -> v2
// CHECK-NEXT:     v17 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *const u8 <- ByVal(v0): *const u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): *mut u8 <- ByVal(v1): *mut u8
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v2): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::ub_checks::check_language_ub()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v3 = call fn0()
// CHECK-NEXT:     v4 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v3): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _4)
// CHECK-NEXT:     brif.i8 v4, block3, block6
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy _1 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): *const () <- ByVal(v5): *const ()
// CHECK-NEXT: ; _7 = copy _2 as *mut () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): *mut () <- ByVal(v6): *mut ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = core::mem::size_of::<T>()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v7 = call fn1()
// CHECK-NEXT:     v11 -> v7
// CHECK-NEXT: ; write_cvalue: Var(_8, var7): usize <- ByVal(v7): usize
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _9 = core::mem::align_of::<T>()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v8 = call fn2()
// CHECK-NEXT:     v12 -> v8
// CHECK-NEXT: ; write_cvalue: Var(_9, var8): usize <- ByVal(v8): usize
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::ptr::copy_nonoverlapping::precondition_check(move _6, move _7, move _8, move _9, copy _3)
// CHECK-NEXT:     v14 = symbol_value.i16 gv0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *mut (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 5, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn3(v9, v10, v11, v12, v13, v14)
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; copy_nonoverlapping(dst = copy _2, src = copy _1, count = copy _3)
// CHECK-NEXT:     v18 = call fn4(v15, v16, v17)
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3ptr4copyhECs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:6(i16, i16, i16) system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3ptr4copyhECs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut u8
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const u8                         2b 2                var=1
// CHECK-NEXT: ; ssa   _2    *mut u8                           2b 2                var=2
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; zst   _5    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _6    *const ()                         2b 2                var=5
// CHECK-NEXT: ; ssa   _7    *mut ()                           2b 2                var=6
// CHECK-NEXT: ; ssa   _8    usize                             2b 2                var=7
// CHECK-NEXT: ; ssa   _9    bool                              1b 1                var=8
// CHECK-EMPTY:
// CHECK-NEXT:     gv0 = symbol colocated userextname2 ; alloc17
// CHECK-NEXT:     sig0 = () -> i8 uext system_v
// CHECK-NEXT:     sig1 = () -> i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16, i8 uext, i16) system_v
// CHECK-NEXT:     sig3 = (i16, i16, i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:29 sig0 ; Instance { def: Item(DefId(1:3622 ~ core[..]::ub_checks::check_language_ub)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(1:2951 ~ core[..]::mem::align_of)), args: [u8] }
// CHECK-NEXT:     fn2 = colocated u0:35 sig2 ; Instance { def: Item(DefId(1:29533 ~ core[..]::ptr::copy::precondition_check)), args: [] }
// CHECK-NEXT:     fn3 = %Memmove sig3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     v5 -> v0
// CHECK-NEXT:     v12 -> v0
// CHECK-NEXT:     v17 -> v0
// CHECK-NEXT:     v6 -> v1
// CHECK-NEXT:     v13 -> v1
// CHECK-NEXT:     v18 -> v1
// CHECK-NEXT:     v9 -> v2
// CHECK-NEXT:     v19 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *const u8 <- ByVal(v0): *const u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): *mut u8 <- ByVal(v1): *mut u8
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v2): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::ub_checks::check_language_ub()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v3 = call fn0()
// CHECK-NEXT:     v4 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v3): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _4)
// CHECK-NEXT:     brif.i8 v4, block3, block8
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy _1 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): *const () <- ByVal(v5): *const ()
// CHECK-NEXT: ; _7 = copy _2 as *mut () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): *mut () <- ByVal(v6): *mut ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = core::mem::align_of::<T>()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v7 = call fn1()
// CHECK-NEXT:     v14 -> v7
// CHECK-NEXT: ; write_cvalue: Var(_8, var7): usize <- ByVal(v7): usize
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(const <T as core::mem::SizedTypeProperties>::IS_ZST)
// CHECK-NEXT:     v8 = iconst.i8 0
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = Eq(copy _3, const 0_usize)
// CHECK-NEXT:     v10 = iconst.i16 0
// CHECK-NEXT:     v11 = icmp.i16 eq v9, v10  ; v10 = 0
// CHECK-NEXT:     v15 -> v11
// CHECK-NEXT: ; write_cvalue: Var(_9, var8): bool <- ByVal(v11): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::ptr::copy::precondition_check(move _6, move _7, move _8, move _9)
// CHECK-NEXT:     v16 = symbol_value.i16 gv0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *mut (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 4, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v12, v13, v14, v15, v16)
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::copy::<T>(copy _1, copy _2, copy _3)
// CHECK-NEXT:     v20 = call fn3(v18, v17, v19)
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3stre3lenCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:7(i16, i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3stre3lenCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    usize                             2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} str
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} str                    4b 2                var=(2, 3)
// CHECK-NEXT: ; ssa   _2    &'{erased} [u8]                   4b 2                var=(4, 5)
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:9 sig0 ; Instance { def: Item(DefId(1:16669 ~ core[..]::str::{impl#0}::as_bytes)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: VarPair(_1, var2, var3): &'{erased} str <- ByValPair(v0, v1): &'{erased} str
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::str::<impl str>::as_bytes(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: &[u8], layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4, v5 = call fn0(v2, v3)
// CHECK-NEXT:     v6 -> v4
// CHECK-NEXT:     v7 -> v5
// CHECK-NEXT: ; write_cvalue: VarPair(_2, var4, var5): &'{erased} [u8] <- ByValPair(v4, v5): &'{erased} [u8]
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = PtrMetadata(copy _2)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v7): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3stre6as_ptr
// CHECK-NEXT: function u0:8(i16, i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3stre6as_ptr
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    *const u8                         2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const u8
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} str
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} str                    4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _2    *const str                        4b 2                var=(2, 3)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} str <- ByValPair(v0, v1): &'{erased} str
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT:     v3 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = &raw const (*_1)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT:     v6 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: VarPair(_2, var2, var3): *const str <- ByValPair(v5, v7): *const str
// CHECK-NEXT: ; _0 = move _2 as *const u8 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): *const u8 <- ByVal(v5): *const u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3stre8as_bytes
// CHECK-NEXT: function u0:9(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3stre8as_bytes
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    &'{erased} [u8]                   4b 2                var=(1, 2)
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} [u8]
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} str
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} str                    4b 2                var=(3, 4)
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: VarPair(_1, var3, var4): &'{erased} str <- ByValPair(v0, v1): &'{erased} str
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = copy _1 as &[u8] (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: VarPair(_0, var1, var2): &'{erased} [u8] <- ByValPair(v2, v3): &'{erased} str
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v2, v3
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core5sliceSh10as_mut_ptrCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:10(i16, i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core5sliceSh10as_mut_ptrCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    *mut u8                           2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut u8
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} mut [u8]
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} mut [u8]               4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _2    *mut [u8]                         4b 2                var=(2, 3)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} mut [u8] <- ByValPair(v0, v1): &'{erased} mut [u8]
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT:     v3 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = &raw mut (*_1)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT:     v6 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: VarPair(_2, var2, var3): *mut [u8] <- ByValPair(v5, v7): *mut [u8]
// CHECK-NEXT: ; _0 = copy _2 as *mut T (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): *mut u8 <- ByVal(v5): *mut u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core5sliceSh6as_ptrCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:11(i16, i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core5sliceSh6as_ptrCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    *const u8                         2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const u8
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} [u8]
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} [u8]                   4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _2    *const [u8]                       4b 2                var=(2, 3)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} [u8] <- ByValPair(v0, v1): &'{erased} [u8]
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT:     v3 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = &raw const (*_1)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT:     v6 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: VarPair(_2, var2, var3): *const [u8] <- ByValPair(v5, v7): *const [u8]
// CHECK-NEXT: ; _0 = copy _2 as *const T (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): *const u8 <- ByVal(v5): *const u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtNtCs40R3J0iFVxM_4core3ptr7mut_ptrOh3addCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:12(i16, i16, i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtNtCs40R3J0iFVxM_4core3ptr7mut_ptrOh3addCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    *mut u8                           2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut u8
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut u8
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg  ???      = v2       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut u8                           2b 2                var=2
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     v3 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *mut u8 <- ByVal(v0): *mut u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Offset(copy _1, copy _2)
// CHECK-NEXT:     v5 = iconst.i16 1
// CHECK-NEXT:     v6 = imul.i16 v4, v5  ; v5 = 1
// CHECK-NEXT:     v7 = iadd.i16 v3, v6
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): *mut u8 <- ByVal(v7): *mut u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtNtCs40R3J0iFVxM_4core3ptr9const_ptrPu4addrCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:13(i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtNtCs40R3J0iFVxM_4core3ptr9const_ptrPu4addrCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    usize                             2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=2
// CHECK-NEXT: ; ssa   _2    *const ()                         2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:0 sig0 ; Instance { def: Item(DefId(1:3299 ~ core[..]::ptr::const_ptr::{impl#0}::cast)), args: [(), ()] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *const () <- ByVal(v0): *const ()
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::ptr::const_ptr::<impl *const T>::cast::<()>(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0(v1)
// CHECK-NEXT:     v3 -> v2
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): *const () <- ByVal(v2): *const ()
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _2 as usize (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_0, var1): usize <- ByVal(v3): *const ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v3
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs0_NtCs40R3J0iFVxM_4core3numl11is_negative
// CHECK-NEXT: function u0:14(i32) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvMs0_NtCs40R3J0iFVxM_4core3numl11is_negative
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    i32                               4b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): i32 <- ByVal(v0): i32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Lt(copy _1, const 0_i32)
// CHECK-NEXT:     v2 = iconst.i32 0
// CHECK-NEXT:     v3 = icmp.i32 slt v1, v2  ; v2 = 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v3): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v3
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs0_NtCs40R3J0iFVxM_4core3numl12unsigned_absCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:15(i32) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvMs0_NtCs40R3J0iFVxM_4core3numl12unsigned_absCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    i32                               4b 2                var=2
// CHECK-NEXT: ; ssa   _2    i32                               4b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i32) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:16 sig0 ; Instance { def: Item(DefId(1:26772 ~ core[..]::num::{impl#2}::wrapping_abs)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): i32 <- ByVal(v0): i32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::num::<impl i32>::wrapping_abs(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0(v1)
// CHECK-NEXT:     v3 -> v2
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): i32 <- ByVal(v2): i32
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _2 as u32 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u32 <- ByVal(v3): u32
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v3
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs0_NtCs40R3J0iFVxM_4core3numl12wrapping_absCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:16(i32) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvMs0_NtCs40R3J0iFVxM_4core3numl12wrapping_absCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    i32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    i32                               4b 2                var=2
// CHECK-NEXT: ; ssa   _2    bool                              1b 1                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i32) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i32) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:14 sig0 ; Instance { def: Item(DefId(1:26811 ~ core[..]::num::{impl#2}::is_negative)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:17 sig1 ; Instance { def: Item(DefId(1:26769 ~ core[..]::num::{impl#2}::wrapping_neg)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     v4 -> v0
// CHECK-NEXT:     v6 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): i32 <- ByVal(v0): i32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::num::<impl i32>::is_negative(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0(v1)
// CHECK-NEXT:     v3 -> v2
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): bool <- ByVal(v2): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _2)
// CHECK-NEXT:     brif.i8 v3, block3, block4
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::num::<impl i32>::wrapping_neg(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v5 = call fn1(v4)
// CHECK-NEXT:     v7 -> v5
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): i32 <- ByVal(v5): i32
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = copy _1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): i32 <- ByVal(v6): i32
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v6
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs0_NtCs40R3J0iFVxM_4core3numl12wrapping_neg
// CHECK-NEXT: function u0:17(i32) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvMs0_NtCs40R3J0iFVxM_4core3numl12wrapping_neg
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    i32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    i32                               4b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i32, i32) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:18 sig0 ; Instance { def: Item(DefId(1:26762 ~ core[..]::num::{impl#2}::wrapping_sub)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): i32 <- ByVal(v0): i32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::num::<impl i32>::wrapping_sub(const 0_i32, copy _1)
// CHECK-NEXT:     v1 = iconst.i32 0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v3 = call fn0(v1, v2)  ; v1 = 0
// CHECK-NEXT:     v4 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): i32 <- ByVal(v3): i32
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
// CHECK-NEXT: ; symbol _RNvMs0_NtCs40R3J0iFVxM_4core3numl12wrapping_sub
// CHECK-NEXT: function u0:18(i32, i32) -> i32 system_v {
// CHECK-NEXT: ; symbol _RNvMs0_NtCs40R3J0iFVxM_4core3numl12wrapping_sub
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    i32                               4b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i32
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i32
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    i32                               4b 2                var=2
// CHECK-NEXT: ; ssa   _2    i32                               4b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i32, v1: i32):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): i32 <- ByVal(v0): i32
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): i32 <- ByVal(v1): i32
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v4 = isub.i32 v2, v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): i32 <- ByVal(v4): i32
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3fmtNtB5_9Arguments8from_strCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:19(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3fmtNtB5_9Arguments8from_strCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::fmt::Arguments<'{erased}>    4b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::fmt::Arguments<'{erased}>
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} str
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} str                    4b 2                var=(1, 2)
// CHECK-NEXT: ; stack _2    core::ptr::NonNull<u8>            2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    *const u8                         2b 2                var=3
// CHECK-NEXT: ; stack _4    core::ptr::NonNull<core::fmt::rt::Argument<'{erased}>>    2b 2                storage=ss2
// CHECK-NEXT: ; ssa   _5    usize                             2b 2                var=4
// CHECK-NEXT: ; ssa   _6    usize                             2b 2                var=5
// CHECK-NEXT: ; ssa   _7    usize                             2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i16 system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:8 sig0 ; Instance { def: Item(DefId(1:46132 ~ core[..]::str::{impl#0}::as_ptr)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:7 sig1 ; Instance { def: Item(DefId(1:16660 ~ core[..]::str::{impl#0}::len)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v7 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v8 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: VarPair(_1, var1, var2): &'{erased} str <- ByValPair(v0, v1): &'{erased} str
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::str::<impl str>::as_ptr(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *const u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4 = call fn0(v2, v3)
// CHECK-NEXT:     v5 -> v4
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *const u8 <- ByVal(v4): *const u8
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = move _3 as core::ptr::NonNull<u8> (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::ptr::NonNull<u8> <- ByVal(v5): *const u8
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT:     store.i16 notrap v5, v6
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = core::str::<impl str>::len(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v9 = call fn1(v7, v8)
// CHECK-NEXT:     v10 -> v9
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): usize <- ByVal(v9): usize
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = Shl(move _7, const 1_i32)
// CHECK-NEXT:     v11 = iconst.i32 1
// CHECK-NEXT:     v12 = ishl.i16 v10, v11  ; v11 = 1
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): usize <- ByVal(v12): usize
// CHECK-NEXT: ; _5 = BitOr(move _6, const 1_usize)
// CHECK-NEXT:     v13 = iconst.i16 1
// CHECK-NEXT:     v14 = bor v12, v13  ; v13 = 1
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): usize <- ByVal(v14): usize
// CHECK-NEXT: ; _4 = move _5 as core::ptr::NonNull<core::fmt::rt::Argument<'_>> (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): core::ptr::NonNull<core::fmt::rt::Argument<'{erased}>> <- ByVal(v14): usize
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v14, v15
// CHECK-NEXT: ; _0 = core::fmt::Arguments::<'_> { template: move _2, args: move _4 }
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<u8> <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::ptr::NonNull<u8>
// CHECK-NEXT:     v16 = stack_addr.i16 ss1
// CHECK-NEXT:     v17 = load.i16 notrap v16
// CHECK-NEXT:     v18 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v17, v18
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): core::ptr::NonNull<core::fmt::rt::Argument<'{erased}>> <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): core::ptr::NonNull<core::fmt::rt::Argument<'{erased}>>
// CHECK-NEXT:     v19 = stack_addr.i16 ss2
// CHECK-NEXT:     v20 = load.i16 notrap v19
// CHECK-NEXT:     v21 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v20, v21
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v22 = stack_addr.i16 ss0
// CHECK-NEXT:     v23 = load.i16 notrap v22
// CHECK-NEXT:     v24 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v25 = load.i16 notrap v24
// CHECK-NEXT:     return v23, v25
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs9_NtCs40R3J0iFVxM_4core3numj11checked_mulCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:20(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs9_NtCs40R3J0iFVxM_4core3numj11checked_mulCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::option::Option<usize>       4b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::option::Option<usize>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    usize                             2b 2                var=1
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=2
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (usize, bool)                     4b 2                var=(5, 6)
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=7
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i8 uext system_v
// CHECK-NEXT:     sig1 = (i8 uext) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:21 sig0 ; Instance { def: Item(DefId(1:28402 ~ core[..]::num::{impl#11}::overflowing_mul)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:23 sig1 ; Instance { def: Item(DefId(1:2345 ~ core[..]::intrinsics::unlikely)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): usize <- ByVal(v0): usize
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::num::<impl usize>::overflowing_mul(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (usize, bool), layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is .., b: u8 is 0..=1, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(2 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4, v5 = call fn0(v2, v3)
// CHECK-NEXT:     v6 -> v4
// CHECK-NEXT:     v16 -> v4
// CHECK-NEXT:     v7 -> v5
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var5, var6): (usize, bool) <- ByValPair(v4, v5): (usize, bool)
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (_5.0: usize)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v6): usize
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
// CHECK-NEXT: ; _0 = core::option::Option::<usize>::None
// CHECK-NEXT:     v10 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u16 <- ByVal(v10): u16
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v10, v11  ; v10 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v12 = stack_addr.i16 ss0
// CHECK-NEXT:     v13 = load.i16 notrap v12
// CHECK-NEXT:     v14 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     return v13, v15
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<usize>::Some(copy _3)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): usize <- ByVal(v16): usize
// CHECK-NEXT:     v17 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store.i16 notrap v16, v17
// CHECK-NEXT:     v18 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u16 <- ByVal(v18): u16
// CHECK-NEXT:     v19 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v18, v19  ; v18 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v20 = stack_addr.i16 ss0
// CHECK-NEXT:     v21 = load.i16 notrap v20
// CHECK-NEXT:     v22 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v23 = load.i16 notrap v22
// CHECK-NEXT:     return v21, v23
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v24 = stack_addr.i16 ss0
// CHECK-NEXT:     v25 = load.i16 notrap v24
// CHECK-NEXT:     v26 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v27 = load.i16 notrap v26
// CHECK-NEXT:     return v25, v27
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs9_NtCs40R3J0iFVxM_4core3numj15overflowing_mul
// CHECK-NEXT: function u0:21(i16, i16) -> i16, i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvMs9_NtCs40R3J0iFVxM_4core3numj15overflowing_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    (usize, bool)                     4b 2                var=(1, 2)
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) (usize, bool)
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=4
// CHECK-NEXT: ; ssa   _3    u16                               2b 2                var=5
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=6
// CHECK-NEXT: ; ssa   _5    (u16, bool)                       4b 2                var=(7, 8)
// CHECK-NEXT: ; ssa   _6    u16                               2b 2                var=9
// CHECK-NEXT: ; ssa   _7    u16                               2b 2                var=10
// CHECK-NEXT: ; ssa   _8    usize                             2b 2                var=11
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var3): usize <- ByVal(v0): usize
// CHECK-NEXT: ; write_cvalue: Var(_2, var4): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy _1 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_6, var9): u16 <- ByVal(v2): u16
// CHECK-NEXT: ; _7 = copy _2 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_7, var10): u16 <- ByVal(v3): u16
// CHECK-NEXT: ; _5 = MulWithOverflow(move _6, move _7)
// CHECK-NEXT:     v4 = uextend.i32 v2
// CHECK-NEXT:     v5 = uextend.i32 v3
// CHECK-NEXT:     v6 = imul v4, v5
// CHECK-NEXT:     v7 = iconst.i32 0xffff
// CHECK-NEXT:     v8 = icmp ugt v6, v7  ; v7 = 0xffff
// CHECK-NEXT:     v9 = ireduce.i16 v6
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var7, var8): (u16, bool) <- ByValPair(v9, v8): (u16, bool)
// CHECK-NEXT: ; _3 = copy (_5.0: u16)
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): u16 <- ByVal(v9): u16
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var6): bool <- ByVal(v8): bool
// CHECK-NEXT: ; _8 = copy _3 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_8, var11): usize <- ByVal(v9): usize
// CHECK-NEXT: ; _0 = (move _8, copy _4)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v9): usize
// CHECK-NEXT: ; write_cvalue: Var(_0, var2): bool <- ByVal(v8): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v9, v8
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs9_NtCs40R3J0iFVxM_4core3numj8abs_diffCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:22(i16, i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs9_NtCs40R3J0iFVxM_4core3numj8abs_diffCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    usize                             2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    usize                             2b 2                var=2
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _4    usize                             2b 2                var=5
// CHECK-NEXT: ; ssa   _5    u32                               4b 2                var=6
// CHECK-NEXT: ; ssa   _6    i32                               4b 2                var=7
// CHECK-NEXT: ; ssa   _7    i32                               4b 2                var=8
// CHECK-NEXT: ; ssa   _8    i32                               4b 2                var=9
// CHECK-NEXT: ; ssa   _9    bool                              1b 1                var=10
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = () -> i16 system_v
// CHECK-NEXT:     sig1 = (i32, i32) -> i32 system_v
// CHECK-NEXT:     sig2 = (i32) -> i32 system_v
// CHECK-NEXT:     fn0 = colocated u0:2 sig0 ; Instance { def: Item(DefId(1:2941 ~ core[..]::mem::size_of)), args: [usize] }
// CHECK-NEXT:     fn1 = colocated u0:18 sig1 ; Instance { def: Item(DefId(1:26762 ~ core[..]::num::{impl#2}::wrapping_sub)), args: [] }
// CHECK-NEXT:     fn2 = colocated u0:15 sig2 ; Instance { def: Item(DefId(1:26773 ~ core[..]::num::{impl#2}::unsigned_abs)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v6 -> v0
// CHECK-NEXT:     v15 -> v0
// CHECK-NEXT:     v19 -> v0
// CHECK-NEXT:     v21 -> v0
// CHECK-NEXT:     v8 -> v1
// CHECK-NEXT:     v16 -> v1
// CHECK-NEXT:     v18 -> v1
// CHECK-NEXT:     v22 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): usize <- ByVal(v0): usize
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::mem::size_of::<usize>()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0()
// CHECK-NEXT:     v3 -> v2
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): usize <- ByVal(v2): usize
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = Eq(move _4, const 1_usize)
// CHECK-NEXT:     v4 = iconst.i16 1
// CHECK-NEXT:     v5 = icmp.i16 eq v3, v4  ; v4 = 1
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): bool <- ByVal(v5): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _3)
// CHECK-NEXT:     brif v5, block3, block6
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _7 = copy _1 as i32 (IntToInt)
// CHECK-NEXT:     v7 = uextend.i32 v6
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): i32 <- ByVal(v7): i32
// CHECK-NEXT: ; _8 = copy _2 as i32 (IntToInt)
// CHECK-NEXT:     v9 = uextend.i32 v8
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): i32 <- ByVal(v9): i32
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::num::<impl i32>::wrapping_sub(move _7, move _8)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v10 = call fn1(v7, v9)
// CHECK-NEXT:     v11 -> v10
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): i32 <- ByVal(v10): i32
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::num::<impl i32>::unsigned_abs(move _6)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: i32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(i32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027873795 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v12 = call fn2(v11)
// CHECK-NEXT:     v13 -> v12
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): u32 <- ByVal(v12): u32
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _5 as usize (IntToInt)
// CHECK-NEXT:     v14 = ireduce.i16 v13
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v14): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v14
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = Lt(copy _1, copy _2)
// CHECK-NEXT:     v17 = icmp.i16 ult v15, v16
// CHECK-NEXT: ; write_cvalue: Var(_9, var10): bool <- ByVal(v17): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _9)
// CHECK-NEXT:     brif v17, block7, block8
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Sub(copy _2, copy _1)
// CHECK-NEXT:     v20 = isub.i16 v18, v19
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v20): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v20
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v23 = isub.i16 v21, v22
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v23): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v23
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     v25 = iconst.i16 0
// CHECK-NEXT:     v24 -> v25
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v24  ; v24 = 0
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics8unlikely
// CHECK-NEXT: function u0:23(i8 uext) -> i8 uext system_v {
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
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:24() system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCs7YLftNnbcrY_13runtime_calls
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
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking14panic_nounwindCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:25(i16, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking14panic_nounwindCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               !
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} str
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} str                    4b 2                var=(1, 2)
// CHECK-NEXT: ; zst   _2    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; stack _3    core::fmt::Arguments<'{erased}>    4b 2                storage=ss0
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname1 ; alloc19
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig1 = (i16, i16, i8 uext, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:19 sig0 ; Instance { def: Item(DefId(1:13533 ~ core[..]::fmt::{impl#6}::from_str)), args: ['{erased}] }
// CHECK-NEXT:     fn1 = colocated u0:26 sig1 ; Instance { def: Item(DefId(1:12245 ~ core[..]::panicking::panic_nounwind_fmt)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: VarPair(_1, var1, var2): &'{erased} str <- ByValPair(v0, v1): &'{erased} str
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::fmt::Arguments::<'_>::from_str(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4, v5 = call fn0(v2, v3)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}> <- ByValPair(v4, v5): core::fmt::Arguments<'{erased}>
// CHECK-NEXT:     v6 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v4, v6
// CHECK-NEXT:     v7 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v5, v7
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::panicking::panic_nounwind_fmt(move _3, const false)
// CHECK-NEXT:     v8 = iconst.i8 0
// CHECK-NEXT:     v9 = symbol_value.i16 gv0
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     v11 = load.i16 notrap v10
// CHECK-NEXT:     v12 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v13 = load.i16 notrap v12
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: !, layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Primitive, largest_niche: None, uninhabited: true, variants: Empty, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn1(v11, v13, v8, v9)  ; v8 = 0
// CHECK-NEXT:     trap user1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking18panic_nounwind_fmtCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:26(i16, i16, i8 uext, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking18panic_nounwind_fmtCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               !
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::fmt::Arguments<'{erased}>
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg  ???      = v3       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    core::fmt::Arguments<'{erased}>    4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _2    bool                              1b 1                var=1
// CHECK-NEXT: ; stack _3    (core::fmt::Arguments<'{erased}>, bool)    6b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 6, align = 2
// CHECK-NEXT:     sig0 = (i16, i16, i8 uext, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:36 sig0 ; Instance { def: Item(DefId(1:43505 ~ core[..]::panicking::panic_nounwind_fmt::runtime)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i8, v3: i16):
// CHECK-NEXT:     v12 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}> <- ByValPair(v0, v1): core::fmt::Arguments<'{erased}>
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v4
// CHECK-NEXT:     v5 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v1, v5
// CHECK-NEXT: ; write_cvalue: Var(_2, var1): bool <- ByVal(v2): bool
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}> <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}>
// CHECK-NEXT:     v6 = stack_addr.i16 ss0
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT:     v8 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v9 = load.i16 notrap v8
// CHECK-NEXT:     v10 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v7, v10
// CHECK-NEXT:     v11 = stack_addr.i16 ss1+2
// CHECK-NEXT:     store notrap v9, v11
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(4) }, None): bool <- ByVal(v12): bool
// CHECK-NEXT:     v13 = stack_addr.i16 ss1+4
// CHECK-NEXT:     store.i8 notrap v12, v13
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::panicking::panic_nounwind_fmt::runtime(move (_3.0: core::fmt::Arguments<'_>), move (_3.1: bool))
// CHECK-NEXT:     v14 = stack_addr.i16 ss1
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = stack_addr.i16 ss1+2
// CHECK-NEXT:     v17 = load.i16 notrap v16
// CHECK-NEXT:     v18 = stack_addr.i16 ss1+4
// CHECK-NEXT:     v19 = load.i8 notrap v18
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: !, layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Primitive, largest_niche: None, uninhabited: true, variants: Empty, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn0(v15, v17, v19, v3)
// CHECK-NEXT:     trap user1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking9panic_fmtCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:27(i16, i16, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking9panic_fmtCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               !
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::fmt::Arguments<'{erased}>
// CHECK-NEXT: ; arg  ???      = v2       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    core::fmt::Arguments<'{erased}>    4b 2                storage=ss0
// CHECK-NEXT: ; zst   _2    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; stack _3    core::panic::PanicInfo<'{erased}>    6b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 6, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}> <- ByValPair(v0, v1): core::fmt::Arguments<'{erased}>
// CHECK-NEXT:     v3 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v3
// CHECK-NEXT:     v4 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v1, v4
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::intrinsics::abort_immediate()
// CHECK-NEXT:     trap user2
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks16maybe_is_alignedCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:28(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks16maybe_is_alignedCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=2
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _3    (*const (), usize)                4b 2                var=(4, 5)
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:37 sig0 ; Instance { def: Item(DefId(1:29565 ~ core[..]::ub_checks::maybe_is_aligned::runtime)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): *const () <- ByVal(v2): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): usize <- ByVal(v3): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::ub_checks::maybe_is_aligned::runtime(move (_3.0: *const ()), move (_3.1: usize))
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4 = call fn0(v2, v3)
// CHECK-NEXT:     v5 -> v4
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v4): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks17check_language_ubCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:29() -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks17check_language_ubCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    bool                              1b 1                var=2
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = () -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:38 sig0 ; Instance { def: Item(DefId(1:29563 ~ core[..]::ub_checks::check_language_ub::runtime)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _1 = core::ub_checks::check_language_ub::runtime()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v0 = call fn0()
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v0): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _1)
// CHECK-NEXT:     brif.i8 v1, block3, block4
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = UbChecks
// CHECK-NEXT:     v2 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v2): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v2  ; v2 = 0
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
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks23maybe_is_nonoverlappingCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:30(i16, i16, i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks23maybe_is_nonoverlappingCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _4      = v3       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=2
// CHECK-NEXT: ; ssa   _2    *const ()                         2b 2                var=3
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=4
// CHECK-NEXT: ; ssa   _4    usize                             2b 2                var=5
// CHECK-NEXT: ; stack _5    (*const (), *const (), usize, usize)    8b 2                storage=ss0
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 8, align = 2
// CHECK-NEXT:     sig0 = (i16, i16, i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:39 sig0 ; Instance { def: Item(DefId(1:29567 ~ core[..]::ub_checks::maybe_is_nonoverlapping::runtime)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16, v3: i16):
// CHECK-NEXT:     v4 -> v0
// CHECK-NEXT:     v6 -> v1
// CHECK-NEXT:     v8 -> v2
// CHECK-NEXT:     v10 -> v3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): *const () <- ByVal(v1): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): usize <- ByVal(v2): usize
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): usize <- ByVal(v3): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = (copy _1, copy _2, copy _3, copy _4)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): *const () <- ByVal(v4): *const ()
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v4, v5
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): *const () <- ByVal(v6): *const ()
// CHECK-NEXT:     v7 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store.i16 notrap v6, v7
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(4) }, None): usize <- ByVal(v8): usize
// CHECK-NEXT:     v9 = stack_addr.i16 ss0+4
// CHECK-NEXT:     store.i16 notrap v8, v9
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(6) }, None): usize <- ByVal(v10): usize
// CHECK-NEXT:     v11 = stack_addr.i16 ss0+6
// CHECK-NEXT:     store.i16 notrap v10, v11
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::ub_checks::maybe_is_nonoverlapping::runtime(move (_5.0: *const ()), move (_5.1: *const ()), move (_5.2: usize), move (_5.3: usize))
// CHECK-NEXT:     v12 = stack_addr.i16 ss0
// CHECK-NEXT:     v13 = load.i16 notrap v12
// CHECK-NEXT:     v14 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = stack_addr.i16 ss0+4
// CHECK-NEXT:     v17 = load.i16 notrap v16
// CHECK-NEXT:     v18 = stack_addr.i16 ss0+6
// CHECK-NEXT:     v19 = load.i16 notrap v18
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 4, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v20 = call fn0(v13, v15, v17, v19)
// CHECK-NEXT:     v21 -> v20
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v20): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v21
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks29maybe_is_aligned_and_not_nullCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:31(i16, i16, i8 uext) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks29maybe_is_aligned_and_not_nullCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=2
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=5
// CHECK-NEXT: ; ssa   _5    bool                              1b 1                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:28 sig0 ; Instance { def: Item(DefId(1:3624 ~ core[..]::ub_checks::maybe_is_aligned)), args: [] }
// CHECK-NEXT:     fn1 = u0:45 sig1 ; Instance { def: Item(DefId(1:3298 ~ core[..]::ptr::const_ptr::{impl#0}::is_null)), args: [()] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i8):
// CHECK-NEXT:     v3 -> v0
// CHECK-NEXT:     v10 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     v7 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v1): usize
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): bool <- ByVal(v2): bool
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::ub_checks::maybe_is_aligned(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v5 = call fn0(v3, v4)
// CHECK-NEXT:     v6 -> v5
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): bool <- ByVal(v5): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _4)
// CHECK-NEXT:     brif.i8 v6, block3, block4
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _3)
// CHECK-NEXT:     brif.i8 v7, block5, block6
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const false
// CHECK-NEXT:     v8 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v8): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v8  ; v8 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const true
// CHECK-NEXT:     v9 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v9): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v9  ; v9 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::ptr::const_ptr::<impl *const ()>::is_null(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v11 = call fn1(v10)
// CHECK-NEXT:     v12 -> v11
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): bool <- ByVal(v11): bool
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Not(move _5)
// CHECK-NEXT:     v13 = iconst.i8 0
// CHECK-NEXT:     v14 = icmp.i8 eq v12, v13  ; v13 = 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v14): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v14
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     v16 = iconst.i8 0
// CHECK-NEXT:     v15 -> v16
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v15  ; v15 = 0
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_div_by_zeroCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:32(i16) system_v {
// CHECK-NEXT: ; symbol _RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_div_by_zeroCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               !
// CHECK-NEXT: ; arg  ???      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; zst   _1    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; stack _2    core::fmt::Arguments<'{erased}>    4b 2                storage=ss0
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc13
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig1 = (i16, i16, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:19 sig0 ; Instance { def: Item(DefId(1:13533 ~ core[..]::fmt::{impl#6}::from_str)), args: ['{erased}] }
// CHECK-NEXT:     fn1 = colocated u0:27 sig1 ; Instance { def: Item(DefId(1:12242 ~ core[..]::panicking::panic_fmt)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::fmt::Arguments::<'_>::from_str(const "attempt to divide by zero")
// CHECK-NEXT:     v1 = symbol_value.i16 gv0
// CHECK-NEXT:     v2 = iconst.i16 25
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v3, v4 = call fn0(v1, v2)  ; v2 = 25
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}> <- ByValPair(v3, v4): core::fmt::Arguments<'{erased}>
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v3, v5
// CHECK-NEXT:     v6 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v4, v6
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _1 = core::panicking::panic_fmt(move _2)
// CHECK-NEXT:     v7 = stack_addr.i16 ss0
// CHECK-NEXT:     v8 = load.i16 notrap v7
// CHECK-NEXT:     v9 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v10 = load.i16 notrap v9
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: !, layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Primitive, largest_niche: None, uninhabited: true, variants: Empty, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn1(v8, v10, v0)
// CHECK-NEXT:     trap user1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr11write_bytes18precondition_checkCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:33(i16, i16, i8 uext, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr11write_bytes18precondition_checkCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg  ???      = v3       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=1
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=2
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; zst   _5    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; stack _6    core::fmt::Arguments<'{erased}>    4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _7    &'{erased} str                    4b 2                var=(5, 6)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname1 ; alloc12
// CHECK-NEXT:     sig0 = (i16, i16, i8 uext) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16, i8 uext, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:31 sig0 ; Instance { def: Item(DefId(1:3623 ~ core[..]::ub_checks::maybe_is_aligned_and_not_null)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:19 sig1 ; Instance { def: Item(DefId(1:13533 ~ core[..]::fmt::{impl#6}::from_str)), args: ['{erased}] }
// CHECK-NEXT:     fn2 = colocated u0:26 sig2 ; Instance { def: Item(DefId(1:12245 ~ core[..]::panicking::panic_nounwind_fmt)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i8, v3: i16):
// CHECK-NEXT:     v4 -> v0
// CHECK-NEXT:     v5 -> v1
// CHECK-NEXT:     v6 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): usize <- ByVal(v1): usize
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): bool <- ByVal(v2): bool
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::ub_checks::maybe_is_aligned_and_not_null(copy _1, copy _2, copy _3)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v7 = call fn0(v4, v5, v6)
// CHECK-NEXT:     v8 -> v7
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v7): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _4)
// CHECK-NEXT:     brif.i8 v8, block3, block4
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _7 = const "unsafe precondition(s) violated: ptr::write_bytes requires that the destination pointer is aligned and non-null\n\nThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety."
// CHECK-NEXT:     v9 = symbol_value.i16 gv0
// CHECK-NEXT:     v10 = iconst.i16 228
// CHECK-NEXT: ; write_cvalue: VarPair(_7, var5, var6): &'{erased} str <- ByValPair(v9, v10): &'{erased} str
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::fmt::Arguments::<'_>::from_str(move _7)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v11, v12 = call fn1(v9, v10)  ; v10 = 228
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}> <- ByValPair(v11, v12): core::fmt::Arguments<'{erased}>
// CHECK-NEXT:     v13 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v11, v13
// CHECK-NEXT:     v14 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v12, v14
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::panicking::panic_nounwind_fmt(move _6, const false)
// CHECK-NEXT:     v15 = iconst.i8 0
// CHECK-NEXT:     v16 = stack_addr.i16 ss0
// CHECK-NEXT:     v17 = load.i16 notrap v16
// CHECK-NEXT:     v18 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v19 = load.i16 notrap v18
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: !, layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Primitive, largest_niche: None, uninhabited: true, variants: Empty, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v17, v19, v15, v3)  ; v15 = 0
// CHECK-NEXT:     trap user1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr19copy_nonoverlapping18precondition_checkCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:34(i16, i16, i16, i16, i16, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr19copy_nonoverlapping18precondition_checkCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut ()
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _4      = v3       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _5      = v4       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg  ???      = v5       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=1
// CHECK-NEXT: ; ssa   _2    *mut ()                           2b 2                var=2
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _4    usize                             2b 2                var=4
// CHECK-NEXT: ; ssa   _5    usize                             2b 2                var=5
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=6
// CHECK-NEXT: ; ssa   _7    bool                              1b 1                var=7
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=8
// CHECK-NEXT: ; ssa   _9    bool                              1b 1                var=9
// CHECK-NEXT: ; ssa   _10   bool                              1b 1                var=10
// CHECK-NEXT: ; ssa   _11   bool                              1b 1                var=11
// CHECK-NEXT: ; ssa   _12   *const ()                         2b 2                var=12
// CHECK-NEXT: ; ssa   _13   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _14   *const ()                         2b 2                var=14
// CHECK-NEXT: ; zst   _15   !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; stack _16   core::fmt::Arguments<'{erased}>    4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _17   &'{erased} str                    4b 2                var=(15, 16)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname2 ; alloc7
// CHECK-NEXT:     sig0 = (i16, i16, i8 uext) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16, i8 uext) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig4 = (i16, i16, i8 uext, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:31 sig0 ; Instance { def: Item(DefId(1:3623 ~ core[..]::ub_checks::maybe_is_aligned_and_not_null)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:31 sig1 ; Instance { def: Item(DefId(1:3623 ~ core[..]::ub_checks::maybe_is_aligned_and_not_null)), args: [] }
// CHECK-NEXT:     fn2 = colocated u0:30 sig2 ; Instance { def: Item(DefId(1:3626 ~ core[..]::ub_checks::maybe_is_nonoverlapping)), args: [] }
// CHECK-NEXT:     fn3 = colocated u0:19 sig3 ; Instance { def: Item(DefId(1:13533 ~ core[..]::fmt::{impl#6}::from_str)), args: ['{erased}] }
// CHECK-NEXT:     fn4 = colocated u0:26 sig4 ; Instance { def: Item(DefId(1:12245 ~ core[..]::panicking::panic_nounwind_fmt)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16, v3: i16, v4: i16, v5: i16):
// CHECK-NEXT:     v14 -> v0
// CHECK-NEXT:     v24 -> v0
// CHECK-NEXT:     v18 -> v1
// CHECK-NEXT:     v23 -> v1
// CHECK-NEXT:     v41 -> v1
// CHECK-NEXT:     v10 -> v2
// CHECK-NEXT:     v25 -> v2
// CHECK-NEXT:     v42 -> v2
// CHECK-NEXT:     v15 -> v3
// CHECK-NEXT:     v20 -> v3
// CHECK-NEXT:     v6 -> v4
// CHECK-NEXT:     v26 -> v4
// CHECK-NEXT:     v43 -> v4
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): *mut () <- ByVal(v1): *mut ()
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v2): usize
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): usize <- ByVal(v3): usize
// CHECK-NEXT: ; write_cvalue: Var(_5, var5): usize <- ByVal(v4): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = Eq(copy _5, const 0_usize)
// CHECK-NEXT:     v7 = iconst.i16 0
// CHECK-NEXT:     v8 = icmp.i16 eq v6, v7  ; v7 = 0
// CHECK-NEXT: ; write_cvalue: Var(_8, var8): bool <- ByVal(v8): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _8)
// CHECK-NEXT:     brif.i16 v6, block3, block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _7 = const true
// CHECK-NEXT:     v9 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_7, var7): bool <- ByVal(v9): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block4(v9)  ; v9 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _7 = Eq(copy _3, const 0_usize)
// CHECK-NEXT:     v11 = iconst.i16 0
// CHECK-NEXT:     v12 = icmp.i16 eq v10, v11  ; v11 = 0
// CHECK-NEXT: ; write_cvalue: Var(_7, var7): bool <- ByVal(v12): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block4(v12)
// CHECK-EMPTY:
// CHECK-NEXT: block4(v13: i8):
// CHECK-NEXT:     v19 -> v13
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = copy _7
// CHECK-NEXT: ; write_cvalue: Var(_10, var10): bool <- ByVal(v13): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _9 = core::ub_checks::maybe_is_aligned_and_not_null(copy _1, copy _4, move _10)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v16 = call fn0(v14, v15, v13)
// CHECK-NEXT:     v17 -> v16
// CHECK-NEXT: ; write_cvalue: Var(_9, var9): bool <- ByVal(v16): bool
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _9)
// CHECK-NEXT:     brif.i8 v17, block6, block9
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _12 = copy _2 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_12, var12): *const () <- ByVal(v18): *const ()
// CHECK-NEXT: ; _13 = copy _7
// CHECK-NEXT: ; write_cvalue: Var(_13, var13): bool <- ByVal(v19): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _11 = core::ub_checks::maybe_is_aligned_and_not_null(move _12, copy _4, move _13)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v21 = call fn1(v18, v20, v19)
// CHECK-NEXT:     v22 -> v21
// CHECK-NEXT: ; write_cvalue: Var(_11, var11): bool <- ByVal(v21): bool
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _11)
// CHECK-NEXT:     brif.i8 v22, block8, block9
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _14 = copy _2 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_14, var14): *const () <- ByVal(v23): *const ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::ub_checks::maybe_is_nonoverlapping(copy _1, move _14, copy _3, copy _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 4, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v27 = call fn2(v24, v23, v25, v26)
// CHECK-NEXT: ; write_cvalue: Var(_6, var6): bool <- ByVal(v27): bool
// CHECK-NEXT:     jump block10(v27)
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = const false
// CHECK-NEXT:     v28 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_6, var6): bool <- ByVal(v28): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block10(v28)  ; v28 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block10(v29: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     brif v29, block13, block11
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _17 = const "unsafe precondition(s) violated: ptr::copy_nonoverlapping requires that both pointer arguments are aligned and non-null and the specified memory ranges do not overlap\n\nThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety."
// CHECK-NEXT:     v30 = symbol_value.i16 gv0
// CHECK-NEXT:     v31 = iconst.i16 283
// CHECK-NEXT: ; write_cvalue: VarPair(_17, var15, var16): &'{erased} str <- ByValPair(v30, v31): &'{erased} str
// CHECK-NEXT: ;
// CHECK-NEXT: ; _16 = core::fmt::Arguments::<'_>::from_str(move _17)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v32, v33 = call fn3(v30, v31)  ; v31 = 283
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}> <- ByValPair(v32, v33): core::fmt::Arguments<'{erased}>
// CHECK-NEXT:     v34 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v32, v34
// CHECK-NEXT:     v35 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v33, v35
// CHECK-NEXT:     jump block12
// CHECK-EMPTY:
// CHECK-NEXT: block12:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _15 = core::panicking::panic_nounwind_fmt(move _16, const false)
// CHECK-NEXT:     v36 = iconst.i8 0
// CHECK-NEXT:     v37 = stack_addr.i16 ss0
// CHECK-NEXT:     v38 = load.i16 notrap v37
// CHECK-NEXT:     v39 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v40 = load.i16 notrap v39
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: !, layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Primitive, largest_niche: None, uninhabited: true, variants: Empty, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn4(v38, v40, v36, v5)  ; v36 = 0
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block13:
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
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr4copy18precondition_checkCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:35(i16, i16, i16, i8 uext, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr4copy18precondition_checkCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut ()
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _4      = v3       Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg  ???      = v4       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=1
// CHECK-NEXT: ; ssa   _2    *mut ()                           2b 2                var=2
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    bool                              1b 1                var=5
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=6
// CHECK-NEXT: ; ssa   _7    *const ()                         2b 2                var=7
// CHECK-NEXT: ; zst   _8    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; stack _9    core::fmt::Arguments<'{erased}>    4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _10   &'{erased} str                    4b 2                var=(8, 9)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname1 ; alloc11
// CHECK-NEXT:     sig0 = (i16, i16, i8 uext) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16, i8 uext) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig3 = (i16, i16, i8 uext, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:31 sig0 ; Instance { def: Item(DefId(1:3623 ~ core[..]::ub_checks::maybe_is_aligned_and_not_null)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:31 sig1 ; Instance { def: Item(DefId(1:3623 ~ core[..]::ub_checks::maybe_is_aligned_and_not_null)), args: [] }
// CHECK-NEXT:     fn2 = colocated u0:19 sig2 ; Instance { def: Item(DefId(1:13533 ~ core[..]::fmt::{impl#6}::from_str)), args: ['{erased}] }
// CHECK-NEXT:     fn3 = colocated u0:26 sig3 ; Instance { def: Item(DefId(1:12245 ~ core[..]::panicking::panic_nounwind_fmt)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16, v3: i8, v4: i16):
// CHECK-NEXT:     v5 -> v0
// CHECK-NEXT:     v10 -> v1
// CHECK-NEXT:     v6 -> v2
// CHECK-NEXT:     v11 -> v2
// CHECK-NEXT:     v7 -> v3
// CHECK-NEXT:     v12 -> v3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): *mut () <- ByVal(v1): *mut ()
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v2): usize
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v3): bool
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::ub_checks::maybe_is_aligned_and_not_null(copy _1, copy _3, copy _4)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v8 = call fn0(v5, v6, v7)
// CHECK-NEXT:     v9 -> v8
// CHECK-NEXT: ; write_cvalue: Var(_5, var5): bool <- ByVal(v8): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _5)
// CHECK-NEXT:     brif.i8 v9, block3, block6
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _7 = copy _2 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_7, var7): *const () <- ByVal(v10): *const ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::ub_checks::maybe_is_aligned_and_not_null(move _7, copy _3, copy _4)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v13 = call fn1(v10, v11, v12)
// CHECK-NEXT:     v14 -> v13
// CHECK-NEXT: ; write_cvalue: Var(_6, var6): bool <- ByVal(v13): bool
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     brif.i8 v14, block5, block6
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = const "unsafe precondition(s) violated: ptr::copy requires that both pointer arguments are aligned and non-null\n\nThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety."
// CHECK-NEXT:     v15 = symbol_value.i16 gv0
// CHECK-NEXT:     v16 = iconst.i16 221
// CHECK-NEXT: ; write_cvalue: VarPair(_10, var8, var9): &'{erased} str <- ByValPair(v15, v16): &'{erased} str
// CHECK-NEXT: ;
// CHECK-NEXT: ; _9 = core::fmt::Arguments::<'_>::from_str(move _10)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v17, v18 = call fn2(v15, v16)  ; v16 = 221
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}> <- ByValPair(v17, v18): core::fmt::Arguments<'{erased}>
// CHECK-NEXT:     v19 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v17, v19
// CHECK-NEXT:     v20 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v18, v20
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = core::panicking::panic_nounwind_fmt(move _9, const false)
// CHECK-NEXT:     v21 = iconst.i8 0
// CHECK-NEXT:     v22 = stack_addr.i16 ss0
// CHECK-NEXT:     v23 = load.i16 notrap v22
// CHECK-NEXT:     v24 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v25 = load.i16 notrap v24
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: !, layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Primitive, largest_niche: None, uninhabited: true, variants: Empty, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn3(v23, v25, v21, v4)  ; v21 = 0
// CHECK-NEXT:     trap user1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9panicking18panic_nounwind_fmt7runtimeCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:36(i16, i16, i8 uext, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9panicking18panic_nounwind_fmt7runtimeCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               !
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::fmt::Arguments<'{erased}>
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg  ???      = v3       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    core::fmt::Arguments<'{erased}>    4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _2    bool                              1b 1                var=1
// CHECK-NEXT: ; zst   _3    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; stack _4    core::panic::PanicInfo<'{erased}>    6b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 6, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i8, v3: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}> <- ByValPair(v0, v1): core::fmt::Arguments<'{erased}>
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v4
// CHECK-NEXT:     v5 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v1, v5
// CHECK-NEXT: ; write_cvalue: Var(_2, var1): bool <- ByVal(v2): bool
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
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks16maybe_is_aligned7runtimeCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:37(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks16maybe_is_aligned7runtimeCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=2
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = u0:46 sig0 ; Instance { def: Item(DefId(1:3348 ~ core[..]::ptr::const_ptr::{impl#0}::is_aligned_to)), args: [()] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::ptr::const_ptr::<impl *const ()>::is_aligned_to(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4 = call fn0(v2, v3)
// CHECK-NEXT:     v5 -> v4
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v4): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks17check_language_ub7runtimeCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:38() -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks17check_language_ub7runtimeCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Not(const false)
// CHECK-NEXT:     v0 = iconst.i8 0
// CHECK-NEXT:     v1 = iconst.i8 0
// CHECK-NEXT:     v2 = icmp eq v0, v1  ; v0 = 0, v1 = 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v2): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v2
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks23maybe_is_nonoverlapping7runtimeCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:39(i16, i16, i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks23maybe_is_nonoverlapping7runtimeCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _3      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg   _4      = v3       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=2
// CHECK-NEXT: ; ssa   _2    *const ()                         2b 2                var=3
// CHECK-NEXT: ; ssa   _3    usize                             2b 2                var=4
// CHECK-NEXT: ; ssa   _4    usize                             2b 2                var=5
// CHECK-NEXT: ; ssa   _5    usize                             2b 2                var=6
// CHECK-NEXT: ; ssa   _6    usize                             2b 2                var=7
// CHECK-NEXT: ; zst   _7    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _8    usize                             2b 2                var=8
// CHECK-NEXT: ; stack _9    core::option::Option<usize>       4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _10   isize                             2b 2                var=9
// CHECK-NEXT: ; ssa   _11   usize                             2b 2                var=10
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname3 ; alloc8
// CHECK-NEXT:     sig0 = (i16) -> i16 system_v
// CHECK-NEXT:     sig1 = (i16) -> i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i16 system_v
// CHECK-NEXT:     sig4 = (i16, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:13 sig0 ; Instance { def: Item(DefId(1:3306 ~ core[..]::ptr::const_ptr::{impl#0}::addr)), args: [()] }
// CHECK-NEXT:     fn1 = colocated u0:13 sig1 ; Instance { def: Item(DefId(1:3306 ~ core[..]::ptr::const_ptr::{impl#0}::addr)), args: [()] }
// CHECK-NEXT:     fn2 = colocated u0:20 sig2 ; Instance { def: Item(DefId(1:28338 ~ core[..]::num::{impl#11}::checked_mul)), args: [] }
// CHECK-NEXT:     fn3 = colocated u0:22 sig3 ; Instance { def: Item(DefId(1:28401 ~ core[..]::num::{impl#11}::abs_diff)), args: [] }
// CHECK-NEXT:     fn4 = colocated u0:25 sig4 ; Instance { def: Item(DefId(1:12250 ~ core[..]::panicking::panic_nounwind)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16, v3: i16):
// CHECK-NEXT:     v4 -> v0
// CHECK-NEXT:     v6 -> v1
// CHECK-NEXT:     v8 -> v2
// CHECK-NEXT:     v9 -> v3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): *const () <- ByVal(v1): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): usize <- ByVal(v2): usize
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): usize <- ByVal(v3): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::ptr::const_ptr::<impl *const ()>::addr(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v5 = call fn0(v4)
// CHECK-NEXT:     v19 -> v5
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): usize <- ByVal(v5): usize
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::ptr::const_ptr::<impl *const ()>::addr(copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v7 = call fn1(v6)
// CHECK-NEXT:     v20 -> v7
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): usize <- ByVal(v7): usize
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _9 = core::num::<impl usize>::checked_mul(copy _3, copy _4)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<usize>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u16, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u16, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u16 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(2 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(4 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u16, b_offset: Size(2 bytes) }, field_offsets: [Size(2 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8189316934619006962 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v10, v11 = call fn2(v8, v9)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::option::Option<usize> <- ByValPair(v10, v11): core::option::Option<usize>
// CHECK-NEXT:     v12 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v10, v12
// CHECK-NEXT:     v13 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v11, v13
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = discriminant(_9)
// CHECK-NEXT:     v14 = stack_addr.i16 ss0
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT: ; write_cvalue: Var(_10, var9): isize <- ByVal(v15): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _10)
// CHECK-NEXT:     v16 = uextend.i32 v15
// CHECK-NEXT:     br_table v16, block8, [block6, block5]
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = copy ((_9 as Some).0: usize)
// CHECK-NEXT: ; write_cvalue: Var(_8, var8): usize <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): usize
// CHECK-NEXT:     v17 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v18 = load.i16 notrap v17
// CHECK-NEXT:     v25 -> v18
// CHECK-NEXT: ;
// CHECK-NEXT: ; _11 = core::num::<impl usize>::abs_diff(copy _5, copy _6)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v21 = call fn3(v19, v20)
// CHECK-NEXT:     v24 -> v21
// CHECK-NEXT: ; write_cvalue: Var(_11, var10): usize <- ByVal(v21): usize
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = core::panicking::panic_nounwind(const "is_nonoverlapping: `size_of::<T>() * count` overflows a usize")
// CHECK-NEXT:     v22 = symbol_value.i16 gv0
// CHECK-NEXT:     v23 = iconst.i16 61
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: !, layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Primitive, largest_niche: None, uninhabited: true, variants: Empty, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn4(v22, v23)  ; v23 = 61
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Ge(copy _11, copy _8)
// CHECK-NEXT:     v26 = icmp.i16 uge v24, v25
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v26): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v26
// CHECK-EMPTY:
// CHECK-NEXT: block8 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; unreachable
// CHECK-NEXT:     trap user1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXNtNtCs40R3J0iFVxM_4core5array8equalityAhj5_NtNtB6_3cmp9PartialEq2eqCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:40(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXNtNtCs40R3J0iFVxM_4core5array8equalityAhj5_NtNtB6_3cmp9PartialEq2eqCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} [u8; 5_usize]
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} [u8; 5_usize]
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} [u8; 5_usize]          2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} [u8; 5_usize]          2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:41 sig0 ; Instance { def: Item(DefId(1:5481 ~ core[..]::array::equality::{impl#9}::spec_eq)), args: [u8, u8, 5_usize] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} [u8; 5_usize] <- ByVal(v0): &'{erased} [u8; 5_usize]
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} [u8; 5_usize] <- ByVal(v1): &'{erased} [u8; 5_usize]
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::array::equality::SpecArrayEq<U, N>>::spec_eq(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &[u8; 5], layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &[u8; 5], layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4 = call fn0(v2, v3)
// CHECK-NEXT:     v5 -> v4
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v4): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXs7_NtNtCs40R3J0iFVxM_4core5array8equalityhINtB5_11SpecArrayEqhKj5_E7spec_eqCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:41(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs7_NtNtCs40R3J0iFVxM_4core5array8equalityhINtB5_11SpecArrayEqhKj5_E7spec_eqCs7YLftNnbcrY_13runtime_calls
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} [u8; 5_usize]
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} [u8; 5_usize]
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} [u8; 5_usize]          2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} [u8; 5_usize]          2b 2                var=3
// CHECK-NEXT: ; ssa   _3    &'{erased} [u8; 5_usize]          2b 2                var=4
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16, i16, i16) -> i32 system_v
// CHECK-NEXT:     fn0 = u0:47 sig0 ; "memcmp"
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v3 -> v0
// CHECK-NEXT:     v2 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} [u8; 5_usize] <- ByVal(v0): &'{erased} [u8; 5_usize]
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} [u8; 5_usize] <- ByVal(v1): &'{erased} [u8; 5_usize]
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy _2 as &[T; N] (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_3, var4): &'{erased} [u8; 5_usize] <- ByVal(v2): &'{erased} [u8; 5_usize]
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::raw_eq::<[T; N]>(copy _1, move _3)
// CHECK-NEXT:     v4 = iconst.i16 5
// CHECK-NEXT: ; lib_call memcmp
// CHECK-NEXT:     v5 = call fn0(v3, v2, v4)  ; v4 = 5
// CHECK-NEXT:     v6 = iconst.i32 0
// CHECK-NEXT:     v7 = icmp eq v5, v6  ; v6 = 0
// CHECK-NEXT:     v8 -> v7
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v7): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v8
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol divide
// CHECK-NEXT: function u0:42(i32, i32) -> i32 system_v {
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
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc21
// CHECK-NEXT:     sig0 = (i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:32 sig0 ; "_RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_div_by_zeroCs7YLftNnbcrY_13runtime_calls"
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
// CHECK-NEXT: function u0:43() -> i8 system_v {
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
// CHECK-NEXT: ; ssa   _4    &'{erased} mut [u8]               4b 2                var=(3, 4)
// CHECK-NEXT: ; ssa   _5    &'{erased} mut [u8; 5_usize]      2b 2                var=5
// CHECK-NEXT: ; ssa   _6    *const u8                         2b 2                var=6
// CHECK-NEXT: ; ssa   _7    &'{erased} [u8]                   4b 2                var=(7, 8)
// CHECK-NEXT: ; ssa   _8    &'{erased} [u8; 3_usize]          2b 2                var=9
// CHECK-NEXT: ; ssa   _9    bool                              1b 1                var=10
// CHECK-NEXT: ; ssa   _10   bool                              1b 1                var=11
// CHECK-NEXT: ; ssa   _11   &'{erased} [u8; 5_usize]          2b 2                var=12
// CHECK-NEXT: ; ssa   _12   &'{erased} [u8; 5_usize]          2b 2                var=13
// CHECK-NEXT: ; ssa   _13   u32                               4b 2                var=14
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 5
// CHECK-NEXT:     gv0 = symbol colocated userextname1 ; alloc2
// CHECK-NEXT:     gv1 = symbol colocated userextname4 ; alloc4
// CHECK-NEXT:     sig0 = (i16, i32, i16) -> i16 system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i16 system_v
// CHECK-NEXT:     sig3 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig4 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig5 = (i32, i32) -> i32 system_v
// CHECK-NEXT:     fn0 = %Memset sig0
// CHECK-NEXT:     fn1 = colocated u0:10 sig1 ; Instance { def: Item(DefId(1:45346 ~ core[..]::slice::{impl#0}::as_mut_ptr)), args: [u8] }
// CHECK-NEXT:     fn2 = colocated u0:11 sig2 ; Instance { def: Item(DefId(1:45345 ~ core[..]::slice::{impl#0}::as_ptr)), args: [u8] }
// CHECK-NEXT:     fn3 = colocated u0:44 sig3 ; Instance { def: Item(DefId(0:3 ~ runtime_calls[..]::memory)), args: [] }
// CHECK-NEXT:     fn4 = colocated u0:40 sig4 ; Instance { def: Item(DefId(1:5402 ~ core[..]::array::equality::{impl#0}::eq)), args: [u8, u8, 5_usize] }
// CHECK-NEXT:     fn5 = colocated u0:42 sig5 ; Instance { def: Item(DefId(0:4 ~ runtime_calls[..]::divide)), args: [] }
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
// CHECK-NEXT: ; write_cvalue: Var(_5, var5): &'{erased} mut [u8; 5_usize] <- ByVal(v5): &'{erased} mut [u8; 5_usize]
// CHECK-NEXT: ; _4 = move _5 as &mut [u8] (PointerCoercion(Unsize, Implicit))
// CHECK-NEXT:     v6 = iconst.i16 5
// CHECK-NEXT: ; write_cvalue: VarPair(_4, var3, var4): &'{erased} mut [u8] <- ByValPair(v5, v6): &'{erased} mut [u8]
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::slice::<impl [u8]>::as_mut_ptr(move _4)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &mut [u8], layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *mut u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v7 = call fn1(v5, v6)  ; v6 = 5
// CHECK-NEXT:     v11 -> v7
// CHECK-NEXT: ; write_cvalue: Var(_3, var2): *mut u8 <- ByVal(v7): *mut u8
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = const gbc_main::promoted[1]
// CHECK-NEXT:     v8 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): &'{erased} [u8; 3_usize] <- ByVal(v8): &'{erased} [u8; 3_usize]
// CHECK-NEXT: ; _7 = move _8 as &[u8] (PointerCoercion(Unsize, Implicit))
// CHECK-NEXT:     v9 = iconst.i16 3
// CHECK-NEXT: ; write_cvalue: VarPair(_7, var7, var8): &'{erased} [u8] <- ByValPair(v8, v9): &'{erased} [u8]
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::slice::<impl [u8]>::as_ptr(move _7)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &[u8], layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *const u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v10 = call fn2(v8, v9)  ; v9 = 3
// CHECK-NEXT:     v12 -> v10
// CHECK-NEXT: ; write_cvalue: Var(_6, var6): *const u8 <- ByVal(v10): *const u8
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = memory(move _3, move _6, const 3_usize)
// CHECK-NEXT:     v13 = iconst.i16 3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *mut u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *const u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn3(v11, v12, v13)  ; v13 = 3
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _11 = &_1
// CHECK-NEXT:     v14 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_11, var12): &'{erased} [u8; 5_usize] <- ByVal(v14): &'{erased} [u8; 5_usize]
// CHECK-NEXT: ; _12 = const gbc_main::promoted[0]
// CHECK-NEXT:     v15 = symbol_value.i16 gv1
// CHECK-NEXT: ; write_cvalue: Var(_12, var13): &'{erased} [u8; 5_usize] <- ByVal(v15): &'{erased} [u8; 5_usize]
// CHECK-NEXT: ;
// CHECK-NEXT: ; _10 = <[u8; 5] as core::cmp::PartialEq>::eq(move _11, move _12)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &[u8; 5], layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &[u8; 5], layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v16 = call fn4(v14, v15)
// CHECK-NEXT:     v17 -> v16
// CHECK-NEXT: ; write_cvalue: Var(_10, var11): bool <- ByVal(v16): bool
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _10)
// CHECK-NEXT:     brif.i8 v17, block6, block7
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = divide(const 1000_u32, const 7_u32)
// CHECK-NEXT:     v18 = iconst.i32 1000
// CHECK-NEXT:     v19 = iconst.i32 7
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v20 = call fn5(v18, v19)  ; v18 = 1000, v19 = 7
// CHECK-NEXT:     v22 -> v20
// CHECK-NEXT: ; write_cvalue: Var(_13, var14): u32 <- ByVal(v20): u32
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = const false
// CHECK-NEXT:     v21 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_9, var10): bool <- ByVal(v21): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block9(v21)  ; v21 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = Eq(move _13, const 142_u32)
// CHECK-NEXT:     v23 = iconst.i32 142
// CHECK-NEXT:     v24 = icmp.i32 eq v22, v23  ; v23 = 142
// CHECK-NEXT: ; write_cvalue: Var(_9, var10): bool <- ByVal(v24): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block9(v24)
// CHECK-EMPTY:
// CHECK-NEXT: block9(v25: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _9 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v25): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v25
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol memory
// CHECK-NEXT: function u0:44(i16, i16, i16) system_v {
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
// CHECK-NEXT: ; zst   _5    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _6    *const u8                         2b 2                var=4
// CHECK-NEXT: ; ssa   _7    *mut u8                           2b 2                var=5
// CHECK-NEXT: ; zst   _8    ()                                0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT:     gv0 = symbol colocated userextname1 ; alloc22
// CHECK-NEXT:     sig0 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig1 = (i16, i16, i16) -> i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig3 = (i16, i8, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:5 sig0 ; Instance { def: Item(DefId(1:3504 ~ core[..]::ptr::copy_nonoverlapping)), args: [u8] }
// CHECK-NEXT:     fn1 = colocated u0:12 sig1 ; Instance { def: Item(DefId(1:3441 ~ core[..]::ptr::mut_ptr::{impl#0}::add)), args: [u8] }
// CHECK-NEXT:     fn2 = colocated u0:6 sig2 ; Instance { def: Item(DefId(1:3506 ~ core[..]::ptr::copy)), args: [u8] }
// CHECK-NEXT:     fn3 = colocated u0:4 sig3 ; Instance { def: Item(DefId(1:3508 ~ core[..]::ptr::write_bytes)), args: [u8] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     v4 -> v0
// CHECK-NEXT:     v6 -> v0
// CHECK-NEXT:     v10 -> v0
// CHECK-NEXT:     v13 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v5 -> v2
// CHECK-NEXT:     v12 -> v2
// CHECK-NEXT:     v15 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut u8 <- ByVal(v0): *mut u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): *const u8 <- ByVal(v1): *const u8
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v2): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::ptr::copy_nonoverlapping::<u8>(copy _2, copy _1, copy _3)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *mut u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn0(v3, v4, v5)
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy _1 as *const u8 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_6, var4): *const u8 <- ByVal(v6): *const u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = core::ptr::mut_ptr::<impl *mut u8>::add(copy _1, const 1_usize)
// CHECK-NEXT:     v7 = iconst.i16 1
// CHECK-NEXT:     v8 = symbol_value.i16 gv0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *mut u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: *mut u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v9 = call fn1(v6, v7, v8)  ; v7 = 1
// CHECK-NEXT:     v11 -> v9
// CHECK-NEXT: ; write_cvalue: Var(_7, var5): *mut u8 <- ByVal(v9): *mut u8
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::ptr::copy::<u8>(move _6, move _7, copy _3)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *mut u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v10, v11, v12)
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = core::ptr::write_bytes::<u8>(copy _1, const 7_u8, copy _3)
// CHECK-NEXT:     v14 = iconst.i8 7
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *mut u8, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn3(v13, v14, v15)  ; v14 = 7
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
