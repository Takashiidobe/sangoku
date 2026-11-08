#![no_std]

use sangoku_console::println;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    println!("Hello, Game Boy!");
    println!("{} * {} = {}", 6u8, 7u8, 6u8 * 7);
    0
}
