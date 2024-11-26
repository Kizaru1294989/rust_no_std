use core::alloc::Layout;
use core::mem::MaybeUninit;
use core::fmt::Write;
use crate::arena::Arena;
use crate::config::BlockSize;

/// Gestionnaire de mémoire utilisant une approche basée sur les slabs.
///
/// Ce gestionnaire divise la mémoire en plusieurs arènes (`Arena`), chaque arène
/// étant responsable de blocs de tailles fixes. Cela permet une allocation rapide
/// et efficace pour des tailles spécifiques.
pub struct SlabMemory;

/// Tableau contenant les arènes. Chaque arène gère des blocs de taille fixe.
///
/// Le tableau est initialisé dynamiquement à l'aide de `MaybeUninit`, car
/// `Option<Arena>` n'implémente pas `Copy`.
static mut ARENAS: MaybeUninit<[Option<Arena>; 8]> = MaybeUninit::uninit();

/// Structure pour écrire des messages de débogage.
///
/// Implémente le trait [`core::fmt::Write`] pour permettre des sorties
/// formatées via `write!` ou `writeln!`.
struct DebugWriter;

impl Write for DebugWriter {
    /// Écrit une chaîne de caractères dans un périphérique ou un buffer.
    ///
    /// # Safety
    ///
    /// Cette méthode utilise une fonction externe [`debug_putchar`] pour
    /// gérer la sortie des caractères.
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            unsafe { debug_putchar(c) };
        }
        Ok(())
    }
}

/// Fonction simulant l'envoi de caractères pour le débogage.
///
/// Cette fonction est un point d'entrée pour rediriger les sorties vers un
/// périphérique, comme un port série ou un buffer. Par défaut, elle est vide.
///
/// # Safety
///
/// L'appelant doit s'assurer que la sortie configurée est prête à recevoir
/// des données.
unsafe fn debug_putchar(byte: u8) {
    // Implémentez ici la logique pour écrire sur un périphérique.
}

impl SlabMemory {
    /// Alloue un bloc de mémoire basé sur le layout spécifié.
    ///
    /// Recherche une arène (`Arena`) adaptée à la taille demandée et retourne
    /// un pointeur vers un bloc libre. Si aucune arène n'est disponible ou si
    /// toutes les arènes sont pleines, retourne `null_mut`.
    ///
    /// # Arguments
    ///
    /// - `layout`: Spécifie la taille et l'alignement du bloc à allouer.
    ///
    /// # Returns
    ///
    /// Un pointeur vers le bloc alloué, ou `null_mut` en cas d'échec.
    ///
    /// # Safety
    ///
    /// L'appelant doit s'assurer que le pointeur retourné est utilisé
    /// correctement et désalloué lorsqu'il n'est plus nécessaire.
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

    /// Désalloue un bloc de mémoire précédemment alloué.
    ///
    /// Retourne le bloc à l'arène correspondante pour qu'il puisse être réutilisé.
    ///
    /// # Arguments
    ///
    /// - `ptr`: Pointeur vers le bloc à désallouer.
    /// - `layout`: Spécifie la taille et l'alignement du bloc.
    ///
    /// # Safety
    ///
    /// L'appelant doit s'assurer que `ptr` est un pointeur valide qui a été
    /// obtenu via [`SlabMemory::allocate`].
    pub unsafe fn deallocate(ptr: *mut u8, layout: Layout) {
        let arenas = ARENAS.assume_init_mut();

        if let Some(block_size) = BlockSize::categorize(layout.size()) {
            let index = block_size as usize / 8 - 1;
            if let Some(ref mut arena) = arenas[index] {
                arena.deallocate(ptr);
            }
        }
    }

    /// Initialise les arènes avec un espace mémoire donné.
    ///
    /// Divise la mémoire en blocs de tailles fixes et configure les arènes
    /// correspondantes. Chaque arène est associée à une taille de bloc spécifique.
    ///
    /// # Arguments
    ///
    /// - `heap_start`: Adresse de début de la mémoire gérée.
    /// - `heap_size`: Taille totale de la mémoire.
    ///
    /// # Safety
    ///
    /// L'appelant doit s'assurer que `heap_start` pointe vers une zone de
    /// mémoire valide et accessible, et que `heap_size` est suffisant pour
    /// initialiser toutes les arènes.
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

    /// Affiche l'état de chaque arène pour le débogage.
    ///
    /// Parcourt toutes les arènes et affiche si elles sont initialisées ou non.
    ///
    /// # Safety
    ///
    /// Cette méthode suppose que les arènes ont été correctement initialisées
    /// via [`SlabMemory::initialize`].
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
