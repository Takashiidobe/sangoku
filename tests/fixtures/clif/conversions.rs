#![feature(f16, f128, no_core, lang_items, auto_traits)]
#![allow(internal_features)]
#![no_core]

#[lang = "pointee_sized"]
pub trait PointeeSized {}
#[lang = "meta_sized"]
pub trait MetaSized: PointeeSized {}
#[lang = "sized"]
pub trait Sized: MetaSized {}
#[lang = "copy"]
pub trait Copy: Sized {}
#[lang = "freeze"]
pub unsafe auto trait Freeze {}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 { 0 }

impl Copy for f128 {}
impl Copy for f16 {}
impl Copy for f32 {}
impl Copy for f64 {}
impl Copy for i128 {}
impl Copy for i64 {}
impl Copy for i8 {}
impl Copy for u128 {}
impl Copy for u64 {}

#[unsafe(no_mangle)]
pub unsafe fn i64_to_f16(memory: *mut i64) {
    unsafe { *(memory as *mut f16) = *memory as f16 };
}

#[unsafe(no_mangle)]
pub unsafe fn u64_to_f32(memory: *mut u64) {
    unsafe { *(memory as *mut f32) = *memory as f32 };
}

#[unsafe(no_mangle)]
pub unsafe fn i128_to_f64(memory: *mut i128) {
    unsafe { *(memory as *mut f64) = *memory as f64 };
}

#[unsafe(no_mangle)]
pub unsafe fn u128_to_f128(memory: *mut u128) {
    unsafe { *(memory as *mut f128) = *memory as f128 };
}

#[unsafe(no_mangle)]
pub unsafe fn f64_to_i8(memory: *mut f64) {
    unsafe { *(memory as *mut i8) = *memory as i8 };
}

#[unsafe(no_mangle)]
pub unsafe fn f128_to_u128(memory: *mut f128) {
    unsafe { *(memory as *mut u128) = *memory as u128 };
}

#[unsafe(no_mangle)]
pub unsafe fn f64_to_f16(memory: *mut f64) {
    unsafe { *(memory as *mut f16) = *memory as f16 };
}

#[unsafe(no_mangle)]
pub unsafe fn f16_to_f128(memory: *mut f16) {
    unsafe { *(memory as *mut f128) = *memory as f128 };
}

#[unsafe(no_mangle)]
pub unsafe fn f128_to_f32(memory: *mut f128) {
    unsafe { *(memory as *mut f32) = *memory as f32 };
}

// CHECK: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol f128_to_f32
// CHECK-NEXT: function u0:0(i16) system_v {
// CHECK-NEXT: ; symbol f128_to_f32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut f128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut f128                         2b 2                var=1
// CHECK-NEXT: ; ssa   _2    f128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    *mut f32                          2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut f128 <- ByVal(v0): *mut f128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): f128 <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f128
// CHECK-NEXT:     v2 = load.f128 notrap v1
// CHECK-NEXT: ; _3 = copy _1 as *mut f32 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *mut f32 <- ByVal(v1): *mut f32
// CHECK-NEXT: ; (*_3) = move _2 as f32 (FloatToFloat)
// CHECK-NEXT:     v3 = fdemote.f32 v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f32 <- ByVal(v3): f32
// CHECK-NEXT:     store notrap v3, v1
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol f128_to_u128
// CHECK-NEXT: function u0:1(i16) system_v {
// CHECK-NEXT: ; symbol f128_to_u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut f128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut f128                         2b 2                var=1
// CHECK-NEXT: ; ssa   _2    f128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    *mut u128                         2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut f128 <- ByVal(v0): *mut f128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): f128 <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f128
// CHECK-NEXT:     v2 = load.f128 notrap v1
// CHECK-NEXT: ; _3 = copy _1 as *mut u128 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *mut u128 <- ByVal(v1): *mut u128
// CHECK-NEXT: ; (*_3) = move _2 as u128 (FloatToInt)
// CHECK-NEXT:     v3 = fcvt_to_uint_sat.i128 v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v1), offset: Offset32(0) }, None): u128 <- ByVal(v3): u128
// CHECK-NEXT:     store notrap v3, v1
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol f16_to_f128
// CHECK-NEXT: function u0:2(i16) system_v {
// CHECK-NEXT: ; symbol f16_to_f128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut f16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut f16                          2b 2                var=1
// CHECK-NEXT: ; ssa   _2    f16                               2b 2                var=2
// CHECK-NEXT: ; ssa   _3    *mut f128                         2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut f16 <- ByVal(v0): *mut f16
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): f16 <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f16
// CHECK-NEXT:     v2 = load.f16 notrap v1
// CHECK-NEXT: ; _3 = copy _1 as *mut f128 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *mut f128 <- ByVal(v1): *mut f128
// CHECK-NEXT: ; (*_3) = move _2 as f128 (FloatToFloat)
// CHECK-NEXT:     v3 = fpromote.f128 v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f128 <- ByVal(v3): f128
// CHECK-NEXT:     store notrap v3, v1
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol f64_to_f16
// CHECK-NEXT: function u0:3(i16) system_v {
// CHECK-NEXT: ; symbol f64_to_f16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut f64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut f64                          2b 2                var=1
// CHECK-NEXT: ; ssa   _2    f64                               8b 2                var=2
// CHECK-NEXT: ; ssa   _3    *mut f16                          2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut f64 <- ByVal(v0): *mut f64
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): f64 <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f64
// CHECK-NEXT:     v2 = load.f64 notrap v1
// CHECK-NEXT: ; _3 = copy _1 as *mut f16 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *mut f16 <- ByVal(v1): *mut f16
// CHECK-NEXT: ; (*_3) = move _2 as f16 (FloatToFloat)
// CHECK-NEXT:     v3 = fdemote.f16 v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f16 <- ByVal(v3): f16
// CHECK-NEXT:     store notrap v3, v1
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol f64_to_i8
// CHECK-NEXT: function u0:4(i16) system_v {
// CHECK-NEXT: ; symbol f64_to_i8
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut f64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut f64                          2b 2                var=1
// CHECK-NEXT: ; ssa   _2    f64                               8b 2                var=2
// CHECK-NEXT: ; ssa   _3    *mut i8                           2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut f64 <- ByVal(v0): *mut f64
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): f64 <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f64
// CHECK-NEXT:     v2 = load.f64 notrap v1
// CHECK-NEXT: ; _3 = copy _1 as *mut i8 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *mut i8 <- ByVal(v1): *mut i8
// CHECK-NEXT: ; (*_3) = move _2 as i8 (FloatToInt)
// CHECK-NEXT:     v3 = fcvt_to_sint_sat.i8 v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v1), offset: Offset32(0) }, None): i8 <- ByVal(v3): i8
// CHECK-NEXT:     store notrap v3, v1
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
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
// CHECK-EMPTY:
// CHECK-NEXT: block0:
// CHECK-NEXT:     nop
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _0 = const 0_u8
// CHECK-NEXT:     v0 = iconst.i8 0
// CHECK-NEXT: ; write_cvalue: Var(_0, var1): u8 <- ByVal(v0): u8
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return v0  ; v0 = 0
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol i128_to_f64
// CHECK-NEXT: function u0:6(i16) system_v {
// CHECK-NEXT: ; symbol i128_to_f64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut i128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut i128                         2b 2                var=1
// CHECK-NEXT: ; ssa   _2    i128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    *mut f64                          2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut i128 <- ByVal(v0): *mut i128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): i128 <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): i128
// CHECK-NEXT:     v2 = load.i128 notrap v1
// CHECK-NEXT: ; _3 = copy _1 as *mut f64 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *mut f64 <- ByVal(v1): *mut f64
// CHECK-NEXT: ; (*_3) = move _2 as f64 (IntToFloat)
// CHECK-NEXT:     v3 = fcvt_from_sint.f64 v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f64 <- ByVal(v3): f64
// CHECK-NEXT:     store notrap v3, v1
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol i64_to_f16
// CHECK-NEXT: function u0:7(i16) system_v {
// CHECK-NEXT: ; symbol i64_to_f16
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut i64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut i64                          2b 2                var=1
// CHECK-NEXT: ; ssa   _2    i64                               8b 2                var=2
// CHECK-NEXT: ; ssa   _3    *mut f16                          2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut i64 <- ByVal(v0): *mut i64
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): i64 <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): i64
// CHECK-NEXT:     v2 = load.i64 notrap v1
// CHECK-NEXT: ; _3 = copy _1 as *mut f16 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *mut f16 <- ByVal(v1): *mut f16
// CHECK-NEXT: ; (*_3) = move _2 as f16 (IntToFloat)
// CHECK-NEXT:     v3 = fcvt_from_sint.f16 v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f16 <- ByVal(v3): f16
// CHECK-NEXT:     store notrap v3, v1
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol u128_to_f128
// CHECK-NEXT: function u0:8(i16) system_v {
// CHECK-NEXT: ; symbol u128_to_f128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut u128
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut u128                         2b 2                var=1
// CHECK-NEXT: ; ssa   _2    u128                             16b 2                var=2
// CHECK-NEXT: ; ssa   _3    *mut f128                         2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut u128 <- ByVal(v0): *mut u128
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): u128 <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): u128
// CHECK-NEXT:     v2 = load.i128 notrap v1
// CHECK-NEXT: ; _3 = copy _1 as *mut f128 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *mut f128 <- ByVal(v1): *mut f128
// CHECK-NEXT: ; (*_3) = move _2 as f128 (IntToFloat)
// CHECK-NEXT:     v3 = fcvt_from_uint.f128 v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f128 <- ByVal(v3): f128
// CHECK-NEXT:     store notrap v3, v1
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
// CHECK-EMPTY:
// CHECK-NEXT: target z80 optimize_for_size=0
// CHECK-EMPTY:
// CHECK-NEXT: ; symbol u64_to_f32
// CHECK-NEXT: function u0:9(i16) system_v {
// CHECK-NEXT: ; symbol u64_to_f32
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  loc.idx   param    pass mode                            ty
// CHECK-NEXT: ; zst   _0    ()                                0b 1                align=1,offset=
// CHECK-NEXT: ; ret   _0      -          Ignore                               ()
// CHECK-NEXT: ; arg   _1      = v0       Direct(ArgAttributes { regular: NoUndef, arg_ext: None, pointee_size: Size(0 bytes), pointee_align: None }) *mut u64
// CHECK-EMPTY:
// CHECK-NEXT: ; kind  local ty                              size align (abi)
// CHECK-NEXT: ; ssa   _1    *mut u64                          2b 2                var=1
// CHECK-NEXT: ; ssa   _2    u64                               8b 2                var=2
// CHECK-NEXT: ; ssa   _3    *mut f32                          2b 2                var=3
// CHECK-EMPTY:
// CHECK-NEXT: block0(v0: i16):
// CHECK-NEXT:     v1 -> v0
// CHECK-NEXT:     nop
// CHECK-NEXT: ; write_cvalue: Var(_1, var1): *mut u64 <- ByVal(v0): *mut u64
// CHECK-NEXT:     jump block1
// CHECK-EMPTY:
// CHECK-NEXT: block1:
// CHECK-NEXT:     nop
// CHECK-NEXT: ; _2 = copy (*_1)
// CHECK-NEXT: ; write_cvalue: Var(_2, var2): u64 <- ByRef(Pointer { base: Addr(v1), offset: Offset32(0) }, None): u64
// CHECK-NEXT:     v2 = load.i64 notrap v1
// CHECK-NEXT: ; _3 = copy _1 as *mut f32 (PtrToPtr)
// CHECK-NEXT: ; write_cvalue: Var(_3, var3): *mut f32 <- ByVal(v1): *mut f32
// CHECK-NEXT: ; (*_3) = move _2 as f32 (IntToFloat)
// CHECK-NEXT:     v3 = fcvt_from_uint.f32 v2
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Addr(v1), offset: Offset32(0) }, None): f32 <- ByVal(v3): f32
// CHECK-NEXT:     store notrap v3, v1
// CHECK-NEXT: ; _0 = const ()
// CHECK-NEXT: ; write_cvalue: Addr(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): () <- ByRef(Pointer { base: Dangling(Align(1 bytes)), offset: Offset32(0) }, None): ()
// CHECK-NEXT: ;
// CHECK-NEXT: ; return
// CHECK-NEXT:     return
// CHECK-NEXT: }
