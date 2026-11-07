#![no_std]

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let data = [3u8, 1, 4, 1, 5, 9, 2, 6];
    let sum: u8 = data.iter().copied().filter(|x| x & 1 == 1).sum();
    let max = data.iter().copied().max().unwrap_or(0);
    sum.wrapping_add(max)
}
