#![no_std]

pub struct Record { pub text: &'static str, pub bytes: &'static [u8], pub function: fn(u8) -> u8 }

#[inline(never)]
fn increment(x: u8) -> u8 { x.wrapping_add(1) }

static BYTES: [u8; 3] = [7, 8, 9];
static RECORD: Record = Record { text: "gb", bytes: &BYTES, function: increment };

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let record = unsafe { core::ptr::read_volatile(&RECORD) };
    (record.text.as_bytes()[1] == b'b' && record.bytes[2] == 9 && (record.function)(4) == 5) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvCslXJhWPDkcuL_16data_relocations9increment
// CHECK-NEXT: function u0:0(i8) -> i8 system_v {
// CHECK-NEXT: ; symbol _RNvCslXJhWPDkcuL_16data_relocations9increment
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
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
// CHECK-NEXT: ; _0 = Add(copy _1, const 1_u8)
// CHECK-NEXT:     v2 = iconst.i8 1
// CHECK-NEXT:     v3 = iadd.i8 v1, v2  ; v2 = 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v3): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v3
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:1(i16, i16, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations
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
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:2() -> i8 system_v {
// CHECK-NEXT: ; symbol gbc_main
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    Record                           10b 2                storage=ss0
// CHECK-NEXT: ; ssa   _2    *const Record                     2b 2                var=2
// CHECK-NEXT: ; stack _3    &'{erased} Record                 2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=3
// CHECK-NEXT: ; ssa   _5    u8                                1b 1                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} [u8]                   4b 2                var=(5, 6)
// CHECK-NEXT: ; ssa   _7    usize                             2b 2                var=7
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=8
// CHECK-NEXT: ; ssa   _9    u8                                1b 1                var=9
// CHECK-NEXT: ; ssa   _10   usize                             2b 2                var=10
// CHECK-NEXT: ; ssa   _11   bool                              1b 1                var=11
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _13   Binder { value: fn(u8) -> u8, bound_vars: [] }    2b 2                var=13
// CHECK-NEXT: ; ssa   _14   &'{erased} str                    4b 2                var=(14, 15)
// CHECK-NEXT: ; ssa   _15   &'{erased} [u8]                   4b 2                var=(16, 17)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; DefId(0:10 ~ data_relocations[..]::RECORD)
// CHECK-NEXT:     gv1 = symbol colocated userextname1 ; alloc9
// CHECK-NEXT:     gv2 = symbol colocated userextname3 ; alloc10
// CHECK-NEXT:     sig0 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig1 = (i8) -> i8 system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; "_RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations"
// CHECK-NEXT:     fn1 = colocated u0:1 sig2 ; "_RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations"
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = const {alloc4: &Record}
// CHECK-NEXT:     v0 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} Record <- ByVal(v0): &'{erased} Record
// CHECK-NEXT:     v1 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v0, v1
// CHECK-NEXT: ; _2 = &raw const (*_3)
// CHECK-NEXT:     v2 = stack_addr.i16 ss1
// CHECK-NEXT:     v3 = load.i16 notrap v2
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): *const Record <- ByVal(v3): *const Record
// CHECK-NEXT: ;
// CHECK-NEXT: ; _1 = core::intrinsics::volatile_load::<Record>(move _2)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): Record <- ByRef(Pointer { base: Addr(v3), offset: Offset32(0) }, None): Record
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap aligned v3
// CHECK-NEXT:     store notrap aligned v5, v4
// CHECK-NEXT:     v6 = load.i16 notrap aligned v3+2
// CHECK-NEXT:     store notrap aligned v6, v4+2
// CHECK-NEXT:     v7 = load.i16 notrap aligned v3+4
// CHECK-NEXT:     store notrap aligned v7, v4+4
// CHECK-NEXT:     v8 = load.i16 notrap aligned v3+6
// CHECK-NEXT:     store notrap aligned v8, v4+6
// CHECK-NEXT:     v9 = load.i16 notrap aligned v3+8
// CHECK-NEXT:     store notrap aligned v9, v4+8
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = copy (*_6)[1 of 2]
// CHECK-NEXT:     v12 = iconst.i16 1
// CHECK-NEXT:     v13 = iconst.i16 1
// CHECK-NEXT:     v14 = imul v12, v13  ; v12 = 1, v13 = 1
// CHECK-NEXT:     v15 = iadd.i16 v10, v14
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): u8 <- ByRef(Pointer { base: Addr(v15), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v16 = load.i8 notrap v15
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _5)
// CHECK-NEXT:     v17 = iconst.i8 98
// CHECK-NEXT:     v18 = icmp eq v16, v17  ; v17 = 98
// CHECK-NEXT:     brif v18, block3, block6
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = no_retag copy (_1.1: &[u8])
// CHECK-NEXT: ; write_cvalue: VarPair(_15, var16, var17): &'{erased} [u8] <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(4) }, None): &'{erased} [u8]
// CHECK-NEXT:     v19 = stack_addr.i16 ss0+4
// CHECK-NEXT:     v20 = load.i16 notrap v19
// CHECK-NEXT:     v28 -> v20
// CHECK-NEXT:     v21 = stack_addr.i16 ss0+6
// CHECK-NEXT:     v22 = load.i16 notrap v21
// CHECK-NEXT:     v25 -> v22
// CHECK-NEXT:     v29 -> v22
// CHECK-NEXT: ; _10 = PtrMetadata(copy _15)
// CHECK-NEXT: ; write_cvalue: Var(_10, var10): usize <- ByVal(v22): usize
// CHECK-NEXT: ; _11 = Lt(const 2_usize, copy _10)
// CHECK-NEXT:     v23 = iconst.i16 2
// CHECK-NEXT:     v24 = icmp ult v23, v22  ; v23 = 2
// CHECK-NEXT: ; write_cvalue: Var(_11, var11): bool <- ByVal(v24): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(move _11, "index out of bounds: the length is {} but the index is {}", move _10, const 2_usize)
// CHECK-NEXT:     brif v24, block4, block10
// CHECK-EMPTY:
// CHECK-NEXT: block10 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v26 = iconst.i16 2
// CHECK-NEXT:     v27 = symbol_value.i16 gv1
// CHECK-NEXT: ; panic _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn0(v26, v25, v27)  ; v26 = 2
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = copy (*_15)[2 of 3]
// CHECK-NEXT:     v30 = iconst.i16 2
// CHECK-NEXT:     v31 = iconst.i16 1
// CHECK-NEXT:     v32 = imul v30, v31  ; v30 = 2, v31 = 1
// CHECK-NEXT:     v33 = iadd.i16 v28, v32
// CHECK-NEXT: ; write_cvalue: Var(_9, var9): u8 <- ByRef(Pointer { base: Addr(v33), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v34 = load.i8 notrap v33
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _9)
// CHECK-NEXT:     v35 = iconst.i8 9
// CHECK-NEXT:     v36 = icmp eq v34, v35  ; v35 = 9
// CHECK-NEXT:     brif v36, block5, block6
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _13 = copy (_1.2: fn(u8) -> u8)
// CHECK-NEXT: ; write_cvalue: Var(_13, var13): Binder { value: fn(u8) -> u8, bound_vars: [] } <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(8) }, None): Binder { value: fn(u8) -> u8, bound_vars: [] }
// CHECK-NEXT:     v37 = stack_addr.i16 ss0+8
// CHECK-NEXT:     v38 = load.i16 notrap v37
// CHECK-NEXT: ;
// CHECK-NEXT: ; _12 = copy _13(const 4_u8)
// CHECK-NEXT:     v39 = iconst.i8 4
// CHECK-NEXT:     nop
// CHECK-NEXT: ; indirect call
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v40 = call_indirect sig1, v38(v39)  ; v39 = 4
// CHECK-NEXT:     v42 -> v40
// CHECK-NEXT: ; write_cvalue: Var(_12, var12): u8 <- ByVal(v40): u8
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = const false
// CHECK-NEXT:     v41 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): bool <- ByVal(v41): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block8(v41)  ; v41 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = Eq(move _12, const 5_u8)
// CHECK-NEXT:     v43 = iconst.i8 5
// CHECK-NEXT:     v44 = icmp.i8 eq v42, v43  ; v43 = 5
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): bool <- ByVal(v44): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block8(v44)
// CHECK-EMPTY:
// CHECK-NEXT: block8(v45: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _4 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v45): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v45
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _14 = no_retag copy (_1.0: &str)
// CHECK-NEXT: ; write_cvalue: VarPair(_14, var14, var15): &'{erased} str <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} str
// CHECK-NEXT:     v46 = stack_addr.i16 ss0
// CHECK-NEXT:     v47 = load.i16 notrap v46
// CHECK-NEXT:     v10 -> v47
// CHECK-NEXT:     v48 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v49 = load.i16 notrap v48
// CHECK-NEXT:     v11 -> v49
// CHECK-NEXT:     v52 -> v49
// CHECK-NEXT: ; _6 = copy _14 as &[u8] (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: VarPair(_6, var5, var6): &'{erased} [u8] <- ByValPair(v47, v49): &'{erased} str
// CHECK-NEXT: ; _7 = PtrMetadata(copy _6)
// CHECK-NEXT: ; write_cvalue: Var(_7, var7): usize <- ByVal(v49): usize
// CHECK-NEXT: ; _8 = Lt(const 1_usize, copy _7)
// CHECK-NEXT:     v50 = iconst.i16 1
// CHECK-NEXT:     v51 = icmp ult v50, v49  ; v50 = 1
// CHECK-NEXT: ; write_cvalue: Var(_8, var8): bool <- ByVal(v51): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(move _8, "index out of bounds: the length is {} but the index is {}", move _7, const 1_usize)
// CHECK-NEXT:     brif v51, block2, block11
// CHECK-EMPTY:
// CHECK-NEXT: block11 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v53 = iconst.i16 1
// CHECK-NEXT:     v54 = symbol_value.i16 gv2
// CHECK-NEXT: ; panic _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn1(v53, v52, v54)  ; v53 = 1
// CHECK-NEXT:     trap user1
// CHECK-NEXT: }
