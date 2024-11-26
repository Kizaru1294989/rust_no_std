#![no_std] 
#![feature(allocator_api)]

mod arena;
mod config;
mod memory;
mod utils;

use core::alloc::{GlobalAlloc, Layout};
use memory::SlabMemory;
