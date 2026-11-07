#![no_std]
#![feature(f16)]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn float16(x: f16, y: f16) -> f16 { x + y }

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 { (float16(1.0, 2.0) == 3.0) as u8 }

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol float16
// CHECK-NEXT: function u0:0(f16, f16) -> f16 system_v {
// CHECK-NEXT: ; symbol float16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    f16                               2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) f16
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) f16
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) f16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    f16                               2b 2                var=2
// CHECK-NEXT: ; ssa   _2    f16                               2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (f16) -> f32 system_v
// CHECK-NEXT:     sig1 = (f16) -> f32 system_v
// CHECK-NEXT:     sig2 = (f32) -> f16 system_v
// CHECK-NEXT:     fn0 = u0:2 sig0 ; "__extendhfsf2"
// CHECK-NEXT:     fn1 = u0:2 sig1 ; "__extendhfsf2"
// CHECK-NEXT:     fn2 = u0:3 sig2 ; "__truncsfhf2"
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: f16, v1: f16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): f16 <- ByVal(v0): f16
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): f16 <- ByVal(v1): f16
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Add(copy _1, copy _2)
// CHECK-NEXT: ; lib_call __extendhfsf2
// CHECK-NEXT:     v4 = call fn0(v2)
// CHECK-NEXT: ; lib_call __extendhfsf2
// CHECK-NEXT:     v5 = call fn1(v3)
// CHECK-NEXT:     v6 = fadd v4, v5
// CHECK-NEXT: ; lib_call __truncsfhf2
// CHECK-NEXT:     v7 = call fn2(v6)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): f16 <- ByVal(v7): f16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
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
// CHECK-NEXT: ; ssa   _2    f16                               2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig1 = (f16) -> f32 system_v
// CHECK-NEXT:     sig2 = (f16) -> f32 system_v
// CHECK-NEXT:     fn0 = colocated u0:0 sig0 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn1 = u0:2 sig1 ; "__extendhfsf2"
// CHECK-NEXT:     fn2 = u0:2 sig2 ; "__extendhfsf2"
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = float16(const 1f16, const 2f16)
// CHECK-NEXT:     v0 = f16const 0x1.000p0
// CHECK-NEXT:     v1 = f16const 0x1.000p1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0(v0, v1)  ; v0 = 0x1.000p0, v1 = 0x1.000p1
// CHECK-NEXT:     v3 -> v2
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): f16 <- ByVal(v2): f16
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = Eq(move _2, const 3f16)
// CHECK-NEXT:     v4 = f16const 0x1.800p1
// CHECK-NEXT: ; lib_call __extendhfsf2
// CHECK-NEXT:     v5 = call fn1(v3)
// CHECK-NEXT: ; lib_call __extendhfsf2
// CHECK-NEXT:     v6 = call fn2(v4)  ; v4 = 0x1.800p1
// CHECK-NEXT:     v7 = fcmp eq v5, v6
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v7): bool
// CHECK-NEXT: ; _0 = move _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v7): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
