#![no_core]
#![feature(no_core)]

extern crate core;
use core::prelude::rust_2024::*;

#[derive(Clone, Copy)]
struct Record {
    bytes: &'static [u8],
    callback: fn(u8) -> u8,
}

static BYTES: [u8; 4] = [11, 22, 33, 44];
static RECORD: Record = Record {
    bytes: unsafe { core::slice::from_raw_parts(BYTES.as_ptr().add(1), 3) },
    callback: increment,
};

fn increment(value: u8) -> u8 {
    value.wrapping_add(1)
}

#[unsafe(no_mangle)]
pub fn read_record(index: u16) -> u8 {
    let record = unsafe { core::ptr::read_volatile(&RECORD) };
    (record.callback)(record.bytes[index as usize])
}
