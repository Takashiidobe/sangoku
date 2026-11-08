#![no_std]

use sangoku_console::println;

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut values = [42u16, 7, 1999, 3, 256, 88, 1024, 5];
    println!("before: {:?}", &values[..]);
    values.sort_unstable();
    println!("after:  {:?}", &values[..]);
    let total: u16 = values.iter().sum();
    println!("sum = {}, max = {:?}", total, values.iter().max());
    0
}
