# Intrinsic call audit

Use this audit to choose shared libcalls for expensive Rust intrinsic paths. It
records emitted calls at optimization level 0; successful lowering does not
establish ROM linkability, size, or execution correctness.

```bash
python3 tools/intrinsics_audit.py --keep build/intrinsics-audit
cargo nextest run --profile extensive --no-fail-fast --test intrinsics
```

The [baseline](../tests/coverage/intrinsics.tsv) covers signed and unsigned
8/16/32/64/128-bit integers, 16-bit `usize`/`isize`, unsigned funnel shifts and
carryless multiplication, `f16`/`f32`/`f64`/`f128`, and memory intrinsics. All
339 explicit cases compile and lower. Inputs remain dynamic to expose runtime
implementations rather than constant evaluation.

`lowering` rows record per-function CLIF results. `callees` rows follow direct
calls and tail jumps through generated assembly and cached `core` and
`compiler_builtins`, stopping at target runtime or undefined symbols.
`rust-functions` counts reachable Rust functions before runtime objects.
`no-external-call` can still mean a large Rust fallback. The graph includes both
sides of branches and does not model indirect calls or data references; it is an
optimization inventory, not a replacement for the ROM linker.

Keep generated `.rs`, `.clif`, `.s`, `.stderr`, and `.probe` files with
`--keep`. Regenerate the baseline after reviewing changes with the command
above; `--check` compares without updating it. The extensive profile contains
this matrix so normal test runs stay bounded.

## Intrinsics observed in core compilation

Reproduce the whole-library and representative generic-driver survey with:

```bash
python3 tools/core_report.py --json --keep build/intrinsics-core-report
```

The 2026-11-08 survey lowers 3,184 core functions, 2,043 compiler-builtins
functions, and 921 driver functions, with no lowering failures or undefined
symbols. Their CLIF references identify the following intrinsic families. This
does not instantiate every possible generic API or include intrinsics already
removed by constant evaluation or MIR lowering.

| Intrinsics observed                                                                                 | Current route/status                                                    |
| --------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| `abort_immediate`, `cold_path`                                                                      | trap or code-generation hint                                            |
| `likely`, `unlikely`, `is_val_statically_known`, `select_unpredictable`                             | compiler handling or Rust fallback; surveyed callers lower              |
| `assert_inhabited`                                                                                  | type-validity check at compilation; surveyed callers lower              |
| `align_of_val`, `size_of_val`                                                                       | layout and pointer metadata operations                                  |
| `arith_offset`, `ptr_offset_from`, `ptr_offset_from_unsigned`                                       | pointer arithmetic                                                      |
| `copy`, `write_bytes`, `compare_bytes`                                                              | memory routines or compiler-builtins; see below                         |
| `typed_swap_nonoverlapping`, `disjoint_bitor`                                                       | Rust fallback bodies                                                    |
| `exact_div`, `saturating_add`, `saturating_sub`                                                     | integer operations and overflow/select sequences                        |
| `integer_min`, `integer_max`                                                                        | CLIF min/max for narrower integers; Rust comparisons for 128-bit values |
| `ctpop`, `ctlz`, `ctlz_nonzero`, `cttz`, `cttz_nonzero`, `bswap`                                    | inline sequences or existing bit helpers                                |
| `rotate_left`, `unchecked_funnel_shl`                                                               | Rust shift/combine fallback plus width-dependent shift lowering         |
| `carrying_mul_add`                                                                                  | widening arithmetic and Rust wide-multiply fallback                     |
| `fabs`, `copysignf64`, `maximum_number_nsz_f32`, `maximum_number_nsz_f64`, `minimum_number_nsz_f64` | inline float bit operations or software float helpers                   |

`copy_nonoverlapping`, arithmetic, transmutation, layout constants, and other
intrinsics can become MIR statements or evaluated constants before these
references are emitted. See
[`lower_intrinsics.rs`](../third_party/rust/compiler/rustc_mir_transform/src/lower_intrinsics.rs)
and the backend's
[intrinsic dispatch](../third_party/rustc_codegen_cranelift/src/intrinsics/mod.rs).
The target declares `max-atomic-width = 0`; atomic source definitions in core
are not evidence that atomic operations are available on this target.

## Calls worth optimizing

| Rust path at level 0                                      | Terminal calls / implementation                                                        | Follow-up                         |
| --------------------------------------------------------- | -------------------------------------------------------------------------------------- | --------------------------------- |
| `u8::count_ones`                                          | `__gbc_popcnt32`; `u16` currently uses a small inline sequence                         | `sangoku-57q.10.4`                |
| narrow leading/trailing-zero counts and bit reversal      | `__gbc_clz32`, `__gbc_ctz32`, `__gbc_bitrev32` after widening                          | `sangoku-57q.10.4`                |
| 32/64-bit counts and bit reversal                         | existing 32-bit helpers, repeated for wider values                                     | `sangoku-57q.10.1`                |
| variable rotate and unsigned funnel shifts                | `__gbc_shift8` for 8-bit; `__gbc_shift16` through 64-bit; inline expansion for 128-bit | `sangoku-clp`, `sangoku-57q.10.1` |
| `carrying_mul_add`                                        | wide Rust arithmetic; no dedicated shared helper for 64/128-bit                        | `sangoku-7qz.9.8`                 |
| wide `mul_with_overflow`                                  | `__rust___gbc_u128_mulo` / `__rust___gbc_i128_mulo`                                    | `sangoku-7qz.9.8`                 |
| unsigned `carryless_mul`                                  | Rust loop and shift operations; no dedicated helper                                    | `sangoku-57q.10.5`                |
| dynamic byte `copy`, `copy_nonoverlapping`, `write_bytes` | `memmove`, `memcpy`, `__gbc_memset`; one shared runtime object                         | `sangoku-9bq.1`                   |
| `compare_bytes`, fixed-size `raw_eq`                      | compiler-builtins/Rust comparison or inline loads                                      | `sangoku-mt4.6`                   |
| saturation, byte swap, integer min/max                    | inline operations or Rust comparisons; preserve cheap forms                            | no new helper proposed            |

For prioritization, counting nonlabel assembly lines in reachable Rust bodies at
level 0 gives roughly 639 for `rotate_left::<u32>`, 1,483 for
`rotate_left::<u128>`, 8,821 for `carrying_mul_add::<u64>`, and 47,408 for
`carrying_mul_add::<u128>`. These exclude runtime objects and are not byte or
cycle measurements. Compare caller, helper, and complete ROM bytes at levels 0
and z before selecting an ABI or replacement.

## Floating-point boundaries

`f32` square root, rounding, and FMA reach `__gbc_fsqrt32`, `__gbc_fround32`,
and `__gbc_ffma32`; absolute value and copy-sign use bit operations. `f64` and
`f128` operations commonly reach the `__gbc_f64`, `__gbc_f128`, and
`__gbc_convert` dispatchers. Some `f32` libm fallback chains also reach these
wider helpers and integer arithmetic.

Named leaves such as `sqrtf16`, `roundf16`, `__powitf2`, and `fmodf128`, plus
some `f128` transcendental functions, remain outside this inventory's supplied
assembly graph. Record their boundary without attempting a complete math library
here. Existing `sangoku-57q.10.2` tracks software-float call routing,
unsupported boundaries, and code-size reduction. Runtime object splitting and
algorithm changes should follow the calls that supported programs retain.
