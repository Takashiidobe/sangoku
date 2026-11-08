#![no_std]

#[inline(never)]
#[unsafe(no_mangle)]
pub fn signed_multiply(lhs: i128, rhs: i128) -> (i128, bool) {
    lhs.overflowing_mul(rhs)
}

static CASES: [(i128, i128, i128, bool); 15] = [
    (0, i128::MIN, 0, false),
    (i128::MIN, 1, i128::MIN, false),
    (i128::MIN, -1, i128::MIN, true),
    (i128::MAX, 1, i128::MAX, false),
    (i128::MAX, -1, -i128::MAX, false),
    (i128::MAX, 2, -2, true),
    (i128::MIN, 2, 0, true),
    (1 << 126, -2, i128::MIN, false),
    (-(1 << 126), 2, i128::MIN, false),
    (-(1 << 126), -2, i128::MIN, true),
    (-3, 5, -15, false),
    (-3, -5, 15, false),
    (i128::MIN, i128::MIN, 0, true),
    (i128::MAX, i128::MAX, 1, true),
    (-1, -1, 1, false),
];

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    for &(lhs, rhs, value, overflow) in &CASES {
        if signed_multiply(lhs, rhs) != (value, overflow) {
            return 0;
        }
    }
    1
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RINvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB6_7NonNullSTnnnbEE4castBV_ECskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:0(i16, i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RINvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB6_7NonNullSTnnnbEE4castBV_ECskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::ptr::NonNull<[(i128, i128, i128, bool)]>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    core::ptr::NonNull<[(i128, i128, i128, bool)]>    4b 2                storage=ss1
// CHECK-NEXT: ; ssa   _2    *mut (i128, i128, i128, bool)     2b 2                var=1
// CHECK-NEXT: ; ssa   _3    *mut [(i128, i128, i128, bool)]    4b 2                var=(2, 3)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 4, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:4 sig0 ; Instance { def: Item(DefId(1:29489 ~ core[..]::ptr::non_null::{impl#3}::as_ptr)), args: {{[[]}}[(i128, i128, i128, bool)]] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::ptr::NonNull<[(i128, i128, i128, bool)]> <- ByValPair(v0, v1): core::ptr::NonNull<[(i128, i128, i128, bool)]>
// CHECK-NEXT:     v2 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT:     v3 = stack_addr.i16 ss1+2
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::ptr::NonNull::<T>::as_ptr(copy _1)
// CHECK-NEXT:     v4 = stack_addr.i16 ss1
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT:     v6 = stack_addr.i16 ss1+2
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<[(i128, i128, i128, bool)]>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530353912465992345 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *mut [(i128, i128, i128, bool)], layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 0..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v8, v9 = call fn0(v5, v7)
// CHECK-NEXT:     v10 -> v8
// CHECK-NEXT:     v11 -> v9
// CHECK-NEXT: ; write_cvalue: VarPair(_3, var2, var3): *mut [(i128, i128, i128, bool)] <- ByValPair(v8, v9): *mut [(i128, i128, i128, bool)]
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = move _3 as *mut U (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_2, var1): *mut (i128, i128, i128, bool) <- ByVal(v10): *mut (i128, i128, i128, bool)
// CHECK-NEXT: ; _0 = copy _2 as core::ptr::NonNull<U> (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v10): *mut (i128, i128, i128, bool)
// CHECK-NEXT:     v12 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v10, v12
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v13 = stack_addr.i16 ss0
// CHECK-NEXT:     v14 = load.i16 notrap v13
// CHECK-NEXT:     return v14
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core5sliceSTnnnbE4iterCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:1(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtCs40R3J0iFVxM_4core5sliceSTnnnbE4iterCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::slice::Iter<'{erased}, (i128, i128, i128, bool)>    4b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::slice::Iter<'{erased}, (i128, i128, i128, bool)>
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} [(i128, i128, i128, bool)]
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} [(i128, i128, i128, bool)]    4b 2                var=(1, 2)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:10 sig0 ; Instance { def: Item(DefId(1:14626 ~ core[..]::slice::iter::{impl#6}::new)), args: ['{erased}, (i128, i128, i128, bool)] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: VarPair(_1, var1, var2): &'{erased} [(i128, i128, i128, bool)] <- ByValPair(v0, v1): &'{erased} [(i128, i128, i128, bool)]
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::slice::Iter::<'_, T>::new(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &[(i128, i128, i128, bool)], layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::slice::Iter<'_, (i128, i128, i128, bool)>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 0..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes), Size(4 bytes)], in_memory_order: [0, 1, 2] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 5327351753192994986 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4, v5 = call fn0(v2, v3)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::slice::Iter<'{erased}, (i128, i128, i128, bool)> <- ByValPair(v4, v5): core::slice::Iter<'{erased}, (i128, i128, i128, bool)>
// CHECK-NEXT:     v6 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v4, v6
// CHECK-NEXT:     v7 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v5, v7
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v8 = stack_addr.i16 ss0
// CHECK-NEXT:     v9 = load.i16 notrap v8
// CHECK-NEXT:     v10 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v11 = load.i16 notrap v10
// CHECK-NEXT:     return v9, v11
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtNtCs40R3J0iFVxM_4core3ptr7mut_ptrOTnnnbE10cast_constCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:2(i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtNtCs40R3J0iFVxM_4core3ptr7mut_ptrOTnnnbE10cast_constCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    *const (i128, i128, i128, bool)    2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *const (i128, i128, i128, bool)
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut (i128, i128, i128, bool)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut (i128, i128, i128, bool)     2b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *mut (i128, i128, i128, bool) <- ByVal(v0): *mut (i128, i128, i128, bool)
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = copy _1 as *const T (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): *const (i128, i128, i128, bool) <- ByVal(v1): *const (i128, i128, i128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v1
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMNtNtCs40R3J0iFVxM_4core3ptr7mut_ptrOTnnnbE3addCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:3(i16, i16, i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMNtNtCs40R3J0iFVxM_4core3ptr7mut_ptrOTnnnbE3addCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    *mut (i128, i128, i128, bool)     2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut (i128, i128, i128, bool)
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut (i128, i128, i128, bool)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-NEXT: ; arg  ???      = v2       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::panic::Location<'{erased}>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut (i128, i128, i128, bool)     2b 2                var=2
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16, v2: i16):
// CHECK-NEXT:     v3 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): *mut (i128, i128, i128, bool) <- ByVal(v0): *mut (i128, i128, i128, bool)
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Offset(copy _1, copy _2)
// CHECK-NEXT:     v5 = iconst.i16 50
// CHECK-NEXT:     v6 = imul.i16 v4, v5  ; v5 = 50
// CHECK-NEXT:     v7 = iadd.i16 v3, v6
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): *mut (i128, i128, i128, bool) <- ByVal(v7): *mut (i128, i128, i128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v7
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullSTnnnbEE6as_ptrCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:4(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullSTnnnbEE6as_ptrCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    *mut [(i128, i128, i128, bool)]    4b 2                var=(1, 2)
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut [(i128, i128, i128, bool)]
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::ptr::NonNull<[(i128, i128, i128, bool)]>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    core::ptr::NonNull<[(i128, i128, i128, bool)]>    4b 2                storage=ss0
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<[(i128, i128, i128, bool)]> <- ByValPair(v0, v1): core::ptr::NonNull<[(i128, i128, i128, bool)]>
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT:     v3 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = copy _1 as *mut T (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: VarPair(_0, var1, var2): *mut [(i128, i128, i128, bool)] <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<[(i128, i128, i128, bool)]>
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT:     v6 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v5, v7
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullSTnnnbEE8from_refCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:5(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullSTnnnbEE8from_refCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::ptr::NonNull<[(i128, i128, i128, bool)]>    4b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::ptr::NonNull<[(i128, i128, i128, bool)]>
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} [(i128, i128, i128, bool)]
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} [(i128, i128, i128, bool)]    4b 2                storage=ss1
// CHECK-NEXT: ; ssa   _2    *const [(i128, i128, i128, bool)]    4b 2                var=(1, 2)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 4, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} [(i128, i128, i128, bool)] <- ByValPair(v0, v1): &'{erased} [(i128, i128, i128, bool)]
// CHECK-NEXT:     v2 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT:     v3 = stack_addr.i16 ss1+2
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = &raw const (*_1)
// CHECK-NEXT:     v4 = stack_addr.i16 ss1
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT:     v6 = stack_addr.i16 ss1+2
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: VarPair(_2, var1, var2): *const [(i128, i128, i128, bool)] <- ByValPair(v5, v7): *const [(i128, i128, i128, bool)]
// CHECK-NEXT: ; _0 = copy _2 as core::ptr::NonNull<T> (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<[(i128, i128, i128, bool)]> <- ByValPair(v5, v7): *const [(i128, i128, i128, bool)]
// CHECK-NEXT:     v8 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v5, v8
// CHECK-NEXT:     v9 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v7, v9
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     v11 = load.i16 notrap v10
// CHECK-NEXT:     v12 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v13 = load.i16 notrap v12
// CHECK-NEXT:     return v11, v13
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullTnnnbEE3addCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:6(i16, i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullTnnnbEE3addCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) usize
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=1
// CHECK-NEXT: ; ssa   _3    *mut (i128, i128, i128, bool)     2b 2                var=2
// CHECK-NEXT: ; ssa   _4    *mut (i128, i128, i128, bool)     2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:7 sig0 ; Instance { def: Item(DefId(1:29489 ~ core[..]::ptr::non_null::{impl#3}::as_ptr)), args: [(i128, i128, i128, bool)] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v7 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v0): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v2 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Var(_2, var1): usize <- ByVal(v1): usize
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::ptr::NonNull::<T>::as_ptr(copy _1)
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     v4 = load.i16 notrap v3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530072454669150871 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *mut (i128, i128, i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v5 = call fn0(v4)
// CHECK-NEXT:     v6 -> v5
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): *mut (i128, i128, i128, bool) <- ByVal(v5): *mut (i128, i128, i128, bool)
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = Offset(move _4, copy _2)
// CHECK-NEXT:     v8 = iconst.i16 50
// CHECK-NEXT:     v9 = imul.i16 v7, v8  ; v8 = 50
// CHECK-NEXT:     v10 = iadd.i16 v6, v9
// CHECK-NEXT: ; write_cvalue: Var(_3, var2): *mut (i128, i128, i128, bool) <- ByVal(v10): *mut (i128, i128, i128, bool)
// CHECK-NEXT: ; _0 = move _3 as core::ptr::NonNull<T> (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v10): *mut (i128, i128, i128, bool)
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v10, v11
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v12 = stack_addr.i16 ss0
// CHECK-NEXT:     v13 = load.i16 notrap v12
// CHECK-NEXT:     return v13
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullTnnnbEE6as_ptrCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:7(i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullTnnnbEE6as_ptrCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    *mut (i128, i128, i128, bool)     2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut (i128, i128, i128, bool)
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss0
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v0): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v1 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v1
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = copy _1 as *mut T (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Var(_0, var1): *mut (i128, i128, i128, bool) <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     v3 = load.i16 notrap v2
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v3
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullTnnnbEE6as_refCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:8(i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs1_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullTnnnbEE6as_refCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    &'{erased} (i128, i128, i128, bool)    2b 2                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (i128, i128, i128, bool)
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                var=2
// CHECK-NEXT: ; stack _2    *const (i128, i128, i128, bool)    2b 2                storage=ss0
// CHECK-NEXT: ; ssa   _3    *mut (i128, i128, i128, bool)     2b 2                var=3
// CHECK-NEXT: ; stack _4    core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16) -> i16 system_v
// CHECK-NEXT:     sig1 = (i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:7 sig0 ; Instance { def: Item(DefId(1:29489 ~ core[..]::ptr::non_null::{impl#3}::as_ptr)), args: [(i128, i128, i128, bool)] }
// CHECK-NEXT:     fn1 = colocated u0:2 sig1 ; Instance { def: Item(DefId(1:3408 ~ core[..]::ptr::mut_ptr::{impl#0}::cast_const)), args: [(i128, i128, i128, bool)] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v0): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v2 = load.i16 notrap v1
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v2, v3
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::ptr::NonNull::<T>::as_ptr(move _4)
// CHECK-NEXT:     v4 = stack_addr.i16 ss1
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530072454669150871 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *mut (i128, i128, i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v6 = call fn0(v5)
// CHECK-NEXT:     v7 -> v6
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *mut (i128, i128, i128, bool) <- ByVal(v6): *mut (i128, i128, i128, bool)
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _2 = core::ptr::mut_ptr::<impl *mut T>::cast_const(move _3)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *mut (i128, i128, i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *const (i128, i128, i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v8 = call fn1(v7)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): *const (i128, i128, i128, bool) <- ByVal(v8): *const (i128, i128, i128, bool)
// CHECK-NEXT:     v9 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v8, v9
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = &(*_2)
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     v11 = load.i16 notrap v10
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): &'{erased} (i128, i128, i128, bool) <- ByVal(v11): &'{erased} (i128, i128, i128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v11
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs2_NtCs40R3J0iFVxM_4core3numn15overflowing_mul
// CHECK-NEXT: function u0:9(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol _RNvMs2_NtCs40R3J0iFVxM_4core3numn15overflowing_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } (i128, bool)
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    i128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    i128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    i128                             16b 2                var=3
// CHECK-NEXT: ; ssa   _4    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _5    (i128, bool)                     18b 2                var=(5, 6)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 16, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 16, align = 2
// CHECK-NEXT:     sig0 = (i16, i16, i16) system_v
// CHECK-NEXT:     fn0 = u0:19 sig0 ; "__gbc_i128_mulo"
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i128, v2: i128):
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v4 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): i128 <- ByVal(v1): i128
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): i128 <- ByVal(v2): i128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = MulWithOverflow(copy _1, copy _2)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): u128 <- ByVal(v3): u128
// CHECK-NEXT:     v5 = stack_addr.i16 ss1
// CHECK-NEXT:     store.i128 notrap v3, v5
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): u128 <- ByVal(v4): u128
// CHECK-NEXT:     v7 = stack_addr.i16 ss2
// CHECK-NEXT:     store.i128 notrap v4, v7
// CHECK-NEXT:     v8 = stack_addr.i16 ss2
// CHECK-NEXT:     v9 = stack_addr.i16 ss0
// CHECK-NEXT: ; lib_call __gbc_i128_mulo
// CHECK-NEXT:     call fn0(v9, v6, v8)
// CHECK-NEXT:     v10 = stack_addr.i16 ss0
// CHECK-NEXT:     v11 = load.i128 notrap v10
// CHECK-NEXT:     v12 = stack_addr.i16 ss0+16
// CHECK-NEXT:     v13 = load.i8 notrap v12
// CHECK-NEXT: ; write_cvalue: VarPair(_5, var5, var6): (i128, bool) <- ByValPair(v11, v13): (i128, bool)
// CHECK-NEXT: ; _3 = copy (_5.0: i128)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): i128 <- ByVal(v11): i128
// CHECK-NEXT: ; _4 = copy (_5.1: bool)
// CHECK-NEXT: ; write_cvalue: Var(_4, var4): bool <- ByVal(v13): bool
// CHECK-NEXT: ; _0 = (copy _3, copy _4)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(0) }, None): i128 <- ByVal(v11): i128
// CHECK-NEXT:     store notrap v11, v0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v0), offset: Offset32(16) }, None): bool <- ByVal(v13): bool
// CHECK-NEXT:     store notrap v13, v0+16
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvMs4_NtNtCs40R3J0iFVxM_4core5slice4iterINtB5_4IterTnnnbEE3newCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:10(i16, i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvMs4_NtNtCs40R3J0iFVxM_4core5slice4iterINtB5_4IterTnnnbEE3newCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::slice::Iter<'{erased}, (i128, i128, i128, bool)>    4b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::slice::Iter<'{erased}, (i128, i128, i128, bool)>
// CHECK-NEXT: ; arg   _1      = v0,v1    Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) &'{erased} [(i128, i128, i128, bool)]
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} [(i128, i128, i128, bool)]    4b 2                var=(1, 2)
// CHECK-NEXT: ; ssa   _2    usize                             2b 2                var=3
// CHECK-NEXT: ; stack _3    core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss1
// CHECK-NEXT: ; stack _4    core::ptr::NonNull<[(i128, i128, i128, bool)]>    4b 2                storage=ss2
// CHECK-NEXT: ; ssa   _5    *const (i128, i128, i128, bool)    2b 2                var=4
// CHECK-NEXT: ; ssa   _6    *mut (i128, i128, i128, bool)     2b 2                var=5
// CHECK-NEXT: ; ssa   _7    *mut (i128, i128, i128, bool)     2b 2                var=6
// CHECK-NEXT: ; ssa   _8    *const (i128, i128, i128, bool)    2b 2                var=7
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 4, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname3 ; alloc25
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i16 system_v
// CHECK-NEXT:     sig2 = (i16) -> i16 system_v
// CHECK-NEXT:     sig3 = (i16, i16, i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:5 sig0 ; Instance { def: Item(DefId(1:3176 ~ core[..]::ptr::non_null::{impl#3}::from_ref)), args: {{[[]}}[(i128, i128, i128, bool)]] }
// CHECK-NEXT:     fn1 = colocated u0:0 sig1 ; Instance { def: Item(DefId(1:3190 ~ core[..]::ptr::non_null::{impl#3}::cast)), args: {{[[]}}[(i128, i128, i128, bool)], (i128, i128, i128, bool)] }
// CHECK-NEXT:     fn2 = colocated u0:7 sig2 ; Instance { def: Item(DefId(1:29489 ~ core[..]::ptr::non_null::{impl#3}::as_ptr)), args: [(i128, i128, i128, bool)] }
// CHECK-NEXT:     fn3 = colocated u0:3 sig3 ; Instance { def: Item(DefId(1:3441 ~ core[..]::ptr::mut_ptr::{impl#0}::add)), args: [(i128, i128, i128, bool)] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v19 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: VarPair(_1, var1, var2): &'{erased} [(i128, i128, i128, bool)] <- ByValPair(v0, v1): &'{erased} [(i128, i128, i128, bool)]
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = PtrMetadata(copy _1)
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): usize <- ByVal(v3): usize
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = core::ptr::NonNull::<[T]>::from_ref(copy _1)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &[(i128, i128, i128, bool)], layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<[(i128, i128, i128, bool)]>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530353912465992345 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v4, v5 = call fn0(v2, v3)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): core::ptr::NonNull<[(i128, i128, i128, bool)]> <- ByValPair(v4, v5): core::ptr::NonNull<[(i128, i128, i128, bool)]>
// CHECK-NEXT:     v6 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v4, v6
// CHECK-NEXT:     v7 = stack_addr.i16 ss2+2
// CHECK-NEXT:     store notrap v5, v7
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::ptr::NonNull::<[T]>::cast::<T>(move _4)
// CHECK-NEXT:     v8 = stack_addr.i16 ss2
// CHECK-NEXT:     v9 = load.i16 notrap v8
// CHECK-NEXT:     v10 = stack_addr.i16 ss2+2
// CHECK-NEXT:     v11 = load.i16 notrap v10
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<[(i128, i128, i128, bool)]>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530353912465992345 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530072454669150871 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v12 = call fn1(v9, v11)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v12): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v13 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v12, v13
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(const <T as core::mem::SizedTypeProperties>::IS_ZST)
// CHECK-NEXT:     v14 = iconst.i8 0
// CHECK-NEXT:     jump block5
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _7 = core::ptr::NonNull::<T>::as_ptr(copy _3)
// CHECK-NEXT:     v15 = stack_addr.i16 ss1
// CHECK-NEXT:     v16 = load.i16 notrap v15
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530072454669150871 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *mut (i128, i128, i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v17 = call fn2(v16)
// CHECK-NEXT:     v18 -> v17
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): *mut (i128, i128, i128, bool) <- ByVal(v17): *mut (i128, i128, i128, bool)
// CHECK-NEXT:     jump block6
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _6 = core::ptr::mut_ptr::<impl *mut T>::add(move _7, copy _2)
// CHECK-NEXT:     v20 = symbol_value.i16 gv0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: *mut (i128, i128, i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &core::panic::Location<'_>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: *mut (i128, i128, i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v21 = call fn3(v18, v19, v20)
// CHECK-NEXT:     v22 -> v21
// CHECK-NEXT:     v23 -> v21
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): *mut (i128, i128, i128, bool) <- ByVal(v21): *mut (i128, i128, i128, bool)
// CHECK-NEXT:     jump block7
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = move _6 as *const T (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): *const (i128, i128, i128, bool) <- ByVal(v22): *const (i128, i128, i128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _8 = copy _5
// CHECK-NEXT: ; write_cvalue: Var(_8, var7): *const (i128, i128, i128, bool) <- ByVal(v23): *const (i128, i128, i128, bool)
// CHECK-NEXT: ; _0 = core::slice::Iter::<'_, T> { ptr: copy _3, end_or_len: move _8, _marker: const ZeroSized: core::marker::PhantomData<&T> }
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v24 = stack_addr.i16 ss1
// CHECK-NEXT:     v25 = load.i16 notrap v24
// CHECK-NEXT:     v26 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v25, v26
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(2) }, None): *const (i128, i128, i128, bool) <- ByVal(v23): *const (i128, i128, i128, bool)
// CHECK-NEXT:     v27 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store.i16 notrap v23, v27
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(4) }, None): core::marker::PhantomData<&'{erased} (i128, i128, i128, bool)> <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): core::marker::PhantomData<&'{erased} (i128, i128, i128, bool)>
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v28 = stack_addr.i16 ss0
// CHECK-NEXT:     v29 = load.i16 notrap v28
// CHECK-NEXT:     v30 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v31 = load.i16 notrap v30
// CHECK-NEXT:     return v29, v31
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXs2J_NtNtCs40R3J0iFVxM_4core5slice4iterINtB6_4IterTnnnbEENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:11(i16) -> i16 system_v {
// CHECK-NEXT: ; symbol _RNvXs2J_NtNtCs40R3J0iFVxM_4core5slice4iterINtB6_4IterTnnnbEENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::option::Option<&'{erased} (i128, i128, i128, bool)>    2b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) core::option::Option<&'{erased} (i128, i128, i128, bool)>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} mut core::slice::Iter<'{erased}, (i128, i128, i128, bool)>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} mut core::slice::Iter<'{erased}, (i128, i128, i128, bool)>    2b 2                var=1
// CHECK-NEXT: ; stack _2    core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    *const (i128, i128, i128, bool)    2b 2                var=2
// CHECK-NEXT: ; ssa   _4    usize                             2b 2                var=3
// CHECK-NEXT: ; ssa   _5    bool                              1b 1                var=4
// CHECK-NEXT: ; ssa   _6    *mut (i128, i128, i128, bool)     2b 2                var=5
// CHECK-NEXT: ; ssa   _7    usize                             2b 2                var=6
// CHECK-NEXT: ; ssa   _8    bool                              1b 1                var=7
// CHECK-NEXT: ; ssa   _9    &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                var=8
// CHECK-NEXT: ; ssa   _10   &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                var=9
// CHECK-NEXT: ; stack _11   core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss2
// CHECK-NEXT: ; stack _12   core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss3
// CHECK-NEXT: ; ssa   _13   &'{erased} (i128, i128, i128, bool)    2b 2                var=10
// CHECK-NEXT: ; ssa   _14   &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                var=11
// CHECK-NEXT: ; stack _15   core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss4
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss3 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss4 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i16 system_v
// CHECK-NEXT:     sig2 = (i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:14 sig0 ; Instance { def: Item(DefId(1:3265 ~ core[..]::ptr::non_null::{impl#15}::eq)), args: [(i128, i128, i128, bool)] }
// CHECK-NEXT:     fn1 = colocated u0:6 sig1 ; Instance { def: Item(DefId(1:3196 ~ core[..]::ptr::non_null::{impl#3}::add)), args: [(i128, i128, i128, bool)] }
// CHECK-NEXT:     fn2 = colocated u0:8 sig2 ; Instance { def: Item(DefId(1:3186 ~ core[..]::ptr::non_null::{impl#3}::as_ref)), args: [(i128, i128, i128, bool), '{erased}] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     v21 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): &'{erased} mut core::slice::Iter<'{erased}, (i128, i128, i128, bool)> <- ByVal(v0): &'{erased} mut core::slice::Iter<'{erased}, (i128, i128, i128, bool)>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy ((*_1).0: core::ptr::NonNull<T>)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v2 = load.i16 notrap v1
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v2, v3
// CHECK-NEXT: ; _3 = copy ((*_1).1: *const T)
// CHECK-NEXT: ; write_cvalue: Var(_3, var2): *const (i128, i128, i128, bool) <- ByRef(Pointer { base: Addr(v1), offset: Offset32(2) }, None): *const (i128, i128, i128, bool)
// CHECK-NEXT:     v4 = load.i16 notrap v1+2
// CHECK-NEXT:     v7 -> v4
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(const <T as core::mem::SizedTypeProperties>::IS_ZST)
// CHECK-NEXT:     v5 = iconst.i8 0
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block5:
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _9 = &_2
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_9, var8): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v6): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT: ; _11 = copy _3 as core::ptr::NonNull<T> (Transmute)
// CHECK-NEXT: ; write_cvalue_transmute: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v7): *const (i128, i128, i128, bool)
// CHECK-NEXT:     v8 = stack_addr.i16 ss2
// CHECK-NEXT:     store.i16 notrap v7, v8
// CHECK-NEXT: ; _10 = &_11
// CHECK-NEXT:     v9 = stack_addr.i16 ss2
// CHECK-NEXT: ; write_cvalue: Var(_10, var9): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v9): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _8 = <core::ptr::NonNull<T> as core::cmp::PartialEq>::eq(move _9, move _10)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v10 = call fn0(v6, v9)
// CHECK-NEXT:     v11 -> v10
// CHECK-NEXT: ; write_cvalue: Var(_8, var7): bool <- ByVal(v10): bool
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _8)
// CHECK-NEXT:     brif.i8 v11, block10, block11
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<&T>::None
// CHECK-NEXT:     v12 = iconst.i16 0
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): *mut () <- ByVal(v12): *mut ()
// CHECK-NEXT:     v13 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v12, v13  ; v12 = 0
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v14 = stack_addr.i16 ss0
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     return v15
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _12 = core::ptr::NonNull::<T>::add(copy _2, const 1_usize)
// CHECK-NEXT:     v16 = iconst.i16 1
// CHECK-NEXT:     v17 = stack_addr.i16 ss1
// CHECK-NEXT:     v18 = load.i16 notrap v17
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530072454669150871 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: usize, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(u16 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462607322775554 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530072454669150871 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v19 = call fn1(v18, v16)  ; v16 = 1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v19): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v20 = stack_addr.i16 ss3
// CHECK-NEXT:     store notrap v19, v20
// CHECK-NEXT:     jump block12
// CHECK-EMPTY:
// CHECK-NEXT: block12:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; ((*_1).0: core::ptr::NonNull<T>) = move _12
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v21), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByRef(Pointer { base: Stack(ss3), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v22 = stack_addr.i16 ss3
// CHECK-NEXT:     v23 = load.i16 notrap v22
// CHECK-NEXT:     store notrap v23, v21
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block13
// CHECK-EMPTY:
// CHECK-NEXT: block13:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _15 = copy _2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByRef(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v24 = stack_addr.i16 ss1
// CHECK-NEXT:     v25 = load.i16 notrap v24
// CHECK-NEXT:     v26 = stack_addr.i16 ss4
// CHECK-NEXT:     store notrap v25, v26
// CHECK-NEXT: ; _14 = &_15
// CHECK-NEXT:     v27 = stack_addr.i16 ss4
// CHECK-NEXT: ; write_cvalue: Var(_14, var11): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v27): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = core::ptr::NonNull::<T>::as_ref::<'_>(move _14)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: &(i128, i128, i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v28 = call fn2(v27)
// CHECK-NEXT:     v29 -> v28
// CHECK-NEXT: ; write_cvalue: Var(_13, var10): &'{erased} (i128, i128, i128, bool) <- ByVal(v28): &'{erased} (i128, i128, i128, bool)
// CHECK-NEXT:     jump block14
// CHECK-EMPTY:
// CHECK-NEXT: block14:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = core::option::Option::<&T>::Some(copy _13)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (i128, i128, i128, bool) <- ByVal(v29): &'{erased} (i128, i128, i128, bool)
// CHECK-NEXT:     v30 = stack_addr.i16 ss0
// CHECK-NEXT:     store.i16 notrap v29, v30
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     v31 = stack_addr.i16 ss0
// CHECK-NEXT:     v32 = load.i16 notrap v31
// CHECK-NEXT:     return v32
// CHECK-EMPTY:
// CHECK-NEXT: block15:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v33 = stack_addr.i16 ss0
// CHECK-NEXT:     v34 = load.i16 notrap v33
// CHECK-NEXT:     return v34
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTnbENtNtB7_3cmp9PartialEq2neCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:12(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXs8_NtCs40R3J0iFVxM_4core5tupleTnbENtNtB7_3cmp9PartialEq2neCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (i128, bool)
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} (i128, bool)
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    &'{erased} (i128, bool)           2b 2                storage=ss0
// CHECK-NEXT: ; stack _2    &'{erased} (i128, bool)           2b 2                storage=ss1
// CHECK-NEXT: ; ssa   _3    bool                              1b 1                var=2
// CHECK-NEXT: ; ssa   _4    &'{erased} i128                   2b 2                var=3
// CHECK-NEXT: ; ssa   _5    &'{erased} i128                   2b 2                var=4
// CHECK-NEXT: ; ssa   _6    &'{erased} bool                   2b 2                var=5
// CHECK-NEXT: ; ssa   _7    &'{erased} bool                   2b 2                var=6
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     sig1 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:16 sig0 ; Instance { def: Item(DefId(1:30003 ~ core[..]::cmp::impls::{impl#32}::ne)), args: [] }
// CHECK-NEXT:     fn1 = colocated u0:15 sig1 ; Instance { def: Item(DefId(1:29964 ~ core[..]::cmp::impls::{impl#19}::ne)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): &'{erased} (i128, bool) <- ByVal(v0): &'{erased} (i128, bool)
// CHECK-NEXT:     v2 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v0, v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): &'{erased} (i128, bool) <- ByVal(v1): &'{erased} (i128, bool)
// CHECK-NEXT:     v3 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = &((*_1).0: U)
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     v5 = load.i16 notrap v4
// CHECK-NEXT: ; write_cvalue: Var(_4, var3): &'{erased} i128 <- ByVal(v5): &'{erased} i128
// CHECK-NEXT: ; _5 = &((*_2).0: U)
// CHECK-NEXT:     v6 = stack_addr.i16 ss1
// CHECK-NEXT:     v7 = load.i16 notrap v6
// CHECK-NEXT: ; write_cvalue: Var(_5, var4): &'{erased} i128 <- ByVal(v7): &'{erased} i128
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = <U as core::cmp::PartialEq>::ne(move _4, move _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &i128, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &i128, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
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
// CHECK-NEXT: ; _0 = const true
// CHECK-NEXT:     v10 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v10): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v10  ; v10 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = &((*_1).1: T)
// CHECK-NEXT:     v11 = stack_addr.i16 ss0
// CHECK-NEXT:     v12 = load.i16 notrap v11
// CHECK-NEXT:     v13 = iconst.i16 16
// CHECK-NEXT:     v14 = iadd v12, v13  ; v13 = 16
// CHECK-NEXT: ; write_cvalue: Var(_6, var5): &'{erased} bool <- ByVal(v14): &'{erased} bool
// CHECK-NEXT: ; _7 = &((*_2).1: T)
// CHECK-NEXT:     v15 = stack_addr.i16 ss1
// CHECK-NEXT:     v16 = load.i16 notrap v15
// CHECK-NEXT:     v17 = iconst.i16 16
// CHECK-NEXT:     v18 = iadd v16, v17  ; v17 = 16
// CHECK-NEXT: ; write_cvalue: Var(_7, var6): &'{erased} bool <- ByVal(v18): &'{erased} bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = <T as core::cmp::PartialEq>::ne(move _6, move _7)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &bool, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: &bool, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v19 = call fn1(v14, v18)
// CHECK-NEXT:     v20 -> v19
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v19): bool
// CHECK-NEXT:     jump block5
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
// CHECK-NEXT: ; symbol _RNvXsb_NtCs40R3J0iFVxM_4core5arrayRATnnnbEjf_NtNtNtNtB7_4iter6traits7collect12IntoIterator9into_iterCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:13(i16) -> i16, i16 system_v {
// CHECK-NEXT: ; symbol _RNvXsb_NtCs40R3J0iFVxM_4core5arrayRATnnnbEjf_NtNtNtNtB7_4iter6traits7collect12IntoIterator9into_iterCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; stack _0    core::slice::Iter<'{erased}, (i128, i128, i128, bool)>    4b 2                storage=ss0
// CHECK-NEXT: ; ret   _0      -          Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) core::slice::Iter<'{erased}, (i128, i128, i128, bool)>
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} [(i128, i128, i128, bool); 15_usize]
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} [(i128, i128, i128, bool); 15_usize]    2b 2                var=1
// CHECK-NEXT: ; ssa   _2    &'{erased} [(i128, i128, i128, bool)]    4b 2                var=(2, 3)
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     sig0 = (i16, i16) -> i16, i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:1 sig0 ; Instance { def: Item(DefId(1:15588 ~ core[..]::slice::{impl#0}::iter)), args: [(i128, i128, i128, bool)] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): &'{erased} [(i128, i128, i128, bool); 15_usize] <- ByVal(v0): &'{erased} [(i128, i128, i128, bool); 15_usize]
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy _1 as &[T] (PointerCoercion(Unsize, Implicit))
// CHECK-NEXT:     v2 = iconst.i16 15
// CHECK-NEXT: ; write_cvalue: VarPair(_2, var2, var3): &'{erased} [(i128, i128, i128, bool)] <- ByValPair(v1, v2): &'{erased} [(i128, i128, i128, bool)]
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::slice::<impl [T]>::iter(move _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &[(i128, i128, i128, bool)], layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: u16 is .., b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::slice::Iter<'_, (i128, i128, i128, bool)>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 0..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes), Size(4 bytes)], in_memory_order: [0, 1, 2] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 5327351753192994986 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v3, v4 = call fn0(v1, v2)  ; v2 = 15
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::slice::Iter<'{erased}, (i128, i128, i128, bool)> <- ByValPair(v3, v4): core::slice::Iter<'{erased}, (i128, i128, i128, bool)>
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v3, v5
// CHECK-NEXT:     v6 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v4, v6
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     v7 = stack_addr.i16 ss0
// CHECK-NEXT:     v8 = load.i16 notrap v7
// CHECK-NEXT:     v9 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v10 = load.i16 notrap v9
// CHECK-NEXT:     return v8, v10
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXsd_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullTnnnbEENtNtB9_3cmp9PartialEq2eqCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:14(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsd_NtNtCs40R3J0iFVxM_4core3ptr8non_nullINtB5_7NonNullTnnnbEENtNtB9_3cmp9PartialEq2eqCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                var=3
// CHECK-NEXT: ; ssa   _3    *mut (i128, i128, i128, bool)     2b 2                var=4
// CHECK-NEXT: ; stack _4    core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss0
// CHECK-NEXT: ; ssa   _5    *mut (i128, i128, i128, bool)     2b 2                var=5
// CHECK-NEXT: ; stack _6    core::ptr::NonNull<(i128, i128, i128, bool)>    2b 2                storage=ss1
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 2, align = 2
// CHECK-NEXT:     sig0 = (i16) -> i16 system_v
// CHECK-NEXT:     sig1 = (i16) -> i16 system_v
// CHECK-NEXT:     fn0 = colocated u0:7 sig0 ; Instance { def: Item(DefId(1:29489 ~ core[..]::ptr::non_null::{impl#3}::as_ptr)), args: [(i128, i128, i128, bool)] }
// CHECK-NEXT:     fn1 = colocated u0:7 sig1 ; Instance { def: Item(DefId(1:29489 ~ core[..]::ptr::non_null::{impl#3}::as_ptr)), args: [(i128, i128, i128, bool)] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v8 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v0): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)> <- ByVal(v1): &'{erased} core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _4 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v3 = load.i16 notrap v2
// CHECK-NEXT:     v4 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v3, v4
// CHECK-NEXT: ;
// CHECK-NEXT: ; _3 = core::ptr::NonNull::<T>::as_ptr(move _4)
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     v6 = load.i16 notrap v5
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530072454669150871 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *mut (i128, i128, i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v7 = call fn0(v6)
// CHECK-NEXT:     v14 -> v7
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): *mut (i128, i128, i128, bool) <- ByVal(v7): *mut (i128, i128, i128, bool)
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)> <- ByRef(Pointer { base: Addr(v8), offset: Offset32(0) }, None): core::ptr::NonNull<(i128, i128, i128, bool)>
// CHECK-NEXT:     v9 = load.i16 notrap v8
// CHECK-NEXT:     v10 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v9, v10
// CHECK-NEXT: ;
// CHECK-NEXT: ; _5 = core::ptr::NonNull::<T>::as_ptr(move _6)
// CHECK-NEXT:     v11 = stack_addr.i16 ss1
// CHECK-NEXT:     v12 = load.i16 notrap v11
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: core::ptr::NonNull<(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 14530072454669150871 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: *mut (i128, i128, i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 18446462615912710146 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v13 = call fn1(v12)
// CHECK-NEXT:     v15 -> v13
// CHECK-NEXT: ; write_cvalue: Var(_5, var5): *mut (i128, i128, i128, bool) <- ByVal(v13): *mut (i128, i128, i128, bool)
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = Eq(move _3, move _5)
// CHECK-NEXT:     v16 = icmp.i16 eq v14, v15
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v16): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v16
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2neCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:15(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsh_NtNtCs40R3J0iFVxM_4core3cmp5implsbNtB7_9PartialEq2neCskzC2rUVn8ZR_19signed_overflow_mul
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
// CHECK-NEXT: ; _0 = Ne(move _3, move _4)
// CHECK-NEXT:     v6 = icmp ne v3, v5
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v6): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v6
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol _RNvXsu_NtNtCs40R3J0iFVxM_4core3cmp5implsnNtB7_9PartialEq2neCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-NEXT: function u0:16(i16, i16) -> i8 uext system_v {
// CHECK-NEXT: ; symbol _RNvXsu_NtNtCs40R3J0iFVxM_4core3cmp5implsnNtB7_9PartialEq2neCskzC2rUVn8ZR_19signed_overflow_mul
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    bool                              1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) bool
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} i128
// CHECK-NEXT: ; arg   _2      = v1       Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) &'{erased} i128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    &'{erased} i128                   2b 2                var=2
// CHECK-NEXT: ; ssa   _2    &'{erased} i128                   2b 2                var=3
// CHECK-NEXT: ; ssa   _3    i128                             16b 2                var=4
// CHECK-NEXT: ; ssa   _4    i128                             16b 2                var=5
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i16):
// CHECK-NEXT:     v2 -> v0
// CHECK-NEXT:     v4 -> v1
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var2): &'{erased} i128 <- ByVal(v0): &'{erased} i128
// CHECK-NEXT: ; write_cvalue: Var(_2, var3): &'{erased} i128 <- ByVal(v1): &'{erased} i128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_3, var4): i128 <- ByRef(Pointer { base: Addr(v2), offset: Offset32(0) }, None): i128
// CHECK-NEXT:     v3 = load.i128 notrap v2
// CHECK-NEXT: ; _4 = copy (*_2)
// CHECK-NEXT: ; write_cvalue: Var(_4, var5): i128 <- ByRef(Pointer { base: Addr(v4), offset: Offset32(0) }, None): i128
// CHECK-NEXT:     v5 = load.i128 notrap v4
// CHECK-NEXT: ; _0 = Ne(move _3, move _4)
// CHECK-NEXT:     v6 = icmp ne v3, v5
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): bool <- ByVal(v6): bool
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v6
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol gbc_main
// CHECK-NEXT: function u0:17() -> i8 system_v {
// CHECK-NEXT: ; symbol gbc_main
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ssa   _0    u8                                1b 1                var=1
// CHECK-NEXT: ; ret   _0      -          Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) u8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; stack _1    core::slice::Iter<'{erased}, (i128, i128, i128, bool)>    4b 2                storage=ss0
// CHECK-NEXT: ; ssa   _2    &'{erased} [(i128, i128, i128, bool); 15_usize]    2b 2                var=2
// CHECK-NEXT: ; stack _3    core::slice::Iter<'{erased}, (i128, i128, i128, bool)>    4b 2                storage=ss1
// CHECK-NEXT: ; stack _4    core::option::Option<&'{erased} (i128, i128, i128, bool)>    2b 2                storage=ss2
// CHECK-NEXT: ; ssa   _5    &'{erased} mut core::slice::Iter<'{erased}, (i128, i128, i128, bool)>    2b 2                var=3
// CHECK-NEXT: ; ssa   _6    isize                             2b 2                var=4
// CHECK-NEXT: ; ssa   _7    i128                             16b 2                var=5
// CHECK-NEXT: ; ssa   _8    i128                             16b 2                var=6
// CHECK-NEXT: ; ssa   _9    i128                             16b 2                var=7
// CHECK-NEXT: ; ssa   _10   bool                              1b 1                var=8
// CHECK-NEXT: ; ssa   _11   bool                              1b 1                var=9
// CHECK-NEXT: ; ssa   _12   &'{erased} (i128, bool)           2b 2                var=10
// CHECK-NEXT: ; stack _13   (i128, bool)                     18b 2                storage=ss3
// CHECK-NEXT: ; ssa   _14   &'{erased} (i128, bool)           2b 2                var=11
// CHECK-NEXT: ; stack _15   (i128, bool)                     18b 2                storage=ss4
// CHECK-NEXT: ; ssa   _16   &'{erased} (i128, i128, i128, bool)    2b 2                var=12
// CHECK-NEXT: ; ssa   _17   &'{erased} (i128, i128, i128, bool)    2b 2                var=13
// CHECK-NEXT: ; ssa   _18   &'{erased} (i128, i128, i128, bool)    2b 2                var=14
// CHECK-NEXT: ; ssa   _19   &'{erased} (i128, i128, i128, bool)    2b 2                var=15
// CHECK-EMPTY:
// CHECK-NEXT:     ss0 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss1 = explicit_slot 4, align = 2
// CHECK-NEXT:     ss2 = explicit_slot 2, align = 2
// CHECK-NEXT:     ss3 = explicit_slot 18, align = 2
// CHECK-NEXT:     ss4 = explicit_slot 18, align = 2
// CHECK-NEXT:     gv0 = symbol colocated userextname0 ; DefId(0:4 ~ signed_overflow_mul[..]::CASES)
// CHECK-NEXT:     sig0 = (i16) -> i16, i16 system_v
// CHECK-NEXT:     sig1 = (i16) -> i16 system_v
// CHECK-NEXT:     sig2 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     sig3 = (i16, i16) -> i8 uext system_v
// CHECK-NEXT:     fn0 = colocated u0:13 sig0 ; Instance { def: Item(DefId(1:5797 ~ core[..]::array::{impl#13}::into_iter)), args: ['{erased}, (i128, i128, i128, bool), 15_usize] }
// CHECK-NEXT:     fn1 = colocated u0:11 sig1 ; Instance { def: Item(DefId(1:45063 ~ core[..]::slice::iter::{impl#171}::next)), args: ['{erased}, (i128, i128, i128, bool)] }
// CHECK-NEXT:     fn2 = colocated u0:18 sig2 ; Instance { def: Item(DefId(0:3 ~ signed_overflow_mul[..]::signed_multiply)), args: [] }
// CHECK-NEXT:     fn3 = colocated u0:12 sig3 ; Instance { def: Item(DefId(1:46740 ~ core[..]::tuple::{impl#10}::ne)), args: [i128, bool] }
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = const {alloc1: &[(i128, i128, i128, bool); 15]}
// CHECK-NEXT:     v0 = symbol_value.i16 gv0
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): &'{erased} [(i128, i128, i128, bool); 15_usize] <- ByVal(v0): &'{erased} [(i128, i128, i128, bool); 15_usize]
// CHECK-NEXT: ;
// CHECK-NEXT: ; _1 = <&[(i128, i128, i128, bool); 15] as core::iter::IntoIterator>::into_iter(move _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &[(i128, i128, i128, bool); 15], layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::slice::Iter<'_, (i128, i128, i128, bool)>, layout: Layout { size: Size(4 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: pointer is 1..=65535, b: pointer is 0..=65535, b_offset: Size(2 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(2 bytes), Size(4 bytes)], in_memory_order: [0, 1, 2] }, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 5327351753192994986 } }, mode: Pair(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }, ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v1, v2 = call fn0(v0)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::slice::Iter<'{erased}, (i128, i128, i128, bool)> <- ByValPair(v1, v2): core::slice::Iter<'{erased}, (i128, i128, i128, bool)>
// CHECK-NEXT:     v3 = stack_addr.i16 ss0
// CHECK-NEXT:     store notrap v1, v3
// CHECK-NEXT:     v4 = stack_addr.i16 ss0+2
// CHECK-NEXT:     store notrap v2, v4
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _3 = move _1
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss1), offset: Offset32(0) }, None): core::slice::Iter<'{erased}, (i128, i128, i128, bool)> <- ByRef(Pointer { base: Stack(ss0), offset: Offset32(0) }, None): core::slice::Iter<'{erased}, (i128, i128, i128, bool)>
// CHECK-NEXT:     v5 = stack_addr.i16 ss0
// CHECK-NEXT:     v6 = load.i16 notrap v5
// CHECK-NEXT:     v7 = stack_addr.i16 ss0+2
// CHECK-NEXT:     v8 = load.i16 notrap v7
// CHECK-NEXT:     v9 = stack_addr.i16 ss1
// CHECK-NEXT:     store notrap v6, v9
// CHECK-NEXT:     v10 = stack_addr.i16 ss1+2
// CHECK-NEXT:     store notrap v8, v10
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     jump block3
// CHECK-EMPTY:
// CHECK-NEXT: block3:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _5 = &mut _3
// CHECK-NEXT:     v11 = stack_addr.i16 ss1
// CHECK-NEXT: ; write_cvalue: Var(_5, var3): &'{erased} mut core::slice::Iter<'{erased}, (i128, i128, i128, bool)> <- ByVal(v11): &'{erased} mut core::slice::Iter<'{erased}, (i128, i128, i128, bool)>
// CHECK-NEXT: ;
// CHECK-NEXT: ; _4 = <core::slice::Iter<'_, (i128, i128, i128, bool)> as core::iter::Iterator>::next(copy _5)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &mut core::slice::Iter<'_, (i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: core::option::Option<&(i128, i128, i128, bool)>, layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 0..=65535), fields: Arbitrary { offsets: [Size(0 bytes)], in_memory_order: [0] }, largest_niche: None, uninhabited: false, variants: Multiple { tag: pointer is 0..=65535, tag_encoding: Niche { untagged_variant: 1, niche_variants: 0..=0, niche_start: 0 }, tag_field: 0, variants: [VariantLayout { size: Size(0 bytes), backend_repr: Memory { sized: true }, field_offsets: [], fields_in_memory_order: [], largest_niche: None, uninhabited: false }, VariantLayout { size: Size(2 bytes), backend_repr: Scalar(pointer is 1..=65535), field_offsets: [Size(0 bytes)], fields_in_memory_order: [0], largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false }] }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 8189598418185652210 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, c_variadic: false, fixed_count: 1, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v12 = call fn1(v11)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): core::option::Option<&'{erased} (i128, i128, i128, bool)> <- ByVal(v12): core::option::Option<&'{erased} (i128, i128, i128, bool)>
// CHECK-NEXT:     v13 = stack_addr.i16 ss2
// CHECK-NEXT:     store notrap v12, v13
// CHECK-NEXT:     jump block4
// CHECK-EMPTY:
// CHECK-NEXT: block4:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _6 = discriminant(_4)
// CHECK-NEXT:     v14 = stack_addr.i16 ss2
// CHECK-NEXT:     v15 = load.i16 notrap v14
// CHECK-NEXT:     v16 = iconst.i16 0
// CHECK-NEXT:     v17 = icmp eq v15, v16  ; v16 = 0
// CHECK-NEXT:     v18 = iconst.i16 0
// CHECK-NEXT:     v19 = iconst.i16 1
// CHECK-NEXT:     v20 = select v17, v18, v19  ; v18 = 0, v19 = 1
// CHECK-NEXT: ; write_cvalue: Var(_6, var4): isize <- ByVal(v20): isize
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _6)
// CHECK-NEXT:     v21 = uextend.i32 v20
// CHECK-NEXT:     br_table v21, block5, [block7, block6]
// CHECK-EMPTY:
// CHECK-NEXT: block5 cold:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; unreachable
// CHECK-NEXT:     trap user1
// CHECK-EMPTY:
// CHECK-NEXT: block6:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _16 = no_retag copy ((_4 as Some).0: &(i128, i128, i128, bool))
// CHECK-NEXT: ; write_cvalue: Var(_16, var12): &'{erased} (i128, i128, i128, bool) <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): &'{erased} (i128, i128, i128, bool)
// CHECK-NEXT:     v22 = stack_addr.i16 ss2
// CHECK-NEXT:     v23 = load.i16 notrap v22
// CHECK-NEXT: ; _7 = copy ((*_16).0: i128)
// CHECK-NEXT: ; write_cvalue: Var(_7, var5): i128 <- ByRef(Pointer { base: Addr(v23), offset: Offset32(0) }, None): i128
// CHECK-NEXT:     v24 = load.i128 notrap v23
// CHECK-NEXT: ; _17 = no_retag copy ((_4 as Some).0: &(i128, i128, i128, bool))
// CHECK-NEXT: ; write_cvalue: Var(_17, var13): &'{erased} (i128, i128, i128, bool) <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): &'{erased} (i128, i128, i128, bool)
// CHECK-NEXT:     v25 = stack_addr.i16 ss2
// CHECK-NEXT:     v26 = load.i16 notrap v25
// CHECK-NEXT: ; _8 = copy ((*_17).1: i128)
// CHECK-NEXT: ; write_cvalue: Var(_8, var6): i128 <- ByRef(Pointer { base: Addr(v26), offset: Offset32(16) }, None): i128
// CHECK-NEXT:     v27 = load.i128 notrap v26+16
// CHECK-NEXT: ; _18 = no_retag copy ((_4 as Some).0: &(i128, i128, i128, bool))
// CHECK-NEXT: ; write_cvalue: Var(_18, var14): &'{erased} (i128, i128, i128, bool) <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): &'{erased} (i128, i128, i128, bool)
// CHECK-NEXT:     v28 = stack_addr.i16 ss2
// CHECK-NEXT:     v29 = load.i16 notrap v28
// CHECK-NEXT: ; _9 = copy ((*_18).2: i128)
// CHECK-NEXT: ; write_cvalue: Var(_9, var7): i128 <- ByRef(Pointer { base: Addr(v29), offset: Offset32(32) }, None): i128
// CHECK-NEXT:     v30 = load.i128 notrap v29+32
// CHECK-NEXT:     v37 -> v30
// CHECK-NEXT: ; _19 = no_retag copy ((_4 as Some).0: &(i128, i128, i128, bool))
// CHECK-NEXT: ; write_cvalue: Var(_19, var15): &'{erased} (i128, i128, i128, bool) <- ByRef(Pointer { base: Stack(ss2), offset: Offset32(0) }, None): &'{erased} (i128, i128, i128, bool)
// CHECK-NEXT:     v31 = stack_addr.i16 ss2
// CHECK-NEXT:     v32 = load.i16 notrap v31
// CHECK-NEXT: ; _10 = copy ((*_19).3: bool)
// CHECK-NEXT: ; write_cvalue: Var(_10, var8): bool <- ByRef(Pointer { base: Addr(v32), offset: Offset32(48) }, None): bool
// CHECK-NEXT:     v33 = load.i8 notrap v32+48
// CHECK-NEXT:     v39 -> v33
// CHECK-NEXT: ;
// CHECK-NEXT: ; _13 = signed_multiply(copy _7, copy _8)
// CHECK-NEXT:     v34 = stack_addr.i16 ss3
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: i128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(i128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4294967311 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: i128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(i128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4294967311 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (i128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: i128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4294967311 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn2(v34, v24, v27)
// CHECK-NEXT:     jump block8
// CHECK-EMPTY:
// CHECK-NEXT: block7:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const 1_u8
// CHECK-NEXT:     v35 = iconst.i8 1
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v35): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v35  ; v35 = 1
// CHECK-EMPTY:
// CHECK-NEXT: block8:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _12 = &_13
// CHECK-NEXT:     v36 = stack_addr.i16 ss3
// CHECK-NEXT: ; write_cvalue: Var(_12, var10): &'{erased} (i128, bool) <- ByVal(v36): &'{erased} (i128, bool)
// CHECK-NEXT: ; _15 = (copy _9, copy _10)
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(0) }, None): i128 <- ByVal(v37): i128
// CHECK-NEXT:     v38 = stack_addr.i16 ss4
// CHECK-NEXT:     store.i128 notrap v37, v38
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Stack(ss4), offset: Offset32(16) }, None): bool <- ByVal(v39): bool
// CHECK-NEXT:     v40 = stack_addr.i16 ss4+16
// CHECK-NEXT:     store.i8 notrap v39, v40
// CHECK-NEXT: ; _14 = &_15
// CHECK-NEXT:     v41 = stack_addr.i16 ss4
// CHECK-NEXT: ; write_cvalue: Var(_14, var11): &'{erased} (i128, bool) <- ByVal(v41): &'{erased} (i128, bool)
// CHECK-NEXT: ;
// CHECK-NEXT: ; _11 = <(i128, bool) as core::cmp::PartialEq>::ne(move _12, move _14)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: &(i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }, ArgAbi { layout: TyAndLayout { ty: &(i128, bool), layout: Layout { size: Size(2 bytes), align: Align(2 bytes), backend_repr: Scalar(pointer is 1..=65535), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: pointer, valid_range: 1..=65535 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 17179869186 } }, mode: Direct(ArgAttributes { regular: NonNull | NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: Some(Align(2 bytes)) }) }], ret: ArgAbi { layout: TyAndLayout { ty: bool, layout: Layout { size: Size(1 bytes), align: Align(1 bytes), backend_repr: Scalar(u8 is 0..=1), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(1 bytes), repr_c: false, randomization_seed: 281483566645249 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: Zext, pointee_size: Size(0 bytes), pointee_align: None }) }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     v42 = call fn3(v36, v41)
// CHECK-NEXT:     v43 -> v42
// CHECK-NEXT: ; write_cvalue: Var(_11, var9): bool <- ByVal(v42): bool
// CHECK-NEXT:     jump block9
// CHECK-EMPTY:
// CHECK-NEXT: block9:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; switchInt(move _11)
// CHECK-NEXT:     brif.i8 v43, block10, block3
// CHECK-EMPTY:
// CHECK-NEXT: block10:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const 0_u8
// CHECK-NEXT:     v44 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v44): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; goto
// CHECK-NEXT:     return v44  ; v44 = 0
// CHECK-EMPTY:
// CHECK-NEXT: block11:
// CHECK-NEXT:     v46 = iconst.i8 0
// CHECK-NEXT:     v45 -> v46
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v45  ; v45 = 0
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol signed_multiply
// CHECK-NEXT: function u0:18(i16 sret, i128, i128) system_v {
// CHECK-NEXT: ; symbol signed_multiply
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; ret   _0      = v0       Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } (i128, bool)
// CHECK-NEXT: ; arg   _1      = v1       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i128
// CHECK-NEXT: ; arg   _2      = v2       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) i128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    i128                             16b 2                var=1
// CHECK-NEXT: ; ssa   _2    i128                             16b 2                var=2
// CHECK-EMPTY:
// CHECK-NEXT:     sig0 = (i16 sret, i128, i128) system_v
// CHECK-NEXT:     fn0 = colocated u0:9 sig0 ; Instance { def: Item(DefId(1:27137 ~ core[..]::num::{impl#4}::overflowing_mul)), args: [] }
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16, v1: i128, v2: i128):
// CHECK-NEXT:     v3 -> v1
// CHECK-NEXT:     v4 -> v2
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): i128 <- ByVal(v1): i128
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): i128 <- ByVal(v2): i128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; _0 = core::num::<impl i128>::overflowing_mul(copy _1, copy _2)
// CHECK-NEXT:     nop
// CHECK-NEXT: ; abi: FnAbi { args: [ArgAbi { layout: TyAndLayout { ty: i128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(i128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4294967311 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }, ArgAbi { layout: TyAndLayout { ty: i128, layout: Layout { size: Size(16 bytes), align: Align(2 bytes), backend_repr: Scalar(i128 is ..), fields: Primitive, largest_niche: None, uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4294967311 } }, mode: Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) }], ret: ArgAbi { layout: TyAndLayout { ty: (i128, bool), layout: Layout { size: Size(18 bytes), align: Align(2 bytes), backend_repr: ScalarPair { a: i128 is .., b: u8 is 0..=1, b_offset: Size(16 bytes) }, fields: Arbitrary { offsets: [Size(0 bytes), Size(16 bytes)], in_memory_order: [0, 1] }, largest_niche: Some(Niche { offset: Size(16 bytes), value: u8, valid_range: 0..=1 }), uninhabited: false, variants: Single { index: 0 }, max_repr_align: None, unadjusted_abi_align: Align(2 bytes), repr_c: false, randomization_seed: 4294967311 } }, mode: Indirect { attrs: ArgAttributes { regular: CapturesAddress | NoAlias | NonNull | NoUndef | NoFree, arg_ext: None, pointee_size: Size(18 bytes), pointee_align: Some(Align(2 bytes)) }, address_space: None, mode: Pointer } }, c_variadic: false, fixed_count: 2, conv: Rust, can_unwind: false }
// CHECK-NEXT:     call fn0(v0, v3, v4)
// CHECK-NEXT:     jump block2
// CHECK-EMPTY:
// CHECK-NEXT: block2:
// CHECK-NEXT:     nop
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
