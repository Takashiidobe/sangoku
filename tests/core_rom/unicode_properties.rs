#![no_std]
use core::hint::black_box;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut passed = 0u8;
    if black_box('é').is_alphabetic() && !black_box('1').is_alphabetic() {
        passed += 1;
    }
    if black_box('\u{3000}').is_whitespace() && !black_box('x').is_whitespace() {
        passed += 1;
    }
    if black_box('٣').is_numeric() && !black_box('a').is_numeric() {
        passed += 1;
    }
    passed
}
