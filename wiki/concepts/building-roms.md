# Building and running ROMs

Build and run an example in the built-in emulator:

```bash
cargo run -- run examples/hello.rs
```

Build a ROM:

```bash
cargo run -- build examples/sort.rs build/sort.gb
```

Build the screen demo and open it in SameBoy:

```bash
cargo run -- build examples/screen.rs build/screen.gb --opt-level z
sameboy build/screen.gb
```

It displays a small uppercase font, a greeting, and arithmetic formatted with
`ufmt`. The CPU halts after setup while the image stays visible. Use an external
emulator for this example; the built-in runner does not emulate LCD timing or
graphics.

Use the [examples](../../examples/) as a starting point. Add `--opt-level z` to
optimize for size. Build artifacts stay in `build/artifacts/` for debugging.

All optimization levels eliminate panic-only formatting. Failed checks still
trap, and panic argument effects are preserved. Explicit formatting and Unicode
tables retain their actual ROM costs. Check the build warnings.

For serial output, use the supplied `sangoku_console` macros:

```rust
#![no_std]

use sangoku_console::println;

#[unsafe(no_mangle)]
pub extern "C" fn gbc_main() -> u8 {
    println!("answer = {}", 42u16);
    0
}
```

The console uses `ufmt` without `core::fmt`. For explicit writers, use
`sangoku_console::{Serial, finish, write, writeln}` and pass the result to
`finish`. Failed checks trap without panic diagnostics.

Inputs are standalone Rust 2024 source files. Sangoku supplies `ufmt` and
`sangoku_console`; it does not yet resolve dependencies from your own
`Cargo.toml` or build arbitrary Cargo projects.

## Compiler setup

With the bundled Linux x86-64 executable and RGBDS installed on `PATH`:

```bash
./sangoku run game.rs
./sangoku build game.rs game.gb --opt-level z
```

No Cargo or rustup is needed. The toolchain unpacks into a cache on first use.
To build `build/dist/sangoku`, see [distribution](../distribution.md).

## Build from source

Install RGBDS (`rgbasm`, `rgblink`, `rgbfix`), then build the pinned Rust fork
and backend:

```bash
./tools/rebuild_toolchain.sh
```
