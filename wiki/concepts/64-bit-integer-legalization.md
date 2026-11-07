# 64- and 128-bit integer legalization

_created 2026-11-06_

LR35902 represents CLIF `i64` as two `i32` limbs and `i128` as four `i32` limbs,
ordered least significant first. CLIF integers are signless, so this handles
signed and unsigned Rust integers; the opcode determines signed behavior.

The target legalization hook runs before and after optimization. It rewrites
function and imported signatures, block parameters, branch arguments, direct and
indirect calls, and returns together. Machine lowering never needs more than
four byte registers for one limb. Split returns can use the ABI's implicit
return area; `cg_clif` enables `enable_multi_ret_implicit_sret` for this
purpose.

Use the [coverage matrix](../../tests/coverage/lower.md) for supported
operations. Unsupported wide operations report an error. Division, remainder,
multiply-high, and checked multiplication still need legalization.

The 128-bit multiplier uses a 128-step shift/add loop; 128-bit shifts and
rotates iterate at most 127 times. These loops bound ROM use at the cost of
execution time.

The generated 32-bit variable shifts are further split into 16-bit operations,
using the existing shift helpers. This provides correct lowering but can produce
large functions; code-size optimization remains separate work.

Run the bounded emulator checks with:

```sh
cargo nextest run --no-fail-fast -E 'test(split_i64_values_match_host) | test(split_i128_values_match_host) | test(rust_u64_memory_add_lowers_and_executes) | test(rust_u128_memory_add_lowers_and_executes)'
```

The checks compile separate operation ROMs to stay within the current ROM0
limit. They cover both unoptimized and optimized CLIF, plus Rust through CLIF,
RGBDS, and the emulator. Build the Rust example as a complete ROM with:

```sh
cargo run -- build examples/u64.rs build/u64.gb
cargo run -- build examples/u128.rs build/u128.gb
cargo run -- build examples/u128-mul.rs build/u128-mul.gb
```

Integer-only ROM builds omit the soft-float runtime. Floating helper references
retain it. Keep operation examples separate while generated code uses ROM0.

Remaining native 128-bit arithmetic is tracked by `sangoku-7qz.9.3`. Checked
128-bit multiplication uses compiler-builtins rather than CLIF overflow opcodes.
