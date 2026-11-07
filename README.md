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
run the assembly execution tests with `cargo nextest run --no-fail-fast`.

Current lowering covers every i8 and i16 op in the coverage matrix: shifts and
rotates (constant counts inline, variable counts through `__gbc_shift8` and
`__gbc_shift16`), overflow ops, cls, trapnz/trapz (i8) and umulhi/smulhi. Wide
32-bit shifts other than constant logical right shifts, and the i32 overflow,
cls and mulhi ops, remain in sangoku-7qz.19/.20/.23 and the shift
specializations sangoku-9tk and sangoku-clp.

Use `Emulator` from `sangoku_test_support::emu` for runtime tests. It executes a
subset of SM83 instructions against flat 64 KiB memory. Set byte registers and
memory directly; use `set_sp()` and `set_hl()` for SP and HL. `step()` returns
one instruction's cost and `run_until(address, max_instructions)` returns the
cost to reach an address. Costs are CPU clock cycles (T-cycles), accumulated in
`cycles`. Unsupported instructions return an error.

Spills and reloads preserve live registers and flags. Native `popcnt.i16`
lowering supports narrow and zero-extended results with live inputs and register
pressure.

Calling convention: each argument byte goes in B, C, A in that order, then on
the stack (low address first, caller-owned, not popped by the callee). Return
values use A, B, C, D, E, H, L; a value that does not fit in the registers left
is returned whole in a caller-allocated return area. That case adds a hidden
16-bit pointer argument after the declared arguments (requires
`enable_multi_ret_implicit_sret`). All registers are caller-saved.

The backend supports a limited set of LR35902 operations and does not yet
support the full Rust ABI, indirect or tail calls, i64 and wider values, or
Rust data objects.

f32 is a soft-float type held in four byte registers. `runtime/softfloat32.asm`
implements correctly rounded add, sub, mul, div, sqrt, compare, min and max,
fma, trunc, floor, ceil and nearest, i8/i16/i32 conversions to and from f32
(trapping and saturating), and f16 promote/demote. f16 arithmetic, compares
and rounding promote to f32, operate and demote, which is correctly rounded.
f64, f128 and i64 conversions are not supported. The fork's ASCII-only changes to `core` are being reverted
(sangoku-30k).

## Testing

Each phase has a fixture type: a directory under `tests/fixtures/` whose files
carry their expected output as `// CHECK:` lines for LLVM `FileCheck`. Run a
phase on its own, for example `cargo run -- frontend file.rs` (MIR) or
`cargo run -- clif file.rs` (CLIF) or `cargo run -- lower file.clif` (RGBDS
assembly from CLIF, no rustc); `-` writes to stdout. Lowering fixtures in
`tests/fixtures/lower` are small hand-written or saved `.clif` files. To add a
fixture, put a small source file in the phase's directory and run
`tools/update_filecheck.py [glob]` to write its CHECK lines, then review the
diff. `tools/update_filecheck.py --check` (also run by
`cargo nextest run --no-fail-fast`) verifies them. Reduce each `core` blocker to
a fixture for the phase where it fails.

## Preserve Rust string behavior

Restore UTF-8 decoding and consistent `str`/`char` behavior in the Rust fork.
The current target-specific `next_code_point` reads one byte per character
without rejecting non-ASCII input, so valid non-ASCII strings can be decoded
incorrectly. Add regression fixtures for multibyte strings, character
boundaries, and forward/reverse iteration. Tracked by sangoku-30k, which also
covers compact Unicode tables and wider integer support.
