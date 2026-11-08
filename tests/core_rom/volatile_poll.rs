#![no_std]

use core::ptr::{read_volatile, write_volatile};

const SERIAL_DATA: *mut u8 = 0xFF01 as *mut u8;
const SERIAL_CONTROL: *mut u8 = 0xFF02 as *mut u8;

fn put(byte: u8) {
    unsafe {
        write_volatile(SERIAL_DATA, byte);
        write_volatile(SERIAL_CONTROL, 0x81);
        while read_volatile(SERIAL_CONTROL) & 0x80 != 0 {}
    }
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    put(b'o');
    put(b'k');
    put(b'\n');
    7
}
