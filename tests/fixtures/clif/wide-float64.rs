#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn float64(x: f64, y: f64) -> f64 { x + y }

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 { (float64(1.0, 2.0) == 3.0) as u8 }

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol float64
// CHECK-NEXT: function u0:0(i16 sret, f64, f64) system_v {
// CHECK-NEXT: ; symbol float64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(8 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } f64
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) f64
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) f64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    f64                               8b 2                var=1
// CHECK-NEXT: ; ssa   _2    f64                               8b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: f64, v2: f64):
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v4 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): f64 <- ByVal(v1): f64
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): f64 <- ByVal(v2): f64
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Add(copy _1, copy _2)
// CHECK-NEXT:     v5 = fadd.f64 v3, v4
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): f64 <- ByVal(v5): f64
// CHECK-NEXT:     store notrap v5, v0
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:1() -> i8 system_v {
// CHECK-NEXT: ; symbol gbc_main
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _2    f64                               8b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 8, align = 2
// CHECK-NEXT:     sig0 = (i16 sret, f64, f64) system_v
// CHECK-NEXT:     fn0 = colocated u0:0 sig0 ; Instance { def: Item(DefId(0:3 ~ wide_float64[..]::float64)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = float64(const 1f64, const 2f64)
// CHECK-NEXT:     v0 = f64const 0x1.0000000000000p0
// CHECK-NEXT:     v1 = f64const 0x1.0000000000000p1
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f64, layout: Layout { size: Size(8 bytes), align: Align(2 bytes), backend_repr: Scalar(f64 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 12884901895 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f64, layout: Layout { size: Size(8 bytes), align: Align(2 bytes), backend_repr: Scalar(f64 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 12884901895 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f64, layout: Layout { size: Size(8 bytes), align: Align(2 bytes), backend_repr: Scalar(f64 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 12884901895 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(8 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn0(v2, v0, v1)  ; v0 = 0x1.0000000000000p0, v1 = 0x1.0000000000000p1
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): f64 <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): f64
// CHECK-NEXT:     v3 = stack_addr.i16 ss0
// CHECK-NEXT:     v4 = load.f64 notrap v3
// CHECK-NEXT:     v5 -> v4
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = Eq(move _2, const 3f64)
// CHECK-NEXT:     v6 = f64const 0x1.8000000000000p1
// CHECK-NEXT:     v7 = fcmp.f64 eq v5, v6  ; v6 = 0x1.8000000000000p1
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v7): bool
// CHECK-NEXT: ; _0 = move _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v7): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
