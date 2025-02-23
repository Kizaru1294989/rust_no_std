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
    pub(crate) fn write(&mut self, cluster: u32, value: u32) {
        let offset = (cluster as usize) * 4;
        let block_offset = offset / BUFFER_SIZE;
        let offset_left = offset % BUFFER_SIZE;
        let offset = self.fat_offset + block_offset * BUFFER_SIZE;
        let mut value: [u8; 4] = value.to_le_bytes();

        self.device.read(self.buffer, offset, 1).ok();
        self.buffer[offset_left..offset_left + 4].copy_from_slice(&value);
        self.device.write(self.buffer, offset, 1).ok();
    }

    /// Réinitialise le parcours de la FAT
    pub(crate) fn refresh(&mut self, start_cluster: u32) {
        self.current_cluster = 0;
        self.start_cluster = start_cluster;
    }

    /// Revenir au cluster précédent
    pub(crate) fn previous(&mut self) {
        if self.current_cluster != 0 {
            self.next_cluster = Some(self.current_cluster);
            self.current_cluster = self.previous_cluster;
        }
    }

    /// Vérifie si `next_cluster` est `None`
    pub(crate) fn next_is_none(&self) -> bool {
        self.next_cluster.is_none()
    }

    fn current_cluster_usize(&self) -> usize {
        self.current_cluster as usize
    }

    /// Libère la mémoire du buffer lorsque FAT n'est plus utilisé
    pub(crate) fn free(self) {
        let layout = Layout::new::<[u8; BUFFER_SIZE]>();
        unsafe {
            SlabMemory::deallocate(self.buffer as *mut [u8; BUFFER_SIZE] as *mut u8, layout);
        }
    }
}

impl<T> Iterator for FAT<T>
where
    T: BlockDevice + Clone + Copy,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_cluster == 0 {
            self.current_cluster = self.start_cluster;
        } else {
            let next_cluster = self.next_cluster;
            if let Some(next_cluster) = next_cluster {
                self.previous_cluster = self.current_cluster;
                self.current_cluster = next_cluster;
            } else {
                return None;
            }
        }

        let offset = self.current_cluster_usize() * 4;
        let block_offset = offset / BUFFER_SIZE;
        let offset_left = offset % BUFFER_SIZE;

        self.device.read(self.buffer, self.fat_offset + block_offset * BUFFER_SIZE, 1).ok();

        let next_cluster = read_le_u32(&self.buffer[offset_left..offset_left + 4]);
        self.next_cluster = if next_cluster == 0x0FFFFFFF {
            None
        } else {
            Some(next_cluster)
        };

        Some(Self {
            next_cluster: self.next_cluster,
            ..(*self)
        })
    }
    
}