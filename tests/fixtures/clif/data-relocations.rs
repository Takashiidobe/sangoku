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
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3mem8align_ofNtCslXJhWPDkcuL_16data_relocations6RecordEBC_
// CHECK-NEXT: function u0:0() -> i16 system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3mem8align_ofNtCslXJhWPDkcuL_16data_relocations6RecordEBC_
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
// CHECK-NEXT:     v0 = iconst.i16 2
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): usize <- ByVal(v0): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v0  ; v0 = 2
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3ptr13read_volatileNtCslXJhWPDkcuL_16data_relocations6RecordEBI_
// CHECK-NEXT: function u0:1(i16 sret, i16, i16) system_v {
// CHECK-NEXT: ; symbol _RINvNtCs40R3J0iFVxM_4core3ptr13read_volatileNtCslXJhWPDkcuL_16data_relocations6RecordEBI_
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(10 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } Record
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const Record
// CHECK-NEXT: ; arg  ???      = v2       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *const Record                     2b 2                var=1
// CHECK-NEXT: ; ssa   _2    bool                              1b 1                var=2
// CHECK-NEXT: ; zst   _3    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ssa   _4    *const ()                         2b 2                var=3
// CHECK-NEXT: ; ssa   _5    usize                             2b 2                var=4
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = () -> i8 uext system_v
// CHECK-NEXT:     sig1 = () -> i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:11 sig0 ; Instance { def: Item(DefId(1:3622 ~ core[..]::ub_checks::check_language_ub)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:0 sig1 ; Instance { def: Item(DefId(1:2951 ~ core[..]::mem::align_of)), args: [Record] }
// CHECK-NEXT:     fn2 = colocated u0:12 sig2 ; Instance { def: Item(DefId(1:29550 ~ core[..]::ptr::read_volatile::precondition_check)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     v5 -> v1
// CHECK-NEXT:     v7 -> v1
// CHECK-NEXT:     v9 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *const Record <- ByVal(v1): *const Record
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::ub_checks::check_language_ub()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v3 = call fn0()
// CHECK-NEXT:     v4 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): bool <- ByVal(v3): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _2)
// CHECK-NEXT:     brif.i8 v4, block3, block5
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = copy _1 as *const () (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): *const () <- ByVal(v5): *const ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::mem::align_of::<T>()
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [], ret: ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 0, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v6 = call fn1()
// CHECK-NEXT:     v8 -> v6
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): usize <- ByVal(v6): usize
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::ptr::read_volatile::precondition_check(move _4, move _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: (), layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [], in_memory_order: [] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v7, v8, v2)
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::intrinsics::volatile_load::<T>(copy _1)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): Record <- ByRef(Pointer { base: Addr(v9), offset: Offset32(0) }, None): Record
// CHECK-NEXT:     v10 = load.i16 notrap aligned v9
// CHECK-NEXT:     v11 = load.i16 notrap aligned v9+2
// CHECK-NEXT:     v12 = load.i16 notrap aligned v9+4
// CHECK-NEXT:     v13 = load.i16 notrap aligned v9+6
// CHECK-NEXT:     v14 = load.i16 notrap aligned v9+8
// CHECK-NEXT:     store notrap aligned v10, v0
// CHECK-NEXT:     store notrap aligned v11, v0+2
// CHECK-NEXT:     store notrap aligned v12, v0+4
// CHECK-NEXT:     store notrap aligned v13, v0+6
// CHECK-NEXT:     store notrap aligned v14, v0+8
// CHECK-NEXT:     jump block6
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
// CHECK-NEXT: ; symbol _RNvCslXJhWPDkcuL_16data_relocations9increment
// CHECK-NEXT: function u0:2(i8) -> i8 system_v {
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
// CHECK-NEXT:     sig0 = (i8, i8) -> i8 system_v
// CHECK-NEXT:     fn0 = colocated u0:7 sig0 ; Instance { def: Item(DefId(1:27478 ~ core[..]::num::{impl#6}::wrapping_add)), args: [] }
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
// CHECK-NEXT: ; _0 = core::num::<impl u8>::wrapping_add(copy _1, const 1_u8)
// CHECK-NEXT:     v2 = iconst.i8 1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v3 = call fn0(v1, v2)  ; v2 = 1
// CHECK-NEXT:     v4 -> v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v3): u8
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
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3stre3lenCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:3(i16, i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3stre3lenCslXJhWPDkcuL_16data_relocations
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
// CHECK-NEXT:     fn0 = colocated u0:5 sig0 ; Instance { def: Item(DefId(1:16669 ~ core[..]::str::{impl#0}::as_bytes)), args: [] }
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
// CHECK-NEXT: function u0:4(i16, i16) -> i16 system_v {
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
// CHECK-NEXT: function u0:5(i16, i16) -> i16, i16 system_v {
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
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3fmtNtB5_9Arguments8from_strCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:6(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3fmtNtB5_9Arguments8from_strCslXJhWPDkcuL_16data_relocations
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
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(1:46132 ~ core[..]::str::{impl#0}::as_ptr)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:3 sig1 ; Instance { def: Item(DefId(1:16660 ~ core[..]::str::{impl#0}::len)), args: [] }
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
// CHECK-NEXT: ; symbol _RNvMs4_NtCs40R3J0iFVxM_4core3numh12wrapping_add
// CHECK-NEXT: function u0:7(i8, i8) -> i8 system_v {
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
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:8(i16, i16, i16) system_v {
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
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking18panic_nounwind_fmtCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:9(i16, i16, i8 uext, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9panicking18panic_nounwind_fmtCslXJhWPDkcuL_16data_relocations
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
// CHECK-NEXT:     fn0 = colocated u0:13 sig0 ; Instance { def: Item(DefId(1:43505 ~ core[..]::panicking::panic_nounwind_fmt::runtime)), args: [] }
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
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks16maybe_is_alignedCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:10(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks16maybe_is_alignedCslXJhWPDkcuL_16data_relocations
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
// CHECK-NEXT:     fn0 = colocated u0:14 sig0 ; Instance { def: Item(DefId(1:29565 ~ core[..]::ub_checks::maybe_is_aligned::runtime)), args: [] }
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
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks17check_language_ubCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:11() -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNtCs40R3J0iFVxM_4core9ub_checks17check_language_ubCslXJhWPDkcuL_16data_relocations
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    bool                              1b 1                var=2
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = () -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:15 sig0 ; Instance { def: Item(DefId(1:29563 ~ core[..]::ub_checks::check_language_ub::runtime)), args: [] }
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
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core3ptr13read_volatile18precondition_checkCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:12(i16, i16, i16) system_v {
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
// CHECK-NEXT: ; zst   _4    !                                 0b 1                align=1,offset=
// CHECK-NEXT: ; stack _5    core::fmt::Arguments<'{erased}>    4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _6    &'{erased} str                    4b 2                var=(4, 5)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname1 ; alloc8
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16, i8 uext, i16) system_v
// CHECK-NEXT:     fn0 = colocated u0:10 sig0 ; Instance { def: Item(DefId(1:3624 ~ core[..]::ub_checks::maybe_is_aligned)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:6 sig1 ; Instance { def: Item(DefId(1:13533 ~ core[..]::fmt::{impl#6}::from_str)), args: ['{erased}] }
// CHECK-NEXT:     fn2 = colocated u0:9 sig2 ; Instance { def: Item(DefId(1:12245 ~ core[..]::panicking::panic_nounwind_fmt)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     v3 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *const () <- ByVal(v0): *const ()
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::ub_checks::maybe_is_aligned(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const (), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v5 = call fn0(v3, v4)
// CHECK-NEXT:     v6 -> v5
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): bool <- ByVal(v5): bool
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _3)
// CHECK-NEXT:     brif.i8 v6, block3, block4
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
// CHECK-NEXT: ; _6 = const "unsafe precondition(s) violated: ptr::read_volatile requires that the pointer argument is aligned\n\nThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety."
// CHECK-NEXT:     v7 = symbol_value.i16 gv0
// CHECK-NEXT:     v8 = iconst.i16 214
// CHECK-NEXT: ; write_cvalue: VarPair(_6, var4, var5): &'{erased} str <- ByValPair(v7, v8): &'{erased} str
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::fmt::Arguments::<'_>::from_str(move _6)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v9, v10 = call fn1(v7, v8)  ; v8 = 214
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::fmt::Arguments<'{erased}> <- ByValPair(v9, v10): core::fmt::Arguments<'{erased}>
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v9, v11
// CHECK-NEXT:     v12 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v10, v12
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::panicking::panic_nounwind_fmt(move _5, const false)
// CHECK-NEXT:     v13 = iconst.i8 0
// CHECK-NEXT:     v14 = stack_addr.i16 ss0
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v17 = load.i16 notrap v16
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::fmt::Arguments<'_>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 1..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4959498110788724035 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: !, layout: Layout { size: Size(0 bytes), align: Align(1 bytes), backend_repr: Memory { sized: true }, fields: Primitive, largest_niche: None, uninhabited: true, variants: Empty, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 0 } }, mode: Ignore }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v15, v17, v13, v2)  ; v13 = 0
// CHECK-NEXT:     trap user1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9panicking18panic_nounwind_fmt7runtimeCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:13(i16, i16, i8 uext, i16) system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9panicking18panic_nounwind_fmt7runtimeCslXJhWPDkcuL_16data_relocations
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
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks16maybe_is_aligned7runtimeCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:14(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks16maybe_is_aligned7runtimeCslXJhWPDkcuL_16data_relocations
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
// CHECK-NEXT:     fn0 = u0:17 sig0 ; Instance { def: Item(DefId(1:3348 ~ core[..]::ptr::const_ptr::{impl#0}::is_aligned_to)), args: [()] }
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
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks17check_language_ub7runtimeCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT: function u0:15() -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvNvNtCs40R3J0iFVxM_4core9ub_checks17check_language_ub7runtimeCslXJhWPDkcuL_16data_relocations
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
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:16() -> i8 system_v {
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
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 10, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; DefId(0:10 ~ data_relocations[..]::RECORD)
// CHECK-NEXT:     gv1 = symbol colocated userextname1 ; alloc10
// CHECK-NEXT:     gv2 = symbol colocated userextname4 ; alloc11
// CHECK-NEXT:     gv3 = symbol colocated userextname6 ; alloc12
// CHECK-NEXT:     sig0 = (i16 sret, i16, i16) system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig2 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig3 = (i16, i16, i16) system_v
// CHECK-NEXT:     sig4 = (i8) -> i8 system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(1:3565 ~ core[..]::ptr::read_volatile)), args: [Record] }
// CHECK-NEXT:     fn1 = colocated u0:5 sig1 ; Instance { def: Item(DefId(1:16669 ~ core[..]::str::{impl#0}::as_bytes)), args: [] }
// CHECK-NEXT:     fn2 = colocated u0:8 sig2 ; "_RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations"
// CHECK-NEXT:     fn3 = colocated u0:8 sig3 ; "_RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations"
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
// CHECK-NEXT: ; _1 = core::ptr::read_volatile::<Record>(move _2)
// CHECK-NEXT:     v4 = symbol_value.i16 gv1
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *const Record, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: Record, layout: Layout { size: Size(10 bytes), align: Align(2 bytes), backend_repr: Memory { sized: true }, fields: Arbitrary { offsets: [Size(0 bytes), Size(4 bytes), Size(8 bytes)], in_memory_order: [0, 1, 2] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 13782714424700682705 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(10 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn0(v5, v3, v4)
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _19 = no_retag copy (_1.0: &str)
// CHECK-NEXT: ; write_cvalue: VarPair(_19, var20, var21): &'{erased} str <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} str
// CHECK-NEXT:     v6 = stack_addr.i16 ss0
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT:     v8 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v9 = load.i16 notrap v8
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = core::str::<impl str>::as_bytes(copy _19)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &str, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: &[u8], layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v10, v11 = call fn1(v7, v9)
// CHECK-NEXT:     v13 -> v10
// CHECK-NEXT:     v19 -> v10
// CHECK-NEXT:     v14 -> v11
// CHECK-NEXT:     v16 -> v11
// CHECK-NEXT:     v20 -> v11
// CHECK-NEXT: ; write_cvalue: VarPair(_7, var6, var7): &'{erased} [u8] <- ByValPair(v10, v11): &'{erased} [u8]
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = const 1_usize
// CHECK-NEXT:     v12 = iconst.i16 1
// CHECK-NEXT:     v17 -> v12
// CHECK-NEXT:     v21 -> v12
// CHECK-NEXT: ; write_cvalue: Var(_8, var8): usize <- ByVal(v12): usize
// CHECK-NEXT: ; _9 = PtrMetadata(copy _7)
// CHECK-NEXT: ; write_cvalue: Var(_9, var9): usize <- ByVal(v14): usize
// CHECK-NEXT: ; _10 = Lt(copy _8, copy _9)
// CHECK-NEXT:     v15 = icmp ult v12, v14  ; v12 = 1
// CHECK-NEXT: ; write_cvalue: Var(_10, var10): bool <- ByVal(v15): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(move _10, "index out of bounds: the length is {} but the index is {}", move _9, copy _8)
// CHECK-NEXT:     brif v15, block4, block11
// CHECK-EMPTY:
// CHECK-NEXT: block11 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v18 = symbol_value.i16 gv2
// CHECK-NEXT: ; panic _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn2(v17, v16, v18)  ; v17 = 1
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy (*_7)[_8]
// CHECK-NEXT:     v22 = iconst.i16 1
// CHECK-NEXT:     v23 = imul.i16 v21, v22  ; v21 = 1, v22 = 1
// CHECK-NEXT:     v24 = iadd.i16 v19, v23
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): u8 <- ByRef(Pointer { base: Addr(v24), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v25 = load.i8 notrap v24
// CHECK-NEXT: ; _5 = Eq(move _6, const 98_u8)
// CHECK-NEXT:     v26 = iconst.i8 98
// CHECK-NEXT:     v27 = icmp eq v25, v26  ; v26 = 98
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): bool <- ByVal(v27): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _5)
// CHECK-NEXT:     brif v27, block5, block8
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _13 = const 2_usize
// CHECK-NEXT:     v28 = iconst.i16 2
// CHECK-NEXT:     v41 -> v28
// CHECK-NEXT:     v47 -> v28
// CHECK-NEXT: ; write_cvalue: Var(_13, var13): usize <- ByVal(v28): usize
// CHECK-NEXT: ; _20 = no_retag copy (_1.1: &[u8])
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): &'{erased} [u8] <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(4) }, None): &'{erased} [u8]
// CHECK-NEXT:     v29 = stack_addr.i16 ss0+4
// CHECK-NEXT:     v30 = load.i16 notrap v29
// CHECK-NEXT:     v31 = stack_addr.i16 ss0+6
// CHECK-NEXT:     v32 = load.i16 notrap v31
// CHECK-NEXT:     v33 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v30, v33
// CHECK-NEXT:     v34 = stack_addr.i16 ss2+2
// CHECK-NEXT:     store notrap v32, v34
// CHECK-NEXT: ; _14 = &raw const (fake) (*_20)
// CHECK-NEXT:     v35 = stack_addr.i16 ss2
// CHECK-NEXT:     v36 = load.i16 notrap v35
// CHECK-NEXT:     v37 = stack_addr.i16 ss2+2
// CHECK-NEXT:     v38 = load.i16 notrap v37
// CHECK-NEXT:     v40 -> v38
// CHECK-NEXT: ; write_cvalue: VarPair(_14, var14, var15): *const [u8] <- ByValPair(v36, v38): *const [u8]
// CHECK-NEXT: ; _15 = PtrMetadata(move _14)
// CHECK-NEXT: ; write_cvalue: Var(_15, var16): usize <- ByVal(v38): usize
// CHECK-NEXT: ; _16 = Lt(copy _13, copy _15)
// CHECK-NEXT:     v39 = icmp ult v28, v38  ; v28 = 2
// CHECK-NEXT: ; write_cvalue: Var(_16, var17): bool <- ByVal(v39): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; assert(move _16, "index out of bounds: the length is {} but the index is {}", move _15, copy _13)
// CHECK-NEXT:     brif v39, block6, block12
// CHECK-EMPTY:
// CHECK-NEXT: block12 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT:     v42 = symbol_value.i16 gv3
// CHECK-NEXT: ; panic _RNvNtCs40R3J0iFVxM_4core9panicking18panic_bounds_checkCslXJhWPDkcuL_16data_relocations
// CHECK-NEXT:     nop
// CHECK-NEXT:     call fn3(v41, v40, v42)  ; v41 = 2
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _21 = no_retag copy (_1.1: &[u8])
// CHECK-NEXT: ; write_cvalue: VarPair(_21, var22, var23): &'{erased} [u8] <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(4) }, None): &'{erased} [u8]
// CHECK-NEXT:     v43 = stack_addr.i16 ss0+4
// CHECK-NEXT:     v44 = load.i16 notrap v43
// CHECK-NEXT:     v45 = stack_addr.i16 ss0+6
// CHECK-NEXT:     v46 = load.i16 notrap v45
// CHECK-NEXT: ; _12 = copy (*_21)[_13]
// CHECK-NEXT:     v48 = iconst.i16 1
// CHECK-NEXT:     v49 = imul.i16 v47, v48  ; v47 = 2, v48 = 1
// CHECK-NEXT:     v50 = iadd v44, v49
// CHECK-NEXT: ; write_cvalue: Var(_12, var12): u8 <- ByRef(Pointer { base: Addr(v50), offset: Offset32(0) }, None): u8
// CHECK-NEXT:     v51 = load.i8 notrap v50
// CHECK-NEXT: ; _11 = Eq(move _12, const 9_u8)
// CHECK-NEXT:     v52 = iconst.i8 9
// CHECK-NEXT:     v53 = icmp eq v51, v52  ; v52 = 9
// CHECK-NEXT: ; write_cvalue: Var(_11, var11): bool <- ByVal(v53): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _11)
// CHECK-NEXT:     brif v53, block7, block8
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _18 = copy (_1.2: fn(u8) -> u8)
// CHECK-NEXT: ; write_cvalue: Var(_18, var19): Binder { value: fn(u8) -> u8, bound_vars: [] } <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(8) }, None): Binder { value: fn(u8) -> u8, bound_vars: [] }
// CHECK-NEXT:     v54 = stack_addr.i16 ss0+8
// CHECK-NEXT:     v55 = load.i16 notrap v54
// CHECK-NEXT: ;
// CHECK-NEXT: ; _17 = move _18(const 4_u8)
// CHECK-NEXT:     v56 = iconst.i8 4
// CHECK-NEXT:     nop
// CHECK-NEXT: ; indirect call
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u8, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 71776127651151873 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v57 = call_indirect sig4, v55(v56)  ; v56 = 4
// CHECK-NEXT:     v59 -> v57
// CHECK-NEXT: ; write_cvalue: Var(_17, var18): u8 <- ByVal(v57): u8
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = const false
// CHECK-NEXT:     v58 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): bool <- ByVal(v58): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block10(v58)  ; v58 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = Eq(move _17, const 5_u8)
// CHECK-NEXT:     v60 = iconst.i8 5
// CHECK-NEXT:     v61 = icmp.i8 eq v59, v60  ; v60 = 5
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): bool <- ByVal(v61): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block10(v61)
// CHECK-EMPTY:
// CHECK-NEXT: block10(v62: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _4 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v62): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v62
// CHECK-NEXT: }
