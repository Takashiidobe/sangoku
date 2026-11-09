#![no_std]
use core::hint::black_box;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut passed = 0;
    for (input, lower) in [
        ('Ä', "ä"),
        ('ß', "ß"),
        ('ǅ', "ǆ"),
        ('🙂', "🙂"),
    ] {
        let input = black_box(input);
        if input.to_lowercase().eq(lower.chars()) {
            passed += 1;
        }
    }
    passed
}
