# sangoku

Compile Rust into Game Boy and Game Boy Color ROMs. Rust `core` and representative
`alloc` collections compile and lower for LR35902; heap execution is pending.

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

Avoid methods that pull in panic or formatting code, even indirectly: unused
failure paths can consume substantial ROM space. Check the build warnings.

## Tests

```bash
cargo nextest run --no-fail-fast
```

See the [wiki](wiki/index.md) for backend details and
[practical limits](wiki/concepts/practical-limits.md) for ROM constraints.

Frontend regression coverage, including f16 arithmetic and shared 64/128-bit
overflowing multiplication, is described in
[frontend fixtures](wiki/concepts/frontend-fixtures.md).

## Core and alloc roadmap

Run `python3 tools/core_report.py --alloc --json` to compile the libraries and
representative generic drivers to CLIF and probe LR35902 lowering. Use
`--keep build/alloc-report` to retain CLIF and alloc compiler diagnostics.
The regression check is `cargo nextest run --no-fail-fast --test alloc_report`.

The 2026-11-07 survey at opt-level 0 with immediate-abort panic reports:

| Coverage | Functions lowered | Failures |
| --- | ---: | ---: |
| core | 3,184 | 0 |
| compiler_builtins | 2,043 | 0 |
| core generic driver | 921 | 0 |
| alloc | 1,137 | 0 |
| alloc collection driver | 1,238 | 0 |

No alloc compilation blockers were observed in this coverage
(`sangoku-30k.9.2`). The driver instantiates `Vec`, UTF-8 `String`, `Box`, `Rc`,
`BTreeMap`, `BTreeSet`, `VecDeque`, `BinaryHeap`, and `LinkedList`; other generic
instantiations can still expose gaps. Long per-function CLIF filename warnings
remain tracked in `sangoku-30k.9.1.11`.

Heap ROM execution still needs allocator hooks and a WRAM heap policy
(`sangoku-30k.9.3`). This survey checks compilation and lowering, without linking or running collections.
