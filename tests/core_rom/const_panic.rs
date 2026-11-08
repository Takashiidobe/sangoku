#![no_std]

const VALUE: u8 = panic!("constant panic remains an error");

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    VALUE
}
