# Fonctions & Contrôle de Flux - Module 03

## Objectifs

- Définir des fonctions avec `fn`
- Comprendre les expressions vs instructions
- Maîtriser `if/else`, `match`
- Utiliser les boucles : `for`, `while`, `loop`
- Retourner des valeurs avec `return` et les expressions

## Résumé

| Concept | Description | Exemple |
|---------|-------------|----------|
| `fn nom()` | Définir une fonction | `fn add(x: i32, y: i32) -> i32` |
| `if/else` | Conditionnel | `if x > 5 { ... } else { ... }` |
| `match` | Pattern matching | `match x { 1 => ..., _ => ... }` |
| `for` | Boucle sur un range | `for i in 0..5 { ... }` |
| `while` | Boucle conditionnelle | `while x > 0 { ... }` |
| `loop` | Boucle infinie | `loop { break; }` |
| Expression | Retourne une valeur | `let x = if y { 1 } else { 2 };` |

## Vocabulaire

- **Function** : Bloc de code réutilisable
- **Expression** : Retourne une valeur
- **Statement** : Instruction (ne retourne rien)
- **Pattern matching** : Comparaison structurée
- **Branch** : Branch d'un match