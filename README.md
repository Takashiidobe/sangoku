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

Select optimization with `--opt-level 2` for speed or `--opt-level z` for size:

```bash
cargo run -- build game.rs game.gb --opt-level 2
```

The default is `0` (unoptimized). Levels `1`, `2`, and `3` prefer speed; `s` and
`z` prefer smaller code. For 16-bit population counts, speed builds use parallel
counting, while unoptimized and size builds use a compact shift loop.

The example ROM calls `gbc_main` and then idles. It does not provide graphics,
input, or audio yet.

## Current focus

The backend can compile small `no_core` functions and link a minimal ROM. Work
is focused on supporting the operations, intrinsics, data, and relocations that
Rust `core` needs, then compiling `core` to assembly.

The runtime supports wrapping 16-bit multiplication for signed and unsigned
operands, and unsigned 16-bit division/remainder. Division and remainder share
an SM83 core and reach the trap vector for a zero divisor. With RGBDS installed,
run the assembly execution tests with `cargo test`.

Use `Emulator` from `sangoku_test_support::emu` for runtime tests. It executes a
subset of SM83 instructions against flat 64 KiB memory. Set byte registers and
memory directly; use `set_sp()` and `set_hl()` for SP and HL. `step()` returns
one instruction's cost and `run_until(address, max_instructions)` returns the
cost to reach an address. Costs are CPU clock cycles (T-cycles), accumulated in
`cycles`. Unsupported instructions return an error.

To compare helper and caller costs, ROM sizes, stack use, and scratch RAM, see
[runtime audit measurements](wiki/runtime-audits.md).

The [division audit](wiki/audits/division16.md) records the selected 95-byte
implementation and its cycle, ROM, and stack costs.

Spills and reloads preserve live registers and flags; the
[spill regression check](tests/bin/check_spills.rs) executes generated compiler
output. Native `popcnt.i16` lowering supports narrow and zero-extended results
with live inputs and register pressure. The
[popcount audit](wiki/audits/popcount16.md) records the algorithms and current
compiler measurements. Run
`cargo run --example check-popcount -- --exhaustive16` to validate the
replacement at all six optimization levels.

The backend supports a limited set of LR35902 operations and does not yet
support the full Rust ABI, indirect or tail calls, stack return values, or Rust
data objects. Current ASCII-only changes to `core` alter string and character
behavior; correcting them is part of the compiler work below.

## Split the compiler into five phases

Make each phase independently runnable, with saved inputs and outputs. These are
development priorities; the current ROM command still runs the stages together.

1. **Rust frontend:** use the pinned rustc fork to check Rust and produce MIR.
2. **CLIF generation:** use `rustc_codegen_cranelift` to translate MIR and ABI
   information into Cranelift IR.
3. **LR35902 lowering:** lower CLIF, allocate registers, and construct target
   instructions. Test this phase directly with small CLIF fixtures.
4. **Assembly and objects:** emit structured RGBDS assembly with explicit
   symbols, labels, data, and relocations, then assemble it with `rgbasm`.
   Replace the current formatted-VCode rewriting with a dedicated emitter.
5. **Link and execute:** link objects with startup code and runtime helpers, fix
   the ROM header, and check results in an emulator.

## Shorten the testing loop

- Preserve failed build artifacts. `TempDir::drop` currently deletes assembly
  and objects on failure; retain them with the command, stderr, CLIF, and fork
  revisions so the failure can be reproduced.
- Separate backend rebuilding from fixture reruns. Support a development backend
  build and reuse it until its sources change; reserve full ROM builds for
  integration checks.
- Reduce each `core` blocker to a regression fixture. Use CLIF for lowering
  failures and Rust for ABI, layout, and intrinsic failures. Check execution
  results for arithmetic, spills, and calls as well as successful compilation.
- Record each `core` attempt in an LLog under `wiki/log/`: command, revisions,
  failing function and operation, artifact paths, change tried, and outcome.
  Link the regression fixture and Beads issue. Distinguish type checking,
  assembly generation, assembly, linking, and execution; list omitted functions
  so a partial build cannot be mistaken for full `core` support.
- Make unavailable RGBDS tools visible in integration checks instead of silently
  passing tests that did not run.

## Preserve Rust string behavior

Restore UTF-8 decoding and consistent `str`/`char` behavior in the Rust fork.
The current target-specific `next_code_point` reads one byte per character
without rejecting non-ASCII input, so valid non-ASCII strings can be decoded
incorrectly. Keep ASCII restrictions in game text types or asset conversion. Add
regression fixtures for multibyte strings, character boundaries, and
forward/reverse iteration. Until corrected, treat the ASCII changes as a
temporary semantic limitation of this fork.
