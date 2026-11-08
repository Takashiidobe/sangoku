#![no_std]

use core::fmt::Write;
use core::hint::black_box;
use core::ptr::read_volatile;

struct Checksum(u8);

impl Write for Checksum {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        for byte in text.bytes() {
            self.0 = self.0.wrapping_add(byte);
        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let data = black_box([3u8, 1, 4, 1, 5, 9, 2, 6]);
    let start = unsafe { read_volatile(0xc000 as *const u8) } as usize;
    let mut checksum = Checksum(0);
    write!(&mut checksum, "{start}").unwrap();
    let end = unsafe { read_volatile(0xc001 as *const u8) } as usize;
    let sum: u8 = data[start..end].iter().copied().fold(0, u8::wrapping_add);
    let c = black_box('Ä');
    let lower = c.to_lowercase().next().unwrap_or(c);
    let mut buffer = black_box([0u8; 2]);
    let encoded = lower.encode_utf8(&mut buffer).len() as u8;
    sum.wrapping_add(checksum.0).wrapping_add(encoded)
        .wrapping_add(lower.is_alphabetic() as u8)
        .wrapping_add(c.to_uppercase().count() as u8)
}
