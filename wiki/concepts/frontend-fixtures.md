# Frontend fixtures

Run the Rust-to-CLIF snapshots and ROM checks with:

```bash
cargo nextest run --no-fail-fast --test filecheck --test frontend_epic
```

Regenerate snapshots with `tools/update_filecheck.py`, then review the generated
checks. CLIF snapshots verify frontend output; ROM tests also assemble, link,
and execute the same Rust source.

| Behavior                                                 | Fixture under `tests/fixtures/clif/`  | ROM result           |
| -------------------------------------------------------- | ------------------------------------- | -------------------- |
| Scalar, ignored, pair, indirect arguments and returns    | `abi-pass-modes.rs`                   | Executes             |
| String, array, struct, data and function relocations     | `data-relocations.rs`                 | Executes             |
| Function pointers and trait object dispatch              | `indirect-dispatch.rs`                | Executes             |
| u64/u128 overflowing add, i128 wrapping add              | `wide-scalars.rs`                     | Executes             |
| f64 and f128 addition                                    | `wide-float64.rs`, `wide-float128.rs` | Execute separately   |
| f16 arithmetic, comparisons, rounding and special values | `wide-float16.rs`                     | Executes             |
| u128 overflowing multiplication                          | `wide-overflow-mul.rs`                | Exceeds ROM capacity |
| Variable copy, overlapping move, fill, u32 division      | `runtime-calls.rs`                    | Executes             |
| u32 bit operations and u16 saturating arithmetic         | `core-intrinsics.rs`                  | Executes             |
| Inlining, constant branch, add-zero baseline             | `frontend-optimization.rs`            | Executes             |

`tests/fixtures/mir/frontend-debug.rs` checks MIR function output. These probes
cover representative operations; they do not establish a complete intrinsic or
runtime-symbol inventory, optimization byte comparisons, per-function diagnostic
filtering, or compilation from saved lowering/object files.

The f16 arithmetic wrappers promote to f32 and demote through the existing
soft-float runtime. The u128 multiplication probe exceeds the 32 KiB cartridge
at opt-level 0; at opt-level z its compiler-builtins multiply helper exceeds a
16 KiB section. Its regression test asserts the capacity failure. Replace that
assertion with an execution check when support lands.
