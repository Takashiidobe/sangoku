# Optimizations

Use `--opt-level 2` to prefer speed or `--opt-level z` to prefer smaller ROMs.
The default is `0`; levels `1`, `2` and `3` prefer speed, while `s` and `z`
prefer size.

## Population counts

For `popcnt.i16`, levels `1`–`3` keep inline parallel counting. Other levels
call a native-width runtime helper: `0` uses the shared speed implementation,
and `s`/`z` use compact shift loops. All widths have independent size/speed
entries; speed entries share one byte counter. Lowering supports narrow and
zero-extended results with live inputs and register pressure. See
[runtime libraries](runtime-libraries.md) for ABI and measured costs.

## Wide-operation expansion

i64 division and remainder use a 64-step restoring loop. Signed operations
convert operands to unsigned magnitudes and restore the result sign. The loop
keeps the extra remainder carry needed for divisors with their top bit set.

The i64 multiply-high and checked-multiply paths reuse the four-limb multiplier
to form a full 128-bit product. Signed products correct the upper half for
negative operands; overflow compares that half with the low half's required sign
extension (or zero for unsigned multiplication).

i128 division/remainder and multiply-high use 128-step outer loops over
stack-memory byte arrays. Inner loops process one byte at a time for shifts,
carry/borrow propagation and sign correction. Division reuses the subtraction
borrow to decide whether to keep or restore the remainder. Sign counts scan
sixteen normalized bytes. These paths avoid keeping full-width intermediate
values live in registers, which otherwise causes excessive spill code. They
still expand inline rather than calling a shared runtime helper.

The i128 low-product multiplier uses a 128-step shift/add loop. Its shifts and
rotates iterate at most 127 times. These loops bound ROM use at the cost of
execution time. Generated i32 variable shifts use narrower operations and
helpers, which can produce large functions; code-size optimization remains
separate work.

See [wide integer legalization](64-bit-integer-legalization.md) for the
representation and [lowering](lowering.md) for runtime helpers.

## Runtime inclusion

Integer-only ROM builds omit both softfloat runtimes. f32 and f64 helper
references retain their respective runtimes; f64-only builds omit the f32
runtime. The f64 helper shares its algorithms across call sites, though operand
copies and stack addressing still add substantial caller code. Keep large
operation examples in separate ROMs while generated code is limited to ROM0.

## Stack-slot accesses

SM83 stack-slot accesses now omit HL saves when physical-register liveness
proves them unnecessary, including spills and reloads inserted by register
allocation. Live HL values and any A scratch value remain preserved.

`tests/fixtures/lower/stack-liveness.clif` covers the shorter assembly;
`cargo run --example check-spills` checks native bytes against RGBDS and runs
register-preservation and stack-pressure cases in the emulator.

Small non-overlapping copies on 16-bit targets interleave loads and stores to
reduce register pressure. Overlapping copies still load the complete source
before storing. i32 comparisons compose native i16 comparisons, and scalar
`select_unpredictable` intrinsics lower directly to CLIF selects; aggregate
intrinsics retain the fallback.

The [selected comparison cases](testing.md#selected-optimization-comparisons)
measured these changes together: uppercase dropped from 17,761 to 17,130 section
bytes, lowercase from 16,074 to 15,413, the iterator repro from 2,994 to 2,957,
and each search repro lost 65 bytes. These are distinct volatile-input fixtures;
they include runtime and fixture code. Sampled casing paths use 36–60 fewer
peak stack bytes, and search paths use 16 fewer. Full upstream Rust Unicode
behavior is preserved on the covered inputs.
