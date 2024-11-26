/// Représente les tailles de blocs supportées par l'allocateur mémoire.
///
/// Chaque taille correspond à une catégorie utilisée pour organiser la mémoire
/// dans des zones (slabs). Les valeurs associées (ex. `8`, `16`, ...) indiquent
/// la taille réelle en octets des blocs de mémoire.
#[derive(Copy, Clone, Debug)]
pub enum BlockSize {
    /// Bloc de 8 octets, adapté pour les allocations très petites.
    Tiny = 8,
    /// Bloc de 16 octets, adapté pour les allocations petites.
    Small = 16,
    /// Bloc de 32 octets, adapté pour les allocations moyennes.
    Medium = 32,
    /// Bloc de 64 octets, adapté pour les allocations légèrement grandes.
    Large = 64,
    /// Bloc de 128 octets, adapté pour les allocations grandes.
    Huge = 128,
    /// Bloc de 256 octets, adapté pour les allocations très grandes.
    Giant = 256,
    /// Bloc de 512 octets, adapté pour les allocations énormes.
    Colossal = 512,
    /// Bloc de 1024 octets, adapté pour les allocations massives.
    Mammoth = 1024,
}

impl BlockSize {
    /// La taille maximale d'un bloc en octets.
    ///
    /// Cette constante est utilisée pour identifier la plus grande catégorie
    /// supportée par l'allocateur. Elle est définie comme la taille associée à
    /// [`BlockSize::Mammoth`].
    pub const MAX: usize = BlockSize::Mammoth as usize;

    /// Catégorise une taille en octets en fonction des tailles supportées.
    ///
    /// # Arguments
    ///
    /// - `size`: La taille en octets à catégoriser.
    ///
    /// # Returns
    ///
    /// Retourne une option contenant la catégorie correspondante si elle existe,
    /// ou `None` si la taille dépasse la limite supportée.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_allocator::BlockSize;
    ///
    /// assert_eq!(BlockSize::categorize(10), Some(BlockSize::Small));
    /// assert_eq!(BlockSize::categorize(500), Some(BlockSize::Colossal));
    /// assert_eq!(BlockSize::categorize(1500), None); // Taille non supportée
    /// ```
    pub fn categorize(size: usize) -> Option<Self> {
        match size {
            1..=8 => Some(BlockSize::Tiny),
            9..=16 => Some(BlockSize::Small),
            17..=32 => Some(BlockSize::Medium),
            33..=64 => Some(BlockSize::Large),
            65..=128 => Some(BlockSize::Huge),
            129..=256 => Some(BlockSize::Giant),
            257..=512 => Some(BlockSize::Colossal),
            513..=1024 => Some(BlockSize::Mammoth),
            _ => None,
        }
    }
}
