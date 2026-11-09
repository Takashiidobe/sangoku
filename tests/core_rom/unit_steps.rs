#![no_std]

use core::hint::black_box;

fn step_up(start: u16, count: u8) -> u16 {
    let (mut value, mut left) = (start, count);
    while left != 0 {
        value = value.wrapping_add(1);
        left -= 1;
    }
    value
}

fn step_down(start: u16, count: u8) -> u16 {
    let (mut value, mut left) = (start, count);
    while left != 0 {
        value = value.wrapping_sub(1);
        left -= 1;
    }
    value
}

fn bytes_up(start: u8, count: u8) -> u8 {
    let (mut value, mut left) = (start, count);
    while left != 0 {
        value = value.wrapping_add(1);
        left -= 1;
    }
    value
}

fn bytes_down(start: u8, count: u8) -> u8 {
    let (mut value, mut left) = (start, count);
    while left != 0 {
        value = value.wrapping_sub(1);
        left -= 1;
    }
    value
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let up = step_up(black_box(0x00F0), black_box(32));
    let across_zero = step_up(black_box(0xFFF8), black_box(16));
    let down = step_down(black_box(0x0105), black_box(32));
    let below_zero = step_down(black_box(0x0004), black_box(16));
    let wrap_up = bytes_up(black_box(250), black_box(10));
    let wrap_down = bytes_down(black_box(3), black_box(5));
    let mut sum = wrap_up.wrapping_add(wrap_down);
    for word in [up, across_zero, down, below_zero] {
        sum = sum.wrapping_add((word >> 8) as u8).wrapping_add(word as u8);
    }
    sum
}
