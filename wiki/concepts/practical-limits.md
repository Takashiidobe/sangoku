# Compilation support and hardware limits

Keep Rust APIs enabled when the backend can lower them. A supported type or
operation can still be too expensive for a particular ROM's size, stack or CPU
budget. Check the operation you use rather than banning a type by width.

## Formatting

Upstream integer and float formatting remain available in core. The standalone
u64 decimal probe compiles, links in a flat 32 KiB image and matches host output
on bounded inputs through `u64::MAX`. It consumes almost that entire image and
requires millions of instructions for large inputs. Treat general wide-number
formatting as an expensive dependency.

The u128, f32 and f64 decimal probes reach Rust-to-RGBDS emission, but their
code sections already exceed the current flat-image limit before adding data or
runtime helpers. Full floating-point formatting is not currently realistic in
this backend's nonbanked ROM path. Execution of those formatting paths has not
been validated. This is a code-size and integration limit, not a reason to
replace formatting implementations or hide their APIs.

The probes use upstream core's `optimize_for_size` feature and the standard
`immediate-abort` panic strategy. Neither changes the set of formatting APIs.
The feature selects smaller upstream algorithms; the panic strategy avoids
retaining panic-message formatting in these standalone measurements.

Reproduce the bounded audit explicitly:

```sh
python3 tools/audit_formatting.py
```

Results and compiler/assembler/linker logs go in `build/formatting-audit`. See
the [measured snapshot](../../tests/coverage/formatting-costs.json). Assembly
text length is not ROM size: successful links report bytes from the linker map,
while overlarge probes report the assembler's code-section size. Those code-only
figures are lower bounds for a complete program.

## Printing and output

`println!` is not supplied by this no_std ROM runtime. A program needs a target
output implementation, such as a serial or screen writer, as well as formatting
and linking support. `core::fmt` does not provide an output device.

The formatting regression calls `Display::fmt` with a bounded test sink. It does
not establish support for general `write!`, `println!`, std, or a complete
core/alloc application. Runtime imports such as `memcpy`, `memmove` and `memcmp`
still need implementations when a compiled path references them.

## Wide integers and floats

Do not classify all i64/i128 or float uses as impractical. Constants, bit
patterns, transport, selected arithmetic and conversions have bounded emulator
coverage. Shared helpers can amortize their ROM cost across many calls. Decimal
formatting pulls in substantially more machinery than loading or adding a wide
value.

Use the [lowering coverage matrix](../../tests/coverage/lower.md) to check
compilation support. Use linked size, stack use and instruction measurements to
assess a specific program; successful lowering alone does not establish that it
fits or runs fast enough on hardware.
