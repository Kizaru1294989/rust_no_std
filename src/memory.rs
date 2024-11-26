use core::alloc::Layout;
use crate::arena::Arena;
use crate::config::BlockSize;

pub struct SlabMemory;

static mut ARENAS: [Option<Arena>; 8] = [None; 8];

impl SlabMemory {
    pub unsafe fn allocate(layout: Layout) -> *mut u8 {
        if let Some(block_size) = BlockSize::categorize(layout.size()) {
            let index = block_size as usize / 8 - 1;
            if let Some(ref mut arena) = ARENAS[index] {
                return arena.allocate();
            }
        }
        core::ptr::null_mut()
    }

    pub unsafe fn deallocate(ptr: *mut u8, layout: Layout) {
        if let Some(block_size) = BlockSize::categorize(layout.size()) {
            let index = block_size as usize / 8 - 1;
            if let Some(ref mut arena) = ARENAS[index] {
                arena.deallocate(ptr);
            }
        }
    }


}
