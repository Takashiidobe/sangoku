#![feature(no_core)]
#![no_core]

extern crate archive_dependency as dependency;

#[unsafe(no_mangle)]
pub fn call_dependency() -> u8 {
    dependency::identity(dependency::dependency_answer())
}

#[unsafe(no_mangle)]
pub fn multiply(a: u16, b: u16) -> u16 {
    dependency::wrapping_mul(a, b)
}

#[unsafe(no_mangle)]
pub static POINTERS: dependency::Pointers = dependency::Pointers {
    bytes: &raw const dependency::BYTES as *const u8,
    function: dependency::dependency_answer,
};

#[unsafe(no_mangle)]
pub static mut ZERO: [u8; 8] = [0; 8];

#[unsafe(no_mangle)]
pub static mut MUTABLE_POINTERS: dependency::Pointers = dependency::Pointers {
    bytes: &raw const dependency::BYTES as *const u8,
    function: dependency::dependency_answer,
};
