#![no_core]
#![feature(no_core, nonzero_strict_ops, random, signed_bigint_helpers, wrapping_int_impl)]

extern crate core;

use core::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex};
use core::num::{NonZeroU128, Saturating, Wrapping};

fn integer<T: Binary + Debug + Display + LowerExp + LowerHex + Octal + UpperExp + UpperHex>() {}
fn float<T: Debug + Display + LowerExp + UpperExp>() {}
fn error<T: core::error::Error>() {}

pub fn restored_apis() {
    integer::<i8>();
    integer::<u8>();
    integer::<i16>();
    integer::<u16>();
    integer::<i32>();
    integer::<u32>();
    integer::<i64>();
    integer::<u64>();
    integer::<i128>();
    integer::<u128>();
    integer::<isize>();
    integer::<usize>();
    float::<f32>();
    float::<f64>();
    error::<core::time::TryFromFloatSecsError>();
    let _ = core::time::Duration::from_secs_f64(1.25);
    let _ = 1u64.carrying_mul_add(2, 3, 4);
    let _ = 1u128.carrying_mul_add(2, 3, 4);
    let _ = 1i64.carrying_mul_add(2, 3, 4);
    let _ = 1i128.carrying_mul_add(2, 3, 4);
    let _ = NonZeroU128::new(3).unwrap().checked_pow(4);
    let _ = NonZeroU128::new(3).unwrap().strict_pow(4);
    let _ = NonZeroU128::new(3).unwrap().saturating_pow(4);
    let _ = Wrapping(3u128).pow(4);
    let _ = Saturating(3u128).pow(4).reverse_bits().to_be();
}

pub fn random_source<T: core::random::Rng>(source: &mut T) {
    source.fill_bytes(&mut [0; 8]);
}

const _: [(); 81] = [(); 3u32.pow(4) as usize];
const _: [(); 81] = [(); 3i128.pow(4) as usize];
const _: [(); 81] = [(); 3u128.checked_pow(4).unwrap() as usize];
