#![no_std]
#![feature(allocator_api, rustc_attrs)]

use core::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
use core::ptr::{NonNull, null_mut};

const HEAP_END: usize = 0xD000;
const CURSOR: *mut usize = 0xC7FE as *mut usize;

pub struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let cursor = unsafe { CURSOR.read_unaligned() };
        let Some(start) = cursor
            .checked_add(layout.align() - 1)
            .map(|address| address & !(layout.align() - 1))
        else {
            return null_mut();
        };
        let Some(end) = start.checked_add(layout.size()) else {
            return null_mut();
        };
        if end > HEAP_END {
            return null_mut();
        }
        unsafe { CURSOR.write_unaligned(end) };
        start as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

unsafe impl Allocator for BumpAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if layout.size() == 0 {
            let dangling = NonNull::new(layout.align() as *mut u8).ok_or(AllocError)?;
            return Ok(NonNull::slice_from_raw_parts(dangling, 0));
        }
        let ptr = NonNull::new(unsafe { self.alloc(layout) }).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
}

pub static HEAP: BumpAllocator = BumpAllocator;

#[rustc_std_internal_symbol]
pub unsafe fn __rust_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe { HEAP.alloc(Layout::from_size_align_unchecked(size, align)) }
}

#[rustc_std_internal_symbol]
pub unsafe fn __rust_dealloc(_ptr: *mut u8, _size: usize, _align: usize) {}

#[rustc_std_internal_symbol]
pub unsafe fn __rust_realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8 {
    let new = unsafe { __rust_alloc(new_size, align) };
    if !new.is_null() {
        unsafe { core::ptr::copy_nonoverlapping(ptr, new, old_size.min(new_size)) };
    }
    new
}

#[rustc_std_internal_symbol]
pub unsafe fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    let ptr = unsafe { __rust_alloc(size, align) };
    if !ptr.is_null() {
        unsafe { ptr.write_bytes(0, size) };
    }
    ptr
}

#[rustc_std_internal_symbol]
pub fn __rust_no_alloc_shim_is_unstable_v2() {}
