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
    let mut composite = [false; 50];
    let mut count = 0u8;
    for n in 2..composite.len() {
        if composite[n] {
            continue;
        }
        count += 1;
        let _ = write!(out, "{n} ");
        for multiple in (n * n..composite.len()).step_by(n) {
            composite[multiple] = true;
        }
    }
    let _ = writeln!(out, "\n{count} primes below 50");
    count
}
