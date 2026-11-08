#![no_std]

use core::ptr::{read_volatile, write_volatile};

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let dividend = unsafe { read_volatile(0xc000 as *const u32) };
    let divisor = unsafe { read_volatile(0xc004 as *const u32) };
    unsafe {
        write_volatile(0xc008 as *mut u32, dividend / divisor);
        write_volatile(0xc00c as *mut u32, dividend % divisor);
    }
    0
}
