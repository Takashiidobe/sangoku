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

Build the example ROM:

```bash
cargo run -- example
```

Build a Rust source file into a ROM:

```bash
cargo run -- build game.rs game.gb
```

Builds print an artifact directory under `build/artifacts/` and retain assembly,
objects, and linker probes even on failure. Successful ROM links also write
`game.map` and `game.sym` there. Remove old directories when no longer needed.

`build` and `run` warn when retained code includes general or numeric formatting,
showing a dependency path and assembled size. Oversized ROMs report the largest
sections and the probe map location. The check inspects emitted dependencies;
it does not prove whether a panic can occur.

Build a source file and run it in the built-in emulator. Programs print by
writing to the serial port (`$FF01`/`$FF02`), and `run` shows that output on
stdout (`examples/` has `hello.rs`, `primes.rs` and `sort.rs`):

```bash
cargo run -- run examples/hello.rs
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

Target builds always use `panic=immediate-abort`, including `core` and compiler
builtins. A failed safety check traps without calling a panic handler.

The default is `0` (unoptimized). Levels `1`, `2`, and `3` prefer speed; `s` and
`z` prefer smaller code.

The example ROM calls `gbc_main` and then idles. It does not provide graphics,
input, or audio yet. Startup places the stack at `$E000`, growing downward into
work RAM; high RAM is too small for unoptimized `core` iterator call chains.

## Goals and status

The backend compiles small `no_core` functions and links minimal ROMs. The next
milestone is compiling Rust `core`, before adding runtime and game features. The
full Rust ABI and operation set are still incomplete.

Numeric conversions involving i64/i128 and f64/f128 now lower through a shared
conversion runtime, including trapping and saturating float-to-integer casts.
Float-width conversions preserve source precision and round once at the
destination width. Build `tests/fixtures/programs/conversions.rs` to exercise Rust casts;
compiling all of `core` remains the next milestone.

Core compilation status (`tools/core_report.py`, also the nextest test
`core_lowers_to_lr35902_without_failures`): compiled to CLIF and lowered one
function at a time, 0 failures out of 3,235 `core` functions, the
`compiler_builtins` crate and 771 functions instantiated by
`tests/core_driver/driver.rs` (iterators, slices, `str`, `char`, formatting
through `u128`, parsing, sorting, hashing, `dyn`). These are lowering counts at
`opt-level=0`, with `compiler_builtins` built with its `unmangled-names` and
`mem` features (2,009 functions). `cargo run -- build PROGRAM.rs OUT.gb` links every program against real `core`
(built once into `build/core/immediate-abort/opt-N`): each function is its own RGBDS section, only
sections reachable from `gbc_main` are kept, and they fill ROM0 then spill into
ROMX bank 1 of a 32 KiB cartridge (`tests/core_rom/rom.rs`, an iterator sum and
max, links at about 16 KB and runs in the emulator). Programs larger than 32 KiB
need MBC banking with cross-bank calls, which is not built. See the
`sangoku-30k.9.1` children.

Avoid methods that can call panic or formatting code, including indirectly.
They can pull substantial code into the ROM even when those paths never run.
Prefer fallible accessors and direct serial output, and check build warnings.

Immediate abort removes the ordinary panic-reporting path, but unoptimized
slice-range failure helpers can still retain formatting arguments. The sort
example uses a small bubble sort over mutable iterators to avoid these helpers.

Compilation support does not guarantee a practical ROM footprint. General float
and wide-integer formatting are currently too large for normal use in the
nonbanked build; even the standalone u64 decimal probe nearly fills 32 KiB. This
does not make all i64/i128 arithmetic unusable. `println!` also needs an output
runtime that is not provided. See
[practical limits](wiki/concepts/practical-limits.md) for measurements and the
distinction between lowering, linking and execution.

The Rust fork is a small patch series (`patches/rust/`, managed by `tools/rust_fork.sh`) on an upstream base, and keeps `library/core` unchanged. Target
limitations belong in backend work rather than disabled core APIs; see the
[Rust fork audit](wiki/concepts/rust-fork.md) for the remaining compiler
accommodation and validation limits.

The [16-bit pointer audit](wiki/concepts/16-bit-pointers.md) verifies core/alloc
frontend support and size boundaries without additional Rust-fork changes.
`usize::MAX` is 65,535, while ordinary objects and allocations are limited to
32,767 bytes before alignment; these limits do not promise available RAM.

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
cargo run -- build examples/hello.rs build/hello.gb
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
