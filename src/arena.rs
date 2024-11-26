use core::{ptr, mem};
use crate::config::BlockSize;

pub struct Arena {
    start: *mut u8,
    capacity: usize,
    block_size: usize,
    free_list: *mut FreeNode,
}

#[repr(C)]
struct FreeNode {
    next: *mut FreeNode,
}

impl Arena {
    pub unsafe fn new(start: *mut u8, capacity: usize, block_size: usize) -> Self {
        let mut arena = Self {
            start,
            capacity,
            block_size,
            free_list: ptr::null_mut(),
        };
        arena.initialize_free_list();
        arena
    }

    unsafe fn initialize_free_list(&mut self) {
        let mut current = self.start;
        for _ in 0..self.capacity / self.block_size {
            let next = current.add(self.block_size);
            (*(current as *mut FreeNode)).next = if next < self.start.add(self.capacity) {
                next as *mut FreeNode
            } else {
                ptr::null_mut()
            };
            current = next;
        }
        self.free_list = self.start as *mut FreeNode;
    }

    pub unsafe fn allocate(&mut self) -> *mut u8 {
        if self.free_list.is_null() {
            return ptr::null_mut();
        }
        let node = self.free_list;
        self.free_list = (*node).next;
        node as *mut u8
    }

    pub unsafe fn deallocate(&mut self, ptr: *mut u8) {
        let node = ptr as *mut FreeNode;
        (*node).next = self.free_list;
        self.free_list = node;
    }
}
