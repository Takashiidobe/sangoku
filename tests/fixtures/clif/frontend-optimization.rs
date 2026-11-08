#![no_std]

#[inline(always)]
fn identity(x: u16) -> u16 { x.wrapping_add(0) }
#[unsafe(no_mangle)]
pub fn folded(x: u16) -> u16 { if false { x.wrapping_mul(13) } else { identity(x) } }
#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 { (folded(42) == 42) as u8 }

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol folded
// CHECK-NEXT: function u0:0(i16) -> i16 system_v {
// CHECK-NEXT: ; symbol folded
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u16                               2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u16
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
// CHECK-NEXT: ; _0 = copy _1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u16 <- ByVal(v1): u16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v1
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
// CHECK-NEXT: ; ssa   _2    u16                               2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:0 sig0 ; Instance { def: Item(DefId(0:4 ~ frontend_optimization[..]::folded)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = folded(const 42_u16)
// CHECK-NEXT:     v0 = iconst.i16 42
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: u16, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v1 = call fn0(v0)  ; v0 = 42
// CHECK-NEXT:     v2 -> v1
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): u16 <- ByVal(v1): u16
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _1 = Eq(move _2, const 42_u16)
// CHECK-NEXT:     v3 = iconst.i16 42
// CHECK-NEXT:     v4 = icmp.i16 eq v2, v3  ; v3 = 42
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): bool <- ByVal(v4): bool
// CHECK-NEXT: ; _0 = move _1 as u8 (IntToInt)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v4): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v4
// CHECK-NEXT: }
