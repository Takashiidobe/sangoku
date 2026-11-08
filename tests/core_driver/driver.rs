#![no_std]
#![allow(dead_code)]

use core::cell::{Cell, RefCell};
use core::fmt::{self, Write};
use core::iter;
use core::mem;
use core::ptr;

struct Sink(u16);

impl Write for Sink {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0 = self.0.wrapping_add(s.len() as u16);
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub fn iter_map_filter_sum(data: &[u8]) -> u16 {
    data.iter().map(|&x| x as u16).filter(|x| x & 1 == 0).sum()
}

#[unsafe(no_mangle)]
pub fn iter_zip_enumerate_fold(a: &[u16], b: &[u16]) -> u16 {
    a.iter().zip(b).enumerate().fold(0, |acc, (i, (x, y))| acc ^ (x.wrapping_mul(*y)) ^ i as u16)
}

#[unsafe(no_mangle)]
pub fn iter_rev_chain_take_skip(a: &[u8], b: &[u8]) -> u8 {
    a.iter().rev().chain(b).skip(1).take(3).copied().max().unwrap_or(0)
}

#[unsafe(no_mangle)]
pub fn iter_windows_chunks(data: &[u8]) -> usize {
    data.windows(2).count() + data.chunks(3).count() + data.chunks_exact(2).len()
}

#[unsafe(no_mangle)]
pub fn iter_position_any_all(data: &[u8]) -> bool {
    data.iter().position(|&x| x == 7).is_some() && data.iter().any(|&x| x > 3) && data.iter().all(|&x| x < 200)
}

#[unsafe(no_mangle)]
pub fn iter_repeat_successors() -> u32 {
    iter::successors(Some(1u32), |x| x.checked_mul(3)).count() as u32 + iter::repeat(2u32).take(4).sum::<u32>()
}

#[unsafe(no_mangle)]
pub fn slice_sort_unstable(data: &mut [u16]) {
    data.sort_unstable();
}

#[unsafe(no_mangle)]
pub fn slice_sort_by_key(data: &mut [(u8, u8)]) {
    data.sort_unstable_by_key(|p| p.1);
}

#[unsafe(no_mangle)]
pub fn slice_binary_search(data: &[u16], x: u16) -> bool {
    data.binary_search(&x).is_ok()
}

#[unsafe(no_mangle)]
pub fn slice_reverse_rotate_swap(data: &mut [u8]) {
    data.reverse();
    data.rotate_left(1);
    if data.len() > 1 {
        data.swap(0, 1);
    }
}

#[unsafe(no_mangle)]
pub fn slice_copy_fill(dst: &mut [u8], src: &[u8]) {
    dst.fill(1);
    dst.copy_from_slice(src);
}

#[unsafe(no_mangle)]
pub fn slice_contains_starts(a: &[u8], b: &[u8]) -> bool {
    a.contains(&1) || a.starts_with(b) || a.ends_with(b)
}

#[unsafe(no_mangle)]
pub fn slice_split_first_last(a: &[u8]) -> u8 {
    let (x, rest) = a.split_first().unwrap_or((&0, &[]));
    *x + rest.last().copied().unwrap_or(0)
}

#[unsafe(no_mangle)]
pub fn str_chars_count(s: &str) -> usize {
    s.chars().count() + s.char_indices().count() + s.bytes().len()
}

#[unsafe(no_mangle)]
pub fn str_find_split(s: &str) -> usize {
    s.find("ab").unwrap_or(0) + s.split(',').count() + s.trim().len() + s.lines().count()
}

#[unsafe(no_mangle)]
pub fn str_parse_u16(s: &str) -> u16 {
    s.parse::<u16>().unwrap_or(0)
}

#[unsafe(no_mangle)]
pub fn str_parse_i32(s: &str) -> i32 {
    s.parse::<i32>().unwrap_or(0)
}

#[unsafe(no_mangle)]
pub fn str_parse_f32(s: &str) -> f32 {
    s.parse::<f32>().unwrap_or(0.0)
}

#[unsafe(no_mangle)]
pub fn str_from_utf8(b: &[u8]) -> usize {
    core::str::from_utf8(b).map(|s| s.len()).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub fn str_case_ascii(s: &mut str) {
    s.make_ascii_uppercase();
}

#[unsafe(no_mangle)]
pub fn char_methods(c: char) -> u32 {
    c.is_alphabetic() as u32 + c.is_numeric() as u32 + c.to_digit(16).unwrap_or(0) + c.len_utf8() as u32
}

#[unsafe(no_mangle)]
pub fn char_case(c: char) -> char {
    c.to_uppercase().next().unwrap_or(c)
}

#[unsafe(no_mangle)]
pub fn fmt_u16(v: u16) -> u16 {
    let mut s = Sink(0);
    let _ = write!(s, "{v}");
    s.0
}

#[unsafe(no_mangle)]
pub fn fmt_hex_i32(v: i32) -> u16 {
    let mut s = Sink(0);
    let _ = write!(s, "{v:#x} {v:08b}");
    s.0
}

#[unsafe(no_mangle)]
pub fn fmt_debug_slice(v: &[u8]) -> u16 {
    let mut s = Sink(0);
    let _ = write!(s, "{v:?}");
    s.0
}

#[unsafe(no_mangle)]
pub fn fmt_debug_option(v: Option<u8>) -> u16 {
    let mut s = Sink(0);
    let _ = write!(s, "{v:?}");
    s.0
}

#[unsafe(no_mangle)]
pub fn fmt_str_padded(v: &str) -> u16 {
    let mut s = Sink(0);
    let _ = write!(s, "{v:>10}");
    s.0
}

#[unsafe(no_mangle)]
pub fn fmt_f32(v: f32) -> u16 {
    let mut s = Sink(0);
    let _ = write!(s, "{v}");
    s.0
}

#[unsafe(no_mangle)]
pub fn fmt_u64(v: u64) -> u16 {
    let mut s = Sink(0);
    let _ = write!(s, "{v}");
    s.0
}

#[unsafe(no_mangle)]
pub fn fmt_u128(v: u128) -> u16 {
    let mut s = Sink(0);
    let _ = write!(s, "{v}");
    s.0
}

#[unsafe(no_mangle)]
pub fn option_combinators(a: Option<u16>, b: Option<u16>) -> u16 {
    a.zip(b).map(|(x, y)| x + y).or(a).unwrap_or_default()
}

#[unsafe(no_mangle)]
pub fn result_combinators(a: Result<u16, u8>) -> u16 {
    a.map_err(|e| e as u16).and_then(|x| x.checked_add(1).ok_or(0)).unwrap_or_else(|e| e)
}

#[unsafe(no_mangle)]
pub fn cell_refcell(c: &RefCell<u16>, d: &Cell<u16>) -> u16 {
    *c.borrow_mut() += 1;
    d.set(d.get() + 1);
    *c.borrow() + d.get()
}

#[unsafe(no_mangle)]
pub fn mem_swap_replace(a: &mut u32, b: &mut u32) -> u32 {
    mem::swap(a, b);
    mem::replace(a, 5) + mem::take(b)
}

#[unsafe(no_mangle)]
pub unsafe fn ptr_ops(p: *mut u16, q: *const u16) -> u16 {
    unsafe {
        ptr::copy_nonoverlapping(q, p, 2);
        ptr::write(p.add(1), 3);
        ptr::read(p) + p.offset_from(q) as u16
    }
}

#[unsafe(no_mangle)]
pub fn num_wide_ops(a: u64, b: u64) -> u64 {
    a.wrapping_mul(b) ^ a.rotate_left(7) ^ (a / (b | 1)) ^ a.leading_zeros() as u64 ^ a.pow(3)
}

#[unsafe(no_mangle)]
pub fn num_i128_ops(a: i128, b: i128) -> i128 {
    a.wrapping_mul(b) ^ (a / (b | 1)) ^ a.abs() ^ a.signum()
}

#[unsafe(no_mangle)]
pub fn num_checked_ops(a: u32, b: u32) -> u32 {
    a.checked_add(b).unwrap_or(0) + a.saturating_sub(b) + a.wrapping_shl(b) + a.isqrt() + a.ilog2()
}

#[unsafe(no_mangle)]
pub fn num_f32_ops(a: f32, b: f32) -> f32 {
    a.abs() + a.max(b) + a.clamp(0.0, 1.0)
}

#[unsafe(no_mangle)]
pub fn num_f64_ops(a: f64, b: f64) -> f64 {
    a * b + a / b + a.abs()
}

#[unsafe(no_mangle)]
pub fn array_ops(a: [u8; 8]) -> u8 {
    a.map(|x| x + 1).iter().sum::<u8>() + a.len() as u8
}

#[unsafe(no_mangle)]
pub fn hash_sip(a: u32) -> u64 {
    use core::hash::{Hash, Hasher};
    let mut h = core::hash::SipHasher::default();
    a.hash(&mut h);
    h.finish()
}

#[unsafe(no_mangle)]
pub fn dyn_dispatch(f: &dyn Fn(u16) -> u16) -> u16 {
    f(3) + f(4)
}

#[unsafe(no_mangle)]
pub fn dyn_fmt_display(v: &dyn fmt::Display) -> u16 {
    let mut s = Sink(0);
    let _ = write!(s, "{v}");
    s.0
}

macro_rules! integer_methods {
    ($name:ident, $ty:ty) => {
        #[unsafe(no_mangle)]
        pub fn $name(a: $ty, b: $ty) -> (Option<$ty>, Option<$ty>, Option<$ty>, $ty, $ty, $ty, ($ty, bool), ($ty, bool), ($ty, bool), u32, u32, u32) {
            (a.checked_add(b), a.checked_sub(b), a.checked_mul(b),
             a.wrapping_add(b), a.wrapping_sub(b), a.wrapping_mul(b),
             a.overflowing_add(b), a.overflowing_sub(b), a.overflowing_mul(b),
             a.leading_zeros(), a.trailing_zeros(), a.count_ones())
        }
    };
}

integer_methods!(methods_i8, i8);
integer_methods!(methods_u8, u8);
integer_methods!(methods_i16, i16);
integer_methods!(methods_u16, u16);
integer_methods!(methods_i32, i32);
integer_methods!(methods_u32, u32);
integer_methods!(methods_i64, i64);
integer_methods!(methods_u64, u64);
integer_methods!(methods_i128, i128);
integer_methods!(methods_u128, u128);
integer_methods!(methods_isize, isize);
integer_methods!(methods_usize, usize);
