#![no_std]

use core::fmt::{self, Write};
use core::ptr::{read_volatile, write_volatile};

const SERIAL_DATA: *mut u8 = 0xFF01 as *mut u8;
const SERIAL_CONTROL: *mut u8 = 0xFF02 as *mut u8;

struct Serial;

impl Write for Serial {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.bytes() {
            unsafe {
                write_volatile(SERIAL_DATA, byte);
                write_volatile(SERIAL_CONTROL, 0x81);
                while read_volatile(SERIAL_CONTROL) & 0x80 != 0 {}
            }
        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut out = Serial;
    let _ = writeln!(out, "Hello, Game Boy!");
    let _ = writeln!(out, "6 * 7 = {}", 6u16 * 7);
    0
}
