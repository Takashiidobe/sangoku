#![no_std]

use core::ptr::{read_volatile, write_volatile};

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let input = unsafe { read_volatile(0xc000 as *const char) };
    let mut count = 0;
    for output in input.to_uppercase() {
        unsafe { write_volatile((0xc010 + count * 4) as *mut char, output) };
        count += 1;
    }
    count as u8
}
