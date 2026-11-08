#![no_std]

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut values = [42u16, 7, 1999, 3, 256, 88, 1024, 5];
    values.sort_unstable();
    values.iter().sum::<u16>() as u8
}
