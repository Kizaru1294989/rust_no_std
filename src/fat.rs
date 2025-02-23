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

/// Structure FAT avec allocation dynamique du buffer via SlabMemory
#[derive(Debug, Copy, Clone)]
pub struct FAT<T>
where
    T: BlockDevice + Clone + Copy,
{
    device: T,
    fat_offset: usize,
    start_cluster: u32,
    previous_cluster: u32,
    pub(crate) current_cluster: u32,
    next_cluster: Option<u32>,
    buffer: &'static mut [u8; BUFFER_SIZE], // Allocation dynamique
}


impl<T> FAT<T>
where
    T: BlockDevice + Clone + Copy,
{
    /// Initialise un nouveau FAT avec allocation du buffer via SlabMemory
    pub(crate) fn new(cluster: u32, device: T, fat_offset: usize) -> Option<Self> {
        // Allocation dynamique du buffer
        let layout = Layout::new::<[u8; BUFFER_SIZE]>();

        unsafe {
            let buffer_ptr = SlabMemory::allocate(layout) as *mut [u8; BUFFER_SIZE];
            if buffer_ptr.is_null() {
                return None;
            }

            Some(Self {
                device,
                fat_offset,
                start_cluster: cluster,
                previous_cluster: 0,
                current_cluster: 0,
                next_cluster: None,
                buffer: &mut *buffer_ptr, // Cast en référence mutable
            })
        }
    }

    /// Recherche un cluster vide
    pub(crate) fn blank_cluster(&mut self) -> u32 {
        let mut cluster = 0;
        let mut done = false;

        for block in 0.. {
            self.device.read(self.buffer, self.fat_offset + block * BUFFER_SIZE, 1).ok();

            for i in (0..BUFFER_SIZE).step_by(4) {
                if read_le_u32(&self.buffer[i..i + 4]) == 0 {
                    done = true;
                    break;
                } else { 
                    cluster += 1; 
                }
            }
            if done { break; }
        }
        cluster
    }

    
}