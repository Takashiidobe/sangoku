#![no_std]

use core::fmt::Write;
use core::ptr::{read_volatile, write_volatile};

struct Serial;

impl Write for Serial {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        for byte in text.bytes() {
            unsafe {
                write_volatile(0xff01 as *mut u8, byte);
                write_volatile(0xff02 as *mut u8, 0x81);
                while read_volatile(0xff02 as *const u8) & 0x80 != 0 {}
            }
        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let value = unsafe { read_volatile(0xc000 as *const u8) };
    writeln!(&mut Serial, "value={value}").unwrap();
    0
}
