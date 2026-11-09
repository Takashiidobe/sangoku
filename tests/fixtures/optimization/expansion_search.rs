#![no_std]

use core::ptr::read_volatile;

static MAPPINGS: [(u16, [u16; 3]); 3] = [
    (0xdf, [0x53, 0x53, 0]),
    (0x390, [0x399, 0x308, 0x301]),
    (0xfb03, [0x46, 0x46, 0x49]),
];

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let input = unsafe { read_volatile(0xc000 as *const u16) };
    match MAPPINGS.binary_search_by_key(&input, |&(key, _)| key) {
        Ok(index) => index as u8 + 1,
        Err(_) => 0,
    }
}
