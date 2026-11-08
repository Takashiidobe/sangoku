#![no_std]

use core::ptr::read_volatile;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let values = [1u8, 2, 3];
    let end = unsafe { read_volatile(0xc000 as *const u8) } as usize;
    values[..end].len() as u8
}
