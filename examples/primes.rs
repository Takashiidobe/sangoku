#![no_std]

use sangoku_console::{print, println};

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut composite = [false; 50];

    for n in 2..50 {
        if let Some(&true) = composite.get(n) {
            continue;
        }

        print!("{} ", n as u8);

        for multiple in (n * n..composite.len()).step_by(n) {
            if let Some(slot) = composite.get_mut(multiple) {
                *slot = true;
            }
        }
    }

    let count = composite.iter().skip(2).filter(|b| !**b).count() as u8;

    println!();
    println!("{} primes below 50", count);

    count
}
