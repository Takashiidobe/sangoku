#![no_core]
#![feature(no_core, const_trait_impl, step_trait)]

extern crate core;
extern crate alloc;
use core::prelude::rust_2024::*;
use core::alloc::Layout;
use core::iter::Step;

const fn layout_size(layout: Result<Layout, core::alloc::LayoutError>) -> usize {
    match layout { Ok(layout) => layout.size(), Err(_) => panic!() }
}

const _: () = {
    assert!(usize::BITS == 16);
    assert!(isize::BITS == 16);
    assert!(usize::MAX == 65535);
    assert!(isize::MAX == 32767);
    assert!(core::mem::size_of::<*const u8>() == 2);
    assert!(core::mem::size_of::<&[u8]>() == 4);
    assert!(core::mem::size_of::<[u8; 32767]>() == 32767);
    assert!(core::mem::size_of::<[u16; 16383]>() == 32766);
    assert!(core::mem::size_of::<[(); 65535]>() == 0);
    assert!(layout_size(Layout::array::<u8>(32767)) == 32767);
    assert!(Layout::array::<u8>(32768).is_err());
    assert!(layout_size(Layout::array::<u16>(16383)) == 32766);
    assert!(Layout::array::<u16>(16384).is_err());
    assert!(Layout::from_size_align(32766, 2).is_ok());
    assert!(Layout::from_size_align(32767, 2).is_err());
    assert!(Layout::from_size_align(1, 3).is_err());
    assert!(layout_size(Layout::array::<()>(65535)) == 0);
    let empty = alloc::vec::Vec::<u8>::new();
    assert!(empty.capacity() == 0);
    core::mem::forget(empty);
    let units = alloc::vec::Vec::<()>::new();
    assert!(units.capacity() == 65535);
    core::mem::forget(units);
    assert!(usize::MAX.checked_add(1).is_none());
    assert!(32768usize.checked_mul(2).is_none());
    assert!(isize::MIN.checked_sub(1).is_none());
    assert!(0x1234usize.to_ne_bytes()[0] == 0x34);
    assert!(0x1234usize.to_ne_bytes()[1] == 0x12);
    assert!(usize::MAX.to_ne_bytes()[0] == 255);
    assert!(usize::MAX.to_ne_bytes()[1] == 255);
    let (lower, upper) = <u32 as Step>::steps_between(&0, &66000);
    assert!(lower == 65535);
    assert!(upper.is_none());
    let (lower, upper) = <u16 as Step>::steps_between(&0, &65535);
    assert!(lower == 65535);
    assert!(upper.unwrap() == 65535);
};

const UNITS: &[()] = unsafe {
    core::slice::from_raw_parts(core::ptr::NonNull::<()>::dangling().as_ptr(), 65535)
};
const _: () = assert!(UNITS.len() == 65535);

pub fn collection_surface() {
    let mut values = alloc::vec::Vec::<usize>::new();
    let _ = values.try_reserve(16);
    let mut map = alloc::collections::BTreeMap::<usize, usize>::new();
    map.insert(1, 2);
}
