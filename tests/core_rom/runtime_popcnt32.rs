#![no_std]

use core::ptr::{read_volatile, write_volatile};

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let lhs = unsafe { read_volatile(0xc000 as *const u32) };
    unsafe { write_volatile(0xc008 as *mut u32, lhs.count_ones()) };
    0
}
