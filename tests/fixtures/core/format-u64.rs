#![no_core]
#![feature(no_core, formatting_options)]

extern crate core;

use core::fmt::{self, Display, Formatter, FormattingOptions, Write};
use core::prelude::rust_2024::*;

struct Output {
    pointer: *mut u8,
    len: u16,
}

impl Write for Output {
    fn write_fmt(&mut self, _: fmt::Arguments<'_>) -> fmt::Result {
        Err(fmt::Error)
    }

    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.bytes() {
            unsafe { self.pointer.add(self.len as usize).write(byte) };
            self.len += 1;
        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub unsafe fn format_u64(value: *const u64, output: *mut u8) -> u16 {
    let mut writer = Output { pointer: output, len: 0 };
    let mut formatter = Formatter::new(&mut writer, FormattingOptions::new());
    match Display::fmt(unsafe { &*value }, &mut formatter) {
        Ok(()) => writer.len,
        Err(_) => u16::MAX,
    }
}
