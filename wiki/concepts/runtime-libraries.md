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

Leading-zero, trailing-zero and bit-reversal operations have native-width
size/speed helpers; see the bit-operation section below.
Original pointer and table
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

## Multiply integers

Wrapping multiplication uses `__gbc_mul8`, `__gbc_mul16` and `__gbc_mul32` with
the public byte ABI. Signed and unsigned inputs share the same low-bit result.
Cranelift selects the native entry; no compiler-builtins override is needed.

| Width | Size ROM bytes | Speed ROM bytes | Size / speed cycles, max × max |
| --- | --- | --- | --- |
| i8 | 13 | 50 | 344 / 212 |
| i16 | 30 | 171 | 912 / 672 |
| i32 | 108 | 236 | 12060 / 10208 |

Size builds use compact loops. Speed builds unroll all i8/i16 steps and four
i32 steps per iteration. Shared assembly includes keep the step logic together;
no additional runtime objects or tables are linked. Only one width/variant
is selected for each referenced entry. All general registers and flags may be
clobbered; i32 allocates eight stack bytes, and i8/i16 allocate none.

Cycles cover helper entry through RET; argument packing, CALL and caller spills
add separate costs. i16 retains the shortcut for an eight-bit right operand.
i32 unrolling can be slower for tiny operands: 1 × 1 takes 776 size cycles or
1424 speed cycles. The table is a representative cost comparison, not an
exhaustive performance claim.

Run `cargo nextest run --no-fail-fast -E 'binary(multiply)'` after rebuilding
the affected backend. Tests use eight explicit operand pairs per width and
small no_core Rust probes at 0/z for signed results and live inputs. They do
not build core or run an exhaustive audit. The legacy runtime audit's `mul16`
name retains the compact implementation for historical measurement stability.

## Widen multiplication

`__gbc_umul16wide` and `__gbc_smul16wide` multiply two native 16-bit arguments
and return the full 32-bit product. Inputs use B/C and A/[SP+2]; output bytes
use A/B/C/D, least significant first. Incoming stack arguments stay unchanged.

| Shape | Size / speed linked bytes | Size / speed cycles, 0xffff × 0xffff | Extra stack |
| --- | --- | --- | --- |
| unsigned 16×16→32 | 32 / 252 | 1292 / 1072 | 0 |
| signed 16×16→32 | 93 / 313 | 1688 / 1468 | 8 |

The unsigned kernel uses 16 combined add/right-shift steps; size loops and
speed unrolls them and returns early for a zero operand. A common signed wrapper calls the selected unsigned kernel,
then subtracts the other operand from the high half for each negative input.
Both signed and unsigned callers retain one unsigned kernel. There are no
lookup tables or scratch RAM; all public registers and flags are caller-saved.
Cycle counts cover entry through RET, excluding caller packing/CALL/spills.

Cranelift uses the helpers for signed/unsigned 16-bit high halves and checked
overflow, and i32 multiplies of two matching sign- or zero-extended i16 values.
Masked low limbs and logical shifts exposing at most 16 bits also qualify.
This lets the existing 32-bit high-half/overflow expansion reuse native
widening products. Mixed signed/unsigned extensions keep the generic i32 path.
Eight-bit high halves/overflow keep their existing i16-product path. General
low-32-bit wrapping multiplication remains separate; no new >32-bit kernel is
introduced.

Run `cargo nextest run --no-fail-fast -E 'binary(widen_multiply)'` after the
backend and standalone CLIF tool are rebuilt. Tests use nine explicit direct
pairs, tiny no_core Rust at 0/z, and six representative cases per width for
8/16/32-bit high halves and overflow flags. This does not require core or a
complete application ROM.

## Scan and reverse bits

CLZ, CTZ and bit reversal have native i8/i16/i32 entries selected by Cranelift.
They follow the public byte ABI and preserve zero semantics: CLZ/CTZ of zero
returns the operand width. Narrow operations no longer widen to i32, insert a
CTZ marker bit, adjust the CLZ count or extract reversed high bytes.

| Operation | Width | Size / speed linked bytes | Size / speed zero cycles |
| --- | --- | --- | --- |
| CLZ or CTZ | i8 | 11 / 52 | 336 / 44 |
| CLZ or CTZ | i16 | 15 / 69 | 792 / 72 |
| CLZ or CTZ | i32 | 25 / 99 | 2104 / 144 |
| bit reversal | i8 | 15 / 29 | 272 / 132 |
| bit reversal | i16 | 20 / 34 | 556 / 276 |
| bit reversal | i32 | 38 / 52 | 1184 / 624 |

Linked bytes include required shared helpers. Speed scans skip empty bytes and
reuse a private unrolled byte scanner. Its `__gbc_clz_byte` or
`__gbc_ctz_byte` entry takes a nonzero byte in A and returns its count in A.
The first byte's fast path uses a tail jump. Private kernels are separate
objects, so a wider-only caller does not retain the public i8 wrapper. Compact scans use bounded whole-word shift loops. Speed scans can
be slower for short counts: 32-bit CLZ of the top bit costs 108 cycles versus
104 compact cycles; CTZ of one has the same costs.

Bit reversal shares one A-to-A byte kernel across widths. It preserves B/C/D/E
and clobbers H/L and flags. Size uses a rotate loop; speed swaps nibbles and
masked bit pairs without a table. Reversal timing is input-independent. All
costs cover helper entry through RET, excluding caller packing, CALL and spills.
Nested byte calls use at most two additional stack bytes. No scratch RAM or
lookup tables are used. Only required variant and byte dependencies are linked.

Run `cargo nextest run --no-fail-fast -E 'binary(bitops)'` after rebuilding the
backend. Tests cover zero, all ones, every single-bit position and a few mixed
patterns; no_core Rust probes at 0/z check signed results, zero extension and
live inputs. They do not build core or perform an exhaustive byte sweep.

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
