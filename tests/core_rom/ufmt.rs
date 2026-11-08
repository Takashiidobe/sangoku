#![no_std]

use core::ptr::read_volatile;
use sangoku_console::{Serial, finish, print, println, ufmt, writeln};

#[derive(ufmt::derive::uDebug)]
struct Pair {
    unsigned: u16,
    signed: i16,
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let value = unsafe { read_volatile(0xc000 as *const u16) };
    let wide = unsafe { read_volatile(0xc004 as *const u32) };
    print!("value={} signed={} wide={} ", value, value as i16, wide);
    finish(writeln!(&mut Serial, "hex={:x}", value));
    println!("pair={:?}", Pair { unsigned: value, signed: value as i16 });
    println!("array={:?} option={:?}", &[0u16, value][..], Some(value));
    println!("utf8={} {} {{}}", "λ", 'Ä');
    0
}
