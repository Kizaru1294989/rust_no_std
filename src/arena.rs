use core::{ptr, mem};
use crate::config::BlockSize;

pub struct Arena {
    start: *mut u8,
    capacity: usize,
    block_size: usize,
    free_list: *mut FreeNode,
}

