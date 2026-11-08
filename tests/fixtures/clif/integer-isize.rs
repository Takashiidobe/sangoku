#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn integer_methods(x: isize, y: isize) -> u8 {
    let add = (isize::MIN + 1, true);
    let sub = (isize::MAX - 2, false);
    let mul = (-2, true);
    (x.checked_add(y).is_none() as u8)
        | ((x.checked_sub(y) == Some(isize::MAX - 2)) as u8) << 1
        | (x.checked_mul(y).is_none() as u8) << 2
        | ((x.wrapping_add(y) == add.0) as u8) << 3
        | ((x.wrapping_sub(y) == sub.0) as u8) << 4
        | ((x.wrapping_mul(y) == mul.0) as u8) << 5
        | ((x.overflowing_add(y) == add
            && x.overflowing_sub(y) == sub
            && x.overflowing_mul(y) == mul) as u8) << 6
        | ((x.leading_zeros() == 1
            && x.trailing_zeros() == 0
            && x.count_ones() == 15) as u8) << 7
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    (integer_methods(isize::MAX, 2) == 255) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs3_NtCs40R3J0iFVxM_4core3numi11checked_addCs2bVpDLuK8uN_13integer_isize
// CHECK-NEXT: function u0:0(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs3_NtCs40R3J0iFVxM_4core3numi11checked_addCs2bVpDLuK8uN_13integer_isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::option::Option<isize>       4b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::option::Option<isize>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) isize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    isize                             2b 2                var=1
// CHECK-NEXT: ; ssa   _2    isize                             2b 2                var=2
// CHECK-NEXT: ; ssa   _3    isize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (isize, bool)                     4b 2                var=(5, 6)
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=7
// CHECK-NEXT: ; ssa   _7    i16                               2b 2                var=8
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=9
// CHECK-NEXT: ; ssa   _9    (i16, bool)                       4b 2                var=(10, 11)
// CHECK-NEXT: ; ssa   _10   i16                               2b 2                var=12
// CHECK-NEXT: ; ssa   _11   i16                               2b 2                var=13
// CHECK-NEXT: ; ssa   _12   isize                             2b 2                var=14
// CHECK-NEXT: ; zst   _13   ()                                0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): isize <- ByVal(v0): isize
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): isize <- ByVal(v1): isize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = copy _1 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): i16 <- ByVal(v2): i16
// CHECK-NEXT: ; _11 = copy _2 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): i16 <- ByVal(v3): i16
// CHECK-NEXT: ; _9 = AddWithOverflow(move _10, move _11)
// CHECK-NEXT:     v4 = iadd.i16 v2, v3
// CHECK-NEXT:     v31 -> v4
// CHECK-NEXT:     v32 -> v4
// CHECK-NEXT:     v5 = iconst.i16 0
// CHECK-NEXT:     v6 = icmp.i16 slt v3, v5  ; v5 = 0
// CHECK-NEXT:     v7 = icmp slt v4, v2
// CHECK-NEXT:     v8 = bxor v6, v7
// CHECK-NEXT: ; write_cvalue: VarPair(_9, var10, var11): (i16, bool) <- ByValPair(v4, v8): (i16, bool)
// CHECK-NEXT: ; _7 = copy (_9.0: i16)
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): i16 <- ByVal(v4): i16
// CHECK-NEXT: ; _8 = copy (_9.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): bool <- ByVal(v8): bool
// CHECK-NEXT: ; _12 = copy _7 as isize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_12, var14): isize <- ByVal(v4): isize
// CHECK-NEXT: ; _5 = (move _12, copy _8)
// CHECK-NEXT: ; write_cvalue: Var(_5, var5): isize <- ByVal(v4): isize
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): bool <- ByVal(v8): bool
// CHECK-NEXT: ; _3 = copy (_5.0: isize)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): isize <- ByVal(v4): isize
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v8): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _4)
// CHECK-NEXT:     brif v8, block6, block8
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<isize>::None
// CHECK-NEXT:     v9 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u16 <- ByVal(v9): u16
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v9, v10  ; v9 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT:     v12 = load.i16 notrap v11
// CHECK-NEXT:     v13 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v14 = load.i16 notrap v13
// CHECK-NEXT:     return v12, v14
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<isize>::Some(copy _3)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): isize <- ByVal(v15): isize
// CHECK-NEXT:     v16 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store.i16 notrap v15, v16
// CHECK-NEXT:     v17 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u16 <- ByVal(v17): u16
// CHECK-NEXT:     v18 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v17, v18  ; v17 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v19 = stack_addr.i16 ss0
// CHECK-NEXT:     v20 = load.i16 notrap v19
// CHECK-NEXT:     v21 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v22 = load.i16 notrap v21
// CHECK-NEXT:     return v20, v22
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v23 = stack_addr.i16 ss0
// CHECK-NEXT:     v24 = load.i16 notrap v23
// CHECK-NEXT:     v25 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v26 = load.i16 notrap v25
// CHECK-NEXT:     return v24, v26
// CHECK-EMPTY:
// CHECK-NEXT: block5(v27: i8, v30: i16):
// CHECK-NEXT:     v15 -> v30
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     brif v27, block2, block3
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
// CHECK-NEXT:     v28 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v28): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block5(v28, v31)  ; v28 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = const false
// CHECK-NEXT:     v29 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v29): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block5(v29, v32)  ; v29 = 0
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs3_NtCs40R3J0iFVxM_4core3numi11checked_mulCs2bVpDLuK8uN_13integer_isize
// CHECK-NEXT: function u0:1(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs3_NtCs40R3J0iFVxM_4core3numi11checked_mulCs2bVpDLuK8uN_13integer_isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::option::Option<isize>       4b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::option::Option<isize>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) isize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    isize                             2b 2                var=1
// CHECK-NEXT: ; ssa   _2    isize                             2b 2                var=2
// CHECK-NEXT: ; ssa   _3    isize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (isize, bool)                     4b 2                var=(5, 6)
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=7
// CHECK-NEXT: ; ssa   _7    i16                               2b 2                var=8
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=9
// CHECK-NEXT: ; ssa   _9    (i16, bool)                       4b 2                var=(10, 11)
// CHECK-NEXT: ; ssa   _10   i16                               2b 2                var=12
// CHECK-NEXT: ; ssa   _11   i16                               2b 2                var=13
// CHECK-NEXT: ; ssa   _12   isize                             2b 2                var=14
// CHECK-NEXT: ; zst   _13   ()                                0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): isize <- ByVal(v0): isize
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): isize <- ByVal(v1): isize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = copy _1 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): i16 <- ByVal(v2): i16
// CHECK-NEXT: ; _11 = copy _2 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): i16 <- ByVal(v3): i16
// CHECK-NEXT: ; _9 = MulWithOverflow(move _10, move _11)
// CHECK-NEXT:     v4 = sextend.i32 v2
// CHECK-NEXT:     v5 = sextend.i32 v3
// CHECK-NEXT:     v6 = imul v4, v5
// CHECK-NEXT:     v7 = iconst.i32 -32768
// CHECK-NEXT:     v8 = icmp slt v6, v7  ; v7 = -32768
// CHECK-NEXT:     v9 = iconst.i32 0x7fff
// CHECK-NEXT:     v10 = icmp sgt v6, v9  ; v9 = 0x7fff
// CHECK-NEXT:     v11 = ireduce.i16 v6
// CHECK-NEXT:     v35 -> v11
// CHECK-NEXT:     v36 -> v11
// CHECK-NEXT:     v12 = bor v8, v10
// CHECK-NEXT: ; write_cvalue: VarPair(_9, var10, var11): (i16, bool) <- ByValPair(v11, v12): (i16, bool)
// CHECK-NEXT: ; _7 = copy (_9.0: i16)
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): i16 <- ByVal(v11): i16
// CHECK-NEXT: ; _8 = copy (_9.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): bool <- ByVal(v12): bool
// CHECK-NEXT: ; _12 = copy _7 as isize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_12, var14): isize <- ByVal(v11): isize
// CHECK-NEXT: ; _5 = (move _12, copy _8)
// CHECK-NEXT: ; write_cvalue: Var(_5, var5): isize <- ByVal(v11): isize
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): bool <- ByVal(v12): bool
// CHECK-NEXT: ; _3 = copy (_5.0: isize)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): isize <- ByVal(v11): isize
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v12): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _4)
// CHECK-NEXT:     brif v12, block6, block8
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<isize>::None
// CHECK-NEXT:     v13 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u16 <- ByVal(v13): u16
// CHECK-NEXT:     v14 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v13, v14  ; v13 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v15 = stack_addr.i16 ss0
// CHECK-NEXT:     v16 = load.i16 notrap v15
// CHECK-NEXT:     v17 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v18 = load.i16 notrap v17
// CHECK-NEXT:     return v16, v18
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<isize>::Some(copy _3)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): isize <- ByVal(v19): isize
// CHECK-NEXT:     v20 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store.i16 notrap v19, v20
// CHECK-NEXT:     v21 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u16 <- ByVal(v21): u16
// CHECK-NEXT:     v22 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v21, v22  ; v21 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v23 = stack_addr.i16 ss0
// CHECK-NEXT:     v24 = load.i16 notrap v23
// CHECK-NEXT:     v25 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v26 = load.i16 notrap v25
// CHECK-NEXT:     return v24, v26
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v27 = stack_addr.i16 ss0
// CHECK-NEXT:     v28 = load.i16 notrap v27
// CHECK-NEXT:     v29 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v30 = load.i16 notrap v29
// CHECK-NEXT:     return v28, v30
// CHECK-EMPTY:
// CHECK-NEXT: block5(v31: i8, v34: i16):
// CHECK-NEXT:     v19 -> v34
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     brif v31, block2, block3
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
// CHECK-NEXT:     v32 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v32): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block5(v32, v35)  ; v32 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = const false
// CHECK-NEXT:     v33 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v33): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block5(v33, v36)  ; v33 = 0
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs3_NtCs40R3J0iFVxM_4core3numi11checked_subCs2bVpDLuK8uN_13integer_isize
// CHECK-NEXT: function u0:2(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs3_NtCs40R3J0iFVxM_4core3numi11checked_subCs2bVpDLuK8uN_13integer_isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::option::Option<isize>       4b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::option::Option<isize>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) isize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    isize                             2b 2                var=1
// CHECK-NEXT: ; ssa   _2    isize                             2b 2                var=2
// CHECK-NEXT: ; ssa   _3    isize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (isize, bool)                     4b 2                var=(5, 6)
// CHECK-NEXT: ; ssa   _6    bool                              1b 1                var=7
// CHECK-NEXT: ; ssa   _7    i16                               2b 2                var=8
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=9
// CHECK-NEXT: ; ssa   _9    (i16, bool)                       4b 2                var=(10, 11)
// CHECK-NEXT: ; ssa   _10   i16                               2b 2                var=12
// CHECK-NEXT: ; ssa   _11   i16                               2b 2                var=13
// CHECK-NEXT: ; ssa   _12   isize                             2b 2                var=14
// CHECK-NEXT: ; zst   _13   ()                                0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): isize <- ByVal(v0): isize
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): isize <- ByVal(v1): isize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = copy _1 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): i16 <- ByVal(v2): i16
// CHECK-NEXT: ; _11 = copy _2 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): i16 <- ByVal(v3): i16
// CHECK-NEXT: ; _9 = SubWithOverflow(move _10, move _11)
// CHECK-NEXT:     v4 = isub.i16 v2, v3
// CHECK-NEXT:     v31 -> v4
// CHECK-NEXT:     v32 -> v4
// CHECK-NEXT:     v5 = iconst.i16 0
// CHECK-NEXT:     v6 = icmp.i16 slt v3, v5  ; v5 = 0
// CHECK-NEXT:     v7 = icmp sgt v4, v2
// CHECK-NEXT:     v8 = bxor v6, v7
// CHECK-NEXT: ; write_cvalue: VarPair(_9, var10, var11): (i16, bool) <- ByValPair(v4, v8): (i16, bool)
// CHECK-NEXT: ; _7 = copy (_9.0: i16)
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): i16 <- ByVal(v4): i16
// CHECK-NEXT: ; _8 = copy (_9.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): bool <- ByVal(v8): bool
// CHECK-NEXT: ; _12 = copy _7 as isize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_12, var14): isize <- ByVal(v4): isize
// CHECK-NEXT: ; _5 = (move _12, copy _8)
// CHECK-NEXT: ; write_cvalue: Var(_5, var5): isize <- ByVal(v4): isize
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): bool <- ByVal(v8): bool
// CHECK-NEXT: ; _3 = copy (_5.0: isize)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): isize <- ByVal(v4): isize
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v8): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _4)
// CHECK-NEXT:     brif v8, block6, block8
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<isize>::None
// CHECK-NEXT:     v9 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u16 <- ByVal(v9): u16
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v9, v10  ; v9 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT:     v12 = load.i16 notrap v11
// CHECK-NEXT:     v13 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v14 = load.i16 notrap v13
// CHECK-NEXT:     return v12, v14
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<isize>::Some(copy _3)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): isize <- ByVal(v15): isize
// CHECK-NEXT:     v16 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store.i16 notrap v15, v16
// CHECK-NEXT:     v17 = iconst.i16 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): u16 <- ByVal(v17): u16
// CHECK-NEXT:     v18 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v17, v18  ; v17 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v19 = stack_addr.i16 ss0
// CHECK-NEXT:     v20 = load.i16 notrap v19
// CHECK-NEXT:     v21 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v22 = load.i16 notrap v21
// CHECK-NEXT:     return v20, v22
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v23 = stack_addr.i16 ss0
// CHECK-NEXT:     v24 = load.i16 notrap v23
// CHECK-NEXT:     v25 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v26 = load.i16 notrap v25
// CHECK-NEXT:     return v24, v26
// CHECK-EMPTY:
// CHECK-NEXT: block5(v27: i8, v30: i16):
// CHECK-NEXT:     v15 -> v30
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     brif v27, block2, block3
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
// CHECK-NEXT:     v28 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v28): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block5(v28, v31)  ; v28 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = const false
// CHECK-NEXT:     v29 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): bool <- ByVal(v29): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block5(v29, v32)  ; v29 = 0
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCs2bVpDLuK8uN_13integer_isize
// CHECK-NEXT: function u0:3() system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core10intrinsics9cold_pathCs2bVpDLuK8uN_13integer_isize
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
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTibENtNtB7_3cmp9PartialEq2eqCs2bVpDLuK8uN_13integer_isize
// CHECK-NEXT: function u0:4(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTibENtNtB7_3cmp9PartialEq2eqCs2bVpDLuK8uN_13integer_isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (isize, bool)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (isize, bool)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} (isize, bool)          2b 2                storage=ss0
// CHECK-NEXT: ; stack _2    &'{erased} (isize, bool)          2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _4    &'{erased} isize                  2b 2                var=3
// CHECK-NEXT: ; ssa   _5    &'{erased} isize                  2b 2                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} bool                   2b 2                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} bool                   2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:7 sig0 ; Instance { def: Item(DefId(1:29987 ~ core[..]::cmp::impls::{impl#27}::eq)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:6 sig1 ; Instance { def: Item(DefId(1:29963 ~ core[..]::cmp::impls::{impl#19}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (isize, bool) <- ByVal(v0): &'{erased} (isize, bool)
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (isize, bool) <- ByVal(v1): &'{erased} (isize, bool)
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = &((*_1).0: U)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): &'{erased} isize <- ByVal(v5): &'{erased} isize
// CHECK-NEXT: ; _5 = &((*_2).0: U)
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): &'{erased} isize <- ByVal(v7): &'{erased} isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = <U as core::cmp::PartialEq>::eq(move _4, move _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): &'{erased} bool <- ByVal(v13): &'{erased} bool
// CHECK-NEXT: ; _7 = &((*_2).1: T)
// CHECK-NEXT:     v14 = stack_addr.i16 ss1
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = iconst.i16 2
// CHECK-NEXT:     v17 = iadd v15, v16  ; v16 = 2
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
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptioniENtNtB7_3cmp9PartialEq2eqCs2bVpDLuK8uN_13integer_isize
// CHECK-NEXT: function u0:5(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsf_NtCs40R3J0iFVxM_4core6optionINtB5_6OptioniENtNtB7_3cmp9PartialEq2eqCs2bVpDLuK8uN_13integer_isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::option::Option<isize>
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::option::Option<isize>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} core::option::Option<isize>    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} core::option::Option<isize>    2b 2                var=3
// CHECK-NEXT: ; ssa   _3    (&'{erased} core::option::Option<isize>, &'{erased} core::option::Option<isize>)    4b 2                var=(4, 5)
// CHECK-NEXT: ; ssa   _4    isize                             2b 2                var=6
// CHECK-NEXT: ; ssa   _5    isize                             2b 2                var=7
// CHECK-NEXT: ; ssa   _6    isize                             2b 2                var=8
// CHECK-NEXT: ; ssa   _7    &'{erased} isize                  2b 2                var=9
// CHECK-NEXT: ; ssa   _8    &'{erased} isize                  2b 2                var=10
// CHECK-NEXT: ; ssa   _9    &'{erased} core::option::Option<isize>    2b 2                var=11
// CHECK-NEXT: ; ssa   _10   &'{erased} core::option::Option<isize>    2b 2                var=12
// CHECK-NEXT: ; ssa   _11   &'{erased} core::option::Option<isize>    2b 2                var=13
// CHECK-NEXT: ; stack _12   &'{erased} core::option::Option<isize>    2b 2                storage=ss0
// CHECK-NEXT: ; stack _13   &'{erased} core::option::Option<isize>    2b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:7 sig0 ; Instance { def: Item(DefId(1:29987 ~ core[..]::cmp::impls::{impl#27}::eq)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v15 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v6 -> v1
// CHECK-NEXT:     v9 -> v1
// CHECK-NEXT:     v21 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} core::option::Option<isize> <- ByVal(v0): &'{erased} core::option::Option<isize>
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} core::option::Option<isize> <- ByVal(v1): &'{erased} core::option::Option<isize>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = (copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): &'{erased} core::option::Option<isize> <- ByVal(v2): &'{erased} core::option::Option<isize>
// CHECK-NEXT: ; write_cvalue: Var(_3, var5): &'{erased} core::option::Option<isize> <- ByVal(v3): &'{erased} core::option::Option<isize>
// CHECK-NEXT: ; _9 = no_retag copy (_3.0: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Var(_9, var11): &'{erased} core::option::Option<isize> <- ByVal(v2): &'{erased} core::option::Option<isize>
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
// CHECK-NEXT: ; write_cvalue: Var(_10, var12): &'{erased} core::option::Option<isize> <- ByVal(v6): &'{erased} core::option::Option<isize>
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
// CHECK-NEXT: ; write_cvalue: Var(_11, var13): &'{erased} core::option::Option<isize> <- ByVal(v9): &'{erased} core::option::Option<isize>
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
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} core::option::Option<isize> <- ByVal(v15): &'{erased} core::option::Option<isize>
// CHECK-NEXT:     v16 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v15, v16
// CHECK-NEXT: ; _7 = &(((*_12) as Some).0: T)
// CHECK-NEXT:     v17 = stack_addr.i16 ss0
// CHECK-NEXT:     v18 = load.i16 notrap v17
// CHECK-NEXT:     v19 = iconst.i16 2
// CHECK-NEXT:     v20 = iadd v18, v19  ; v19 = 2
// CHECK-NEXT: ; write_cvalue: Var(_7, var9): &'{erased} isize <- ByVal(v20): &'{erased} isize
// CHECK-NEXT: ; _13 = no_retag copy (_3.1: &core::option::Option<T>)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} core::option::Option<isize> <- ByVal(v21): &'{erased} core::option::Option<isize>
// CHECK-NEXT:     v22 = stack_addr.i16 ss1
// CHECK-NEXT:     store.i16 notrap v21, v22
// CHECK-NEXT: ; _8 = &(((*_13) as Some).0: T)
// CHECK-NEXT:     v23 = stack_addr.i16 ss1
// CHECK-NEXT:     v24 = load.i16 notrap v23
// CHECK-NEXT:     v25 = iconst.i16 2
// CHECK-NEXT:     v26 = iadd v24, v25  ; v25 = 2
// CHECK-NEXT: ; write_cvalue: Var(_8, var10): &'{erased} isize <- ByVal(v26): &'{erased} isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::cmp::PartialEq>::eq(copy _7, copy _8)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCs2bVpDLuK8uN_13integer_isize
// CHECK-NEXT: function u0:6(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2eqCs2bVpDLuK8uN_13integer_isize
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
// CHECK-NEXT: ; symbol _RNvXsp_NtNtCs40R3J0iFVxM_4core3cmp5implsiNtB7_9PartialEq2eqCs2bVpDLuK8uN_13integer_isize
// CHECK-NEXT: function u0:7(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsp_NtNtCs40R3J0iFVxM_4core3cmp5implsiNtB7_9PartialEq2eqCs2bVpDLuK8uN_13integer_isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} isize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} isize                  2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} isize                  2b 2                var=3
// CHECK-NEXT: ; ssa   _3    isize                             2b 2                var=4
// CHECK-NEXT: ; ssa   _4    isize                             2b 2                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} isize <- ByVal(v0): &'{erased} isize
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} isize <- ByVal(v1): &'{erased} isize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): isize <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): isize
// CHECK-NEXT:     v3 = load.i16 notrap v2
// CHECK-NEXT: ; _4 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): isize <- ByRef(Pointer { base: Addr(v4), offset: Offset32(0) }, None): isize
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
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:8() -> i8 system_v {
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
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 system_v
// CHECK-NEXT:     fn0 = colocated u0:9 sig0 ; Instance { def: Item(DefId(0:3 ~ integer_isize[..]::integer_methods)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = integer_methods(const core::num::<impl isize>::MAX, const 2_isize)
// CHECK-NEXT:     v0 = iconst.i16 0x7fff
// CHECK-NEXT:     v1 = iconst.i16 2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(i16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027808258 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(i16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027808258 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0(v0, v1)  ; v0 = 0x7fff, v1 = 2
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
// CHECK-NEXT: function u0:9(i16, i16) -> i8 system_v {
// CHECK-NEXT: ; symbol integer_methods
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) isize
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) isize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    isize                             2b 2                var=2
// CHECK-NEXT: ; ssa   _2    isize                             2b 2                var=3
// CHECK-NEXT: ; stack _3    (isize, bool)                     4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _4    isize                             2b 2                var=4
// CHECK-NEXT: ; stack _5    (isize, bool)                     4b 2                storage=ss1
// CHECK-NEXT: ; ssa   _6    isize                             2b 2                var=5
// CHECK-NEXT: ; stack _7    (isize, bool)                     4b 2                storage=ss2
// CHECK-NEXT: ; ssa   _8    u8                                1b 1                var=6
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=7
// CHECK-NEXT: ; ssa   _10   u8                                1b 1                var=8
// CHECK-NEXT: ; ssa   _11   u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=10
// CHECK-NEXT: ; ssa   _13   u8                                1b 1                var=11
// CHECK-NEXT: ; ssa   _14   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _15   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _16   &'{erased} core::option::Option<isize>    2b 2                var=14
// CHECK-NEXT: ; stack _17   core::option::Option<isize>       4b 2                storage=ss3
// CHECK-NEXT: ; ssa   _18   u8                                1b 1                var=15
// CHECK-NEXT: ; ssa   _19   u8                                1b 1                var=16
// CHECK-NEXT: ; ssa   _20   bool                              1b 1                var=17
// CHECK-NEXT: ; ssa   _21   &'{erased} core::option::Option<isize>    2b 2                var=18
// CHECK-NEXT: ; stack _22   core::option::Option<isize>       4b 2                storage=ss4
// CHECK-NEXT: ; ssa   _23   &'{erased} core::option::Option<isize>    2b 2                var=19
// CHECK-NEXT: ; ssa   _24   u8                                1b 1                var=20
// CHECK-NEXT: ; ssa   _25   u8                                1b 1                var=21
// CHECK-NEXT: ; ssa   _26   bool                              1b 1                var=22
// CHECK-NEXT: ; ssa   _27   &'{erased} core::option::Option<isize>    2b 2                var=23
// CHECK-NEXT: ; stack _28   core::option::Option<isize>       4b 2                storage=ss5
// CHECK-NEXT: ; ssa   _29   u8                                1b 1                var=24
// CHECK-NEXT: ; ssa   _30   u8                                1b 1                var=25
// CHECK-NEXT: ; ssa   _31   bool                              1b 1                var=26
// CHECK-NEXT: ; ssa   _32   isize                             2b 2                var=27
// CHECK-NEXT: ; ssa   _33   isize                             2b 2                var=28
// CHECK-NEXT: ; ssa   _34   u8                                1b 1                var=29
// CHECK-NEXT: ; ssa   _35   u8                                1b 1                var=30
// CHECK-NEXT: ; ssa   _36   bool                              1b 1                var=31
// CHECK-NEXT: ; ssa   _37   isize                             2b 2                var=32
// CHECK-NEXT: ; ssa   _38   isize                             2b 2                var=33
// CHECK-NEXT: ; ssa   _39   u8                                1b 1                var=34
// CHECK-NEXT: ; ssa   _40   u8                                1b 1                var=35
// CHECK-NEXT: ; ssa   _41   bool                              1b 1                var=36
// CHECK-NEXT: ; ssa   _42   isize                             2b 2                var=37
// CHECK-NEXT: ; ssa   _43   isize                             2b 2                var=38
// CHECK-NEXT: ; ssa   _44   u8                                1b 1                var=39
// CHECK-NEXT: ; ssa   _45   u8                                1b 1                var=40
// CHECK-NEXT: ; ssa   _46   bool                              1b 1                var=41
// CHECK-NEXT: ; ssa   _47   bool                              1b 1                var=42
// CHECK-NEXT: ; ssa   _48   &'{erased} (isize, bool)          2b 2                var=43
// CHECK-NEXT: ; stack _49   (isize, bool)                     4b 2                storage=ss6
// CHECK-NEXT: ; ssa   _50   &'{erased} (isize, bool)          2b 2                var=44
// CHECK-NEXT: ; ssa   _51   bool                              1b 1                var=45
// CHECK-NEXT: ; ssa   _52   &'{erased} (isize, bool)          2b 2                var=46
// CHECK-NEXT: ; stack _53   (isize, bool)                     4b 2                storage=ss7
// CHECK-NEXT: ; ssa   _54   &'{erased} (isize, bool)          2b 2                var=47
// CHECK-NEXT: ; ssa   _55   &'{erased} (isize, bool)          2b 2                var=48
// CHECK-NEXT: ; stack _56   (isize, bool)                     4b 2                storage=ss8
// CHECK-NEXT: ; ssa   _57   &'{erased} (isize, bool)          2b 2                var=49
// CHECK-NEXT: ; ssa   _58   u8                                1b 1                var=50
// CHECK-NEXT: ; ssa   _59   u8                                1b 1                var=51
// CHECK-NEXT: ; ssa   _60   bool                              1b 1                var=52
// CHECK-NEXT: ; ssa   _61   bool                              1b 1                var=53
// CHECK-NEXT: ; ssa   _62   u32                               4b 2                var=54
// CHECK-NEXT: ; ssa   _63   bool                              1b 1                var=55
// CHECK-NEXT: ; ssa   _64   u32                               4b 2                var=56
// CHECK-NEXT: ; ssa   _65   u32                               4b 2                var=57
// CHECK-NEXT: ; ssa   _66   bool                              1b 1                var=58
// CHECK-NEXT: ; ssa   _67   isize                             2b 2                var=59
// CHECK-NEXT: ; ssa   _68   bool                              1b 1                var=60
// CHECK-NEXT: ; ssa   _69   isize                             2b 2                var=61
// CHECK-NEXT: ; ssa   _70   i16                               2b 2                var=62
// CHECK-NEXT: ; ssa   _71   bool                              1b 1                var=63
// CHECK-NEXT: ; ssa   _72   (i16, bool)                       4b 2                var=(64, 65)
// CHECK-NEXT: ; ssa   _73   i16                               2b 2                var=66
// CHECK-NEXT: ; ssa   _74   i16                               2b 2                var=67
// CHECK-NEXT: ; ssa   _75   isize                             2b 2                var=68
// CHECK-NEXT: ; ssa   _76   i16                               2b 2                var=69
// CHECK-NEXT: ; ssa   _77   bool                              1b 1                var=70
// CHECK-NEXT: ; ssa   _78   (i16, bool)                       4b 2                var=(71, 72)
// CHECK-NEXT: ; ssa   _79   i16                               2b 2                var=73
// CHECK-NEXT: ; ssa   _80   i16                               2b 2                var=74
// CHECK-NEXT: ; ssa   _81   isize                             2b 2                var=75
// CHECK-NEXT: ; ssa   _82   i16                               2b 2                var=76
// CHECK-NEXT: ; ssa   _83   bool                              1b 1                var=77
// CHECK-NEXT: ; ssa   _84   (i16, bool)                       4b 2                var=(78, 79)
// CHECK-NEXT: ; ssa   _85   i16                               2b 2                var=80
// CHECK-NEXT: ; ssa   _86   i16                               2b 2                var=81
// CHECK-NEXT: ; ssa   _87   isize                             2b 2                var=82
// CHECK-NEXT: ; ssa   _88   usize                             2b 2                var=83
// CHECK-NEXT: ; ssa   _89   u16                               2b 2                var=84
// CHECK-NEXT: ; ssa   _90   usize                             2b 2                var=85
// CHECK-NEXT: ; ssa   _91   usize                             2b 2                var=86
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss3 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss4 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss5 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss6 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss7 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss8 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname1 ; alloc5
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig2 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig4 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig5 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig6 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:0 sig0 ; Instance { def: Item(DefId(1:27238 ~ core[..]::num::{impl#5}::checked_add)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:5 sig1 ; Instance { def: Item(DefId(1:11950 ~ core[..]::option::{impl#17}::eq)), args: [isize] }
// CHECK-NEXT:     fn2 = colocated u0:1 sig2 ; Instance { def: Item(DefId(1:27248 ~ core[..]::num::{impl#5}::checked_mul)), args: [] }
// CHECK-NEXT:     fn3 = colocated u0:4 sig3 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [isize, bool] }
// CHECK-NEXT:     fn4 = colocated u0:4 sig4 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [isize, bool] }
// CHECK-NEXT:     fn5 = colocated u0:2 sig5 ; Instance { def: Item(DefId(1:27243 ~ core[..]::num::{impl#5}::checked_sub)), args: [] }
// CHECK-NEXT:     fn6 = colocated u0:4 sig6 ; Instance { def: Item(DefId(1:46739 ~ core[..]::tuple::{impl#10}::eq)), args: [isize, bool] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v18 -> v0
// CHECK-NEXT:     v165 -> v0
// CHECK-NEXT:     v166 -> v0
// CHECK-NEXT:     v19 -> v1
// CHECK-NEXT:     v167 -> v1
// CHECK-NEXT:     v168 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): isize <- ByVal(v0): isize
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): isize <- ByVal(v1): isize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = Add(const core::num::<impl isize>::MIN, const 1_isize)
// CHECK-NEXT:     v2 = iconst.i16 -32768
// CHECK-NEXT:     v3 = iconst.i16 1
// CHECK-NEXT:     v4 = iadd v2, v3  ; v2 = -32768, v3 = 1
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): isize <- ByVal(v4): isize
// CHECK-NEXT: ; _3 = (move _4, const true)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): isize <- ByVal(v4): isize
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v4, v5
// CHECK-NEXT:     v6 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): bool <- ByVal(v6): bool
// CHECK-NEXT:     v7 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v6, v7  ; v6 = 1
// CHECK-NEXT: ; _6 = Sub(const core::num::<impl isize>::MAX, const 2_isize)
// CHECK-NEXT:     v8 = iconst.i16 0x7fff
// CHECK-NEXT:     v9 = iconst.i16 2
// CHECK-NEXT:     v10 = isub v8, v9  ; v8 = 0x7fff, v9 = 2
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): isize <- ByVal(v10): isize
// CHECK-NEXT: ; _5 = (move _6, const false)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): isize <- ByVal(v10): isize
// CHECK-NEXT:     v11 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v10, v11
// CHECK-NEXT:     v12 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(2) }, None): bool <- ByVal(v12): bool
// CHECK-NEXT:     v13 = stack_addr.i16 ss1+2
// CHECK-NEXT:     store notrap v12, v13  ; v12 = 0
// CHECK-NEXT: ; _7 = (const -2_isize, const true)
// CHECK-NEXT:     v14 = iconst.i16 -2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): isize <- ByVal(v14): isize
// CHECK-NEXT:     v15 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v14, v15  ; v14 = -2
// CHECK-NEXT:     v16 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(2) }, None): bool <- ByVal(v16): bool
// CHECK-NEXT:     v17 = stack_addr.i16 ss2+2
// CHECK-NEXT:     store notrap v16, v17  ; v16 = 1
// CHECK-NEXT: ;
// CHECK-NEXT: ; _17 = core::num::<impl isize>::checked_add(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(i16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027808258 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(i16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027808258 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<isize>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union i16, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u16, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u16 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(2 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(4 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union i16, b_offset: Size(2 bytes) }, field_offsets: [Size(2 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8189316930324039666 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v20, v21 = call fn0(v18, v19)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): core::option::Option<isize> <- ByValPair(v20, v21): core::option::Option<isize>
// CHECK-NEXT:     v22 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v20, v22
// CHECK-NEXT:     v23 = stack_addr.i16 ss3+2
// CHECK-NEXT:     store notrap v21, v23
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = &_17
// CHECK-NEXT:     v24 = stack_addr.i16 ss3
// CHECK-NEXT: ; write_cvalue: Var(_16, var14): &'{erased} core::option::Option<isize> <- ByVal(v24): &'{erased} core::option::Option<isize>
// CHECK-NEXT: ; _67 = discriminant((*_16))
// CHECK-NEXT:     v25 = load.i16 notrap v24
// CHECK-NEXT: ; write_cvalue: Var(_67, var59): isize <- ByVal(v25): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _67)
// CHECK-NEXT:     v26 = uextend.i32 v25
// CHECK-NEXT:     br_table v26, block19, [block17, block18]
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _21 = &_22
// CHECK-NEXT:     v27 = stack_addr.i16 ss4
// CHECK-NEXT: ; write_cvalue: Var(_21, var18): &'{erased} core::option::Option<isize> <- ByVal(v27): &'{erased} core::option::Option<isize>
// CHECK-NEXT: ; _23 = const integer_methods::promoted[0]
// CHECK-NEXT:     v28 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_23, var19): &'{erased} core::option::Option<isize> <- ByVal(v28): &'{erased} core::option::Option<isize>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _20 = <core::option::Option<isize> as core::cmp::PartialEq>::eq(move _21, move _23)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::option::Option<isize>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &core::option::Option<isize>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v29 = call fn1(v27, v28)
// CHECK-NEXT:     v30 -> v29
// CHECK-NEXT: ; write_cvalue: Var(_20, var17): bool <- ByVal(v29): bool
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _19 = move _20 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_19, var16): u8 <- ByVal(v30): u8
// CHECK-NEXT: ; _18 = Shl(move _19, const 1_i32)
// CHECK-NEXT:     v31 = iconst.i32 1
// CHECK-NEXT:     v32 = ishl.i8 v30, v31  ; v31 = 1
// CHECK-NEXT: ; write_cvalue: Var(_18, var15): u8 <- ByVal(v32): u8
// CHECK-NEXT: ; _13 = BitOr(move _14, move _18)
// CHECK-NEXT:     v34 = bor.i8 v33, v32
// CHECK-NEXT:     v169 -> v34
// CHECK-NEXT:     v170 -> v34
// CHECK-NEXT: ; write_cvalue: Var(_13, var11): u8 <- ByVal(v34): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _28 = core::num::<impl isize>::checked_mul(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(i16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027808258 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(i16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027808258 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<isize>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union i16, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u16, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u16 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(2 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(4 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union i16, b_offset: Size(2 bytes) }, field_offsets: [Size(2 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8189316930324039666 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v37, v38 = call fn2(v35, v36)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss5), offset: Offset32(0) }, None): core::option::Option<isize> <- ByValPair(v37, v38): core::option::Option<isize>
// CHECK-NEXT:     v39 = stack_addr.i16 ss5
// CHECK-NEXT:     store notrap v37, v39
// CHECK-NEXT:     v40 = stack_addr.i16 ss5+2
// CHECK-NEXT:     store notrap v38, v40
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _27 = &_28
// CHECK-NEXT:     v41 = stack_addr.i16 ss5
// CHECK-NEXT: ; write_cvalue: Var(_27, var23): &'{erased} core::option::Option<isize> <- ByVal(v41): &'{erased} core::option::Option<isize>
// CHECK-NEXT: ; _69 = discriminant((*_27))
// CHECK-NEXT:     v42 = load.i16 notrap v41
// CHECK-NEXT: ; write_cvalue: Var(_69, var61): isize <- ByVal(v42): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _69)
// CHECK-NEXT:     v43 = uextend.i32 v42
// CHECK-NEXT:     br_table v43, block19, [block21, block22]
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _47)
// CHECK-NEXT:     brif.i8 v44, block7, block10
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _79 = copy _1 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_79, var73): i16 <- ByVal(v45): i16
// CHECK-NEXT: ; _80 = copy _2 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_80, var74): i16 <- ByVal(v46): i16
// CHECK-NEXT: ; _78 = SubWithOverflow(move _79, move _80)
// CHECK-NEXT:     v47 = isub.i16 v45, v46
// CHECK-NEXT:     v48 = iconst.i16 0
// CHECK-NEXT:     v49 = icmp.i16 slt v46, v48  ; v48 = 0
// CHECK-NEXT:     v50 = icmp sgt v47, v45
// CHECK-NEXT:     v51 = bxor v49, v50
// CHECK-NEXT: ; write_cvalue: VarPair(_78, var71, var72): (i16, bool) <- ByValPair(v47, v51): (i16, bool)
// CHECK-NEXT: ; _76 = copy (_78.0: i16)
// CHECK-NEXT: ; write_cvalue: Var(_76, var69): i16 <- ByVal(v47): i16
// CHECK-NEXT: ; _77 = copy (_78.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_77, var70): bool <- ByVal(v51): bool
// CHECK-NEXT: ; _81 = copy _76 as isize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_81, var75): isize <- ByVal(v47): isize
// CHECK-NEXT: ; _53 = (move _81, copy _77)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(0) }, None): isize <- ByVal(v47): isize
// CHECK-NEXT:     v52 = stack_addr.i16 ss7
// CHECK-NEXT:     store notrap v47, v52
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss7), offset: Offset32(2) }, None): bool <- ByVal(v51): bool
// CHECK-NEXT:     v53 = stack_addr.i16 ss7+2
// CHECK-NEXT:     store notrap v51, v53
// CHECK-NEXT: ; _52 = &_53
// CHECK-NEXT:     v54 = stack_addr.i16 ss7
// CHECK-NEXT: ; write_cvalue: Var(_52, var46): &'{erased} (isize, bool) <- ByVal(v54): &'{erased} (isize, bool)
// CHECK-NEXT: ; _54 = &_5
// CHECK-NEXT:     v55 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_54, var47): &'{erased} (isize, bool) <- ByVal(v55): &'{erased} (isize, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _51 = <(isize, bool) as core::cmp::PartialEq>::eq(move _52, move _54)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(isize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(isize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v56 = call fn3(v54, v55)
// CHECK-NEXT:     v57 -> v56
// CHECK-NEXT: ; write_cvalue: Var(_51, var45): bool <- ByVal(v56): bool
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _51)
// CHECK-NEXT:     brif.i8 v57, block9, block10
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _85 = copy _1 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_85, var80): i16 <- ByVal(v58): i16
// CHECK-NEXT: ; _86 = copy _2 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_86, var81): i16 <- ByVal(v59): i16
// CHECK-NEXT: ; _84 = MulWithOverflow(move _85, move _86)
// CHECK-NEXT:     v60 = sextend.i32 v58
// CHECK-NEXT:     v61 = sextend.i32 v59
// CHECK-NEXT:     v62 = imul v60, v61
// CHECK-NEXT:     v63 = iconst.i32 -32768
// CHECK-NEXT:     v64 = icmp slt v62, v63  ; v63 = -32768
// CHECK-NEXT:     v65 = iconst.i32 0x7fff
// CHECK-NEXT:     v66 = icmp sgt v62, v65  ; v65 = 0x7fff
// CHECK-NEXT:     v67 = ireduce.i16 v62
// CHECK-NEXT:     v68 = bor v64, v66
// CHECK-NEXT: ; write_cvalue: VarPair(_84, var78, var79): (i16, bool) <- ByValPair(v67, v68): (i16, bool)
// CHECK-NEXT: ; _82 = copy (_84.0: i16)
// CHECK-NEXT: ; write_cvalue: Var(_82, var76): i16 <- ByVal(v67): i16
// CHECK-NEXT: ; _83 = copy (_84.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_83, var77): bool <- ByVal(v68): bool
// CHECK-NEXT: ; _87 = copy _82 as isize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_87, var82): isize <- ByVal(v67): isize
// CHECK-NEXT: ; _56 = (move _87, copy _83)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(0) }, None): isize <- ByVal(v67): isize
// CHECK-NEXT:     v69 = stack_addr.i16 ss8
// CHECK-NEXT:     store notrap v67, v69
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss8), offset: Offset32(2) }, None): bool <- ByVal(v68): bool
// CHECK-NEXT:     v70 = stack_addr.i16 ss8+2
// CHECK-NEXT:     store notrap v68, v70
// CHECK-NEXT: ; _55 = &_56
// CHECK-NEXT:     v71 = stack_addr.i16 ss8
// CHECK-NEXT: ; write_cvalue: Var(_55, var48): &'{erased} (isize, bool) <- ByVal(v71): &'{erased} (isize, bool)
// CHECK-NEXT: ; _57 = &_7
// CHECK-NEXT:     v72 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_57, var49): &'{erased} (isize, bool) <- ByVal(v72): &'{erased} (isize, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _46 = <(isize, bool) as core::cmp::PartialEq>::eq(move _55, move _57)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(isize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(isize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v73 = call fn4(v71, v72)
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v73): bool
// CHECK-NEXT:     jump block11(v73)
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _46 = const false
// CHECK-NEXT:     v74 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_46, var41): bool <- ByVal(v74): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block11(v74)  ; v74 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block11(v75: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _45 = move _46 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_45, var40): u8 <- ByVal(v75): u8
// CHECK-NEXT: ; _44 = Shl(move _45, const 6_i32)
// CHECK-NEXT:     v76 = iconst.i32 6
// CHECK-NEXT:     v77 = ishl v75, v76  ; v76 = 6
// CHECK-NEXT: ; write_cvalue: Var(_44, var39): u8 <- ByVal(v77): u8
// CHECK-NEXT: ; _8 = BitOr(move _9, move _44)
// CHECK-NEXT:     v79 = bor.i8 v78, v77
// CHECK-NEXT:     v162 -> v79
// CHECK-NEXT:     v163 -> v79
// CHECK-NEXT:     v164 -> v79
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): u8 <- ByVal(v79): u8
// CHECK-NEXT: ; _88 = copy _1 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_88, var83): usize <- ByVal(v80): usize
// CHECK-NEXT: ; _89 = copy _88 as u16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_89, var84): u16 <- ByVal(v80): u16
// CHECK-NEXT: ;
// CHECK-NEXT: ; _62 = core::intrinsics::ctlz::<u16>(move _89)
// CHECK-NEXT:     v81 = clz.i16 v80
// CHECK-NEXT:     v82 = uextend.i32 v81
// CHECK-NEXT:     v148 -> v82
// CHECK-NEXT: ; write_cvalue: Var(_62, var54): u32 <- ByVal(v82): u32
// CHECK-NEXT:     jump block23
// CHECK-EMPTY:
// CHECK-NEXT: block12:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _90 = copy _1 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_90, var85): usize <- ByVal(v83): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; _64 = core::intrinsics::cttz::<usize>(copy _90)
// CHECK-NEXT:     v84 = ctz.i16 v83
// CHECK-NEXT:     v85 = uextend.i32 v84
// CHECK-NEXT:     v151 -> v85
// CHECK-NEXT: ; write_cvalue: Var(_64, var56): u32 <- ByVal(v85): u32
// CHECK-NEXT:     jump block24
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _91 = copy _1 as usize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_91, var86): usize <- ByVal(v86): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; _65 = core::intrinsics::ctpop::<usize>(copy _91)
// CHECK-NEXT:     v87 = popcnt.i16 v86
// CHECK-NEXT:     v88 = uextend.i32 v87
// CHECK-NEXT:     v154 -> v88
// CHECK-NEXT: ; write_cvalue: Var(_65, var57): u32 <- ByVal(v88): u32
// CHECK-NEXT:     jump block25
// CHECK-EMPTY:
// CHECK-NEXT: block14(v161: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = const false
// CHECK-NEXT:     v89 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v89): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block15(v89, v161)  ; v89 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block15(v90: i8, v93: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _59 = move _60 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_59, var51): u8 <- ByVal(v90): u8
// CHECK-NEXT: ; _58 = Shl(move _59, const 7_i32)
// CHECK-NEXT:     v91 = iconst.i32 7
// CHECK-NEXT:     v92 = ishl v90, v91  ; v91 = 7
// CHECK-NEXT: ; write_cvalue: Var(_58, var50): u8 <- ByVal(v92): u8
// CHECK-NEXT: ; _0 = BitOr(move _8, move _58)
// CHECK-NEXT:     v94 = bor v93, v92
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v94): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v94
// CHECK-EMPTY:
// CHECK-NEXT: block16(v95: i8, v98: i16, v99: i16):
// CHECK-NEXT:     v35 -> v98
// CHECK-NEXT:     v171 -> v98
// CHECK-NEXT:     v172 -> v98
// CHECK-NEXT:     v36 -> v99
// CHECK-NEXT:     v173 -> v99
// CHECK-NEXT:     v174 -> v99
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = Not(move _66)
// CHECK-NEXT:     v96 = iconst.i8 0
// CHECK-NEXT:     v97 = icmp eq v95, v96  ; v96 = 0
// CHECK-NEXT:     v33 -> v97
// CHECK-NEXT: ; write_cvalue: Var(_15, var13): bool <- ByVal(v97): bool
// CHECK-NEXT: ; _14 = move _15 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_14, var12): u8 <- ByVal(v97): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _22 = core::num::<impl isize>::checked_sub(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(i16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027808258 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: isize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(i16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462603027808258 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<isize>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union i16, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: u16, valid_range: 0..=1 }), uninhabited: false, variants: Multiple { tag: u16 is 0..=1, tag_encoding: Direct, tag_field: 0, variants: [VariantLayout { size: Size(2 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(4 bytes), backend_repr: ScalarPair { a: u16 is 0..=1, b: union i16, b_offset: Size(2 bytes) }, field_offsets: [Size(2 bytes)], fields_in_memory_order: [0], largest_niche: None, uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8189316930324039666 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: , arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v100, v101 = call fn5(v98, v99)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): core::option::Option<isize> <- ByValPair(v100, v101): core::option::Option<isize>
// CHECK-NEXT:     v102 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v100, v102
// CHECK-NEXT:     v103 = stack_addr.i16 ss4+2
// CHECK-NEXT:     store notrap v101, v103
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block17:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _66 = const false
// CHECK-NEXT:     v104 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_66, var58): bool <- ByVal(v104): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block16(v104, v165, v167)  ; v104 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block18:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _66 = const true
// CHECK-NEXT:     v105 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_66, var58): bool <- ByVal(v105): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block16(v105, v166, v168)  ; v105 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block19 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; unreachable
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block20(v106: i8, v111: i8, v113: i16, v114: i16):
// CHECK-NEXT:     v45 -> v113
// CHECK-NEXT:     v58 -> v113
// CHECK-NEXT:     v80 -> v113
// CHECK-NEXT:     v158 -> v113
// CHECK-NEXT:     v159 -> v113
// CHECK-NEXT:     v160 -> v113
// CHECK-NEXT:     v86 -> v160
// CHECK-NEXT:     v83 -> v159
// CHECK-NEXT:     v46 -> v114
// CHECK-NEXT:     v59 -> v114
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _26 = Not(move _68)
// CHECK-NEXT:     v107 = iconst.i8 0
// CHECK-NEXT:     v108 = icmp eq v106, v107  ; v107 = 0
// CHECK-NEXT: ; write_cvalue: Var(_26, var22): bool <- ByVal(v108): bool
// CHECK-NEXT: ; _25 = move _26 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_25, var21): u8 <- ByVal(v108): u8
// CHECK-NEXT: ; _24 = Shl(move _25, const 2_i32)
// CHECK-NEXT:     v109 = iconst.i32 2
// CHECK-NEXT:     v110 = ishl v108, v109  ; v109 = 2
// CHECK-NEXT: ; write_cvalue: Var(_24, var20): u8 <- ByVal(v110): u8
// CHECK-NEXT: ; _12 = BitOr(move _13, move _24)
// CHECK-NEXT:     v112 = bor v111, v110
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): u8 <- ByVal(v112): u8
// CHECK-NEXT: ; _32 = Add(copy _1, copy _2)
// CHECK-NEXT:     v115 = iadd v113, v114
// CHECK-NEXT: ; write_cvalue: Var(_32, var27): isize <- ByVal(v115): isize
// CHECK-NEXT: ; _33 = copy (_3.0: isize)
// CHECK-NEXT: ; write_cvalue: Var(_33, var28): isize <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): isize
// CHECK-NEXT:     v116 = stack_addr.i16 ss0
// CHECK-NEXT:     v117 = load.i16 notrap v116
// CHECK-NEXT: ; _31 = Eq(move _32, move _33)
// CHECK-NEXT:     v118 = icmp eq v115, v117
// CHECK-NEXT: ; write_cvalue: Var(_31, var26): bool <- ByVal(v118): bool
// CHECK-NEXT: ; _30 = move _31 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_30, var25): u8 <- ByVal(v118): u8
// CHECK-NEXT: ; _29 = Shl(move _30, const 3_i32)
// CHECK-NEXT:     v119 = iconst.i32 3
// CHECK-NEXT:     v120 = ishl v118, v119  ; v119 = 3
// CHECK-NEXT: ; write_cvalue: Var(_29, var24): u8 <- ByVal(v120): u8
// CHECK-NEXT: ; _11 = BitOr(move _12, move _29)
// CHECK-NEXT:     v121 = bor v112, v120
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): u8 <- ByVal(v121): u8
// CHECK-NEXT: ; _37 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v122 = isub v113, v114
// CHECK-NEXT: ; write_cvalue: Var(_37, var32): isize <- ByVal(v122): isize
// CHECK-NEXT: ; _38 = copy (_5.0: isize)
// CHECK-NEXT: ; write_cvalue: Var(_38, var33): isize <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): isize
// CHECK-NEXT:     v123 = stack_addr.i16 ss1
// CHECK-NEXT:     v124 = load.i16 notrap v123
// CHECK-NEXT: ; _36 = Eq(move _37, move _38)
// CHECK-NEXT:     v125 = icmp eq v122, v124
// CHECK-NEXT: ; write_cvalue: Var(_36, var31): bool <- ByVal(v125): bool
// CHECK-NEXT: ; _35 = move _36 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_35, var30): u8 <- ByVal(v125): u8
// CHECK-NEXT: ; _34 = Shl(move _35, const 4_i32)
// CHECK-NEXT:     v126 = iconst.i32 4
// CHECK-NEXT:     v127 = ishl v125, v126  ; v126 = 4
// CHECK-NEXT: ; write_cvalue: Var(_34, var29): u8 <- ByVal(v127): u8
// CHECK-NEXT: ; _10 = BitOr(move _11, move _34)
// CHECK-NEXT:     v128 = bor v121, v127
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): u8 <- ByVal(v128): u8
// CHECK-NEXT: ; _42 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v129 = imul v113, v114
// CHECK-NEXT: ; write_cvalue: Var(_42, var37): isize <- ByVal(v129): isize
// CHECK-NEXT: ; _43 = copy (_7.0: isize)
// CHECK-NEXT: ; write_cvalue: Var(_43, var38): isize <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): isize
// CHECK-NEXT:     v130 = stack_addr.i16 ss2
// CHECK-NEXT:     v131 = load.i16 notrap v130
// CHECK-NEXT: ; _41 = Eq(move _42, move _43)
// CHECK-NEXT:     v132 = icmp eq v129, v131
// CHECK-NEXT: ; write_cvalue: Var(_41, var36): bool <- ByVal(v132): bool
// CHECK-NEXT: ; _40 = move _41 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_40, var35): u8 <- ByVal(v132): u8
// CHECK-NEXT: ; _39 = Shl(move _40, const 5_i32)
// CHECK-NEXT:     v133 = iconst.i32 5
// CHECK-NEXT:     v134 = ishl v132, v133  ; v133 = 5
// CHECK-NEXT: ; write_cvalue: Var(_39, var34): u8 <- ByVal(v134): u8
// CHECK-NEXT: ; _9 = BitOr(move _10, move _39)
// CHECK-NEXT:     v135 = bor v128, v134
// CHECK-NEXT:     v78 -> v135
// CHECK-NEXT:     v157 -> v135
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): u8 <- ByVal(v135): u8
// CHECK-NEXT: ; _73 = copy _1 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_73, var66): i16 <- ByVal(v113): i16
// CHECK-NEXT: ; _74 = copy _2 as i16 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_74, var67): i16 <- ByVal(v114): i16
// CHECK-NEXT: ; _72 = AddWithOverflow(move _73, move _74)
// CHECK-NEXT:     v136 = iadd v113, v114
// CHECK-NEXT:     v137 = iconst.i16 0
// CHECK-NEXT:     v138 = icmp slt v114, v137  ; v137 = 0
// CHECK-NEXT:     v139 = icmp slt v136, v113
// CHECK-NEXT:     v140 = bxor v138, v139
// CHECK-NEXT: ; write_cvalue: VarPair(_72, var64, var65): (i16, bool) <- ByValPair(v136, v140): (i16, bool)
// CHECK-NEXT: ; _70 = copy (_72.0: i16)
// CHECK-NEXT: ; write_cvalue: Var(_70, var62): i16 <- ByVal(v136): i16
// CHECK-NEXT: ; _71 = copy (_72.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_71, var63): bool <- ByVal(v140): bool
// CHECK-NEXT: ; _75 = copy _70 as isize (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_75, var68): isize <- ByVal(v136): isize
// CHECK-NEXT: ; _49 = (move _75, copy _71)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(0) }, None): isize <- ByVal(v136): isize
// CHECK-NEXT:     v141 = stack_addr.i16 ss6
// CHECK-NEXT:     store notrap v136, v141
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss6), offset: Offset32(2) }, None): bool <- ByVal(v140): bool
// CHECK-NEXT:     v142 = stack_addr.i16 ss6+2
// CHECK-NEXT:     store notrap v140, v142
// CHECK-NEXT: ; _48 = &_49
// CHECK-NEXT:     v143 = stack_addr.i16 ss6
// CHECK-NEXT: ; write_cvalue: Var(_48, var43): &'{erased} (isize, bool) <- ByVal(v143): &'{erased} (isize, bool)
// CHECK-NEXT: ; _50 = &_3
// CHECK-NEXT:     v144 = stack_addr.i16 ss0
// CHECK-NEXT: ; write_cvalue: Var(_50, var44): &'{erased} (isize, bool) <- ByVal(v144): &'{erased} (isize, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _47 = <(isize, bool) as core::cmp::PartialEq>::eq(move _48, move _50)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(isize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(isize, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v145 = call fn6(v143, v144)
// CHECK-NEXT:     v44 -> v145
// CHECK-NEXT: ; write_cvalue: Var(_47, var42): bool <- ByVal(v145): bool
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _68 = const false
// CHECK-NEXT:     v146 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_68, var60): bool <- ByVal(v146): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block20(v146, v169, v171, v173)  ; v146 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _68 = const true
// CHECK-NEXT:     v147 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_68, var60): bool <- ByVal(v147): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block20(v147, v170, v172, v174)  ; v147 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block23:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _61 = Eq(move _62, const 1_u32)
// CHECK-NEXT:     v149 = iconst.i32 1
// CHECK-NEXT:     v150 = icmp.i32 eq v148, v149  ; v149 = 1
// CHECK-NEXT: ; write_cvalue: Var(_61, var53): bool <- ByVal(v150): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _61)
// CHECK-NEXT:     brif v150, block12, block14(v162)
// CHECK-EMPTY:
// CHECK-NEXT: block24:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _63 = Eq(move _64, const 0_u32)
// CHECK-NEXT:     v152 = iconst.i32 0
// CHECK-NEXT:     v153 = icmp.i32 eq v151, v152  ; v152 = 0
// CHECK-NEXT: ; write_cvalue: Var(_63, var55): bool <- ByVal(v153): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _63)
// CHECK-NEXT:     brif.i32 v151, block14(v163), block13
// CHECK-EMPTY:
// CHECK-NEXT: block25:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _60 = Eq(move _65, const 15_u32)
// CHECK-NEXT:     v155 = iconst.i32 15
// CHECK-NEXT:     v156 = icmp.i32 eq v154, v155  ; v155 = 15
// CHECK-NEXT: ; write_cvalue: Var(_60, var52): bool <- ByVal(v156): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block15(v156, v164)
// CHECK-NEXT: }
