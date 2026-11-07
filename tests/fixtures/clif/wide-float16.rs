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
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3f16C3f166is_nanCscST64DsK93A_12wide_float16
// CHECK-NEXT: function u0:0(f16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3f16C3f166is_nanCscST64DsK93A_12wide_float16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) f16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    f16                               2b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: f16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): f16 <- ByVal(v0): f16
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Ne(copy _1, copy _1)
// CHECK-NEXT:     v2 = fpromote.f32 v1
// CHECK-NEXT:     v3 = fpromote.f32 v1
// CHECK-NEXT:     v4 = fcmp ne v2, v3
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v4): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3f16C3f167to_bitsCscST64DsK93A_12wide_float16
// CHECK-NEXT: function u0:1(f16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3f16C3f167to_bitsCscST64DsK93A_12wide_float16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u16                               2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u16
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) f16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    f16                               2b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: f16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): f16 <- ByVal(v0): f16
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = copy _1 as u16 (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_0, var1): u16 <- ByVal(v1): f16
// CHECK-NEXT:     v2 = bitcast.i16 little v1
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v2
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3f16C3f169from_bitsCscST64DsK93A_12wide_float16
// CHECK-NEXT: function u0:2(i16) -> f16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core3f16C3f169from_bitsCscST64DsK93A_12wide_float16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    f16                               2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) f16
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    u16                               2b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): u16 <- ByVal(v0): u16
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = copy _1 as f16 (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_0, var1): f16 <- ByVal(v1): u16
// CHECK-NEXT:     v2 = bitcast.f16 little v1
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v2
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol divide16
// CHECK-NEXT: function u0:3(f16, f16) -> f16 system_v {
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
// CHECK-NEXT: function u0:4(f16, f16) -> f16 system_v {
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
// CHECK-NEXT: function u0:5() -> i8 system_v {
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
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=5
// CHECK-NEXT: ; ssa   _5    u16                               2b 2                var=6
// CHECK-NEXT: ; ssa   _6    f16                               2b 2                var=7
// CHECK-NEXT: ; ssa   _7    f16                               2b 2                var=8
// CHECK-NEXT: ; ssa   _8    f16                               2b 2                var=9
// CHECK-NEXT: ; ssa   _9    bool                              1b 1                var=10
// CHECK-NEXT: ; ssa   _10   u16                               2b 2                var=11
// CHECK-NEXT: ; ssa   _11   f16                               2b 2                var=12
// CHECK-NEXT: ; ssa   _12   bool                              1b 1                var=13
// CHECK-NEXT: ; ssa   _13   u16                               2b 2                var=14
// CHECK-NEXT: ; ssa   _14   f16                               2b 2                var=15
// CHECK-NEXT: ; ssa   _15   bool                              1b 1                var=16
// CHECK-NEXT: ; ssa   _16   u16                               2b 2                var=17
// CHECK-NEXT: ; ssa   _17   f16                               2b 2                var=18
// CHECK-NEXT: ; ssa   _18   f16                               2b 2                var=19
// CHECK-NEXT: ; ssa   _19   bool                              1b 1                var=20
// CHECK-NEXT: ; ssa   _20   u16                               2b 2                var=21
// CHECK-NEXT: ; ssa   _21   f16                               2b 2                var=22
// CHECK-NEXT: ; ssa   _22   f16                               2b 2                var=23
// CHECK-NEXT: ; ssa   _23   f16                               2b 2                var=24
// CHECK-NEXT: ; ssa   _24   bool                              1b 1                var=25
// CHECK-NEXT: ; ssa   _25   f16                               2b 2                var=26
// CHECK-NEXT: ; ssa   _26   bool                              1b 1                var=27
// CHECK-NEXT: ; ssa   _27   f16                               2b 2                var=28
// CHECK-NEXT: ; ssa   _28   bool                              1b 1                var=29
// CHECK-NEXT: ; ssa   _29   f16                               2b 2                var=30
// CHECK-NEXT: ; ssa   _30   bool                              1b 1                var=31
// CHECK-NEXT: ; ssa   _31   f16                               2b 2                var=32
// CHECK-NEXT: ; ssa   _32   bool                              1b 1                var=33
// CHECK-NEXT: ; ssa   _33   bool                              1b 1                var=34
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig1 = (i16) -> f16 system_v
// CHECK-NEXT:     sig2 = (i16) -> f16 system_v
// CHECK-NEXT:     sig3 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig4 = (f16) -> i16 system_v
// CHECK-NEXT:     sig5 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig6 = (f16) -> i16 system_v
// CHECK-NEXT:     sig7 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig8 = (f16) -> i16 system_v
// CHECK-NEXT:     sig9 = (i16) -> f16 system_v
// CHECK-NEXT:     sig10 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig11 = (f16) -> i16 system_v
// CHECK-NEXT:     sig12 = (i16) -> f16 system_v
// CHECK-NEXT:     sig13 = (i16) -> f16 system_v
// CHECK-NEXT:     sig14 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig15 = (f16) -> i16 system_v
// CHECK-NEXT:     sig16 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig17 = (f16) -> i8 uext system_v
// CHECK-NEXT:     sig18 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig19 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig20 = (f16, f16) -> f16 system_v
// CHECK-NEXT:     sig21 = (f16, f16) -> i8 uext system_v
// CHECK-NEXT:     sig22 = (f16, f16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:2 sig1 ; Instance { def: Item(DefId(1:1513 ~ core[..]::f16::{impl#0}::from_bits)), args: [] }
// CHECK-NEXT:     fn2 = colocated u0:2 sig2 ; Instance { def: Item(DefId(1:1513 ~ core[..]::f16::{impl#0}::from_bits)), args: [] }
// CHECK-NEXT:     fn3 = colocated u0:4 sig3 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn4 = colocated u0:1 sig4 ; Instance { def: Item(DefId(1:1512 ~ core[..]::f16::{impl#0}::to_bits)), args: [] }
// CHECK-NEXT:     fn5 = colocated u0:4 sig5 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn6 = colocated u0:1 sig6 ; Instance { def: Item(DefId(1:1512 ~ core[..]::f16::{impl#0}::to_bits)), args: [] }
// CHECK-NEXT:     fn7 = colocated u0:4 sig7 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn8 = colocated u0:1 sig8 ; Instance { def: Item(DefId(1:1512 ~ core[..]::f16::{impl#0}::to_bits)), args: [] }
// CHECK-NEXT:     fn9 = colocated u0:2 sig9 ; Instance { def: Item(DefId(1:1513 ~ core[..]::f16::{impl#0}::from_bits)), args: [] }
// CHECK-NEXT:     fn10 = colocated u0:4 sig10 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn11 = colocated u0:1 sig11 ; Instance { def: Item(DefId(1:1512 ~ core[..]::f16::{impl#0}::to_bits)), args: [] }
// CHECK-NEXT:     fn12 = colocated u0:2 sig12 ; Instance { def: Item(DefId(1:1513 ~ core[..]::f16::{impl#0}::from_bits)), args: [] }
// CHECK-NEXT:     fn13 = colocated u0:2 sig13 ; Instance { def: Item(DefId(1:1513 ~ core[..]::f16::{impl#0}::from_bits)), args: [] }
// CHECK-NEXT:     fn14 = colocated u0:4 sig14 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn15 = colocated u0:1 sig15 ; Instance { def: Item(DefId(1:1512 ~ core[..]::f16::{impl#0}::to_bits)), args: [] }
// CHECK-NEXT:     fn16 = colocated u0:4 sig16 ; Instance { def: Item(DefId(0:3 ~ wide_float16[..]::float16)), args: [] }
// CHECK-NEXT:     fn17 = colocated u0:0 sig17 ; Instance { def: Item(DefId(1:1481 ~ core[..]::f16::{impl#0}::is_nan)), args: [] }
// CHECK-NEXT:     fn18 = colocated u0:8 sig18 ; Instance { def: Item(DefId(0:4 ~ wide_float16[..]::subtract16)), args: [] }
// CHECK-NEXT:     fn19 = colocated u0:7 sig19 ; Instance { def: Item(DefId(0:5 ~ wide_float16[..]::multiply16)), args: [] }
// CHECK-NEXT:     fn20 = colocated u0:3 sig20 ; Instance { def: Item(DefId(0:6 ~ wide_float16[..]::divide16)), args: [] }
// CHECK-NEXT:     fn21 = colocated u0:6 sig21 ; Instance { def: Item(DefId(0:7 ~ wide_float16[..]::less16)), args: [] }
// CHECK-NEXT:     fn22 = colocated u0:6 sig22 ; Instance { def: Item(DefId(0:7 ~ wide_float16[..]::less16)), args: [] }
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
// CHECK-NEXT:     brif v7, block3, block35
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = core::f16::<impl f16>::from_bits(const 1_u16)
// CHECK-NEXT:     v8 = iconst.i16 1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v9 = call fn1(v8)  ; v8 = 1
// CHECK-NEXT:     v12 -> v9
// CHECK-NEXT: ; write_cvalue: Var(_7, var8): f16 <- ByVal(v9): f16
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = core::f16::<impl f16>::from_bits(const 1_u16)
// CHECK-NEXT:     v10 = iconst.i16 1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v11 = call fn2(v10)  ; v10 = 1
// CHECK-NEXT:     v13 -> v11
// CHECK-NEXT: ; write_cvalue: Var(_8, var9): f16 <- ByVal(v11): f16
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = float16(move _7, move _8)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v14 = call fn3(v12, v13)
// CHECK-NEXT:     v15 -> v14
// CHECK-NEXT: ; write_cvalue: Var(_6, var7): f16 <- ByVal(v14): f16
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::f16::<impl f16>::to_bits(move _6)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v16 = call fn4(v15)
// CHECK-NEXT:     v17 -> v16
// CHECK-NEXT: ; write_cvalue: Var(_5, var6): u16 <- ByVal(v16): u16
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = Eq(move _5, const 2_u16)
// CHECK-NEXT:     v18 = iconst.i16 2
// CHECK-NEXT:     v19 = icmp.i16 eq v17, v18  ; v18 = 2
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): bool <- ByVal(v19): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _4)
// CHECK-NEXT:     brif v19, block8, block35
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _11 = float16(const -0f16, const -0f16)
// CHECK-NEXT:     v20 = f16const -0.0
// CHECK-NEXT:     v21 = f16const -0.0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v22 = call fn5(v20, v21)  ; v20 = -0.0, v21 = -0.0
// CHECK-NEXT:     v23 -> v22
// CHECK-NEXT: ; write_cvalue: Var(_11, var12): f16 <- ByVal(v22): f16
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _10 = core::f16::<impl f16>::to_bits(move _11)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v24 = call fn6(v23)
// CHECK-NEXT:     v25 -> v24
// CHECK-NEXT: ; write_cvalue: Var(_10, var11): u16 <- ByVal(v24): u16
// CHECK-NEXT:     jump block10
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = Eq(move _10, const 32768_u16)
// CHECK-NEXT:     v26 = iconst.i16 -32768
// CHECK-NEXT:     v27 = icmp.i16 eq v25, v26  ; v26 = -32768
// CHECK-NEXT: ; write_cvalue: Var(_9, var10): bool <- ByVal(v27): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _9)
// CHECK-NEXT:     brif v27, block11, block35
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _14 = float16(const core::f16::<impl f16>::MAX, const core::f16::<impl f16>::MAX)
// CHECK-NEXT:     v28 = f16const 0x1.ffcp15
// CHECK-NEXT:     v29 = f16const 0x1.ffcp15
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v30 = call fn7(v28, v29)  ; v28 = 0x1.ffcp15, v29 = 0x1.ffcp15
// CHECK-NEXT:     v31 -> v30
// CHECK-NEXT: ; write_cvalue: Var(_14, var15): f16 <- ByVal(v30): f16
// CHECK-NEXT:     jump block12
// CHECK-EMPTY:
// CHECK-NEXT: block12:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = core::f16::<impl f16>::to_bits(move _14)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v32 = call fn8(v31)
// CHECK-NEXT:     v33 -> v32
// CHECK-NEXT: ; write_cvalue: Var(_13, var14): u16 <- ByVal(v32): u16
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _12 = Eq(move _13, const 31744_u16)
// CHECK-NEXT:     v34 = iconst.i16 0x7c00
// CHECK-NEXT:     v35 = icmp.i16 eq v33, v34  ; v34 = 0x7c00
// CHECK-NEXT: ; write_cvalue: Var(_12, var13): bool <- ByVal(v35): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _12)
// CHECK-NEXT:     brif v35, block14, block35
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _18 = core::f16::<impl f16>::from_bits(const 4096_u16)
// CHECK-NEXT:     v36 = iconst.i16 4096
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v37 = call fn9(v36)  ; v36 = 4096
// CHECK-NEXT:     v39 -> v37
// CHECK-NEXT: ; write_cvalue: Var(_18, var19): f16 <- ByVal(v37): f16
// CHECK-NEXT:     jump block15
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _17 = float16(const 1f16, move _18)
// CHECK-NEXT:     v38 = f16const 0x1.000p0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v40 = call fn10(v38, v39)  ; v38 = 0x1.000p0
// CHECK-NEXT:     v41 -> v40
// CHECK-NEXT: ; write_cvalue: Var(_17, var18): f16 <- ByVal(v40): f16
// CHECK-NEXT:     jump block16
// CHECK-EMPTY:
// CHECK-NEXT: block16:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _16 = core::f16::<impl f16>::to_bits(move _17)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v42 = call fn11(v41)
// CHECK-NEXT:     v43 -> v42
// CHECK-NEXT: ; write_cvalue: Var(_16, var17): u16 <- ByVal(v42): u16
// CHECK-NEXT:     jump block17
// CHECK-EMPTY:
// CHECK-NEXT: block17:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = Eq(move _16, const 15360_u16)
// CHECK-NEXT:     v44 = iconst.i16 0x3c00
// CHECK-NEXT:     v45 = icmp.i16 eq v43, v44  ; v44 = 0x3c00
// CHECK-NEXT: ; write_cvalue: Var(_15, var16): bool <- ByVal(v45): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _15)
// CHECK-NEXT:     brif v45, block18, block35
// CHECK-EMPTY:
// CHECK-NEXT: block18:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _22 = core::f16::<impl f16>::from_bits(const 15361_u16)
// CHECK-NEXT:     v46 = iconst.i16 0x3c01
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v47 = call fn12(v46)  ; v46 = 0x3c01
// CHECK-NEXT:     v50 -> v47
// CHECK-NEXT: ; write_cvalue: Var(_22, var23): f16 <- ByVal(v47): f16
// CHECK-NEXT:     jump block19
// CHECK-EMPTY:
// CHECK-NEXT: block19:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _23 = core::f16::<impl f16>::from_bits(const 4096_u16)
// CHECK-NEXT:     v48 = iconst.i16 4096
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v49 = call fn13(v48)  ; v48 = 4096
// CHECK-NEXT:     v51 -> v49
// CHECK-NEXT: ; write_cvalue: Var(_23, var24): f16 <- ByVal(v49): f16
// CHECK-NEXT:     jump block20
// CHECK-EMPTY:
// CHECK-NEXT: block20:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _21 = float16(move _22, move _23)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v52 = call fn14(v50, v51)
// CHECK-NEXT:     v53 -> v52
// CHECK-NEXT: ; write_cvalue: Var(_21, var22): f16 <- ByVal(v52): f16
// CHECK-NEXT:     jump block21
// CHECK-EMPTY:
// CHECK-NEXT: block21:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _20 = core::f16::<impl f16>::to_bits(move _21)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v54 = call fn15(v53)
// CHECK-NEXT:     v55 -> v54
// CHECK-NEXT: ; write_cvalue: Var(_20, var21): u16 <- ByVal(v54): u16
// CHECK-NEXT:     jump block22
// CHECK-EMPTY:
// CHECK-NEXT: block22:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _19 = Eq(move _20, const 15362_u16)
// CHECK-NEXT:     v56 = iconst.i16 0x3c02
// CHECK-NEXT:     v57 = icmp.i16 eq v55, v56  ; v56 = 0x3c02
// CHECK-NEXT: ; write_cvalue: Var(_19, var20): bool <- ByVal(v57): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _19)
// CHECK-NEXT:     brif v57, block23, block35
// CHECK-EMPTY:
// CHECK-NEXT: block23:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _25 = float16(const core::f16::<impl f16>::INFINITY, const core::f16::<impl f16>::NEG_INFINITY)
// CHECK-NEXT:     v58 = f16const +Inf
// CHECK-NEXT:     v59 = f16const -Inf
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v60 = call fn16(v58, v59)  ; v58 = +Inf, v59 = -Inf
// CHECK-NEXT:     v61 -> v60
// CHECK-NEXT: ; write_cvalue: Var(_25, var26): f16 <- ByVal(v60): f16
// CHECK-NEXT:     jump block24
// CHECK-EMPTY:
// CHECK-NEXT: block24:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _24 = core::f16::<impl f16>::is_nan(move _25)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v62 = call fn17(v61)
// CHECK-NEXT:     v63 -> v62
// CHECK-NEXT: ; write_cvalue: Var(_24, var25): bool <- ByVal(v62): bool
// CHECK-NEXT:     jump block25
// CHECK-EMPTY:
// CHECK-NEXT: block25:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _24)
// CHECK-NEXT:     brif.i8 v63, block26, block35
// CHECK-EMPTY:
// CHECK-NEXT: block26:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _27 = subtract16(const 3f16, const 2f16)
// CHECK-NEXT:     v64 = f16const 0x1.800p1
// CHECK-NEXT:     v65 = f16const 0x1.000p1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v66 = call fn18(v64, v65)  ; v64 = 0x1.800p1, v65 = 0x1.000p1
// CHECK-NEXT:     v67 -> v66
// CHECK-NEXT: ; write_cvalue: Var(_27, var28): f16 <- ByVal(v66): f16
// CHECK-NEXT:     jump block27
// CHECK-EMPTY:
// CHECK-NEXT: block27:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _26 = Eq(move _27, const 1f16)
// CHECK-NEXT:     v68 = f16const 0x1.000p0
// CHECK-NEXT:     v69 = fpromote.f32 v67
// CHECK-NEXT:     v70 = fpromote.f32 v68  ; v68 = 0x1.000p0
// CHECK-NEXT:     v71 = fcmp eq v69, v70
// CHECK-NEXT: ; write_cvalue: Var(_26, var27): bool <- ByVal(v71): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _26)
// CHECK-NEXT:     brif v71, block28, block35
// CHECK-EMPTY:
// CHECK-NEXT: block28:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _29 = multiply16(const -2f16, const 3f16)
// CHECK-NEXT:     v72 = f16const -0x1.000p1
// CHECK-NEXT:     v73 = f16const 0x1.800p1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v74 = call fn19(v72, v73)  ; v72 = -0x1.000p1, v73 = 0x1.800p1
// CHECK-NEXT:     v75 -> v74
// CHECK-NEXT: ; write_cvalue: Var(_29, var30): f16 <- ByVal(v74): f16
// CHECK-NEXT:     jump block29
// CHECK-EMPTY:
// CHECK-NEXT: block29:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _28 = Eq(move _29, const -6f16)
// CHECK-NEXT:     v76 = f16const -0x1.800p2
// CHECK-NEXT:     v77 = fpromote.f32 v75
// CHECK-NEXT:     v78 = fpromote.f32 v76  ; v76 = -0x1.800p2
// CHECK-NEXT:     v79 = fcmp eq v77, v78
// CHECK-NEXT: ; write_cvalue: Var(_28, var29): bool <- ByVal(v79): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _28)
// CHECK-NEXT:     brif v79, block30, block35
// CHECK-EMPTY:
// CHECK-NEXT: block30:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _31 = divide16(const 3f16, const 2f16)
// CHECK-NEXT:     v80 = f16const 0x1.800p1
// CHECK-NEXT:     v81 = f16const 0x1.000p1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v82 = call fn20(v80, v81)  ; v80 = 0x1.800p1, v81 = 0x1.000p1
// CHECK-NEXT:     v83 -> v82
// CHECK-NEXT: ; write_cvalue: Var(_31, var32): f16 <- ByVal(v82): f16
// CHECK-NEXT:     jump block31
// CHECK-EMPTY:
// CHECK-NEXT: block31:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _30 = Eq(move _31, const 1.5f16)
// CHECK-NEXT:     v84 = f16const 0x1.800p0
// CHECK-NEXT:     v85 = fpromote.f32 v83
// CHECK-NEXT:     v86 = fpromote.f32 v84  ; v84 = 0x1.800p0
// CHECK-NEXT:     v87 = fcmp eq v85, v86
// CHECK-NEXT: ; write_cvalue: Var(_30, var31): bool <- ByVal(v87): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _30)
// CHECK-NEXT:     brif v87, block32, block35
// CHECK-EMPTY:
// CHECK-NEXT: block32:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _32 = less16(const -1f16, const 1f16)
// CHECK-NEXT:     v88 = f16const -0x1.000p0
// CHECK-NEXT:     v89 = f16const 0x1.000p0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v90 = call fn21(v88, v89)  ; v88 = -0x1.000p0, v89 = 0x1.000p0
// CHECK-NEXT:     v91 -> v90
// CHECK-NEXT: ; write_cvalue: Var(_32, var33): bool <- ByVal(v90): bool
// CHECK-NEXT:     jump block33
// CHECK-EMPTY:
// CHECK-NEXT: block33:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _32)
// CHECK-NEXT:     brif.i8 v91, block34, block35
// CHECK-EMPTY:
// CHECK-NEXT: block34:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _33 = less16(const core::f16::<impl f16>::NAN, const 1f16)
// CHECK-NEXT:     v92 = f16const +NaN
// CHECK-NEXT:     v93 = f16const 0x1.000p0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: f16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(f16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462611617742850 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v94 = call fn22(v92, v93)  ; v92 = +NaN, v93 = 0x1.000p0
// CHECK-NEXT:     v96 -> v94
// CHECK-NEXT: ; write_cvalue: Var(_33, var34): bool <- ByVal(v94): bool
// CHECK-NEXT:     jump block36
// CHECK-EMPTY:
// CHECK-NEXT: block35:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = const false
// CHECK-NEXT:     v95 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v95): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block37(v95)  ; v95 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block36:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = Not(move _33)
// CHECK-NEXT:     v97 = iconst.i8 0
// CHECK-NEXT:     v98 = icmp.i8 eq v96, v97  ; v97 = 0
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v98): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block37(v98)
// CHECK-EMPTY:
// CHECK-NEXT: block37(v99: i8):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = move _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v99): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v99
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol less16
// CHECK-NEXT: function u0:6(f16, f16) -> i8 uext system_v {
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
// CHECK-NEXT: function u0:7(f16, f16) -> f16 system_v {
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
// CHECK-NEXT: function u0:8(f16, f16) -> f16 system_v {
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
