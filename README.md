# sangoku

Write Game Boy and Game Boy Color ROMs in Rust. Sangoku targets LR35902 with
Rust `core`, opt-in `alloc`, and lightweight `ufmt` output. Inputs are
standalone `no_std` Rust files; Cargo project builds are not supported yet.

## Usage

Write `game.rs` using the supplied serial console:

```rust
#![no_std]

use sangoku_console::println;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    println!("Hello, Game Boy! The answer is {}", 42u8);
    0
}
```

With the standalone compiler and RGBDS (`rgbasm`, `rgblink`, `rgbfix`) on
`PATH`, build a ROM or run it in the built-in serial runner:

```bash
sangoku build game.rs game.gb --opt-level z
sangoku run game.rs
```

For visible output, build the screen example from a source checkout and open it
in SameBoy:

```bash
cargo run -- build examples/screen.rs build/screen.gb --opt-level z
sameboy build/screen.gb
```

## Testing

```bash
cargo nextest run --no-fail-fast
```
