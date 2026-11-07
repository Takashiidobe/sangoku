# sangoku

Compile Rust into Game Boy and Game Boy Color ROMs. Rust `core` compiles; the
next goal is compiling `alloc`.

## Setup

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

For a single-file compiler package, see [distribution](wiki/distribution.md).

See the [wiki](wiki/index.md) for backend details and
[practical limits](wiki/concepts/practical-limits.md) for ROM constraints.
