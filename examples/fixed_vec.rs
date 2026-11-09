#![no_std]

use sangoku_vec::FixedVec;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut values = FixedVec::<u8, 16>::new();
    let _ = values.push(10);
    let _ = values.push(20);
    values.iter().copied().fold(0, u8::wrapping_add)
}
