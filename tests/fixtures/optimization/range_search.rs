#![no_std]

use core::{cmp::Ordering, ptr::read_volatile};

struct Range {
    start: u16,
    len: u8,
    parity: bool,
}

static RANGES: [(Range, i16); 4] = [
    (
        Range {
            start: 0x100,
            len: 3,
            parity: false,
        },
        1,
    ),
    (
        Range {
            start: 0x200,
            len: 4,
            parity: true,
        },
        1,
    ),
    (
        Range {
            start: 0x300,
            len: 0,
            parity: false,
        },
        -1,
    ),
    (
        Range {
            start: 0xff00,
            len: 7,
            parity: false,
        },
        32,
    ),
];

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let input = unsafe { read_volatile(0xc000 as *const u16) };
    match RANGES.binary_search_by(|(range, _)| {
        if input < range.start {
            Ordering::Greater
        } else if input > range.start + u16::from(range.len) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }) {
        Ok(index) => {
            let range = &RANGES[index].0;
            if range.parity && input & 1 != range.start & 1 {
                0
            } else {
                index as u8 + 1
            }
        }
        Err(_) => 0,
    }
}
