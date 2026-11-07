# Optimizations

Use `--opt-level 2` to prefer speed or `--opt-level z` to prefer smaller ROMs.
The default is `0`; levels `1`, `2` and `3` prefer speed, while `s` and `z`
prefer size.

## Population counts

For `popcnt.i16`, speed builds use parallel counting. Unoptimized and size
builds use a compact shift loop. Lowering supports narrow and zero-extended
results with live inputs and register pressure.

## Wide-operation expansion

i64 division and remainder use a 64-step restoring loop. Signed operations
convert operands to unsigned magnitudes and restore the result sign. The loop
keeps the extra remainder carry needed for divisors with their top bit set.

The i64 multiply-high and checked-multiply paths reuse the four-limb multiplier
to form a full 128-bit product. Signed products correct the upper half for
negative operands; overflow compares that half with the low half's required sign
extension (or zero for unsigned multiplication).

The i128 multiplier uses a 128-step shift/add loop. Its shifts and rotates
iterate at most 127 times. These loops bound ROM use at the cost of execution
time. Generated i32 variable shifts use narrower operations and helpers, which
can produce large functions; code-size optimization remains separate work.

See [wide integer legalization](64-bit-integer-legalization.md) for the
representation and [lowering](lowering.md) for runtime helpers.

## Runtime inclusion

Integer-only ROM builds omit the softfloat runtime. Floating helper references
retain it. Keep large operation examples in separate ROMs while generated code
is limited to ROM0.
