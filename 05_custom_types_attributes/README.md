# Custom Types & Attributes - Module 05

## Objectifs

- Définir des structures (`struct`) avec des champs nommés ou tuples
- Implémenter des méthodes avec `impl`
- Utiliser les attributs `#[derive(Debug, Clone)]`
- Différence entre `self`, `&self`, `&mut self`
- Méthodes associées (sans `self`)

## Résumé

| Concept | Description | Exemple |
|---------|-------------|----------|
| **Struct** | Type personnalisé avec champs | `struct Point { x: i32, y: i32 }` |
| **Tuple struct** | Struct sans noms de champs | `struct Point(i32, i32)` |
| **Unit struct** | Struct sans champs | `struct Marker;` |
| **impl** | Bloc de méthodes pour une struct | `impl Point { fn new() {} }` |
| **`&self`** | Méthode lecture seule | `fn afficher(&self)` |
| **`&mut self`** | Méthode modifiable | `fn changer(&mut self)` |
| **`self`** | Méthode qui prend possession | `fn detruire(self)` |
| **Méthode associée** | Fonction sans `self` | `fn new() -> Self` |
| **`#[derive]`** | Génère automatiquement des traits | `#[derive(Debug, Clone)]` |

## Vocabulaire

- **Struct** = structure (type personnalisé)
- **Field** = champ (donnée dans une struct)
- **Method** = méthode (fonction attachée à une struct)
- **Associated function** = fonction associée (sans `self`)
- **Attribute** = métadonnée (`#[derive]`, `#[allow]`)
- **Tuple struct** = struct avec champs positionnels
- **Unit struct** = struct vide (sans données)
- **Constructor** = constructeur (convention `new()`)
