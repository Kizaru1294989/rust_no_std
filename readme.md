
# Arborescence

src/
├── lib.rs           // Point d'entrée principal
├── arena.rs         // Gestionnaire des slabs
├── config.rs        // Définitions de constantes et tailles des blocs
├── memory.rs        // fonctionnalités principale pour l'allocation et libération de mémoire
└── utils.rs         // Fonctions utilitaires communes


# Slabs


## **Pourquoi un Allocateur Basé sur des Slabs ?**

* **Performance pour les petites allocations** : Les slabs sont optimisés pour des blocs de tailles fixes, ce qui accélère les opérations d'allocation et de libération.
* **Réduction de la fragmentation** : Les blocs de taille uniforme évitent les problèmes courants liés aux allocations dynamiques générales.

## **Architecture**

* **Segmentation mémoire** : Chaque taille (8, 16, 32, 64, etc.) est gérée dans une zone dédiée.
* **Liste chaînée** : Les blocs libres sont suivis via une liste chaînée pour des opérations rapides.
* **Préallocation** : Toute la mémoire est réservée au démarrage, garantissant un comportement prévisible.
