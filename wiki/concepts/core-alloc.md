# Core and alloc support

Builds use `core` by default, including `#![no_core]` fixtures. Pass `--alloc`
to `build`, `run`, `frontend`, or `clif` to enable `alloc` and the heap crate:

```bash
cargo run -- build examples/heap.rs build/heap.gb --alloc --opt-level z
```

Program fixtures live in `tests/fixtures/core/programs`; allocator fixtures live
in `tests/fixtures/alloc` and require `--alloc`.

Run `python3 tools/core_report.py --alloc --json` to compile the libraries and
representative generic drivers to CLIF and probe LR35902 lowering. Use
`--keep build/alloc-report` to retain CLIF and alloc compiler diagnostics. The
[intrinsic audit](../intrinsics.md) records source-level intrinsic call routes
and candidates for compact shared libcalls. Its broader matrix runs only in the
extensive profile.

The regression check is
`cargo nextest run --profile extensive --no-fail-fast --test alloc_report`.

The 2026-11-07 survey at opt-level 0 with immediate-abort panic reports:

| Coverage                | Functions lowered | Failures |
| ----------------------- | ----------------: | -------: |
| core                    |             3,184 |        0 |
| compiler_builtins       |             2,043 |        0 |
| core generic driver     |               921 |        0 |
| alloc                   |             1,137 |        0 |
| alloc collection driver |             1,238 |        0 |

ROM builds include the 16-bit and 32-bit arithmetic runtime objects only when
retained code references their helpers. `benchmarks/empty.rs` excludes these
runtime objects; `tests/core_rom/runtime32.rs` checks dynamic 32-bit division
and remainder in the emulator.

`wide32.asm` remains a compatibility include for assembly harnesses. ROM builds
select individual multiplication, shift and bit-operation helpers. Division and
remainder wrappers share `div32_core.asm`, selected through their symbol
references.

The default compiler options combine immediate-abort panics with MIR level 2 and
stripped MIR local debug information at every optimization level. This removes
dead formatter references before code generation. `examples/sort.rs` uses
upstream `sort_unstable` and fits at all six levels. The library cache rebuilds
when compiler options change.

No alloc compilation blockers were observed in this coverage
(`sangoku-30k.9.2`). The driver instantiates `Vec`, UTF-8 `String`, `Box`, `Rc`,
`BTreeMap`, `BTreeSet`, `VecDeque`, `BinaryHeap`, and `LinkedList`; other
generic instantiations can still expose gaps. Long per-function CLIF filename
warnings remain tracked in `sangoku-30k.9.1.11`.

Target ROM tests in `tests/core_link.rs` execute UTF-8 iteration across every
width boundary, alphabetic/whitespace/numeric properties, and `str` character
counting and prefix/suffix checks with non-ASCII text. At opt-level z, separate
`unicode_case.rs` and `unicode_upper.rs` ROMs execute upstream case mappings for
Ä, ß, ǅ, and emoji, including the two-character uppercase expansion ß → SS
(`sangoku-6o2`). Whole-byte logical right-shift lowering (`sangoku-dr7`) reduced
the combined fixture from 32,864 to 32,715 bytes, leaving only 53 bytes of section
capacity. Larger combinations still need size reductions or banking
(`sangoku-30k.9.1.12`).

`cargo run -- run examples/heap.rs --alloc` executes `Vec` and `Box` allocations
in WRAM and returns 52, including at opt-level 0. The collection survey above
checks compilation and lowering; it does not link or execute every collection.
