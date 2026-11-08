#![no_std]

use core::ptr::{read_volatile, write_volatile};

const SERIAL_DATA: *mut u8 = 0xFF01 as *mut u8;
const SERIAL_CONTROL: *mut u8 = 0xFF02 as *mut u8;

struct Serial;

impl Serial {
    fn write_byte(&mut self, byte: u8) {
        unsafe {
            write_volatile(SERIAL_DATA, byte);
            write_volatile(SERIAL_CONTROL, 0x81);
            while read_volatile(SERIAL_CONTROL) & 0x80 != 0 {}
        }
    }

    fn write_str(&mut self, text: &str) {
        for byte in text.bytes() {
            self.write_byte(byte);
        }
    }

    fn write_u16(&mut self, mut value: u16) {
        let mut started = false;
        for &place in &[10000, 1000, 100, 10, 1] {
            let mut digit = 0;
            while value >= place {
                value -= place;
                digit += 1;
            }
            if digit != 0 || started || place == 1 {
                self.write_byte(b'0' + digit);
                started = true;
            }
        }
    }

    fn write_values(&mut self, values: &[u16]) {
        self.write_byte(b'[');
        let mut first = true;
        for &value in values {
            if !first {
                self.write_str(", ");
            }
            self.write_u16(value);
            first = false;
        }
        self.write_str("]\n");
    }
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut out = Serial;
    let mut values = [42u16, 7, 1999, 3, 256, 88, 1024, 5];
    out.write_str("before: ");
    out.write_values(&values);
    values.sort_unstable();
    out.write_str("after:  ");
    out.write_values(&values);
    let total: u16 = values.iter().sum();
    out.write_str("sum = ");
    out.write_u16(total);
    out.write_str(", max = ");
    match values.iter().max() {
        Some(&value) => {
            out.write_str("Some(");
            out.write_u16(value);
            out.write_byte(b')');
        }
        None => out.write_str("None"),
    }
    out.write_byte(b'\n');
    0
}
