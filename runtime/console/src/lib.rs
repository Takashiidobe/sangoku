#![no_std]

use core::convert::Infallible;
use core::ptr::{read_volatile, write_volatile};

pub use ufmt;
pub use ufmt::{uWrite, uwrite as write, uwriteln as writeln};

pub struct Serial;

impl uWrite for Serial {
    type Error = Infallible;

    fn write_str(&mut self, text: &str) -> Result<(), Infallible> {
        for byte in text.bytes() {
            unsafe {
                write_volatile(0xff01 as *mut u8, byte);
                write_volatile(0xff02 as *mut u8, 0x81);
                while read_volatile(0xff02 as *const u8) & 0x80 != 0 {}
            }
        }
        Ok(())
    }

    // avoid ufmt-write's uninitialized default buffer.
    fn write_char(&mut self, value: char) -> Result<(), Infallible> {
        let mut bytes = [0u8; 4];
        self.write_str(value.encode_utf8(&mut bytes))
    }
}

pub fn finish(result: Result<(), Infallible>) {
    match result {
        Ok(()) => {}
        Err(never) => match never {},
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::finish($crate::ufmt::uwrite!(&mut $crate::Serial, $($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => { $crate::println!("") };
    ($($arg:tt)*) => {
        $crate::finish($crate::ufmt::uwriteln!(&mut $crate::Serial, $($arg)*))
    };
}
