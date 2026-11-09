#![no_std]
#![crate_type = "lib"]

#[unsafe(no_mangle)]
pub fn mix16(a: u16, b: u16, c: u16) -> u16 {
    ((a ^ b).wrapping_add(c)) ^ ((c << 3) | (c >> 13))
}

#[unsafe(no_mangle)]
pub fn pressure(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> u8 {
    let t1 = a.wrapping_add(b);
    let t2 = c ^ d;
    let t3 = e & f;
    let t4 = t1.wrapping_add(t2);
    let t5 = t2 | t3;
    let t6 = t1 ^ t3;
    t4.wrapping_add(t5) ^ t6.wrapping_add(a ^ f)
}

#[unsafe(no_mangle)]
pub fn pressure16(a: u16, b: u16, c: u16) -> u16 {
    let x = a.wrapping_add(b);
    let y = b ^ c;
    let z = c.wrapping_sub(a);
    (x & y).wrapping_add(y | z) ^ x.wrapping_add(z)
}

#[unsafe(no_mangle)]
pub unsafe fn sum_u8(p: *const u8, n: u8) -> u16 {
    let (mut s, mut p, mut n) = (0u16, p, n);
    while n != 0 {
        s = s.wrapping_add(unsafe { *p } as u16);
        p = unsafe { p.add(1) };
        n -= 1;
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe fn sum_u16(p: *const u16, n: u8) -> u16 {
    let (mut s, mut p, mut n) = (0u16, p, n);
    while n != 0 {
        s = s.wrapping_add(unsafe { *p });
        p = unsafe { p.add(1) };
        n -= 1;
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe fn fill(p: *mut u8, v: u8, n: u8) {
    let (mut p, mut n) = (p, n);
    while n != 0 {
        unsafe { *p = v };
        p = unsafe { p.add(1) };
        n -= 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn copy_u8(dst: *mut u8, src: *const u8, n: u8) {
    let (mut d, mut s, mut n) = (dst, src, n);
    while n != 0 {
        unsafe { *d = *s };
        d = unsafe { d.add(1) };
        s = unsafe { s.add(1) };
        n -= 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn max_u8(p: *const u8, n: u8) -> u8 {
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

#[unsafe(no_mangle)]
pub unsafe fn count_eq(p: *const u8, v: u8, n: u8) -> u8 {
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

#[unsafe(no_mangle)]
pub unsafe fn strlen16(p: *const u8) -> u16 {
    let (mut len, mut p) = (0u16, p);
    while unsafe { *p } != 0 {
        len = len.wrapping_add(1);
        p = unsafe { p.add(1) };
    }
    len
}

#[unsafe(no_mangle)]
pub fn fib16(n: u8) -> u16 {
    let (mut a, mut b, mut n) = (0u16, 1u16, n);
    while n != 0 {
        let t = a.wrapping_add(b);
        a = b;
        b = t;
        n -= 1;
    }
    a
}

#[unsafe(no_mangle)]
pub fn sat_chain(a: u8, b: u8, c: u8) -> u8 {
    let x = a.saturating_add(b);
    let y = if x > c { x - c } else { c - x };
    y.saturating_add(a)
}
