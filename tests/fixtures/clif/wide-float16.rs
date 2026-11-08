#![no_std]
#![feature(f16)]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn float16(x: f16, y: f16) -> f16 {
    x + y
}

#[inline(never)]
#[unsafe(no_mangle)]
pub fn subtract16(x: f16, y: f16) -> f16 {
    x - y
}

#[inline(never)]
#[unsafe(no_mangle)]
pub fn multiply16(x: f16, y: f16) -> f16 {
    x * y
}

#[inline(never)]
#[unsafe(no_mangle)]
pub fn divide16(x: f16, y: f16) -> f16 {
    x / y
}

#[inline(never)]
#[unsafe(no_mangle)]
pub fn less16(x: f16, y: f16) -> bool {
    x < y
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    (float16(1.0, 2.0) == 3.0
        && float16(f16::from_bits(1), f16::from_bits(1)).to_bits() == 2
        && float16(-0.0, -0.0).to_bits() == 0x8000
        && float16(f16::MAX, f16::MAX).to_bits() == 0x7c00
        && float16(1.0, f16::from_bits(0x1000)).to_bits() == 0x3c00
        && float16(f16::from_bits(0x3c01), f16::from_bits(0x1000)).to_bits() == 0x3c02
        && float16(f16::INFINITY, f16::NEG_INFINITY).is_nan()
        && subtract16(3.0, 2.0) == 1.0
        && multiply16(-2.0, 3.0) == -6.0
        && divide16(3.0, 2.0) == 1.5
        && less16(-1.0, 1.0)
        && !less16(f16::NAN, 1.0)) as u8
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol divide16
// CHECK-NEXT: function u0:0(f16, f16) -> f16 system_v {
// CHECK-NEXT: ; symbol divide16
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
// CHECK-NEXT: ; _0 = Div(copy _1, copy _2)
// CHECK-NEXT:     v4 = fpromote.f32 v2
// CHECK-NEXT:     v5 = fpromote.f32 v3
// CHECK-NEXT:     v6 = fdiv v4, v5
// CHECK-NEXT:     v7 = fdemote.f16 v6
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): f16 <- ByVal(v7): f16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol float16
// CHECK-NEXT: function u0:1(f16, f16) -> f16 system_v {
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
// CHECK-NEXT:     v4 = fpromote.f32 v2
// CHECK-NEXT:     v5 = fpromote.f32 v3
// CHECK-NEXT:     v6 = fadd v4, v5
// CHECK-NEXT:     v7 = fdemote.f16 v6
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): f16 <- ByVal(v7): f16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
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
// CHECK-NEXT: ; ssa   _1    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _2    bool                              1b 1                var=3
// CHECK-NEXT: ; ssa   _3    f16                               2b 2                var=4
// CHECK-NEXT: ; ssa   _4    u16                               2b 2                var=5
// CHECK-NEXT: ; ssa   _5    f16                               2b 2                var=6
// CHECK-NEXT: ; ssa   _6    u16                               2b 2                var=7
// CHECK-NEXT: ; ssa   _7    f16                               2b 2                var=8
// CHECK-NEXT: ; ssa   _8    u16                               2b 2                var=9
// CHECK-NEXT: ; ssa   _9    f16                               2b 2                var=10
// CHECK-NEXT: ; ssa   _10   u16                               2b 2                var=11
// CHECK-NEXT: ; ssa   _11   f16                               2b 2                var=12
// CHECK-NEXT: ; ssa   _12   u16                               2b 2                var=13
// CHECK-NEXT: ; ssa   _13   f16                               2b 2                var=14
// CHECK-NEXT: ; ssa   _14   bool                              1b 1                var=15
// CHECK-NEXT: ; ssa   _15   f16                               2b 2                var=16
// CHECK-NEXT: ; ssa   _16   bool                              1b 1                var=17
// CHECK-NEXT: ; ssa   _17   f16                               2b 2                var=18
// CHECK-NEXT: ; ssa   _18   bool                              1b 1                var=19
// CHECK-NEXT: ; ssa   _19   f16                               2b 2                var=20
// CHECK-NEXT: ; ssa   _20   bool                              1b 1                var=21
// CHECK-NEXT: ; ssa   _21   f16                               2b 2                var=22
// CHECK-NEXT: ; ssa   _22   bool                              1b 1                var=23
// CHECK-NEXT: ; ssa   _23   bool                              1b 1                var=24
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig1 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig2 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig3 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig4 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig5 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig6 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig7 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig8 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig9 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig10 = (f16, f16) -> i8 uext system_v
// CHECK-NEXT:     sig11 = (f16, f16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:1 sig1 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn2 = colocated u0:1 sig2 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn3 = colocated u0:1 sig3 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn4 = colocated u0:1 sig4 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn5 = colocated u0:1 sig5 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn6 = colocated u0:1 sig6 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn7 = colocated u0:5 sig7 ; Instance { def: Item(DefId(0:4 ~ wide_float16[..]::subtract16)), args: [] }
// CHECK-NEXT:     fn8 = colocated u0:4 sig8 ; Instance { def: Item(DefId(0:5 ~ wide_float16[..]::multiply16)), args: [] }
// CHECK-NEXT:     fn9 = colocated u0:0 sig9 ; Instance { def: Item(DefId(0:6 ~ wide_float16[..]::divide16)), args: [] }
// CHECK-NEXT:     fn10 = colocated u0:3 sig10 ; Instance { def: Item(DefId(0:7 ~ wide_float16[..]::less16)), args: [] }
// CHECK-NEXT:     fn11 = colocated u0:3 sig11 ; Instance { def: Item(DefId(0:7 ~ wide_float16[..]::less16)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = float16(const 1f16, const 2f16)
// CHECK-NEXT:     v0 = f16const 0x1.000p0
// CHECK-NEXT:     v1 = f16const 0x1.000p1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v2 = call fn0(v0, v1)  ; v0 = 0x1.000p0, v1 = 0x1.000p1
// CHECK-NEXT:     v3 -> v2
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): f16 <- ByVal(v2): f16
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = Eq(move _3, const 3f16)
// CHECK-NEXT:     v4 = f16const 0x1.800p1
// CHECK-NEXT:     v5 = fpromote.f32 v3
// CHECK-NEXT:     v6 = fpromote.f32 v4  ; v4 = 0x1.800p1
// CHECK-NEXT:     v7 = fcmp eq v5, v6
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): bool <- ByVal(v7): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _2)
// CHECK-NEXT:     brif v7, block3, block21
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = float16(const 5.9605E-8f16, const 5.9605E-8f16)
// CHECK-NEXT:     v8 = f16const 0x0.004p-14
// CHECK-NEXT:     v9 = f16const 0x0.004p-14
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v10 = call fn1(v8, v9)  ; v8 = 0x0.004p-14, v9 = 0x0.004p-14
// CHECK-NEXT:     v11 -> v10
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): f16 <- ByVal(v10): f16
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = copy _5 as u16 (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_4, var5): u16 <- ByVal(v11): f16
// CHECK-NEXT:     v12 = bitcast.i16 little v11
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _4)
// CHECK-NEXT:     v13 = iconst.i16 2
// CHECK-NEXT:     v14 = icmp eq v12, v13  ; v13 = 2
// CHECK-NEXT:     brif v14, block5, block21
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = float16(const -0f16, const -0f16)
// CHECK-NEXT:     v15 = f16const -0.0
// CHECK-NEXT:     v16 = f16const -0.0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v17 = call fn2(v15, v16)  ; v15 = -0.0, v16 = -0.0
// CHECK-NEXT:     v74 -> v17
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): f16 <- ByVal(v17): f16
// CHECK-NEXT:     jump block24
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _9 = float16(const 65504f16, const 65504f16)
// CHECK-NEXT:     v18 = f16const 0x1.ffcp15
// CHECK-NEXT:     v19 = f16const 0x1.ffcp15
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v20 = call fn3(v18, v19)  ; v18 = 0x1.ffcp15, v19 = 0x1.ffcp15
// CHECK-NEXT:     v78 -> v20
// CHECK-NEXT: ; write_cvalue: Var(_9, var10): f16 <- ByVal(v20): f16
// CHECK-NEXT:     jump block25
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _11 = float16(const 1f16, const 4.8828E-4f16)
// CHECK-NEXT:     v21 = f16const 0x1.000p0
// CHECK-NEXT:     v22 = f16const 0x1.000p-11
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v23 = call fn4(v21, v22)  ; v21 = 0x1.000p0, v22 = 0x1.000p-11
// CHECK-NEXT:     v24 -> v23
// CHECK-NEXT: ; write_cvalue: Var(_11, var12): f16 <- ByVal(v23): f16
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _10 = copy _11 as u16 (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_10, var11): u16 <- ByVal(v24): f16
// CHECK-NEXT:     v25 = bitcast.i16 little v24
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _10)
// CHECK-NEXT:     v26 = iconst.i16 0x3c00
// CHECK-NEXT:     v27 = icmp eq v25, v26  ; v26 = 0x3c00
// CHECK-NEXT:     brif v27, block9, block21
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = float16(const 1.001f16, const 4.8828E-4f16)
// CHECK-NEXT:     v28 = f16const 0x1.004p0
// CHECK-NEXT:     v29 = f16const 0x1.000p-11
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v30 = call fn5(v28, v29)  ; v28 = 0x1.004p0, v29 = 0x1.000p-11
// CHECK-NEXT:     v31 -> v30
// CHECK-NEXT: ; write_cvalue: Var(_13, var14): f16 <- ByVal(v30): f16
// CHECK-NEXT:     jump block10
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _12 = copy _13 as u16 (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_12, var13): u16 <- ByVal(v31): f16
// CHECK-NEXT:     v32 = bitcast.i16 little v31
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _12)
// CHECK-NEXT:     v33 = iconst.i16 0x3c02
// CHECK-NEXT:     v34 = icmp eq v32, v33  ; v33 = 0x3c02
// CHECK-NEXT:     brif v34, block11, block21
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _15 = float16(const +Inf_f16, const -Inf_f16)
// CHECK-NEXT:     v35 = f16const +Inf
// CHECK-NEXT:     v36 = f16const -Inf
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v37 = call fn6(v35, v36)  ; v35 = +Inf, v36 = -Inf
// CHECK-NEXT:     v82 -> v37
// CHECK-NEXT: ; write_cvalue: Var(_15, var16): f16 <- ByVal(v37): f16
// CHECK-NEXT:     jump block26
// CHECK-EMPTY:
// CHECK-NEXT: block12:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _17 = subtract16(const 3f16, const 2f16)
// CHECK-NEXT:     v38 = f16const 0x1.800p1
// CHECK-NEXT:     v39 = f16const 0x1.000p1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v40 = call fn7(v38, v39)  ; v38 = 0x1.800p1, v39 = 0x1.000p1
// CHECK-NEXT:     v41 -> v40
// CHECK-NEXT: ; write_cvalue: Var(_17, var18): f16 <- ByVal(v40): f16
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = Eq(move _17, const 1f16)
// CHECK-NEXT:     v42 = f16const 0x1.000p0
// CHECK-NEXT:     v43 = fpromote.f32 v41
// CHECK-NEXT:     v44 = fpromote.f32 v42  ; v42 = 0x1.000p0
// CHECK-NEXT:     v45 = fcmp eq v43, v44
// CHECK-NEXT: ; write_cvalue: Var(_16, var17): bool <- ByVal(v45): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _16)
// CHECK-NEXT:     brif v45, block14, block21
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _19 = multiply16(const -2f16, const 3f16)
// CHECK-NEXT:     v46 = f16const -0x1.000p1
// CHECK-NEXT:     v47 = f16const 0x1.800p1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v48 = call fn8(v46, v47)  ; v46 = -0x1.000p1, v47 = 0x1.800p1
// CHECK-NEXT:     v49 -> v48
// CHECK-NEXT: ; write_cvalue: Var(_19, var20): f16 <- ByVal(v48): f16
// CHECK-NEXT:     jump block15
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _18 = Eq(move _19, const -6f16)
// CHECK-NEXT:     v50 = f16const -0x1.800p2
// CHECK-NEXT:     v51 = fpromote.f32 v49
// CHECK-NEXT:     v52 = fpromote.f32 v50  ; v50 = -0x1.800p2
// CHECK-NEXT:     v53 = fcmp eq v51, v52
// CHECK-NEXT: ; write_cvalue: Var(_18, var19): bool <- ByVal(v53): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _18)
// CHECK-NEXT:     brif v53, block16, block21
// CHECK-EMPTY:
// CHECK-NEXT: block16:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _21 = divide16(const 3f16, const 2f16)
// CHECK-NEXT:     v54 = f16const 0x1.800p1
// CHECK-NEXT:     v55 = f16const 0x1.000p1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v56 = call fn9(v54, v55)  ; v54 = 0x1.800p1, v55 = 0x1.000p1
// CHECK-NEXT:     v57 -> v56
// CHECK-NEXT: ; write_cvalue: Var(_21, var22): f16 <- ByVal(v56): f16
// CHECK-NEXT:     jump block17
// CHECK-EMPTY:
// CHECK-NEXT: block17:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _20 = Eq(move _21, const 1.5f16)
// CHECK-NEXT:     v58 = f16const 0x1.800p0
// CHECK-NEXT:     v59 = fpromote.f32 v57
// CHECK-NEXT:     v60 = fpromote.f32 v58  ; v58 = 0x1.800p0
// CHECK-NEXT:     v61 = fcmp eq v59, v60
// CHECK-NEXT: ; write_cvalue: Var(_20, var21): bool <- ByVal(v61): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _20)
// CHECK-NEXT:     brif v61, block18, block21
// CHECK-EMPTY:
// CHECK-NEXT: block18:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _22 = less16(const -1f16, const 1f16)
// CHECK-NEXT:     v62 = f16const -0x1.000p0
// CHECK-NEXT:     v63 = f16const 0x1.000p0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v64 = call fn10(v62, v63)  ; v62 = -0x1.000p0, v63 = 0x1.000p0
// CHECK-NEXT:     v65 -> v64
// CHECK-NEXT: ; write_cvalue: Var(_22, var23): bool <- ByVal(v64): bool
// CHECK-NEXT:     jump block19
// CHECK-EMPTY:
// CHECK-NEXT: block19:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _22)
// CHECK-NEXT:     brif.i8 v65, block20, block21
// CHECK-EMPTY:
// CHECK-NEXT: block20:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _23 = less16(const NaN_f16, const 1f16)
// CHECK-NEXT:     v66 = f16const +NaN
// CHECK-NEXT:     v67 = f16const 0x1.000p0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v68 = call fn11(v66, v67)  ; v66 = +NaN, v67 = 0x1.000p0
// CHECK-NEXT:     v70 -> v68
// CHECK-NEXT: ; write_cvalue: Var(_23, var24): bool <- ByVal(v68): bool
// CHECK-NEXT:     jump block22
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = const false
// CHECK-NEXT:     v69 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v69): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block23(v69)  ; v69 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = Not(move _23)
// CHECK-NEXT:     v71 = iconst.i8 0
// CHECK-NEXT:     v72 = icmp.i8 eq v70, v71  ; v71 = 0
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v72): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block23(v72)
// CHECK-EMPTY:
// CHECK-NEXT: block23(v73: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v73): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v73
// CHECK-EMPTY:
// CHECK-NEXT: block24:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy _7 as u16 (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_6, var7): u16 <- ByVal(v74): f16
// CHECK-NEXT:     v75 = bitcast.i16 little v74
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _6)
// CHECK-NEXT:     v76 = iconst.i16 -32768
// CHECK-NEXT:     v77 = icmp eq v75, v76  ; v76 = -32768
// CHECK-NEXT:     brif v77, block6, block21
// CHECK-EMPTY:
// CHECK-NEXT: block25:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = copy _9 as u16 (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_8, var9): u16 <- ByVal(v78): f16
// CHECK-NEXT:     v79 = bitcast.i16 little v78
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(copy _8)
// CHECK-NEXT:     v80 = iconst.i16 0x7c00
// CHECK-NEXT:     v81 = icmp eq v79, v80  ; v80 = 0x7c00
// CHECK-NEXT:     brif v81, block7, block21
// CHECK-EMPTY:
// CHECK-NEXT: block26:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _14 = Ne(copy _15, copy _15)
// CHECK-NEXT:     v83 = fpromote.f32 v82
// CHECK-NEXT:     v84 = fpromote.f32 v82
// CHECK-NEXT:     v85 = fcmp ne v83, v84
// CHECK-NEXT: ; write_cvalue: Var(_14, var15): bool <- ByVal(v85): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _14)
// CHECK-NEXT:     brif v85, block12, block21
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol less16
// CHECK-NEXT: function u0:3(f16, f16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol less16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) f16
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) f16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    f16                               2b 2                var=2
// CHECK-NEXT: ; ssa   _2    f16                               2b 2                var=3
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
// CHECK-NEXT: ; _0 = Lt(copy _1, copy _2)
// CHECK-NEXT:     v4 = fpromote.f32 v2
// CHECK-NEXT:     v5 = fpromote.f32 v3
// CHECK-NEXT:     v6 = fcmp lt v4, v5
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v6): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v6
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol multiply16
// CHECK-NEXT: function u0:4(f16, f16) -> f16 system_v {
// CHECK-NEXT: ; symbol multiply16
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
// CHECK-NEXT: ; _0 = Mul(copy _1, copy _2)
// CHECK-NEXT:     v4 = fpromote.f32 v2
// CHECK-NEXT:     v5 = fpromote.f32 v3
// CHECK-NEXT:     v6 = fmul v4, v5
// CHECK-NEXT:     v7 = fdemote.f16 v6
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): f16 <- ByVal(v7): f16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol subtract16
// CHECK-NEXT: function u0:5(f16, f16) -> f16 system_v {
// CHECK-NEXT: ; symbol subtract16
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
// CHECK-NEXT: ; _0 = Sub(copy _1, copy _2)
// CHECK-NEXT:     v4 = fpromote.f32 v2
// CHECK-NEXT:     v5 = fpromote.f32 v3
// CHECK-NEXT:     v6 = fsub v4, v5
// CHECK-NEXT:     v7 = fdemote.f16 v6
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): f16 <- ByVal(v7): f16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
