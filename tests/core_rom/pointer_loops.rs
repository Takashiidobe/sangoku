#![no_std]

use core::hint::black_box;

unsafe fn fill(p: *mut u8, v: u8, n: u8) {
    let (mut p, mut n) = (p, n);
    while n != 0 {
        unsafe { *p = v };
        p = unsafe { p.add(1) };
        n -= 1;
    }
}

unsafe fn copy_u8(dst: *mut u8, src: *const u8, n: u8) {
    let (mut d, mut s, mut n) = (dst, src, n);
    while n != 0 {
        unsafe { *d = *s };
        d = unsafe { d.add(1) };
        s = unsafe { s.add(1) };
        n -= 1;
    }
}

unsafe fn max_u8(p: *const u8, n: u8) -> u8 {
    let (mut m, mut p, mut n) = (0u8, p, n);
    while n != 0 {
        let x = unsafe { *p };
        if x > m {
            m = x;
        }
        p = unsafe { p.add(1) };
        n -= 1;
    }
    m
}

unsafe fn count_eq(p: *const u8, v: u8, n: u8) -> u8 {
    let (mut c, mut p, mut n) = (0u8, p, n);
    while n != 0 {
        if unsafe { *p } == v {
            c += 1;
        }
        p = unsafe { p.add(1) };
        n -= 1;
    }
    c
}

unsafe fn strlen(p: *const u8) -> u8 {
    let (mut len, mut p) = (0u8, p);
    while unsafe { *p } != 0 {
        len += 1;
        p = unsafe { p.add(1) };
    }
    len
}

#[unsafe(no_mangle)]
pub fn gbc_main() -> u8 {
    let mut src = [3u8, 9, 4, 9, 1, 0, 7, 9];
    let mut dst = [0u8; 8];
    unsafe {
        copy_u8(black_box(dst.as_mut_ptr()), black_box(src.as_ptr()), 8);
        fill(black_box(src.as_mut_ptr()), 5, 3);
        let counted = count_eq(black_box(dst.as_ptr()), 9, 8);
        let largest = max_u8(black_box(dst.as_ptr()), 8);
        let length = strlen(black_box(src.as_ptr()));
        counted * 16 + largest + length + src[2] + dst[4]
    }
}
