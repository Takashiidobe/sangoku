#![no_std]

use core::{
    hint::select_unpredictable,
    ptr::{read_volatile, write_volatile},
};

struct Value(u8);

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            let counter = 0xc020 as *mut u8;
            write_volatile(counter, read_volatile(counter).wrapping_add(self.0));
        }
    }
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let flag = unsafe { read_volatile(0xc000 as *const u8) } != 0;
    let a = unsafe { read_volatile(0xc004 as *const u32) };
    let b = unsafe { read_volatile(0xc008 as *const u32) };
    unsafe {
        write_volatile(0xc010 as *mut u32, select_unpredictable(flag, a, b));
        write_volatile(
            0xc014 as *mut usize,
            select_unpredictable(flag, a as usize, b as usize),
        );
        write_volatile(
            0xc016 as *mut [u16; 3],
            select_unpredictable(flag, [a as u16, 2, 3], [b as u16, 5, 6]),
        );
    }
    let chosen = select_unpredictable(flag, Value(a as u8), Value(b as u8));
    unsafe { write_volatile(0xc01c as *mut u8, chosen.0) };
    drop(chosen);
    0
}
