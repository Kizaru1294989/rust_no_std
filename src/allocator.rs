use core::alloc::{GlobalAlloc, Layout}; 
use core::ptr::null_mut; 

pub struct SimpleAllocator {
    memory: [u8; 1024], 
    offset: usize,      
}

