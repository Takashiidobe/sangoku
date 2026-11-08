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
// CHECK-NEXT: ; symbol _RNvMs9_NtCs40R3J0iFVxM_4core3numj11checked_mulCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:0(i16, i16) -> i16, i16 system_v {
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
// CHECK-NEXT: ; ssa   _7    u16                               2b 2                var=8
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=9
// CHECK-NEXT: ; ssa   _9    (u16, bool)                       4b 2                var=(10, 11)
// CHECK-NEXT: ; ssa   _10   u16                               2b 2                var=12
// CHECK-NEXT: ; ssa   _11   u16                               2b 2                var=13
// CHECK-NEXT: ; ssa   _12   usize                             2b 2                var=14
// CHECK-NEXT: ; zst   _13   ()                                0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
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
// CHECK-NEXT: ; _10 = copy _1 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): u16 <- ByVal(v2): u16
// CHECK-NEXT: ; _11 = copy _2 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): u16 <- ByVal(v3): u16
// CHECK-NEXT: ; _9 = MulWithOverflow(move _10, move _11)
// CHECK-NEXT:     v4 = uextend.i32 v2
// CHECK-NEXT:     v5 = uextend.i32 v3
// CHECK-NEXT:     v6 = imul v4, v5
// CHECK-NEXT:     v7 = iconst.i32 0xffff
// CHECK-NEXT:     v8 = icmp ugt v6, v7  ; v7 = 0xffff
// CHECK-NEXT:     v9 = ireduce.i16 v6
// CHECK-NEXT:     v32 -> v9
// CHECK-NEXT:     v33 -> v9
// CHECK-NEXT: ; write_cvalue: VarPair(_9, var10, var11): (u16, bool) <- ByValPair(v9, v8): (u16, bool)
// CHECK-NEXT: ; _7 = copy (_9.0: u16)
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): u16 <- ByVal(v9): u16
// CHECK-NEXT: ; _8 = copy (_9.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): bool <- ByVal(v8): bool
// CHECK-NEXT: ; _12 = copy _7 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_12, var14): usize <- ByVal(v9): usize
// CHECK-NEXT: ; _5 = (move _12, copy _8)
// CHECK-NEXT: ; write_cvalue: Var(_5, var5): usize <- ByVal(v9): usize
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): bool <- ByVal(v8): bool
// CHECK-NEXT: ; _3 = copy (_5.0: usize)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v9): usize
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v8): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _4)
// CHECK-NEXT:     brif v8, block6, block8
// CHECK-EMPTY:
// CHECK-NEXT: block2:
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
// CHECK-NEXT: block3:
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
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v24 = stack_addr.i16 ss0
// CHECK-NEXT:     v25 = load.i16 notrap v24
// CHECK-NEXT:     v26 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v27 = load.i16 notrap v26
// CHECK-NEXT:     return v25, v27
// CHECK-EMPTY:
// CHECK-NEXT: block5(v28: i8, v31: i16):
// CHECK-NEXT:     v16 -> v31
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     brif v28, block2, block3
// CHECK-EMPTY:
// CHECK-NEXT: block6 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = core::intrinsics::cold_path()
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = const true
// CHECK-NEXT:     v29 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v29): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block5(v29, v32)  ; v29 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = const false
// CHECK-NEXT:     v30 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v30): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block5(v30, v33)  ; v30 = 0
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs9_NtCs40R3J0iFVxM_4core3numj8abs_diffCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:1(i16, i16) -> i16 system_v {
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
// CHECK-NEXT: ; ssa   _4    u32                               4b 2                var=5
// CHECK-NEXT: ; ssa   _5    i32                               4b 2                var=6
// CHECK-NEXT: ; ssa   _6    i32                               4b 2                var=7
// CHECK-NEXT: ; ssa   _7    i32                               4b 2                var=8
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=9
// CHECK-NEXT: ; ssa   _9    i32                               4b 2                var=10
// CHECK-NEXT: ; ssa   _10   bool                              1b 1                var=11
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v5 -> v0
// CHECK-NEXT:     v12 -> v0
// CHECK-NEXT:     v16 -> v0
// CHECK-NEXT:     v18 -> v0
// CHECK-NEXT:     v7 -> v1
// CHECK-NEXT:     v13 -> v1
// CHECK-NEXT:     v15 -> v1
// CHECK-NEXT:     v19 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): usize <- ByVal(v0): usize
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = Eq(const <usize as core::mem::SizedTypeProperties>::SIZE, const 1_usize)
// CHECK-NEXT:     v2 = iconst.i16 2
// CHECK-NEXT:     v3 = iconst.i16 1
// CHECK-NEXT:     v4 = icmp eq v2, v3  ; v2 = 2, v3 = 1
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): bool <- ByVal(v4): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _3)
// CHECK-NEXT:     brif v4, block2, block3
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy _1 as i32 (IntToInt)
// CHECK-NEXT:     v6 = uextend.i32 v5
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): i32 <- ByVal(v6): i32
// CHECK-NEXT: ; _7 = copy _2 as i32 (IntToInt)
// CHECK-NEXT:     v8 = uextend.i32 v7
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): i32 <- ByVal(v8): i32
// CHECK-NEXT: ; _5 = Sub(copy _6, copy _7)
// CHECK-NEXT:     v9 = isub v6, v8
// CHECK-NEXT:     v25 -> v9
// CHECK-NEXT:     v27 -> v9
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): i32 <- ByVal(v9): i32
// CHECK-NEXT: ; _10 = Lt(copy _5, const 0_i32)
// CHECK-NEXT:     v10 = iconst.i32 0
// CHECK-NEXT:     v11 = icmp slt v9, v10  ; v10 = 0
// CHECK-NEXT: ; write_cvalue: Var(_10, var11): bool <- ByVal(v11): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _10)
// CHECK-NEXT:     brif v11, block8, block9
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = Lt(copy _1, copy _2)
// CHECK-NEXT:     v14 = icmp.i16 ult v12, v13
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): bool <- ByVal(v14): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _8)
// CHECK-NEXT:     brif v14, block4, block5
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Sub(copy _2, copy _1)
// CHECK-NEXT:     v17 = isub.i16 v15, v16
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v17): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v17
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v20 = isub.i16 v18, v19
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v20): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v20
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     v28 = iconst.i16 0
// CHECK-NEXT:     v21 -> v28
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v21  ; v21 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block7(v22: i32):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = move _9 as u32 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): u32 <- ByVal(v22): u32
// CHECK-NEXT: ; _0 = move _4 as usize (IntToInt)
// CHECK-NEXT:     v23 = ireduce.i16 v22
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v23): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v23
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = Sub(const 0_i32, copy _5)
// CHECK-NEXT:     v24 = iconst.i32 0
// CHECK-NEXT:     v26 = isub v24, v25  ; v24 = 0
// CHECK-NEXT: ; write_cvalue: Var(_9, var10): i32 <- ByVal(v26): i32
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block7(v26)
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = copy _5
// CHECK-NEXT: ; write_cvalue: Var(_9, var10): i32 <- ByVal(v27): i32
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block7(v27)
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:2() system_v {
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
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks29maybe_is_aligned_and_not_nullCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:3(i16, i16, i8 uext) -> i8 uext system_v {
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
// CHECK-NEXT: ; ssa   _6    (*const (), usize)                4b 2                var=(7, 8)
// CHECK-NEXT: ; ssa   _7    *const ()                         2b 2                var=9
// CHECK-NEXT: ; ssa   _8    usize                             2b 2                var=10
// CHECK-NEXT: ; ssa   _9    bool                              1b 1                var=11
// CHECK-NEXT: ; ssa   _10   usize                             2b 2                var=12
// CHECK-NEXT: ; ssa   _11   usize                             2b 2                var=13
// CHECK-NEXT: ; ssa   _12   usize                             2b 2                var=14
// CHECK-NEXT: ; ssa   _13   u32                               4b 2                var=15
// CHECK-NEXT: ; zst   _14   !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _15   *const u8                         2b 2                var=16
// CHECK-NEXT: ; stack _16   (*const u8,)                      2b 2                storage=ss0
// CHECK-NEXT: ; ssa   _17   *const u8                         2b 2                var=17
// CHECK-NEXT: ; ssa   _18   usize                             2b 2                var=18
// CHECK-NEXT: ; ssa   _19   *const ()                         2b 2                var=19
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i8):
// CHECK-NEXT:     v3 -> v0
// CHECK-NEXT:     v32 -> v0
// CHECK-NEXT:     v35 -> v0
// CHECK-NEXT:     v30 -> v35
// CHECK-NEXT:     v10 -> v30
// CHECK-NEXT:     v19 -> v32
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     v33 -> v1
// CHECK-NEXT:     v20 -> v33
// CHECK-NEXT:     v34 -> v2
// CHECK-NEXT:     v29 -> v34
// CHECK-NEXT:     v7 -> v29
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v1): usize
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): bool <- ByVal(v2): bool
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = (copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): *const () <- ByVal(v3): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_6, var8): usize <- ByVal(v4): usize
// CHECK-NEXT: ; _7 = move (_6.0: *const ())
// CHECK-NEXT: ; write_cvalue: Var(_7, var9): *const () <- ByVal(v3): *const ()
// CHECK-NEXT: ; _8 = move (_6.1: usize)
// CHECK-NEXT: ; write_cvalue: Var(_8, var10): usize <- ByVal(v4): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = core::intrinsics::ctpop::<usize>(copy _8)
// CHECK-NEXT:     v5 = popcnt.i16 v4
// CHECK-NEXT:     v6 = uextend.i32 v5
// CHECK-NEXT:     v26 -> v6
// CHECK-NEXT: ; write_cvalue: Var(_13, var15): u32 <- ByVal(v6): u32
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _3)
// CHECK-NEXT:     brif.i8 v7, block4, block5
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const false
// CHECK-NEXT:     v8 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v8): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v8  ; v8 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const true
// CHECK-NEXT:     v9 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v9): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v9  ; v9 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = copy _1 as *const u8 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_15, var16): *const u8 <- ByVal(v10): *const u8
// CHECK-NEXT: ; _16 = (copy _15,)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): *const u8 <- ByVal(v10): *const u8
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v10, v11
// CHECK-NEXT: ; _17 = move (_16.0: *const u8)
// CHECK-NEXT: ; write_cvalue: Var(_17, var17): *const u8 <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): *const u8
// CHECK-NEXT:     v12 = stack_addr.i16 ss0
// CHECK-NEXT:     v13 = load.i16 notrap v12
// CHECK-NEXT: ; _19 = copy _17 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_19, var19): *const () <- ByVal(v13): *const ()
// CHECK-NEXT: ; _18 = move _19 as usize (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_18, var18): usize <- ByVal(v13): *const ()
// CHECK-NEXT: ; _5 = Eq(move _18, const 0_usize)
// CHECK-NEXT:     v14 = iconst.i16 0
// CHECK-NEXT:     v15 = icmp eq v13, v14  ; v14 = 0
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): bool <- ByVal(v15): bool
// CHECK-NEXT: ; _0 = Not(move _5)
// CHECK-NEXT:     v16 = iconst.i8 0
// CHECK-NEXT:     v17 = icmp eq v15, v16  ; v16 = 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v17): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v17
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     v31 = iconst.i8 0
// CHECK-NEXT:     v18 -> v31
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v18  ; v18 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _11 = copy _7 as usize (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_11, var13): usize <- ByVal(v19): *const ()
// CHECK-NEXT: ; _12 = Sub(copy _8, const 1_usize)
// CHECK-NEXT:     v21 = iconst.i16 1
// CHECK-NEXT:     v22 = isub.i16 v20, v21  ; v21 = 1
// CHECK-NEXT: ; write_cvalue: Var(_12, var14): usize <- ByVal(v22): usize
// CHECK-NEXT: ; _10 = BitAnd(move _11, move _12)
// CHECK-NEXT:     v23 = band.i16 v19, v22
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): usize <- ByVal(v23): usize
// CHECK-NEXT: ; _4 = Eq(move _10, const 0_usize)
// CHECK-NEXT:     v24 = iconst.i16 0
// CHECK-NEXT:     v25 = icmp eq v23, v24  ; v24 = 0
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): bool <- ByVal(v25): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _4)
// CHECK-NEXT:     brif v23, block3, block2
// CHECK-EMPTY:
// CHECK-NEXT: block8 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _14 = core::intrinsics::abort_immediate()
// CHECK-NEXT:     trap user2
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = Eq(move _13, const 1_u32)
// CHECK-NEXT:     v27 = iconst.i32 1
// CHECK-NEXT:     v28 = icmp.i32 eq v26, v27  ; v27 = 1
// CHECK-NEXT: ; write_cvalue: Var(_9, var11): bool <- ByVal(v28): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _9)
// CHECK-NEXT:     brif v28, block7, block8
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_div_by_zeroCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:4(i16) system_v {
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
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr11write_bytes18precondition_checkCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:5(i16, i16, i8 uext, i16) system_v {
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
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16, i16, i8 uext) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:3 sig0 ; Instance { def: Item(DefId(1:3623 ~ core[..]::ub_checks::maybe_is_aligned_and_not_null)), args: [] }
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
// CHECK-NEXT: block4 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::intrinsics::abort_immediate()
// CHECK-NEXT:     trap user2
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr19copy_nonoverlapping18precondition_checkCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:6(i16, i16, i16, i16, i16, i16) system_v {
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
// CHECK-NEXT: ; stack _15   (*const (), *const (), usize, usize)    8b 2                storage=ss0
// CHECK-NEXT: ; zst   _16   !                                 0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 8, align = 2
// CHECK-NEXT:     sig0 = (i16, i16, i8 uext) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16, i8 uext) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:3 sig0 ; Instance { def: Item(DefId(1:3623 ~ core[..]::ub_checks::maybe_is_aligned_and_not_null)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(1:3623 ~ core[..]::ub_checks::maybe_is_aligned_and_not_null)), args: [] }
// CHECK-NEXT:     fn2 = colocated u0:8 sig2 ; Instance { def: Item(DefId(1:29567 ~ core[..]::ub_checks::maybe_is_nonoverlapping::runtime)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16, v3: i16, v4: i16, v5: i16):
// CHECK-NEXT:     v14 -> v0
// CHECK-NEXT:     v24 -> v0
// CHECK-NEXT:     v18 -> v1
// CHECK-NEXT:     v23 -> v1
// CHECK-NEXT:     v42 -> v1
// CHECK-NEXT:     v10 -> v2
// CHECK-NEXT:     v27 -> v2
// CHECK-NEXT:     v43 -> v2
// CHECK-NEXT:     v15 -> v3
// CHECK-NEXT:     v20 -> v3
// CHECK-NEXT:     v6 -> v4
// CHECK-NEXT:     v29 -> v4
// CHECK-NEXT:     v44 -> v4
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
// CHECK-NEXT: ; _15 = (copy _1, copy _14, copy _3, copy _5)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): *const () <- ByVal(v24): *const ()
// CHECK-NEXT:     v25 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v24, v25
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): *const () <- ByVal(v23): *const ()
// CHECK-NEXT:     v26 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store.i16 notrap v23, v26
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(4) }, None): usize <- ByVal(v27): usize
// CHECK-NEXT:     v28 = stack_addr.i16 ss0+4
// CHECK-NEXT:     store.i16 notrap v27, v28
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(6) }, None): usize <- ByVal(v29): usize
// CHECK-NEXT:     v30 = stack_addr.i16 ss0+6
// CHECK-NEXT:     store.i16 notrap v29, v30
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::ub_checks::maybe_is_nonoverlapping::runtime(move (_15.0: *const ()), move (_15.1: *const ()), move (_15.2: usize), move (_15.3: usize))
// CHECK-NEXT:     v31 = stack_addr.i16 ss0
// CHECK-NEXT:     v32 = load.i16 notrap v31
// CHECK-NEXT:     v33 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v34 = load.i16 notrap v33
// CHECK-NEXT:     v35 = stack_addr.i16 ss0+4
// CHECK-NEXT:     v36 = load.i16 notrap v35
// CHECK-NEXT:     v37 = stack_addr.i16 ss0+6
// CHECK-NEXT:     v38 = load.i16 notrap v37
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 4, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v39 = call fn2(v32, v34, v36, v38)
// CHECK-NEXT: ; write_cvalue: Var(_6, var6): bool <- ByVal(v39): bool
// CHECK-NEXT:     jump block10(v39)
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = const false
// CHECK-NEXT:     v40 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_6, var6): bool <- ByVal(v40): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block10(v40)  ; v40 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block10(v41: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     brif v41, block12, block11
// CHECK-EMPTY:
// CHECK-NEXT: block11 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _16 = core::intrinsics::abort_immediate()
// CHECK-NEXT:     trap user2
// CHECK-EMPTY:
// CHECK-NEXT: block12:
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
// CHECK-NEXT: function u0:7(i16, i16, i16, i8 uext, i16) system_v {
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
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16, i16, i8 uext) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16, i8 uext) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:3 sig0 ; Instance { def: Item(DefId(1:3623 ~ core[..]::ub_checks::maybe_is_aligned_and_not_null)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(1:3623 ~ core[..]::ub_checks::maybe_is_aligned_and_not_null)), args: [] }
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
// CHECK-NEXT: block6 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = core::intrinsics::abort_immediate()
// CHECK-NEXT:     trap user2
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks23maybe_is_nonoverlapping7runtimeCs7YLftNnbcrY_13runtime_calls
// CHECK-NEXT: function u0:8(i16, i16, i16, i16) -> i8 uext system_v {
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
// CHECK-NEXT: ; ssa   _7    usize                             2b 2                var=8
// CHECK-NEXT: ; stack _8    core::option::Option<usize>       4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _9    isize                             2b 2                var=9
// CHECK-NEXT: ; ssa   _10   usize                             2b 2                var=10
// CHECK-NEXT: ; zst   _11   !                                 0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:0 sig0 ; Instance { def: Item(DefId(1:28338 ~ core[..]::num::{impl#11}::checked_mul)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(1:28401 ~ core[..]::num::{impl#11}::abs_diff)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16, v3: i16):
// CHECK-NEXT:     v4 -> v0
// CHECK-NEXT:     v17 -> v0
// CHECK-NEXT:     v5 -> v1
// CHECK-NEXT:     v18 -> v1
// CHECK-NEXT:     v6 -> v2
// CHECK-NEXT:     v7 -> v3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): *const () <- ByVal(v1): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): usize <- ByVal(v2): usize
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): usize <- ByVal(v3): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = copy _1 as usize (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_5, var6): usize <- ByVal(v4): *const ()
// CHECK-NEXT: ; _6 = copy _2 as usize (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_6, var7): usize <- ByVal(v5): *const ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = core::num::<impl usize>::checked_mul(copy _3, copy _4)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<usize>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u16, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u16, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u16 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(2 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(4 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union u16, b_offset: Size(2 bytes) }, field_offsets: [Size(2 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8189316934619006962 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v8, v9 = call fn0(v6, v7)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::option::Option<usize> <- ByValPair(v8, v9): core::option::Option<usize>
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v8, v10
// CHECK-NEXT:     v11 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v9, v11
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = discriminant(_8)
// CHECK-NEXT:     v12 = stack_addr.i16 ss0
// CHECK-NEXT:     v13 = load.i16 notrap v12
// CHECK-NEXT: ; write_cvalue: Var(_9, var9): isize <- ByVal(v13): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _9)
// CHECK-NEXT:     v14 = uextend.i32 v13
// CHECK-NEXT:     br_table v14, block6, [block4, block3]
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _7 = copy ((_8 as Some).0: usize)
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): usize <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): usize
// CHECK-NEXT:     v15 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v16 = load.i16 notrap v15
// CHECK-NEXT:     v21 -> v16
// CHECK-NEXT: ;
// CHECK-NEXT: ; _10 = core::num::<impl usize>::abs_diff(copy _5, copy _6)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v19 = call fn1(v17, v18)
// CHECK-NEXT:     v20 -> v19
// CHECK-NEXT: ; write_cvalue: Var(_10, var10): usize <- ByVal(v19): usize
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block4 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _11 = core::intrinsics::abort_immediate()
// CHECK-NEXT:     trap user2
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Ge(copy _10, copy _7)
// CHECK-NEXT:     v22 = icmp.i16 uge v20, v21
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v22): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v22
// CHECK-EMPTY:
// CHECK-NEXT: block6 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; unreachable
// CHECK-NEXT:     trap user1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol divide
// CHECK-NEXT: function u0:9(i32, i32) -> i32 system_v {
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
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; "_RNvNtNtCs40R3J0iFVxM_4core9panicking11panic_const23panic_const_div_by_zeroCs7YLftNnbcrY_13runtime_calls"
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
// CHECK-NEXT: function u0:10() -> i8 system_v {
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
// CHECK-NEXT: ; ssa   _8    &'{erased} [u8; 3_usize]          2b 2                var=5
// CHECK-NEXT: ; ssa   _9    bool                              1b 1                var=6
// CHECK-NEXT: ; ssa   _10   bool                              1b 1                var=7
// CHECK-NEXT: ; ssa   _11   &'{erased} [u8; 5_usize]          2b 2                var=8
// CHECK-NEXT: ; ssa   _12   &'{erased} [u8; 5_usize]          2b 2                var=9
// CHECK-NEXT: ; ssa   _13   u32                               4b 2                var=10
// CHECK-NEXT: ; ssa   _14   *mut [u8]                         4b 2                var=(11, 12)
// CHECK-NEXT: ; ssa   _15   *const [u8]                       4b 2                var=(13, 14)
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
// CHECK-NEXT:     fn1 = colocated u0:11 sig1 ; Instance { def: Item(DefId(0:3 ~ runtime_calls[..]::memory)), args: [] }
// CHECK-NEXT:     fn2 = u0:12 sig2 ; "memcmp"
// CHECK-NEXT:     fn3 = colocated u0:9 sig3 ; Instance { def: Item(DefId(0:4 ~ runtime_calls[..]::divide)), args: [] }
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
// CHECK-NEXT: ; _4 = move _5 as &mut [u8] (PointerCoercion(Unsize, Implicit))
// CHECK-NEXT:     v6 = iconst.i16 5
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} mut [u8] <- ByValPair(v5, v6): &'{erased} mut [u8]
// CHECK-NEXT:     v7 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v5, v7
// CHECK-NEXT:     v8 = stack_addr.i16 ss1+2
// CHECK-NEXT:     store notrap v6, v8  ; v6 = 5
// CHECK-NEXT: ; _14 = &raw mut (*_4)
// CHECK-NEXT:     v9 = stack_addr.i16 ss1
// CHECK-NEXT:     v10 = load.i16 notrap v9
// CHECK-NEXT:     v11 = stack_addr.i16 ss1+2
// CHECK-NEXT:     v12 = load.i16 notrap v11
// CHECK-NEXT: ; write_cvalue: VarPair(_14, var11, var12): *mut [u8] <- ByValPair(v10, v12): *mut [u8]
// CHECK-NEXT: ; _3 = copy _14 as *mut u8 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var2): *mut u8 <- ByVal(v10): *mut u8
// CHECK-NEXT: ; _8 = const gbc_main::promoted[1]
// CHECK-NEXT:     v13 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_8, var5): &'{erased} [u8; 3_usize] <- ByVal(v13): &'{erased} [u8; 3_usize]
// CHECK-NEXT: ; _7 = move _8 as &[u8] (PointerCoercion(Unsize, Implicit))
// CHECK-NEXT:     v14 = iconst.i16 3
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): &'{erased} [u8] <- ByValPair(v13, v14): &'{erased} [u8]
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v13, v15
// CHECK-NEXT:     v16 = stack_addr.i16 ss2+2
// CHECK-NEXT:     store notrap v14, v16  ; v14 = 3
// CHECK-NEXT: ; _15 = &raw const (*_7)
// CHECK-NEXT:     v17 = stack_addr.i16 ss2
// CHECK-NEXT:     v18 = load.i16 notrap v17
// CHECK-NEXT:     v19 = stack_addr.i16 ss2+2
// CHECK-NEXT:     v20 = load.i16 notrap v19
// CHECK-NEXT: ; write_cvalue: VarPair(_15, var13, var14): *const [u8] <- ByValPair(v18, v20): *const [u8]
// CHECK-NEXT: ; _6 = copy _15 as *const u8 (PtrToPtr)
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
// CHECK-NEXT: ; _11 = &_1
// CHECK-NEXT:     v22 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_11, var8): &'{erased} [u8; 5_usize] <- ByVal(v22): &'{erased} [u8; 5_usize]
// CHECK-NEXT: ; _12 = const gbc_main::promoted[0]
// CHECK-NEXT:     v23 = symbol_value.i16 gv1
// CHECK-NEXT: ; write_cvalue: Var(_12, var9): &'{erased} [u8; 5_usize] <- ByVal(v23): &'{erased} [u8; 5_usize]
// CHECK-NEXT: ;
// CHECK-NEXT: ; _10 = core::intrinsics::raw_eq::<[u8; 5]>(copy _11, copy _12)
// CHECK-NEXT:     v24 = iconst.i16 5
// CHECK-NEXT: ; lib_call memcmp
// CHECK-NEXT:     v25 = call fn2(v22, v23, v24)  ; v24 = 5
// CHECK-NEXT:     v26 = iconst.i32 0
// CHECK-NEXT:     v27 = icmp eq v25, v26  ; v26 = 0
// CHECK-NEXT:     v36 -> v27
// CHECK-NEXT: ; write_cvalue: Var(_10, var7): bool <- ByVal(v27): bool
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = divide(const 1000_u32, const 7_u32)
// CHECK-NEXT:     v28 = iconst.i32 1000
// CHECK-NEXT:     v29 = iconst.i32 7
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u32, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: Scalar(u32 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322841091 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v30 = call fn3(v28, v29)  ; v28 = 1000, v29 = 7
// CHECK-NEXT:     v32 -> v30
// CHECK-NEXT: ; write_cvalue: Var(_13, var10): u32 <- ByVal(v30): u32
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = const false
// CHECK-NEXT:     v31 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_9, var6): bool <- ByVal(v31): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block6(v31)  ; v31 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = Eq(move _13, const 142_u32)
// CHECK-NEXT:     v33 = iconst.i32 142
// CHECK-NEXT:     v34 = icmp.i32 eq v32, v33  ; v33 = 142
// CHECK-NEXT: ; write_cvalue: Var(_9, var6): bool <- ByVal(v34): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block6(v34)
// CHECK-EMPTY:
// CHECK-NEXT: block6(v35: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _9 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v35): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v35
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _10)
// CHECK-NEXT:     brif.i8 v36, block3, block4
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol memory
// CHECK-NEXT: function u0:11(i16, i16, i16) system_v {
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
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=6
// CHECK-NEXT: ; zst   _9    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _10   *const ()                         2b 2                var=7
// CHECK-NEXT: ; ssa   _11   *mut ()                           2b 2                var=8
// CHECK-NEXT: ; ssa   _12   bool                              1b 1                var=9
// CHECK-NEXT: ; ssa   _13   bool                              1b 1                var=10
// CHECK-NEXT: ; zst   _14   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _15   *const ()                         2b 2                var=11
// CHECK-NEXT: ; ssa   _16   *mut ()                           2b 2                var=12
// CHECK-NEXT: ; ssa   _17   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _18   bool                              1b 1                var=14
// CHECK-NEXT: ; ssa   _19   bool                              1b 1                var=15
// CHECK-NEXT: ; zst   _20   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _21   *const ()                         2b 2                var=16
// CHECK-NEXT: ; ssa   _22   bool                              1b 1                var=17
// CHECK-NEXT: ; ssa   _23   bool                              1b 1                var=18
// CHECK-EMPTY:
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; alloc15
// CHECK-NEXT:     gv1 = symbol colocated userextname2 ; alloc16
// CHECK-NEXT:     gv2 = symbol colocated userextname4 ; alloc17
// CHECK-NEXT:     sig0 = (i16, i16, i16, i16, i16, i16) system_v
// CHECK-NEXT:     sig1 = (i16, i16, i16) -> i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16, i8 uext, i16) system_v
// CHECK-NEXT:     sig3 = (i16, i16, i16) -> i16 system_v
// CHECK-NEXT:     sig4 = (i16, i16, i8 uext, i16) system_v
// CHECK-NEXT:     sig5 = (i16, i32, i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:6 sig0 ; Instance { def: Item(DefId(1:29532 ~ core[..]::ptr::copy_nonoverlapping::precondition_check)), args: [] }
// CHECK-NEXT:     fn1 = %Memcpy sig1
// CHECK-NEXT:     fn2 = colocated u0:7 sig2 ; Instance { def: Item(DefId(1:29533 ~ core[..]::ptr::copy::precondition_check)), args: [] }
// CHECK-NEXT:     fn3 = %Memmove sig3
// CHECK-NEXT:     fn4 = colocated u0:5 sig4 ; Instance { def: Item(DefId(1:29534 ~ core[..]::ptr::write_bytes::precondition_check)), args: [] }
// CHECK-NEXT:     fn5 = %Memset sig5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     v62 -> v0
// CHECK-NEXT:     v60 -> v1
// CHECK-NEXT:     v64 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut u8 <- ByVal(v0): *mut u8
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): *const u8 <- ByVal(v1): *const u8
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): usize <- ByVal(v2): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _12 = Not(const false)
// CHECK-NEXT:     v3 = iconst.i8 0
// CHECK-NEXT:     v4 = iconst.i8 0
// CHECK-NEXT:     v5 = icmp eq v3, v4  ; v3 = 0, v4 = 0
// CHECK-NEXT: ; write_cvalue: Var(_12, var9): bool <- ByVal(v5): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _12)
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = copy _2 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_10, var7): *const () <- ByVal(v6): *const ()
// CHECK-NEXT: ; _11 = copy _1 as *mut () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_11, var8): *mut () <- ByVal(v7): *mut ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; _9 = core::ptr::copy_nonoverlapping::precondition_check(move _10, move _11, const <u8 as core::mem::SizedTypeProperties>::SIZE, const <u8 as core::mem::SizedTypeProperties>::ALIGN, copy _3)
// CHECK-NEXT:     v8 = iconst.i16 1
// CHECK-NEXT:     v9 = iconst.i16 1
// CHECK-NEXT:     v11 = symbol_value.i16 gv0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *mut (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 5, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn0(v6, v7, v8, v9, v10, v11)  ; v8 = 1, v9 = 1
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; copy_nonoverlapping(dst = copy _1, src = copy _2, count = copy _3)
// CHECK-NEXT:     v15 = call fn1(v12, v13, v14)
// CHECK-NEXT: ; _5 = copy _1 as *const u8 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): *const u8 <- ByVal(v12): *const u8
// CHECK-NEXT: ; _6 = Offset(copy _1, const 1_usize)
// CHECK-NEXT:     v16 = iconst.i16 1
// CHECK-NEXT:     v17 = iconst.i16 1
// CHECK-NEXT:     v18 = imul v16, v17  ; v16 = 1, v17 = 1
// CHECK-NEXT:     v19 = iadd.i16 v12, v18
// CHECK-NEXT:     v74 -> v19
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): *mut u8 <- ByVal(v19): *mut u8
// CHECK-NEXT: ; _18 = Not(const false)
// CHECK-NEXT:     v20 = iconst.i8 0
// CHECK-NEXT:     v21 = iconst.i8 0
// CHECK-NEXT:     v22 = icmp eq v20, v21  ; v20 = 0, v21 = 0
// CHECK-NEXT: ; write_cvalue: Var(_18, var14): bool <- ByVal(v22): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _18)
// CHECK-NEXT:     jump block11
// CHECK-EMPTY:
// CHECK-NEXT: block4(v23: i8, v57: i16, v58: i16, v59: i16):
// CHECK-NEXT:     v6 -> v57
// CHECK-NEXT:     v13 -> v57
// CHECK-NEXT:     v7 -> v58
// CHECK-NEXT:     v12 -> v58
// CHECK-NEXT:     v72 -> v58
// CHECK-NEXT:     v10 -> v59
// CHECK-NEXT:     v14 -> v59
// CHECK-NEXT:     v76 -> v59
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _8)
// CHECK-NEXT:     brif v23, block2, block3
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = const false
// CHECK-NEXT:     v24 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): bool <- ByVal(v24): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block4(v24, v60, v62, v64)  ; v24 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     v68 = iconst.i16 0
// CHECK-NEXT:     v65 -> v68
// CHECK-NEXT:     v67 = iconst.i16 0
// CHECK-NEXT:     v63 -> v67
// CHECK-NEXT:     v66 = iconst.i16 0
// CHECK-NEXT:     v61 -> v66
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = const false
// CHECK-NEXT:     v25 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): bool <- ByVal(v25): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block4(v25, v61, v63, v65)  ; v25 = 0, v61 = 0, v63 = 0, v65 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _23 = Not(const false)
// CHECK-NEXT:     v26 = iconst.i8 0
// CHECK-NEXT:     v27 = iconst.i8 0
// CHECK-NEXT:     v28 = icmp eq v26, v27  ; v26 = 0, v27 = 0
// CHECK-NEXT: ; write_cvalue: Var(_23, var18): bool <- ByVal(v28): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _23)
// CHECK-NEXT:     jump block17
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = copy _5 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_15, var11): *const () <- ByVal(v29): *const ()
// CHECK-NEXT: ; _16 = copy _6 as *mut () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_16, var12): *mut () <- ByVal(v30): *mut ()
// CHECK-NEXT: ; _17 = Eq(copy _3, const 0_usize)
// CHECK-NEXT:     v32 = iconst.i16 0
// CHECK-NEXT:     v33 = icmp.i16 eq v31, v32  ; v32 = 0
// CHECK-NEXT: ; write_cvalue: Var(_17, var13): bool <- ByVal(v33): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _14 = core::ptr::copy::precondition_check(move _15, move _16, const <u8 as core::mem::SizedTypeProperties>::ALIGN, move _17)
// CHECK-NEXT:     v34 = iconst.i16 1
// CHECK-NEXT:     v35 = symbol_value.i16 gv1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: *mut (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 4, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v29, v30, v34, v33, v35)  ; v34 = 1
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::intrinsics::copy::<u8>(copy _5, copy _6, copy _3)
// CHECK-NEXT:     v39 = call fn3(v37, v36, v38)
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block10(v40: i8, v69: i16, v70: i16, v71: i16, v88: i16):
// CHECK-NEXT:     v29 -> v69
// CHECK-NEXT:     v36 -> v69
// CHECK-NEXT:     v30 -> v70
// CHECK-NEXT:     v37 -> v70
// CHECK-NEXT:     v31 -> v71
// CHECK-NEXT:     v38 -> v71
// CHECK-NEXT:     v85 -> v71
// CHECK-NEXT:     v83 -> v88
// CHECK-NEXT:     v87 -> v88
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _13)
// CHECK-NEXT:     brif v40, block8, block9
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _13 = const false
// CHECK-NEXT:     v41 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_13, var10): bool <- ByVal(v41): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block10(v41, v72, v74, v76, v12)  ; v41 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block12:
// CHECK-NEXT:     v90 = iconst.i16 0
// CHECK-NEXT:     v89 -> v90
// CHECK-NEXT:     v80 = iconst.i16 0
// CHECK-NEXT:     v77 -> v80
// CHECK-NEXT:     v79 = iconst.i16 0
// CHECK-NEXT:     v75 -> v79
// CHECK-NEXT:     v78 = iconst.i16 0
// CHECK-NEXT:     v73 -> v78
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _13 = const false
// CHECK-NEXT:     v42 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_13, var10): bool <- ByVal(v42): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block10(v42, v73, v75, v77, v90)  ; v42 = 0, v73 = 0, v75 = 0, v77 = 0, v90 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _21 = copy _1 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_21, var16): *const () <- ByVal(v43): *const ()
// CHECK-NEXT: ; _22 = Eq(copy _3, const 0_usize)
// CHECK-NEXT:     v45 = iconst.i16 0
// CHECK-NEXT:     v46 = icmp.i16 eq v44, v45  ; v45 = 0
// CHECK-NEXT: ; write_cvalue: Var(_22, var17): bool <- ByVal(v46): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _20 = core::ptr::write_bytes::precondition_check(move _21, const <u8 as core::mem::SizedTypeProperties>::ALIGN, move _22)
// CHECK-NEXT:     v47 = iconst.i16 1
// CHECK-NEXT:     v48 = symbol_value.i16 gv2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 3, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn4(v43, v47, v46, v48)  ; v47 = 1
// CHECK-NEXT:     jump block15
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = core::intrinsics::write_bytes::<u8>(copy _1, const 7_u8, copy _3)
// CHECK-NEXT:     v50 = iconst.i8 7
// CHECK-NEXT:     v52 = uextend.i32 v50  ; v50 = 7
// CHECK-NEXT:     v53 = call fn5(v49, v52, v51)
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block16(v54: i8, v81: i16, v82: i16):
// CHECK-NEXT:     v43 -> v81
// CHECK-NEXT:     v49 -> v81
// CHECK-NEXT:     v44 -> v82
// CHECK-NEXT:     v51 -> v82
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _19)
// CHECK-NEXT:     brif v54, block14, block15
// CHECK-EMPTY:
// CHECK-NEXT: block17:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _19 = const false
// CHECK-NEXT:     v55 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_19, var15): bool <- ByVal(v55): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block16(v55, v83, v85)  ; v55 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block18:
// CHECK-NEXT:     v92 = iconst.i16 0
// CHECK-NEXT:     v86 -> v92
// CHECK-NEXT:     v91 = iconst.i16 0
// CHECK-NEXT:     v84 -> v91
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _19 = const false
// CHECK-NEXT:     v56 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_19, var15): bool <- ByVal(v56): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block16(v56, v84, v86)  ; v56 = 0, v84 = 0, v86 = 0
// CHECK-NEXT: }
