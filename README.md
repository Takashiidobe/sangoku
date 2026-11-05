# gbc-rs

gbc-rs aims to compile Rust game code to LR35902 assembly, link it into a Game
Boy ROM, and provide a Rust runtime for Game Boy hardware.

## Build the example ROM

Install the pinned nightly with the components required by
rustc_codegen_cranelift, then install RGBDS (`rgbasm`, `rgblink`, and `rgbfix`):

```sh
rustup toolchain install nightly-2026-11-05 \
  --component rustc-dev --component rust-src
cargo +nightly-2026-11-05 build --release --locked \
  --manifest-path third_party/rustc_codegen_cranelift/Cargo.toml
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

- [x] Lower basic CLIF functions through register allocation and emission.
- [x] Lower `i8` arithmetic, bitwise operations, spills, and jumps.
- [x] Print allocated LR35902 instructions as RGBDS assembly.
- [x] Emit one RGBDS source unit per codegen unit with sections and symbols.
- [ ] Emit Rust data and preserve function/data references as RGBDS relocations.
- [x] Add an LR35902 `--emit=asm` path that skips object writing.
- [x] Assemble and link generated source with `rgbasm` and `rgblink`.
- [x] Lower direct calls with byte arguments in registers and on the stack.
- [ ] Complete rustc's LR35902 ABI mapping for parameters, returns, and foreign
      ABIs.
- [ ] Build a runtime for startup, graphics, input, audio, and cartridge memory.
- [x] Add a `gbc-rs` command that compiles a `no_core` source file and links a
      ROM.

## Current state

The backend can compile small `no_core` Rust functions, including direct calls
with byte arguments passed in registers and on the stack, to RGBDS assembly.
`rgbasm` and `rgblink` can assemble and link the result. The CLIF subset
includes `i8` arithmetic, bitwise operations, returns, spills, and unconditional
jumps. Indirect and tail calls, stack return values, and the full Rust ABI are
not supported. The assembly path still rejects Rust data objects.
