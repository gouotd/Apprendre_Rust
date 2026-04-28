# Variables & Types - Module 02

## Objectifs

- Comprendre `let` et la mutabilité (`mut`)
- Connaître les types primitifs (i32, u64, f64, bool, char, &str)
- Savoir typer explicitement et l'inférence de types
- Comprendre le shadowing et les constantes
- Maîtriser le cast avec `as`

## Résumé

| Concept | Description | Exemple |
|---------|-------------|----------|
| `let x = 5;` | Variable immuable | `x = 10` → erreur |
| `let mut x = 5;` | Variable muable | `x = 10` → OK |
| `let x: i32 = 5;` | Type explicite | `x: i32` |
| `const PI = 3.14;` | Constante compile-time | ne peut changer |
| Shadowing | Redéfinir une variable | `let x = 5; let x = 10;` |
| `as` | Conversion de type | `x as f64` |

## Vocabulaire

- **Variable binding** : Association nom ↔ valeur
- **Mutable** : Modifiable
- **Shadowing** : Écrase une variable existante
- **Type inference** : Déduction automatique du type
- **Casting** : Conversion explicite entre types