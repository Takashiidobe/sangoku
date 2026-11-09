#![no_std]
use core::hint::black_box;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut passed = 0;
    for (input, upper) in [('Ä', "Ä"), ('ß', "SS"), ('ǅ', "Ǆ"), ('🙂', "🙂")] {
        if black_box(input).to_uppercase().eq(upper.chars()) {
            passed += 1;
        }
    }
    passed
}
