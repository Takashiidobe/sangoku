#![no_std]

use core::hint::black_box;

#[inline(never)]
fn outcomes(a: u8, b: u8) -> u8 {
    let mut bits = 0u8;
    if a == b {
        bits |= 1;
    }
    if a != b {
        bits |= 2;
    }
    if a < b {
        bits |= 4;
    }
    if a <= b {
        bits |= 8;
    }
    if a > b {
        bits |= 16;
    }
    if a >= b {
        bits |= 32;
    }
    bits
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let values = [0u8, 1, 127, 128, 255];
    let mut sum = 0u8;
    for a in values {
        for b in values {
            sum = sum.wrapping_mul(3).wrapping_add(outcomes(black_box(a), black_box(b)));
        }
    }
    sum
}
