# Frontend fixtures

Run the Rust-to-CLIF snapshots and ROM checks with:

```bash
cargo nextest run --no-fail-fast --test filecheck --test frontend_epic
```

Regenerate snapshots with `tools/update_filecheck.py`, then review the generated
checks. CLIF snapshots verify frontend output; ROM tests also assemble, link,
and execute the same Rust source.

| Behavior                                                 | Fixture under `tests/fixtures/clif/`  | ROM result          |
| -------------------------------------------------------- | ------------------------------------- | ------------------- |
| Scalar, ignored, pair, indirect arguments and returns    | `abi-pass-modes.rs`                   | Executes            |
| String, array, struct, data and function relocations     | `data-relocations.rs`                 | Executes            |
| Function pointers and trait object dispatch              | `indirect-dispatch.rs`                | Executes            |
| u64/u128 overflowing add, i128 wrapping add              | `wide-scalars.rs`                     | Executes            |
| f64 and f128 addition                                    | `wide-float64.rs`, `wide-float128.rs` | Execute separately  |
| f16 arithmetic, comparisons, rounding and special values | `wide-float16.rs`                     | Executes            |
| u128 overflowing multiplication                          | `wide-overflow-mul.rs`                | Executes at 0 and z |
| Variable copy, overlapping move, fill, u32 division      | `runtime-calls.rs`                    | Executes            |
| u32 bit operations and u16 saturating arithmetic         | `core-intrinsics.rs`                  | Executes            |
| Inlining, constant branch, add-zero baseline             | `frontend-optimization.rs`            | Executes            |

`tests/fixtures/mir/frontend-debug.rs` checks MIR function output.
`integer-*.rs` snapshots checked, wrapping, overflowing and bit-count methods
for all signed and unsigned integer widths, including 16-bit `isize` and
`usize`. The core driver instantiates the same method families for lowering
checks. The combined wide-method probes can exceed ROM capacity; the narrower
integer probes execute independently.

`signed-overflow-mul.rs`, `signed-overflow-mul64.rs`,
`unsigned-overflow-mul64.rs` and `wide-overflow-mul.rs` execute overflow
multiplication at opt-levels 0 and z. The shared unsigned routine occupies 122
bytes; its signed wrapper adds 170 bytes. Direct runtime tests compare both
product and overflow against host Rust and check input preservation, output
bounds and stack restoration. The target 64-bit calls widen their operands and
check that the 128-bit result fits the original signed or unsigned width.

Inspect mapped runtime symbols and their providers with:

```bash
python3 tools/core_report.py
```

The report checks the target backend's libcall mappings against runtime assembly
and lowered compiler-builtins definitions. The tests also check that exported
runtime entries select their runtime objects. Frontend math fallbacks such as
floating-point remainder need a separate audit; mapped-symbol completeness does
not establish support for every math method.

The f16 arithmetic wrappers promote to f32 and demote through the existing
soft-float runtime. f64/f128 arithmetic already uses shared target dispatchers.
Wide integer algorithm bodies and caller copies still need further sharing to
make larger combinations fit a ROM.
