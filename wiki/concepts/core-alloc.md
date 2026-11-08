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
width boundary and the alphabetic, whitespace and numeric properties. Upstream
Unicode support is kept for correctness but costs about 47 KB at opt-level z
(about 42 KB lookup code, 11 KB tables); one `char::to_lowercase` needs a 40 KB
ROM, so case conversion cannot run on a 32 KiB cartridge yet (`sangoku-6o2`).
Games should use an ASCII string crate instead (`sangoku-yjb`).

`cargo run -- run examples/heap.rs --alloc` executes `Vec` and `Box` allocations
in WRAM and returns 52, including at opt-level 0. The collection survey above
checks compilation and lowering; it does not link or execute every collection.
