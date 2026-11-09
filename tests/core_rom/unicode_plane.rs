#![no_std]

use core::ptr::{read_volatile, write_volatile};

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let input = unsafe { read_volatile(0xc000 as *const u32) };
    let plane = (input >> 16) as u16;
    let low = input as u16;
    unsafe {
        write_volatile(0xc004 as *mut u16, plane);
        write_volatile(0xc006 as *mut u16, low);
    }
    0
}
