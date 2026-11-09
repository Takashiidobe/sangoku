# Lowering

Rust passes through MIR and Cranelift IR (CLIF) before the LR35902 backend emits
RGBDS assembly. RGBDS assembles and links the result with startup code and the
required runtime helpers into a ROM.

## Integer operations

Narrow operations lower to SM83 instructions or runtime helpers. Constant i8 and
i16 shifts and rotates expand inline; variable counts use `__gbc_shift8` and
`__gbc_shift16`. Generated i32 variable shifts split further into i16
operations. [Legalization](legalization.md) handles wider values first.

The runtime provides wrapping multiplication at native 8-, 16- and 32-bit
widths for signed and unsigned operands. Levels s/z select compact loops;
0–3 select unrolled speed variants. i8 multiplication no longer widens to i16.
Unsigned 16-bit division and remainder share an SM83 core and reach the trap
vector for a zero divisor. Spills and reloads preserve live registers and flags.

Population count has native 8-, 16- and 32-bit runtime entries. Levels `s`/`z`
select independent compact shift loops; `0`–`3` select wrappers sharing one
register-only parallel byte counter. Optimized non-size i16 lowering remains
inline to avoid call overhead. Other i16 builds call the native helper, and
i8 no longer widens to i32. See [runtime libraries](runtime-libraries.md) for
the calling convention and measured size/speed tradeoffs.

Leading/trailing-zero counts and bit reversal also use native i8/i16/i32
runtime entries, with size/speed selection. Counts of zero return the native
width. Speed scans share their eight-bit scanner, and bit reversal shares a
byte kernel. See [runtime libraries](runtime-libraries.md) for costs and ABI.

Signed i8/i16 division and remainder widen through the i32 helpers. Division
checks that the result fits the original width and traps on `MIN / -1`.
Remainder returns zero for `MIN % -1`; both operations trap on division by zero.

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

[runtime/float/all_f32.asm](../../runtime/float/all_f32.asm) implements correctly
rounded f32 add, sub, mul, div, sqrt, compare, min, max, fma, trunc, floor, ceil
and nearest. It also supplies i8/i16/i32 conversions to and from f32 (trapping
and saturating), and f16 promote/demote. See [legalization](legalization.md) for
f64 support and remaining limits.

The f64 helper uses a memory context containing operands, result, opcode and
comparison mask. Its arithmetic state lives on the stack, so calls are
reentrant. [runtime/float/f64/common/implementation.c](../../runtime/float/f64/common/implementation.c) implements the
byte-array algorithms; [runtime/float/f64/common/dispatch.asm](../../runtime/float/f64/common/dispatch.asm)
is the checked-in RGBDS output. Calls do not inline the algorithms.

Working significands use 128-bit byte arrays with the leading bit at bit 120.
This retains exact 106-bit products and room for guard, round and sticky bits.
Fused multiply-add combines the unrounded product with its addend before the
single rounding step.

Regenerate it with SDCC's SM83 backend (generated with SDCC 4.6):

```sh
python3 tools/generate_softfloat64.py
python3 tools/generate_softfloat64.py --check
```

SDCC is needed for regeneration only. Normal builds assemble the checked-in
output with RGBDS. Emulator tests verify the source hash, arithmetic results,
caller size and runtime size.

## Numeric conversions

[runtime/float/convert/common/implementation.c](../../runtime/float/convert/common/implementation.c) handles conversions involving wide
integers or floats. A stack context holds 16 source bytes, 16 result bytes,
source and destination byte widths, a mode byte and a status byte.
`__gbc_convert` takes the context pointer through the split calling convention.
The helper has no mutable global state.

Its 17-byte significand holds a leading bit at bit 128, retaining every i128
input bit and binary128 significand bit. Narrowing preserves guard and sticky
bits and rounds once, to nearest with ties to even. Integer results truncate
before range checking; the caller turns NaN or overflow status into a trap for
trapping opcodes. Saturating opcodes use the clamped result directly.

Regenerate the checked-in RGBDS output and bounded integer-only reference cases:

```sh
python3 tools/generate_convert.py
python3 tools/generate_conversion_cases.py
```

Run `cargo nextest run --no-fail-fast -E 'test(numeric_conversions)'` to check
both CLIF lowering and representative Rust casts at none/speed optimization.

## Rust data and formatting

RGBDS emission supports defined read-only data objects, including 16-bit
function/data relocations and per-codegen-unit anonymous names. Writable data,
TLS and custom data sections remain unsupported in this path. The target layout
aligns ordinary wide scalars to two bytes, matching the backend stack ABI.

The runtime supplies independently linked `__gbc_memset`, `memcpy`, and
`memmove` helpers. Build with `--opt-level s` or `--opt-level z` to select
compact byte loops for all three helpers; levels `0`, `1`, `2`, and `3` select
four-byte-unrolled implementations. `memmove` unrolls both copy directions. Only
the selected variant of each referenced helper is linked. All three return the
destination pointer and accept zero lengths; `memmove` also handles overlap.
`memcmp` uses the runtime's independently selected size/speed implementations
for both compiler libcalls and Rust named calls. They export `__rust_memcmp`,
replacing compiler-builtins' definition during linkage. Ordering compares
unsigned bytes and returns a negative, zero or positive i32; zero lengths are
accepted. Small fixed-size equality comparisons retain their inline loads.
Branch-table assembly uses executable comparisons and jumps for full-width
indices.

Run `cargo run --example audit-memory-inline` for a bounded comparison of
backend-generated inline copy/fill sequences and helper calls at 1–16 bytes.
Results, callers, assembly, and provenance are written to
`build/memory-inline/`. This probe uses a dynamic destination and constant
source/value without unrelated live values; it does not establish a universal
inlining threshold.

Run `cargo run --example audit-move-compare` for bounded overlap-safe move,
unsigned memory ordering, and equality comparisons. Results and assembled
candidates are written to `build/move-compare/`. The comparison candidates
remain audit fixtures until the runtime replacement task is implemented.

Formatting APIs remain upstream implementations. See
[practical limits](practical-limits.md) for why compilation support does not
imply that general formatting fits a ROM.

## Calling convention

Argument bytes go in B, C, A, then on the stack, low address first. The caller
owns the stack arguments; the callee does not pop them. All registers are
caller-saved.

Return values use A, B, C, D, E, H, L. A value that does not fit in the
registers left is returned whole in a caller-allocated return area. This adds a
hidden 16-bit pointer argument after the declared arguments and requires
`enable_multi_ret_implicit_sret`, which cg_clif enables for split returns.

The full Rust ABI, Rust data objects and tail calls remain incomplete.
