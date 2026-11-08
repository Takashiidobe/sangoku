# Compilation support and hardware limits

Keep Rust APIs enabled when the backend can lower them. A supported type or
operation can still be too expensive for a particular ROM's size, stack or CPU
budget. Check the operation you use rather than banning a type by width.

## Panics

Every optimization level uses `-Cpanic=immediate-abort`, `-Zmir-opt-level=2` and
`-Zmir-strip-debuginfo=all-locals`. MIR local debug records can otherwise keep
dead formatter arguments reachable, even when the panic itself becomes a trap.
These defaults apply to programs, core and alloc.

Failed bounds checks, assertions, unwraps and formatted panics still trap.
Argument side effects still execute, and constant panics remain compilation
errors. Sorting can use upstream `sort_unstable` without retaining panic-only
formatting. Explicit formatting remains supported and retains its own code.

Run the optimization-level matrix explicitly:

```sh
cargo nextest run --profile extensive --no-fail-fast --test panic_profiles
```

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

Use `sangoku_console::{print, println}` for serial output with `ufmt`, or
implement `uWrite` for your output device. See [building ROMs](building-roms.md)
for the serial and screen examples. These writers avoid `core::fmt`; the
standard library's `println!` is not available in a `no_std` ROM.

The core formatting probes use bounded sinks and serial output. They do not
establish support for every formatting type or `std`. The runtime supplies
`memcpy` and `memmove`; `memcmp` still needs an implementation when a compiled
path references it.

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
