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

## Check changes

Run `cargo nextest run --no-fail-fast`. Linkage tests check that f32 addition
excludes other float operations and that 16-bit quotient and remainder select
separate wrappers. Assembly tests cover existing arithmetic behavior.

Generated float sources remain beside their assembly files. Regenerate them
with `tools/generate_softfloat64.py --width 64`, `--width 128`, or
`tools/generate_convert.py`. Distribution packaging copies nested runtime
assembly and includes into the bundle.
