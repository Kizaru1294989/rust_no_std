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

    pub unsafe fn initialize(heap_start: *mut u8, heap_size: usize) {
        let block_count = heap_size / ARENAS.len();
        let mut current = heap_start;
        for i in 0..ARENAS.len() {
            let block_size = (1 << (3 + i)) as usize; // 8, 16, 32, ...
            ARENAS[i] = Some(Arena::new(current, block_count, block_size));
            current = current.add(block_count * block_size);
        }
    }
}
