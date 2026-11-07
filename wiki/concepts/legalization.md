# Legalization

The LR35902 legalization pass rewrites CLIF values and operations into types
that fit the backend before machine lowering. It runs before and after
optimization and rewrites signatures, block parameters, branch arguments, calls
and returns together.

## Wide integers

`i64` becomes two `i32` limbs and `i128` becomes four, least significant first.
CLIF integers are signless; the opcode determines signed behavior, so the same
representation handles Rust's signed and unsigned integers. Each limb fits in
four byte registers.

See [wide integer legalization](64-bit-integer-legalization.md) for operation
expansion, examples and focused tests. Division, remainder, multiply-high and
checked multiplication still need legalization. Checked Rust `i128`
multiplication uses compiler-builtins rather than CLIF overflow opcodes.

## Floating point

`f32` uses four byte registers and softfloat helpers. `f16` arithmetic,
comparisons and rounding promote to `f32`, operate, and demote.

`f64` uses two `i32` bit-pattern halves. Constants, loads/stores, `i64`
bitcasts, select, block arguments, direct calls/returns, negation, absolute
value and copysign preserve every bit, including NaN payloads and signed zero.
Arithmetic, comparisons, rounding and conversions still need f64 softfloat
helpers (`sangoku-7qz.9.4.1`). Splitting the representation does not make f32
arithmetic sufficient for f64 precision or range.

`f128` uses four `i32` bit-pattern limbs, least significant first. It supports
the same transport and sign operations as f64, with `i128` bitcasts. Constants
retain all 128 bits; negation, absolute value and copysign modify only the sign
bit in the highest limb. Rust f128 constants and memory access use this path
without frontend representation changes. The LR35902 frontend float gate allows
f128 through to legalization; its other float exclusions remain.

f128 arithmetic, comparisons, rounding and numeric conversions still need
binary128 softfloat helpers (`sangoku-7qz.9.6`). cg_clif emits compiler-builtins
calls for several of these operations; basic legalization does not provide their
runtime bodies. Conversions between `i64` and floating point also remain
unsupported.

Build `examples/f128.rs` with `cargo run -- build` for a Rust-to-ROM example.
Run the bounded transport tests with:

```sh
cargo nextest run --no-fail-fast -E 'test(split_f128_preserves_bits) | test(rust_f128_memory_lowers_and_executes)'
```

The CLIF tests use separate operation ROMs to stay within ROM0 and cover both
unoptimized and optimized lowering, including explicit big-endian memory access.

## Operation support

The [coverage matrix](../../tests/coverage/lower.md) records which CLIF
operations lower. Unsupported wide operations report an error. Successful
lowering alone does not establish that all of Rust `core` can compile.
