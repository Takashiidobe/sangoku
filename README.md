# sangoku

sangoku compiles Rust code to LR35902 assembly and links it into a Game Boy or
Game Boy Color ROM. The current development goal is compiling Rust `core` for
the target.

## Build the toolchain

The backend uses private rustc APIs, so build it with the repository's Rust fork
rather than a stock nightly:

```bash
(cd third_party/rust && ./x.py build --stage 2 compiler library)
rustup toolchain link gbc-lr35902 third_party/rust/build/x86_64-unknown-linux-gnu/stage2
cargo +gbc-lr35902 build --release --manifest-path third_party/rustc_codegen_cranelift/Cargo.toml
```

Install RGBDS (`rgbasm`, `rgblink`, and `rgbfix`) to assemble and link ROMs.

## Build a ROM

Build the blank example ROM:

```bash
cargo run -- example
```

Build a Rust source file into a ROM:

```bash
cargo run -- build game.rs game.gb
```

Run only the Rust frontend, saving MIR to `build/game.mir`:

```bash
cargo run -- frontend game.rs
```

Run only CLIF generation, saving Cranelift IR (with ABI and MIR comments) to
`build/game.clif`. The backend runs inside rustc, so this takes the Rust source
rather than saved MIR, and stops before LR35902 lowering:

```bash
cargo run -- clif game.rs
```

Select optimization with `--opt-level 2` for speed or `--opt-level z` for size:

```bash
cargo run -- build game.rs game.gb --opt-level 2
```

The default is `0` (unoptimized). Levels `1`, `2`, and `3` prefer speed; `s` and
`z` prefer smaller code.

The example ROM calls `gbc_main` and then idles. It does not provide graphics,
input, or audio yet.

## Goals and status

The backend compiles small `no_core` functions and links minimal ROMs. The next
milestone is compiling Rust `core`, before adding runtime and game features. The
full Rust ABI and operation set are still incomplete.

Numeric conversions involving i64/i128 and f64/f128 now lower through a shared
conversion runtime, including trapping and saturating float-to-integer casts.
Float-width conversions preserve source precision and round once at the
destination width. Build `examples/conversions.rs` to exercise Rust casts;
compiling all of `core` remains the next milestone.

The Rust fork keeps `library/core` unchanged from its upstream base. Target
limitations belong in backend work rather than disabled core APIs; see the
[Rust fork audit](wiki/concepts/rust-fork.md) for the remaining compiler
accommodation and validation limits.

Rust lowers through Cranelift IR to LR35902 assembly. See
[legalization](wiki/concepts/legalization.md),
[lowering](wiki/concepts/lowering.md), and
[optimizations](wiki/concepts/optimizations.md) for the representation and
lowering rules. The [coverage matrix](tests/coverage/lower.md) records supported
operations; [Beads](AGENTS.md) tracks remaining work.

## Testing

Run the tests with RGBDS and Prettier installed:

```bash
cargo nextest run --no-fail-fast
```

Run the Rust-to-ROM smoke check after rebuilding an affected toolchain
component:

```bash
cargo run -- build examples/blank.rs build/blank.gb
```

FileCheck fixtures:

- Each phase has a directory under `tests/fixtures/`. Fixture files carry
  expected output as `// CHECK:` lines for LLVM `FileCheck`.
- Run a phase independently: `cargo run -- frontend file.rs` for MIR,
  `cargo run -- clif file.rs` for CLIF, or `cargo run -- lower file.clif` for
  RGBDS assembly without rustc. Use `-` as the output path to write to stdout.
- Lowering fixtures in `tests/fixtures/lower` contain hand-written or saved
  CLIF.
- To add a fixture, put a source file in the phase's directory, run
  `tools/update_filecheck.py [glob]`, and review the generated CHECK lines.
- Verify expectations with `tools/update_filecheck.py --check`. The test suite
  also runs this check.
- Reduce each `core` blocker to a fixture for the phase where it fails.

Runtime tests:

- Use `Emulator` from `sangoku_test_support::emu`. It executes a subset of SM83
  instructions against flat 64 KiB memory.
- Set byte registers and memory directly. Use `set_sp()` and `set_hl()` for SP
  and HL.
- `step()` executes one instruction and returns its cost.
- `run_until(address, max_instructions)` returns the cost to reach an address.
- Costs are CPU clock cycles (T-cycles), accumulated in `cycles`.
- Unsupported instructions return an error.
