#![no_std]

use core::ptr::{read_volatile, write_volatile};

fn value() -> u16 {
    unsafe {
        write_volatile(0xc010 as *mut u8, 0x5a);
        read_volatile(0xc000 as *const u16)
    }
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    match unsafe { read_volatile(0xc020 as *const u8) } {
        0 => panic!("value: {}", value()),
        1 => {
            assert_eq!(value(), 0);
            0
        }
        2 => Err::<u8, u16>(value()).unwrap(),
        _ => {
            value();
            None::<u8>.unwrap()
        }
    }
}
