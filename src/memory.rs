use core::alloc::Layout;
use crate::arena::Arena;
use crate::config::BlockSize;

pub struct SlabMemory;

static mut ARENAS: [Option<Arena>; 8] = [None; 8];


