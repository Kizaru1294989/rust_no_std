use core::ptr;

/// Une arène mémoire simple pour gérer les allocations de taille fixe.
///
/// L'arène gère une mémoire continue, divisée en blocs de taille fixe. Elle utilise une
/// liste chaînée pour suivre les blocs libres. Cela permet des allocations rapides
/// et des désallocations simples.
///
/// # Champs
/// - `start`: Adresse de début de la mémoire gérée.
/// - `capacity`: Nombre total d'octets dans l'arène.
/// - `block_size`: Taille de chaque bloc géré.
/// - `free_list`: Pointeur vers le premier bloc libre.
pub struct Arena {
    /// Pointeur vers le début de la mémoire de l'arène.
    start: *mut u8,
    /// Capacité totale en octets.
    capacity: usize,
    /// Taille de chaque bloc de mémoire.
    block_size: usize,
    /// Pointeur vers le premier bloc libre.
    free_list: *mut FreeNode,
}

/// Un nœud de la liste chaînée des blocs libres.
///
/// Chaque bloc libre contient un pointeur vers le bloc suivant, ou `null` s'il n'y en a pas.
#[repr(C)]
struct FreeNode {
    /// Pointeur vers le prochain bloc libre.
    next: *mut FreeNode,
}

impl Arena {
    /// Crée une nouvelle arène mémoire.
    ///
    /// # Arguments
    ///
    /// - `start`: Adresse de début de la mémoire gérée.
    /// - `capacity`: Capacité totale de la mémoire (en octets).
    /// - `block_size`: Taille de chaque bloc géré.
    ///
    /// # Safety
    ///
    /// L'appelant doit s'assurer que la mémoire pointée par `start` est valide
    /// et qu'elle est accessible en lecture et écriture pour une taille de `capacity` octets.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use my_allocator::Arena;
    /// unsafe {
    ///     let mut buffer = [0u8; 1024];
    ///     let arena = Arena::new(buffer.as_mut_ptr(), 1024, 32);
    /// }
    /// ```
    pub unsafe fn new(start: *mut u8, capacity: usize, block_size: usize) -> Self {
        let mut arena = Self {
            start,
            capacity,
            block_size,
            free_list: ptr::null_mut(),
        };
        arena.initialize_free_list();
        arena
    }

    /// Initialise la liste chaînée des blocs libres.
    ///
    /// Cette méthode divise la mémoire en blocs de taille `block_size` et les
    /// relie pour former une liste chaînée.
    ///
    /// # Safety
    ///
    /// Cette méthode modifie directement la mémoire pointée par `start`. Elle doit
    /// être appelée uniquement lorsque l'arène est correctement configurée.
    unsafe fn initialize_free_list(&mut self) {
        let mut current = self.start;
        for _ in 0..self.capacity / self.block_size {
            let next = current.add(self.block_size);
            (*(current as *mut FreeNode)).next = if next < self.start.add(self.capacity) {
                next as *mut FreeNode
            } else {
                ptr::null_mut()
            };
            current = next;
        }
        self.free_list = self.start as *mut FreeNode;
    }

    /// Alloue un bloc de mémoire depuis l'arène.
    ///
    /// Retourne un pointeur vers un bloc libre, ou `null_mut` si l'arène est pleine.
    ///
    /// # Safety
    ///
    /// L'appelant doit s'assurer que le pointeur retourné est utilisé correctement
    /// et désalloué en appelant [`deallocate`].
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use my_allocator::Arena;
    /// unsafe {
    ///     let mut buffer = [0u8; 1024];
    ///     let mut arena = Arena::new(buffer.as_mut_ptr(), 1024, 32);
    ///     let ptr = arena.allocate();
    ///     assert!(!ptr.is_null());
    /// }
    /// ```
    pub unsafe fn allocate(&mut self) -> *mut u8 {
        if self.free_list.is_null() {
            return ptr::null_mut();
        }
        let node = self.free_list;
        self.free_list = (*node).next;
        node as *mut u8
    }

    /// Désalloue un bloc de mémoire et le remet dans la liste des blocs libres.
    ///
    /// # Arguments
    ///
    /// - `ptr`: Pointeur vers le bloc à désallouer.
    ///
    /// # Safety
    ///
    /// L'appelant doit s'assurer que `ptr` a été obtenu via [`allocate`] et qu'il
    /// pointe vers un bloc valide de cette arène.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use my_allocator::Arena;
    /// unsafe {
    ///     let mut buffer = [0u8; 1024];
    ///     let mut arena = Arena::new(buffer.as_mut_ptr(), 1024, 32);
    ///     let ptr = arena.allocate();
    ///     arena.deallocate(ptr);
    /// }
    /// ```
    pub unsafe fn deallocate(&mut self, ptr: *mut u8) {
        let node = ptr as *mut FreeNode;
        (*node).next = self.free_list;
        self.free_list = node;
    }
}
