# Lowering

Rust passes through MIR and Cranelift IR (CLIF) before the LR35902 backend emits
RGBDS assembly. RGBDS assembles and links the result with startup code and the
required runtime helpers into a ROM.

## Integer operations

Narrow operations lower to SM83 instructions or runtime helpers. Constant i8 and
i16 shifts and rotates expand inline; variable counts use `__gbc_shift8` and
`__gbc_shift16`. Generated i32 variable shifts split further into i16
operations. [Legalization](legalization.md) handles wider values first.

The runtime provides wrapping 16-bit multiplication for signed and unsigned
operands, and unsigned 16-bit division/remainder. Division and remainder share
an SM83 core and reach the trap vector for a zero divisor. Spills and reloads
preserve live registers and flags.

Truncating stores accept a wider input and write only its low bits. `istore8`,
`istore16` and `istore32` lower for every CLIF-expressible input width through
i64. Extending loads read 8, 16 or 32 bits and sign- or zero-extend to the
requested wider type, including i64. The split path preserves explicit memory
byte order.

i32 checked arithmetic, multiply-high and leading-sign-bit counts expand to
narrower operations before machine lowering. Checked arithmetic uses a full
64-bit intermediate and compares its upper half with the required extension of
the low half.

Use the [coverage matrix](../../tests/coverage/lower.md) for current operation
support rather than assuming every width supports the same opcodes.

## Floating-point helpers

[runtime/softfloat32.asm](../../runtime/softfloat32.asm) implements correctly
rounded f32 add, sub, mul, div, sqrt, compare, min, max, fma, trunc, floor, ceil
and nearest. It also supplies i8/i16/i32 conversions to and from f32 (trapping
and saturating), and f16 promote/demote. See [legalization](legalization.md) for
f64 support and remaining limits.

## Calling convention

Argument bytes go in B, C, A, then on the stack, low address first. The caller
owns the stack arguments; the callee does not pop them. All registers are
caller-saved.

Return values use A, B, C, D, E, H, L. A value that does not fit in the
registers left is returned whole in a caller-allocated return area. This adds a
hidden 16-bit pointer argument after the declared arguments and requires
`enable_multi_ret_implicit_sret`, which cg_clif enables for split returns.

The full Rust ABI, Rust data objects and tail calls remain incomplete.
