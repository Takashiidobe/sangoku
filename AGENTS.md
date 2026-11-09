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
`cargo run -- example` for the hello ROM or `cargo run -- build game.rs game.gb`
to compile a source file. Use the repository's documented build path when
validating changes; core compilation is the target milestone.

## Testing Loop

Run tests with `cargo nextest run --no-fail-fast`, not `cargo test`. This uses
`.config/nextest.toml`'s default profile. Move tests measured above 10 seconds,
except FileCheck, out of default and into extensive. Run the full suite with
`cargo nextest run --profile extensive --no-fail-fast`; use that profile when
selecting a test excluded from default.

After backend or compiler changes, rebuild the affected toolchain component,
then use the smallest reproducer for the feature being worked on. Use
`cargo run -- build examples/hello.rs build/hello.gb` as the end-to-end smoke
check; it rebuilds the backend, compiles Rust to assembly, and links a ROM. For
core support work, test the failing phase directly
(`cargo run -- frontend|clif`, FileCheck fixtures in `tests/fixtures/<phase>`,
`tools/update_filecheck.py`) so failures in code generation are distinct from
RGBDS or ROM-linking failures. Record newly supported or still-blocked core
behavior in the relevant wiki concept page.

## Working Rules

- Do not update `README.md` unless the user explicitly requests it. Put usage,
  testing, support, and implementation updates in the relevant page under
  `wiki/concepts/`. Add new concept pages to `wiki/index.md`. Keep the README
  user-facing, with concise inline examples and testing instructions; do not use
  it to link developer concept pages.
- Use Beads (`bd`) for shared project work. Run `bd prime` for the full workflow
  and `bd ready` to find available issues. Inspect with `bd show <id>`, claim
  with `bd update <id> --claim`, and close completed issues with
  `bd close <id>`. Create durable follow-up work with `bd create`; use
  `bd update` flags instead of interactive `bd edit`. Do not close work until it
  is complete.
- Run `cargo clippy --allow-dirty --fix` and `cargo fmt` before finishing a
  turn.
- Add an `llog` of around 60 lines at most and commit at the end of a turn,
  following "Writing an llog" below. Don't push unless asked. Format logs with
  `prettier <file> --print-width 80 --prose-wrap always --write`.
- Use the LSP tool (definitions, references, symbols, diagnostics) to navigate
  and check Rust code, especially in the large `third_party` backends, instead
  of reading files or grepping blindly.
- Keep changes scoped and explain the next steps before multi-step work.
- Do not modify vendored `third_party` sources unless the task requires a
  compiler or backend change.
- Commit all backend changes (`third_party/wasmtime`,
  `third_party/rustc_codegen_cranelift`, `third_party/rust`) inside their
  submodules, then commit sangoku with the updated submodule pointers in the
  same turn. Never leave backend work uncommitted.
- After changing any of them, rebuild what that change affects, then rerun the
  tests. `tools/rebuild_toolchain.sh` rebuilds the whole chain (rust stage 2,
  relink `gbc-lr35902`, cranelift backend). A `core` change needs the full
  rebuild; do not test against a stale toolchain.
- Backend crates depend on each other by `path`, not git, so edits are picked up
  immediately. Use path dependencies wherever possible.

## Writing an llog

Logs live in `wiki/log/<date>-<time>.md`. They are Markdown: backticks for
code, identifiers, files, registers and issue ids, and fenced blocks
with a language (`asm`, `rust`, `text`) for listings. Inline what a reader needs
from the bead (why it was filed, what it asked for, how it closed) so nobody has
to run `bd show`. Title line is `# <summary> (<issue id>)`, then the timestamp.
Pick the kind that fits; stay around 60 lines or fewer.

### Optimization

Sections in order:

1. **Context**: why we looked (what we noticed), what this change does, the
   issue id and the measured result.
2. **Case**: the source pattern, in one or two lines.
3. **Currently emits**: output before the change, as a fenced listing.
4. **Should emit**: the intended output as a fenced listing; if the real output
   differs from the ideal, show the real one and name the gap.
5. **How**: the lowering rule, pass or files changed, and the tests.
6. **Alternatives**: approaches considered or deferred, and why. Known leftovers
   go here too.

### Feature

Something had to lower, link or run at all. Sections: **Context** (what was
failing or missing, from the bead, and why it mattered for the milestone),
**Case** (the Rust or CLIF that failed and how: error text, missing symbol,
trap), **Before** and **After** (error output or generated code, whichever shows
the change), **How**, **Alternatives**.

### Bug

Sections: **Context** (symptom and how it was found), **Case** (minimal
reproducer), **Cause** (the verified root cause, not a guess), **Fix** (what
changed and the regression test), **Alternatives** (other fixes, and related
suspects not checked).

### Bookkeeping

Docs, issue triage, renames, regenerated snapshots, tool or CI tweaks. Say
plainly what was done and why in a few lines; no sections needed.

For optimizations also mirror the case, rule and before/after in the relevant
`wiki/concepts/` page.
