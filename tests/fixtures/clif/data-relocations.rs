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
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr13read_volatile18precondition_checkCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:2(i16, i16, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr13read_volatile18precondition_checkCslXJhWPDkcuL_16data_relocations
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const ()
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg  ???      = v2       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const ()                         2b 2                var=1
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=2
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=3
// CHECK-NEXT: ; ssa   _4    (*const (), usize)                4b 2                var=(4, 5)
// CHECK-NEXT: ; ssa   _5    *const ()                         2b 2                var=6
// CHECK-NEXT: ; ssa   _6    usize                             2b 2                var=7
// CHECK-NEXT: ; ssa   _7    bool                              1b 1                var=8
// CHECK-NEXT: ; ssa   _8    usize                             2b 2                var=9
// CHECK-NEXT: ; ssa   _9    usize                             2b 2                var=10
// CHECK-NEXT: ; ssa   _10   usize                             2b 2                var=11
// CHECK-NEXT: ; ssa   _11   u32                               4b 2                var=12
// CHECK-NEXT: ; zst   _12   !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; zst   _13   !                                 0b 1                align=1,offset=
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     v3 -> v0
// CHECK-NEXT:     v17 -> v0
// CHECK-NEXT:     v7 -> v17
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     v18 -> v1
// CHECK-NEXT:     v8 -> v18
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = (copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): *const () <- ByVal(v3): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): usize <- ByVal(v4): usize
// CHECK-NEXT: ; _5 = move (_4.0: *const ())
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): *const () <- ByVal(v3): *const ()
// CHECK-NEXT: ; _6 = move (_4.1: usize)
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): usize <- ByVal(v4): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; _11 = core::intrinsics::ctpop::<usize>(copy _6)
// CHECK-NEXT:     v5 = popcnt.i16 v4
// CHECK-NEXT:     v6 = uextend.i32 v5
// CHECK-NEXT:     v14 -> v6
// CHECK-NEXT: ; write_cvalue: Var(_11, var12): u32 <- ByVal(v6): u32
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-EMPTY:
// CHECK-NEXT: block3 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = core::intrinsics::abort_immediate()
// CHECK-NEXT:     trap user2
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = copy _5 as usize (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_9, var10): usize <- ByVal(v7): *const ()
// CHECK-NEXT: ; _10 = Sub(copy _6, const 1_usize)
// CHECK-NEXT:     v9 = iconst.i16 1
// CHECK-NEXT:     v10 = isub.i16 v8, v9  ; v9 = 1
// CHECK-NEXT: ; write_cvalue: Var(_10, var11): usize <- ByVal(v10): usize
// CHECK-NEXT: ; _8 = BitAnd(move _9, move _10)
// CHECK-NEXT:     v11 = band.i16 v7, v10
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): usize <- ByVal(v11): usize
// CHECK-NEXT: ; _3 = Eq(move _8, const 0_usize)
// CHECK-NEXT:     v12 = iconst.i16 0
// CHECK-NEXT:     v13 = icmp eq v11, v12  ; v12 = 0
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): bool <- ByVal(v13): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _3)
// CHECK-NEXT:     brif v11, block3, block2
// CHECK-EMPTY:
// CHECK-NEXT: block5 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _12 = core::intrinsics::abort_immediate()
// CHECK-NEXT:     trap user2
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _7 = Eq(move _11, const 1_u32)
// CHECK-NEXT:     v15 = iconst.i32 1
// CHECK-NEXT:     v16 = icmp.i32 eq v14, v15  ; v15 = 1
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): bool <- ByVal(v16): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _7)
// CHECK-NEXT:     brif v16, block4, block5
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:3() -> i8 system_v {
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
// CHECK-NEXT: ; ssa   _5    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _6    u8                                1b 1                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} [u8]                   4b 2                var=(6, 7)
// CHECK-NEXT: ; ssa   _8    usize                             2b 2                var=8
// CHECK-NEXT: ; ssa   _9    usize                             2b 2                var=9
// CHECK-NEXT: ; ssa   _10   bool                              1b 1                var=10
// CHECK-NEXT: ; ssa   _11   bool                              1b 1                var=11
// CHECK-NEXT: ; ssa   _12   u8                                1b 1                var=12
// CHECK-NEXT: ; ssa   _13   usize                             2b 2                var=13
// CHECK-NEXT: ; ssa   _14   *const [u8]                       4b 2                var=(14, 15)
// CHECK-NEXT: ; ssa   _15   usize                             2b 2                var=16
// CHECK-NEXT: ; ssa   _16   bool                              1b 1                var=17
// CHECK-NEXT: ; ssa   _17   u8                                1b 1                var=18
// CHECK-NEXT: ; ssa   _18   Binder { value: fn(u8) -> u8, bound_vars: [] }    2b 2                var=19
// CHECK-NEXT: ; ssa   _19   &'{erased} str                    4b 2                var=(20, 21)
// CHECK-NEXT: ; stack _20   &'{erased} [u8]                   4b 2                storage=ss2
// CHECK-NEXT: ; ssa   _21   &'{erased} [u8]                   4b 2                var=(22, 23)
// CHECK-NEXT: ; ssa   _22   bool                              1b 1                var=24
// CHECK-NEXT: ; zst   _23   ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _24   *const ()                         2b 2                var=25
// CHECK-NEXT: ; ssa   _25   bool                              1b 1                var=26
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; DefId(0:10 ~ data_relocations[..]::RECORD)
// CHECK-NEXT:     gv1 = symbol colocated userextname1 ; alloc10
// CHECK-NEXT:     gv2 = symbol colocated userextname3 ; alloc11
// CHECK-NEXT:     gv3 = symbol colocated userextname4 ; alloc12
// CHECK-NEXT:     sig0 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig1 = (i8) -> i8 system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig3 = (i16, i16, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; "_RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations"
// CHECK-NEXT:     fn1 = colocated u0:1 sig2 ; "_RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations"
// CHECK-NEXT:     fn2 = colocated u0:2 sig3 ; Instance { def: Item(DefId(1:29550 ~ core[..]::ptr::read_volatile::precondition_check)), args: [] }
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
// CHECK-NEXT:     v74 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): *const Record <- ByVal(v3): *const Record
// CHECK-NEXT: ; _25 = Not(const false)
// CHECK-NEXT:     v4 = iconst.i8 0
// CHECK-NEXT:     v5 = iconst.i8 0
// CHECK-NEXT:     v6 = icmp eq v4, v5  ; v4 = 0, v5 = 0
// CHECK-NEXT: ; write_cvalue: Var(_25, var26): bool <- ByVal(v6): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _25)
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy (*_7)[_8]
// CHECK-NEXT:     v10 = iconst.i16 1
// CHECK-NEXT:     v11 = imul.i16 v9, v10  ; v9 = 1, v10 = 1
// CHECK-NEXT:     v12 = iadd.i16 v7, v11
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): u8 <- ByRef(Pointer { base: Addr(v12), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v13 = load.i8 notrap v12
// CHECK-NEXT: ; _5 = Eq(move _6, const 98_u8)
// CHECK-NEXT:     v14 = iconst.i8 98
// CHECK-NEXT:     v15 = icmp eq v13, v14  ; v14 = 98
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): bool <- ByVal(v15): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _5)
// CHECK-NEXT:     brif v15, block3, block6
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _13 = const 2_usize
// CHECK-NEXT:     v16 = iconst.i16 2
// CHECK-NEXT:     v29 -> v16
// CHECK-NEXT:     v35 -> v16
// CHECK-NEXT: ; write_cvalue: Var(_13, var13): usize <- ByVal(v16): usize
// CHECK-NEXT: ; _20 = no_retag copy (_1.1: &[u8])
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): &'{erased} [u8] <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(4) }, None): &'{erased} [u8]
// CHECK-NEXT:     v17 = stack_addr.i16 ss0+4
// CHECK-NEXT:     v18 = load.i16 notrap v17
// CHECK-NEXT:     v19 = stack_addr.i16 ss0+6
// CHECK-NEXT:     v20 = load.i16 notrap v19
// CHECK-NEXT:     v21 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v18, v21
// CHECK-NEXT:     v22 = stack_addr.i16 ss2+2
// CHECK-NEXT:     store notrap v20, v22
// CHECK-NEXT: ; _14 = &raw const (fake) (*_20)
// CHECK-NEXT:     v23 = stack_addr.i16 ss2
// CHECK-NEXT:     v24 = load.i16 notrap v23
// CHECK-NEXT:     v25 = stack_addr.i16 ss2+2
// CHECK-NEXT:     v26 = load.i16 notrap v25
// CHECK-NEXT:     v28 -> v26
// CHECK-NEXT: ; write_cvalue: VarPair(_14, var14, var15): *const [u8] <- ByValPair(v24, v26): *const [u8]
// CHECK-NEXT: ; _15 = PtrMetadata(move _14)
// CHECK-NEXT: ; write_cvalue: Var(_15, var16): usize <- ByVal(v26): usize
// CHECK-NEXT: ; _16 = Lt(copy _13, copy _15)
// CHECK-NEXT:     v27 = icmp ult v16, v26  ; v16 = 2
// CHECK-NEXT: ; write_cvalue: Var(_16, var17): bool <- ByVal(v27): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(move _16, "index out of bounds: the length is {} but the index is {}", move _15, copy _13)
// CHECK-NEXT:     brif v27, block4, block15
// CHECK-EMPTY:
// CHECK-NEXT: block15 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v30 = symbol_value.i16 gv1
// CHECK-NEXT: ; panic _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn0(v29, v28, v30)  ; v29 = 2
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _21 = no_retag copy (_1.1: &[u8])
// CHECK-NEXT: ; write_cvalue: VarPair(_21, var22, var23): &'{erased} [u8] <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(4) }, None): &'{erased} [u8]
// CHECK-NEXT:     v31 = stack_addr.i16 ss0+4
// CHECK-NEXT:     v32 = load.i16 notrap v31
// CHECK-NEXT:     v33 = stack_addr.i16 ss0+6
// CHECK-NEXT:     v34 = load.i16 notrap v33
// CHECK-NEXT: ; _12 = copy (*_21)[_13]
// CHECK-NEXT:     v36 = iconst.i16 1
// CHECK-NEXT:     v37 = imul.i16 v35, v36  ; v35 = 2, v36 = 1
// CHECK-NEXT:     v38 = iadd v32, v37
// CHECK-NEXT: ; write_cvalue: Var(_12, var12): u8 <- ByRef(Pointer { base: Addr(v38), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v39 = load.i8 notrap v38
// CHECK-NEXT: ; _11 = Eq(move _12, const 9_u8)
// CHECK-NEXT:     v40 = iconst.i8 9
// CHECK-NEXT:     v41 = icmp eq v39, v40  ; v40 = 9
// CHECK-NEXT: ; write_cvalue: Var(_11, var11): bool <- ByVal(v41): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _11)
// CHECK-NEXT:     brif v41, block5, block6
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _18 = copy (_1.2: fn(u8) -> u8)
// CHECK-NEXT: ; write_cvalue: Var(_18, var19): Binder { value: fn(u8) -> u8, bound_vars: [] } <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(8) }, None): Binder { value: fn(u8) -> u8, bound_vars: [] }
// CHECK-NEXT:     v42 = stack_addr.i16 ss0+8
// CHECK-NEXT:     v43 = load.i16 notrap v42
// CHECK-NEXT: ;
// CHECK-NEXT: ; _17 = move _18(const 4_u8)
// CHECK-NEXT:     v44 = iconst.i8 4
// CHECK-NEXT:     nop
// CHECK-NEXT: ; indirect call
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v45 = call_indirect sig1, v43(v44)  ; v44 = 4
// CHECK-NEXT:     v47 -> v45
// CHECK-NEXT: ; write_cvalue: Var(_17, var18): u8 <- ByVal(v45): u8
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = const false
// CHECK-NEXT:     v46 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): bool <- ByVal(v46): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block8(v46)  ; v46 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = Eq(move _17, const 5_u8)
// CHECK-NEXT:     v48 = iconst.i8 5
// CHECK-NEXT:     v49 = icmp.i8 eq v47, v48  ; v48 = 5
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): bool <- ByVal(v49): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block8(v49)
// CHECK-EMPTY:
// CHECK-NEXT: block8(v50: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _4 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v50): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v50
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _19 = no_retag copy (_1.0: &str)
// CHECK-NEXT: ; write_cvalue: VarPair(_19, var20, var21): &'{erased} str <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} str
// CHECK-NEXT:     v51 = stack_addr.i16 ss0
// CHECK-NEXT:     v52 = load.i16 notrap v51
// CHECK-NEXT:     v7 -> v52
// CHECK-NEXT:     v53 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v54 = load.i16 notrap v53
// CHECK-NEXT:     v8 -> v54
// CHECK-NEXT:     v57 -> v54
// CHECK-NEXT: ; _7 = copy _19 as &[u8] (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: VarPair(_7, var6, var7): &'{erased} [u8] <- ByValPair(v52, v54): &'{erased} str
// CHECK-NEXT: ; _8 = const 1_usize
// CHECK-NEXT:     v55 = iconst.i16 1
// CHECK-NEXT:     v9 -> v55
// CHECK-NEXT:     v58 -> v55
// CHECK-NEXT: ; write_cvalue: Var(_8, var8): usize <- ByVal(v55): usize
// CHECK-NEXT: ; _9 = PtrMetadata(copy _7)
// CHECK-NEXT: ; write_cvalue: Var(_9, var9): usize <- ByVal(v54): usize
// CHECK-NEXT: ; _10 = Lt(copy _8, copy _9)
// CHECK-NEXT:     v56 = icmp ult v55, v54  ; v55 = 1
// CHECK-NEXT: ; write_cvalue: Var(_10, var10): bool <- ByVal(v56): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(move _10, "index out of bounds: the length is {} but the index is {}", move _9, copy _8)
// CHECK-NEXT:     brif v56, block2, block16
// CHECK-EMPTY:
// CHECK-NEXT: block16 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v59 = symbol_value.i16 gv2
// CHECK-NEXT: ; panic _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn1(v58, v57, v59)  ; v58 = 1
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _24 = copy _2 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_24, var25): *const () <- ByVal(v60): *const ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; _23 = core::ptr::read_volatile::precondition_check(move _24, const <Record as core::mem::SizedTypeProperties>::ALIGN)
// CHECK-NEXT:     v61 = iconst.i16 2
// CHECK-NEXT:     v62 = symbol_value.i16 gv3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v60, v61, v62)  ; v61 = 2
// CHECK-NEXT:     jump block11
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _1 = core::intrinsics::volatile_load::<Record>(copy _2)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): Record <- ByRef(Pointer { base: Addr(v63), offset: Offset32(0) }, None): Record
// CHECK-NEXT:     v64 = stack_addr.i16 ss0
// CHECK-NEXT:     v65 = load.i16 notrap aligned v63
// CHECK-NEXT:     v66 = load.i16 notrap aligned v63+2
// CHECK-NEXT:     v67 = load.i16 notrap aligned v63+4
// CHECK-NEXT:     v68 = load.i16 notrap aligned v63+6
// CHECK-NEXT:     v69 = load.i16 notrap aligned v63+8
// CHECK-NEXT:     store notrap aligned v65, v64
// CHECK-NEXT:     store notrap aligned v66, v64+2
// CHECK-NEXT:     store notrap aligned v67, v64+4
// CHECK-NEXT:     store notrap aligned v68, v64+6
// CHECK-NEXT:     store notrap aligned v69, v64+8
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block12(v70: i8, v73: i16):
// CHECK-NEXT:     v60 -> v73
// CHECK-NEXT:     v63 -> v73
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _22)
// CHECK-NEXT:     brif v70, block10, block11
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = const false
// CHECK-NEXT:     v71 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_22, var24): bool <- ByVal(v71): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v71, v74)  ; v71 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     v76 = iconst.i16 0
// CHECK-NEXT:     v75 -> v76
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _22 = const false
// CHECK-NEXT:     v72 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_22, var24): bool <- ByVal(v72): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block12(v72, v75)  ; v72 = 0, v75 = 0
// CHECK-NEXT: }
