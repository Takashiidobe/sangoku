# 64-bit integer legalization

_created 2026-11-06_

LR35902 represents each CLIF `i64` value as `(low: i32, high: i32)` before
optimization and machine lowering. CLIF integers are signless, so this handles
Rust `u64` and `i64`; the opcode determines signed behavior.

The target legalization hook runs before and after optimization. It rewrites
function and imported signatures, block parameters, branch arguments, direct and
indirect calls, and returns together. Machine lowering never needs eight byte
registers for one value. Split returns can use the ABI's implicit return area;
`cg_clif` enables `enable_multi_ret_implicit_sret` for this purpose.

Use the [coverage matrix](../../tests/coverage/lower.md) for supported
operations. Unsupported wide operations report an error. Division, remainder,
multiply-high, and checked multiplication still need legalization.

The generated 32-bit variable shifts are further split into 16-bit operations,
using the existing shift helpers. This provides correct lowering but can produce
large functions; code-size optimization remains separate work.

Run the bounded emulator checks with:

```sh
cargo nextest run --no-fail-fast -E 'test(split_i64_values_match_host) | test(rust_u64_memory_add_lowers_and_executes)'
```

The checks compile separate operation ROMs to stay within the current ROM0
limit. They cover both unoptimized and optimized CLIF, plus Rust through CLIF,
RGBDS, and the emulator. Build the Rust example as a complete ROM with:

```sh
cargo run -- build examples/u64.rs build/u64.gb
```
