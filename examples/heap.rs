#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut global = Vec::new();
    global.push(10u8);
    global.push(20);
    global.push(12);

    let mut scoped = Vec::new_in(&gbc_heap::HEAP);
    scoped.push(1u8);
    scoped.push(2);

    let boxed = Box::new(7u8);

    let sum = global
        .iter()
        .chain(scoped.iter())
        .copied()
        .fold(0, u8::wrapping_add);
    sum.wrapping_add(*boxed)
}
