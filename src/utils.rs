/// Aligne une valeur à un multiple donné.
///
/// Cette fonction ajuste la valeur spécifiée (`value`) pour qu'elle soit un multiple
/// de l'alignement (`alignment`). Si `value` est déjà alignée, elle est retournée
/// inchangée.
///
/// # Arguments
///
/// - `value`: La valeur à aligner.
/// - `alignment`: L'alignement souhaité (doit être une puissance de 2).
///
/// # Returns
///
/// La valeur alignée au multiple le plus proche de `alignment`.
///
/// # Exemple
///
/// ```rust
/// use my_allocator::utils::align_to;
///
/// let aligned = align_to(13, 8);
/// assert_eq!(aligned, 16); // Le multiple de 8 supérieur à 13 est 16.
///
/// let already_aligned = align_to(16, 8);
/// assert_eq!(already_aligned, 16); // 16 est déjà aligné.
/// ```
pub fn align_to(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}
