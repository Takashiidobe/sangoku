#![no_std]

extern crate alloc;

use alloc::boxed::Box;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    *Box::new(42)
}
