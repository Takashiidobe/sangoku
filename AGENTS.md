# Agent Instructions

## Project

This project targets Game Boy and Game Boy Color with Rust. The current goal is
to compile Rust `core` for LR35902; keep changes focused on reaching that
milestone before expanding the runtime or game features.

The compiler uses a pinned Rust fork and `rustc_codegen_cranelift` backend.
Stock nightly is not compatible with the backend's private rustc APIs.

## Build

Build and link the compiler toolchain in this order:

```bash
cd third_party/rust && ./x.py build --stage 2 compiler library
rustup toolchain link gbc-lr35902 third_party/rust/build/x86_64-unknown-linux-gnu/stage2
cargo +gbc-lr35902 build --release --manifest-path third_party/rustc_codegen_cranelift/Cargo.toml
```

Install RGBDS (`rgbasm`, `rgblink`, `rgbfix`) to assemble and link a ROM. Run
`cargo run -- example` for the blank ROM or `cargo run -- build game.rs game.gb`
to compile a source file. Use the repository's documented build path when
validating changes; core compilation is the target milestone.

## Testing Loop

Run tests with `cargo nextest run --no-fail-fast`, not `cargo test`.

After backend or compiler changes, rebuild the affected toolchain component,
then use the smallest reproducer for the feature being worked on. Use
`cargo run -- build examples/blank.rs build/blank.gb` as the end-to-end smoke
check; it rebuilds the backend, compiles Rust to assembly, and links a ROM. For
core support work, test the failing phase directly (`cargo run -- frontend|clif`, FileCheck fixtures in `tests/fixtures/<phase>`, `tools/update_filecheck.py`) so failures
in code generation are distinct from RGBDS or ROM-linking failures. Record newly
supported or still-blocked core behavior in the README roadmap.

## Working Rules

- Use Beads (`bd`) for shared project work. Run `bd prime` for the full workflow
  and `bd ready` to find available issues. Inspect with `bd show <id>`, claim
  with `bd update <id> --claim`, and close completed issues with
  `bd close <id>`. Create durable follow-up work with `bd create`; use
  `bd update` flags instead of interactive `bd edit`. Do not close work until it
  is complete.
- Run `cargo clippy --allow-dirty --fix` and `cargo fmt` before finishing a
  turn.
- Add an up to 50 line `llog` and commit at the end of a turn. Don't push unless
  asked.
- Keep changes scoped and explain the next steps before multi-step work.
- Do not modify vendored `third_party` sources unless the task requires a
  compiler or backend change.
- Commit all backend changes (`third_party/wasmtime`,
  `third_party/rustc_codegen_cranelift`, `third_party/rust`) inside their submodules, then commit
  sangoku with the updated submodule pointers in the same turn. Never leave
  backend work uncommitted.
- Backend crates depend on each other by `path`, not git, so edits are picked
  up immediately. Use path dependencies wherever possible.

## Current Constraints

- LR35902 code generation is incomplete. Check the README roadmap and current
  backend support before assuming an instruction or Rust feature is available.
- Rust `core` support is the priority. Runtime, graphics, input, and audio work
  comes after it can compile.
- Rust strings on target are ASCII-only; non-ASCII Unicode behavior is not
  supported.
