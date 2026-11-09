# Runtime libraries

Runtime helpers link only when generated assembly references their exported
symbols. Run `cargo run -- build examples/hello.rs build/hello.gb` to check
assembly and ROM linkage after changing a helper.

## Choose a helper

Use `runtime/<family>/<operation>/<variant>/<width>.asm` for integer,
bitshift and memory helpers. Families are `integer`, `bitshift`, `float`
and `memory`. Widths use `i8`, `i16`, `i32`, or `pointer` for memory routines.

- `size`: selected at optimization levels `s` and `z`.
- `speed`: selected at levels `0`, `1`, `2`, and `3`.
- `common`: selected at every level when no separate algorithms exist.

Register each linkable file in its family dispatcher under `src/runtime/`.
The registry supplies both the assembly path and the source used to resolve
transitive helper dependencies. Keep one operation per object; place shared
cores in separate objects. Do not include a family aggregate in a registered
object: RGBDS retains everything in that object.

Use `memcpy` as the reference for separate size and speed implementations.
Existing ABIs and supported widths are unchanged. Operations without native
helpers still use the backend's inline lowering or wider implementations;
adding an empty width file does not add support.

## Follow the calling convention

Public helpers use the backend ABI. Integer bytes are ordered least significant
first. Arguments consume `B`, `C`, `A` across arguments, then stack bytes;
at helper entry the first stacked byte is at `SP+2`, above the return address.
The caller cleans up stack arguments. The stack alignment is two bytes.

| Single argument | Input | Return |
| --- | --- | --- |
| i8 | B | A |
| i16 | B, C | A, B |
| i32 | B, C, A, [SP+2] | A, B, C, D |

All general registers and flags are caller-saved. Restore SP before returning;
do not remove incoming arguments. Pointer values use the i16 convention.
Population-count results zero their upper return bytes. The Rust methods
return u32, so callers extend native i8/i16 helper results as needed.

Private shared routines may use narrower contracts. `__gbc_popcnt_byte` takes
and returns A, clobbers H/L and flags, and preserves B/C/D/E for its wrappers.
It uses no table or scratch RAM. Only speed wrappers reference it, and linking
several widths retains one copy.

| Popcount width | Size ROM / cycles | Speed ROM / cycles |
| --- | --- | --- |
| i8 | 11 / 280 | 28 / 128 |
| i16 | 15 / 672 | 37 / 304 |
| i32 | 25 / 1856 | 55 / 624 |

ROM counts include the shared core for a single speed entry. All three speed
entries together occupy 72 bytes. Cycles cover entry through RET; an outer
CALL adds 24 cycles. Speed i16/i32 wrappers use two additional stack bytes for
nested calls; speed i8 tail-jumps to the byte core. Compact entries allocate no
stack. Caller argument packing and spills add separate costs.

Sharing replaces the previous unrolled i32 speed helper's 112 bytes and 468
cycles with 55 bytes and 624 cycles. Optimized non-size i16 lowering keeps its
inline parallel sequence; levels 0/s/z use the native runtime entry.

Leading-zero counts already link through `__gbc_clz32`. Narrow widths widen
and subtract the leading padding count. The register-shift CLZ candidate is
validated and used; it needs no additional wiring. Original pointer and table
popcount proposals are retained under `tests/fixtures/popcount/candidates/`.
They use different ABIs; the table proposal clobbers the pointer loop's HL and
its 256-byte placement constraint can leave gaps. At a fixed `$400` entry,
explicitly padding its table produces a 272-byte object; a floating section
could instead satisfy the alignment through placement.

## Share implementation code

Unsigned 16-bit division and remainder have separate entry objects and share
`integer/div/common/i16_core.asm`. Selecting either entry retains the core
without retaining the other entry.

The f32 runtime has separate operation objects and shared helpers under
`float/shared/common/f32/`. Keep fallthrough entry labels in the same file.
Use absolute jumps between files because the linker can place their sections
farther apart than a relative jump permits. Shared macros live in a guarded
include; the registry also includes their source for dependency discovery.
The f32 scratch state remains shared and non-reentrant.

`float/all_f32.asm` and `integer/all.asm` assemble aggregates for tests.
Production linkage uses individual registered objects. The f64/f128 and wide
conversion runtimes retain their existing operation-code dispatchers; their
internal operations are still bundled within each dispatcher object.

## Replace a compiler builtin

`memcmp` exports `__rust_memcmp`, the symbol used by both the compiler libcall
and named Rust call paths. Production linkage excludes that definition from
compiler-builtins before selecting reachable sections. This also removes its
otherwise-unused dependencies; it prevents duplicate definitions when the
runtime variant is added. Program-provided definitions remain intact.

`memory/memcmp/size/pointer.asm` is 48 bytes; the speed variant is 89 bytes.
Levels s/z choose size and 0–3 choose speed. Both compare unsigned bytes,
return a signed i32 ordering result, preserve caller-owned stack arguments,
and accept zero lengths without reading either buffer. Small equality loads
remain inline. Use `cargo nextest run --no-fail-fast -E 'binary(memcmp)'`
for bounded ABI and no_core Rust/CLIF call tests.

The move/compare audit uses the promoted runtime files and the cached builtins
assembly as a baseline. It builds the baseline only if that cache is missing.
Run `cargo run --example audit-move-compare` to reproduce its comparison.

## Check changes

For popcount changes, rebuild the backend once using the documented toolchain
build, then run `cargo nextest run --no-fail-fast -E 'binary(popcount32)'`.
This compiles small `no_core` Rust probes at 0/2/z and links only the relevant
helpers; it does not build core or a complete application ROM.

Run `cargo nextest run --no-fail-fast` for the default suite. Linkage tests check that f32 addition
excludes other float operations and that 16-bit quotient and remainder select
separate wrappers. Assembly tests cover existing arithmetic behavior.

Generated float sources remain beside their assembly files. Regenerate them
with `tools/generate_softfloat64.py --width 64`, `--width 128`, or
`tools/generate_convert.py`. Distribution packaging copies nested runtime
assembly and includes into the bundle.
