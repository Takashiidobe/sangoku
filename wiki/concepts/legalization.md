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

`f128` and conversions between `i64` and floating point remain unsupported.

## Operation support

The [coverage matrix](../../tests/coverage/lower.md) records which CLIF
operations lower. Unsupported wide operations report an error. Successful
lowering alone does not establish that all of Rust `core` can compile.
