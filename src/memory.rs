use core::alloc::Layout;
use core::mem::MaybeUninit;
use core::fmt::Write;
use crate::arena::Arena;
use crate::config::BlockSize;

pub struct SlabMemory;

static mut ARENAS: MaybeUninit<[Option<Arena>; 8]> = MaybeUninit::uninit();

struct DebugWriter;

impl Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            unsafe { debug_putchar(c) };
        }
        Ok(())
    }
}


unsafe fn debug_putchar(byte: u8) {
}

impl SlabMemory {
    pub unsafe fn allocate(layout: Layout) -> *mut u8 {
        let arenas = ARENAS.assume_init_mut();

        if let Some(block_size) = BlockSize::categorize(layout.size()) {
            let index = block_size as usize / 8 - 1;
            if let Some(ref mut arena) = arenas[index] {
                return arena.allocate();
            }
        }
        core::ptr::null_mut()
    }

    pub unsafe fn deallocate(ptr: *mut u8, layout: Layout) {
        let arenas = ARENAS.assume_init_mut();

        if let Some(block_size) = BlockSize::categorize(layout.size()) {
            let index = block_size as usize / 8 - 1;
            if let Some(ref mut arena) = arenas[index] {
                arena.deallocate(ptr);
            }
        }
    }


    pub unsafe fn initialize(heap_start: *mut u8, heap_size: usize) {
        let mut temp_arenas: [Option<Arena>; 8] = [None, None, None, None, None, None, None, None];
        let block_count = heap_size / temp_arenas.len();
        let mut current = heap_start;

        for i in 0..temp_arenas.len() {
            let block_size = (1 << (3 + i)) as usize; // 8, 16, 32, ...
            temp_arenas[i] = Some(Arena::new(current, block_count, block_size));
            current = current.add(block_count * block_size);
        }

        ARENAS.write(temp_arenas);
    }


    pub unsafe fn debug_print() {
        let arenas = ARENAS.assume_init_mut();
        let mut writer = DebugWriter;

        for (i, arena) in arenas.iter().enumerate() {
            if arena.is_some() {
                let _ = write!(writer, "Arena {}: Initialized\n", i);
            } else {
                let _ = write!(writer, "Arena {}: Not initialized\n", i);
            }
        }
    }
}
