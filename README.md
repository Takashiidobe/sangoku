# gbc-rs

gbc-rs aims to compile Rust game code to LR35902 assembly, link it into a Game
Boy ROM, and provide a Rust runtime for Game Boy hardware.

## Build the example ROM

Build and link the `gbc-lr35902` Rust toolchain with `rustc-dev`, then install
RGBDS (`rgbasm`, `rgblink`, and `rgbfix`). A stock nightly cannot load the
backend because it uses private rustc APIs from the Rust fork.

```sh
cargo run -- example
```

The `gbc-rs` binary builds the backend, compiles Rust to assembly, and runs
RGBDS. Open `build/blank.gb` in a Game Boy or Game Boy Color emulator. The
cartridge calls `gbc_main` and then idles; it has no graphics or input runtime
yet. To build a source file directly, run `cargo run -- build game.rs game.gb`.

## End goal

Write a Game Boy game in Rust, compile it with the LR35902 Cranelift backend,
and link the generated assembly with RGBDS.

## Roadmap

- [x] Lower basic `i8`/`i16` ops, comparisons, branches, loads, and stores.
- [x] Emit RGBDS assembly and link a minimal cartridge ROM.
- [x] Add rustc's LR35902 C ABI mapping in the pinned Rust fork.
- [x] Build the Rust fork and compile `rustc_codegen_cranelift` against that
      compiler.
- [x] Lower `stack_addr.i16` and bytewise stack accesses.
- [x] Represent `i32` values as four byte registers.
- [x] Limit small memory-copy chunks to the target pointer width.
- [x] Lower Cranelift traps through the LR35902 `rst $10` vector.
- [x] Disable experimental `f16`/`f128` compiler-builtins for LR35902.
- [x] Omit float-bearing `core` and compiler-builtins routines and reject float
      use in game code.
- [x] Reject C variadic function definitions and calls on LR35902.
- [ ] Lower unsigned `i16` division required by `core` slice binary search.
- [ ] Add the intrinsics, data objects, and relocations required by `core`.
- [ ] Build `core` for LR35902 with assembly-only output.
- [ ] Add a runtime for ROM startup, panics, memory, and hardware access.
- [ ] Build a small game and extend the runtime for graphics, input, and audio.
- [ ] Make `gbc-rs` a reliable driver for compiling and linking Rust games.

## Current state

The backend compiles small `no_core` Rust functions to RGBDS assembly. Its
LR35902 lowering covers a small `i8`/`i16` subset, direct calls, and basic stack
arguments, comparisons, conditional branches, and little-endian loads and
stores. `rgbasm` and `rgblink` can assemble and link a minimal ROM. A build of
`stack_addr.i16` now uses the LR35902 `ld hl, sp + offset` instruction, and
spill loads and stores operate bytewise. Both Rust forks use the `gbc-lr35902`
toolchain. A `core` build now gets past `core::str::split_inclusive`, but stops
at unsupported `udiv.i16` in a Unicode slice binary-search routine. Trap
instructions use the ROM's `rst $10` vector, which disables interrupts and loops
forever. The full Rust ABI, indirect and tail calls, stack return values, and
Rust data objects are still unsupported.
