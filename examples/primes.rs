#![no_std]

use core::fmt::{self, Write};
use core::ptr::{read_volatile, write_volatile};

const SERIAL_DATA: *mut u8 = 0xFF01 as *mut u8;
const SERIAL_CONTROL: *mut u8 = 0xFF02 as *mut u8;

struct Serial;

impl Write for Serial {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl Serial {
    fn write_byte(&mut self, byte: u8) {
        unsafe {
            write_volatile(SERIAL_DATA, byte);
            write_volatile(SERIAL_CONTROL, 0x81);

            while read_volatile(SERIAL_CONTROL) & 0x80 != 0 {}
        }
    }
}

fn write_u8(out: &mut Serial, mut n: u8) {
    if n >= 100 {
        return;
    }

    if n >= 10 {
        let mut tens = 0u8;

        while n >= 10 {
            n -= 10;
            tens += 1;
        }

        out.write_byte(b'0' + tens);
    }

    out.write_byte(b'0' + n);
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut out = Serial;
    let mut composite = [false; 50];

    for n in 2..50 {
        if let Some(&true) = composite.get(n) {
            continue;
        }

        write_u8(&mut out, n as u8);
        let _ = out.write_str(" ");

        for multiple in (n * n..composite.len()).step_by(n) {
            if let Some(slot) = composite.get_mut(multiple) {
                *slot = true;
            }
        }
    }

    let count = composite.iter().skip(2).filter(|b| !**b).count() as u8;

    let _ = out.write_str("\n");
    write_u8(&mut out, count);
    let _ = out.write_str(" primes below 50\n");

    count
}
