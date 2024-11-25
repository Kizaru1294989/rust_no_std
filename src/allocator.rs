use core::alloc::{GlobalAlloc, Layout}; 
use core::ptr::null_mut; 

pub struct SimpleAllocator {
    memory: [u8; 1024], 
    offset: usize,      
}


pub trait Allocator<T> {
    type AllocatedMemory : super::AllocatedSlice<T>;
    fn alloc_cell(&mut self, len : usize) -> Self::AllocatedMemory;
    fn free_cell(&mut self, data : Self::AllocatedMemory);
}