#![no_std] // Désactive la bibliothèque standard (std), car on est en mode embarqué.

use core::alloc::{GlobalAlloc, Layout}; // Gestion manuelle de la mémoire.
use core::fmt::Debug; // Trait nécessaire pour afficher des erreurs.
use memory::SlabMemory; // Importation de l'allocateur mémoire personnalisé.
use crate::BUFFER_SIZE; // Taille des buffers pour la lecture/écriture.
use crate::tool::read_le_u32; // Fonction utilitaire pour lire des entiers en little-endian.

/// 🔹 **Trait BlockDevice**
/// Définit une interface générique pour un périphérique de stockage en mode bloc.
/// Le périphérique doit pouvoir **lire** et **écrire** des blocs de données.
pub trait BlockDevice {
    type Error: Debug; // Définition d'un type d'erreur générique.

    fn read(&self, buffer: &mut [u8], offset: usize, blocks: usize) -> Result<(), Self::Error>;
    fn write(&self, buffer: &[u8], offset: usize, blocks: usize) -> Result<(), Self::Error>;
}

/// 🔹 **Structure FAT**
/// Gère la lecture et l'écriture des clusters dans la table d'allocation des fichiers.
#[derive(Debug, Copy, Clone)]
pub struct FAT<T>
where
    T: BlockDevice + Clone + Copy,
{
    device: T, // Périphérique de stockage.
    fat_offset: usize, // Offset où commence la table FAT.
    start_cluster: u32, // Cluster de départ.
    previous_cluster: u32, // Dernier cluster visité.
    pub(crate) current_cluster: u32, // Cluster actuel dans le parcours.
    next_cluster: Option<u32>, // Cluster suivant, s'il existe.
    buffer: &'static mut [u8; BUFFER_SIZE], // Buffer alloué dynamiquement avec `SlabMemory`.
}

/// 🔹 **Implémentation de FAT**
impl<T> FAT<T>
where
    T: BlockDevice + Clone + Copy,
{
    /// ✅ **Constructeur : `new()`**
    /// Initialise une nouvelle instance de FAT avec un **buffer alloué dynamiquement**.
    pub(crate) fn new(cluster: u32, device: T, fat_offset: usize) -> Option<Self> {
        let layout = Layout::new::<[u8; BUFFER_SIZE]>(); // Définition de la mémoire requise.

        unsafe {
            let buffer_ptr = SlabMemory::allocate(layout) as *mut [u8; BUFFER_SIZE];
            if buffer_ptr.is_null() {
                return None; // Retourne `None` si l'allocation a échoué.
            }

            Some(Self {
                device,
                fat_offset,
                start_cluster: cluster,
                previous_cluster: 0,
                current_cluster: 0,
                next_cluster: None,
                buffer: &mut *buffer_ptr, // Conversion en référence mutable.
            })
        }
    }

    /// ✅ **Recherche d'un cluster libre : `blank_cluster()`**
    /// Parcourt la FAT pour trouver une entrée vide (valeur `0`).
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

    /// ✅ **Écriture dans la FAT : `write()`**
    /// Écrit la valeur d'un cluster dans la table FAT.
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

    /// ✅ **Réinitialisation du parcours FAT : `refresh()`**
    /// Remet `current_cluster` à 0 et recommence à `start_cluster`.
    pub(crate) fn refresh(&mut self, start_cluster: u32) {
        self.current_cluster = 0;
        self.start_cluster = start_cluster;
    }

    /// ✅ **Retour au cluster précédent : `previous()`**
    /// Revient en arrière dans le parcours de la FAT.
    pub(crate) fn previous(&mut self) {
        if self.current_cluster != 0 {
            self.next_cluster = Some(self.current_cluster);
            self.current_cluster = self.previous_cluster;
        }
    }

    /// ✅ **Vérifie si `next_cluster` est `None`**
    pub(crate) fn next_is_none(&self) -> bool {
        self.next_cluster.is_none()
    }

    /// ✅ **Convertit `current_cluster` en `usize`**
    fn current_cluster_usize(&self) -> usize {
        self.current_cluster as usize
    }

    /// ✅ **Libère la mémoire allouée dynamiquement**
    /// Permet d'éviter les fuites mémoire en `no_std`.
    pub(crate) fn free(self) {
        let layout = Layout::new::<[u8; BUFFER_SIZE]>();
        unsafe {
            SlabMemory::deallocate(self.buffer as *mut [u8; BUFFER_SIZE] as *mut u8, layout);
        }
    }
}

/// 🔹 **Implémentation de l'itérateur pour FAT**
impl<T> Iterator for FAT<T>
where
    T: BlockDevice + Clone + Copy,
{
    type Item = Self;

    /// ✅ **Avance au cluster suivant**
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

        // Calcul des offsets pour accéder à la FAT
        let offset = self.current_cluster_usize() * 4;
        let block_offset = offset / BUFFER_SIZE;
        let offset_left = offset % BUFFER_SIZE;

        // Lecture du prochain cluster
        self.device.read(self.buffer, self.fat_offset + block_offset * BUFFER_SIZE, 1).ok();
        let next_cluster = read_le_u32(&self.buffer[offset_left..offset_left + 4]);

        // Si la valeur est 0x0FFFFFFF, c'est la fin de la chaîne.
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
