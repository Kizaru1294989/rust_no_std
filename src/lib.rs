#![no_std] 
#![feature(allocator_api)]

/// Module contenant l'implémentation des arènes mémoire.
mod arena;
/// Module contenant les définitions des tailles de blocs et leur catégorisation.
mod config;
/// Module principal gérant l'allocateur mémoire.
mod memory;
/// Module pour les fonctions utilitaires (vide ou à compléter selon les besoins).
mod utils;

use core::alloc::{GlobalAlloc, Layout};
use memory::SlabMemory;

/// Implémentation d'un allocateur global basé sur `SlabMemory`.
///
/// Cet allocateur utilise une approche basée sur des arènes (slabs) pour gérer
/// efficacement les allocations de tailles fixes. Il est configuré comme
/// l'allocateur global à travers l'attribut `#[global_allocator]`.
///
/// # Exemple
///
/// ```rust
/// use my_allocator::GLOBAL_ALLOCATOR;
/// use std::alloc::{alloc, dealloc, Layout};
///
/// let layout = Layout::from_size_align(32, 8).unwrap();
/// unsafe {
///     let ptr = alloc(layout);
///     assert!(!ptr.is_null());
///     dealloc(ptr, layout);
/// }
/// ```
pub struct SlabAllocator;

/// Déclare l'allocateur global pour le programme.
///
/// Grâce à l'attribut `#[global_allocator]`, toutes les allocations dans
/// le programme utilisent l'instance de `SlabAllocator` définie ici.
#[global_allocator]
static GLOBAL_ALLOCATOR: SlabAllocator = SlabAllocator;

unsafe impl GlobalAlloc for SlabAllocator {
    /// Alloue un bloc de mémoire avec le layout spécifié.
    ///
    /// # Arguments
    ///
    /// - `layout`: Layout spécifiant la taille et l'alignement du bloc.
    ///
    /// # Returns
    ///
    /// Retourne un pointeur vers le bloc alloué, ou `null_mut` en cas d'échec.
    ///
    /// # Safety
    ///
    /// L'appelant doit s'assurer que le pointeur retourné est utilisé correctement
    /// et désalloué via `dealloc` lorsqu'il n'est plus nécessaire.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        SlabMemory::allocate(layout)
    }

    /// Désalloue un bloc de mémoire précédemment alloué.
    ///
    /// # Arguments
    ///
    /// - `ptr`: Pointeur vers le bloc à désallouer.
    /// - `layout`: Layout correspondant au bloc.
    ///
    /// # Safety
    ///
    /// L'appelant doit s'assurer que `ptr` est valide et a été précédemment
    /// alloué via cet allocateur.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        SlabMemory::deallocate(ptr, layout)
    }
}
