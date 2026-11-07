#![no_std]

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let index = unsafe { core::ptr::read_volatile(0xc000 as *const u8) };
    [7u8][index as usize]
}
