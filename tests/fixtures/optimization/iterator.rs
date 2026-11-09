#![no_std]

use core::ptr::{read_volatile, write_volatile};

#[inline(never)]
fn mapping(chars: [char; 3]) -> core::array::IntoIter<char, 3> {
    let mut iter = chars.into_iter();
    if chars[2] == '\0' {
        iter.next_back();
        if chars[1] == '\0' {
            iter.next_back();
        }
    }
    iter
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let input = unsafe { read_volatile(0xc000 as *const [char; 3]) };
    let mut count = 0;
    for output in mapping(input).rev() {
        unsafe { write_volatile((0xc010 + count * 4) as *mut char, output) };
        count += 1;
    }
    count as u8
}
