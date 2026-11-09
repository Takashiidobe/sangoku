#![no_std]
use core::hint::black_box;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let text = black_box("Äßǅ🙂");
    let mut passed = 0;
    if text.chars().count() == 4 {
        passed += 1;
    }
    if text.starts_with(black_box("Ä")) {
        passed += 1;
    }
    if text.ends_with(black_box("🙂")) {
        passed += 1;
    }
    passed
}
