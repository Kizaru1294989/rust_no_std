#![no_std] 
#![feature(allocator_api)]

mod arena;
mod config;
mod memory;
mod utils;

use core::alloc::{GlobalAlloc, Layout};
use memory::SlabMemory;

pub struct SlabAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: SlabAllocator = SlabAllocator;

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        SlabMemory::allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        SlabMemory::deallocate(ptr, layout)
    }
}
