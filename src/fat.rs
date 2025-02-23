#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::fmt::Debug;
use memory::SlabMemory; // Utilisation de SlabMemory pour gérer les allocations
use crate::BUFFER_SIZE;
use crate::tool::read_le_u32;

/// Trait `BlockDevice` compatible `no_std`
pub trait BlockDevice {
    type Error: Debug;
    
    fn read(&self, buffer: &mut [u8], offset: usize, blocks: usize) -> Result<(), Self::Error>;
    fn write(&self, buffer: &[u8], offset: usize, blocks: usize) -> Result<(), Self::Error>;
}

