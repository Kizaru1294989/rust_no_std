Ryan Rais

# Arborescence

* src/
* lib.rs           # Point d'entrée principal
* arena.rs         # Gestionnaire des slabs (zones mémoire dédiées aux tailles fixes)
* config.rs        # Définitions des constantes et tailles des blocs
* memory.rs        # Fonctionnalités principales pour l'allocation et la libération de mémoire
* utils.rs         # Fonctions utilitaires communes (ex. alignement)

# Slabs

## **Pourquoi un Allocateur Basé sur des Slabs ?**

* **Performance pour les petites allocations** : Les slabs sont optimisés pour des blocs de tailles fixes, ce qui accélère les opérations d'allocation et de libération.
* **Réduction de la fragmentation** : Les blocs de taille uniforme évitent les problèmes courants liés aux allocations dynamiques générales.

## **Architecture**

* **Segmentation mémoire** : Chaque taille (8, 16, 32, 64, etc.) est gérée dans une zone dédiée.
* **Liste chaînée** : Les blocs libres sont suivis via une liste chaînée pour des opérations rapides.
* **Préallocation** : Toute la mémoire est réservée au démarrage, garantissant un comportement prévisible.

## **Source**

* https://docs.rs/slab/latest/slab/struct.Slab.html
* https://github.com/SFBdragon/talc
* https://github.com/daniel5151/libc_alloc
* https://bd103.github.io/blog/2023-06-27-global-allocators
* https://siliconislandblog.wordpress.com/2022/04/24/writing-a-no_std-compatible-cratein-rust/
* https://github.com/hobofan/cargo-nono
