# sangoku

Compile Rust into Game Boy and Game Boy Color ROMs. Rust `core` and representative
`alloc` collections compile and lower for LR35902. Sorting and heap examples
run in the emulator.

## Standalone compiler

With the bundled Linux x86-64 executable and RGBDS installed on `PATH`:

```bash
./sangoku run game.rs
./sangoku build game.rs game.gb --opt-level z
```

No Cargo or rustup is needed. The toolchain unpacks into a cache on first use.
To build `build/dist/sangoku`, see [distribution](wiki/distribution.md).

## Build from source

Install RGBDS (`rgbasm`, `rgblink`, `rgbfix`), then build the pinned Rust fork
and backend:

```bash
./tools/rebuild_toolchain.sh
```

## Usage

Build and run an example in the built-in emulator:

```bash
cargo run -- run examples/hello.rs
```

Build a ROM:

```bash
cargo run -- build examples/sort.rs build/sort.gb
```

Use the [examples](examples/) as a starting point. Add `--opt-level z` to
optimize for size. Build artifacts stay in `build/artifacts/` for debugging.

All optimization levels eliminate panic-only formatting. Failed checks still
trap, and panic argument effects are preserved. Explicit formatting and
Unicode tables retain their actual ROM costs. Check the build warnings.

For serial output, use the supplied `sangoku_console` macros:

```rust
#![no_std]

use sangoku_console::println;

#[unsafe(no_mangle)]
pub extern "C" fn gbc_main() -> u8 {
    println!("answer = {}", 42u16);
    0
}
```

The console uses `ufmt` without `core::fmt`. For explicit writers, use
`sangoku_console::{Serial, finish, write, writeln}` and pass the result to
`finish`. Failed checks trap without panic diagnostics.

Inputs are standalone Rust 2024 source files. Sangoku supplies `ufmt` and
`sangoku_console`; it does not yet resolve dependencies from your own
`Cargo.toml` or build arbitrary Cargo projects.

## Tests

```bash
cargo nextest run --no-fail-fast
```

The default profile keeps FileCheck and fast regression tests. Tests measured
above 10 seconds, except FileCheck, belong in the extensive profile configured
in [`.config/nextest.toml`](.config/nextest.toml). Run the full suite, including
the core/alloc reports, coverage matrix, panic-profile matrix, and broader
numeric checks, with:

```bash
cargo nextest run --profile extensive --no-fail-fast
```

See the [wiki](wiki/index.md) for backend details and
[practical limits](wiki/concepts/practical-limits.md) for ROM constraints.

Frontend regression coverage, including f16 arithmetic and shared 64/128-bit
overflowing multiplication, is described in
[frontend fixtures](wiki/concepts/frontend-fixtures.md).

## Core and alloc roadmap

Builds use `core` by default, including `#![no_core]` fixtures. Pass `--alloc`
to `build`, `run`, `frontend`, or `clif` to enable `alloc` and the heap crate:

```bash
cargo run -- build examples/heap.rs build/heap.gb --alloc --opt-level z
```

Program fixtures live in `tests/fixtures/core/programs`; allocator fixtures live
in `tests/fixtures/alloc` and require `--alloc`.

Run `python3 tools/core_report.py --alloc --json` to compile the libraries and
representative generic drivers to CLIF and probe LR35902 lowering. Use
`--keep build/alloc-report` to retain CLIF and alloc compiler diagnostics.
The regression check is
`cargo nextest run --profile extensive --no-fail-fast --test alloc_report`.

The 2026-11-07 survey at opt-level 0 with immediate-abort panic reports:

| Coverage | Functions lowered | Failures |
| --- | ---: | ---: |
| core | 3,184 | 0 |
| compiler_builtins | 2,043 | 0 |
| core generic driver | 921 | 0 |
| alloc | 1,137 | 0 |
| alloc collection driver | 1,238 | 0 |

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

SM83 stack-slot accesses now omit HL saves when physical-register liveness
proves them unnecessary, including spills and reloads inserted by register
allocation. Live HL values and any A scratch value remain preserved.

`tests/fixtures/lower/stack-liveness.clif` covers the shorter assembly;
`cargo run --example check-spills` checks native bytes against RGBDS and runs
register-preservation and stack-pressure cases in the emulator.

No alloc compilation blockers were observed in this coverage
(`sangoku-30k.9.2`). The driver instantiates `Vec`, UTF-8 `String`, `Box`, `Rc`,
`BTreeMap`, `BTreeSet`, `VecDeque`, `BinaryHeap`, and `LinkedList`; other generic
instantiations can still expose gaps. Long per-function CLIF filename warnings
remain tracked in `sangoku-30k.9.1.11`.

Target ROM tests in `tests/core_link.rs` execute UTF-8 iteration across every
width boundary and the alphabetic, whitespace and numeric properties. Upstream
Unicode support is kept for correctness but costs about 47 KB at opt-level z
(about 42 KB lookup code, 11 KB tables); one `char::to_lowercase` needs a 40 KB
ROM, so case conversion cannot run on a 32 KiB cartridge yet (`sangoku-6o2`).
Games should use an ASCII string crate instead (`sangoku-yjb`).

`cargo run -- run examples/heap.rs --alloc` executes `Vec` and `Box`
allocations in WRAM and returns 52, including at opt-level 0. The collection
survey above checks compilation and lowering; it does not link or execute
every collection.
