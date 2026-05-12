# Enums & Pattern Matching - Module 06

## Objectifs

- Définir et utiliser des énumérations (`enum`)
- Maîtriser le pattern matching avec `match` et `if let`
- Comprendre `Option<T>` et `Result<T, E>`
- Utiliser les enums pour représenter des états multiples

## Résumé

| Concept | Description | Exemple |
|---------|-------------|----------|
| **Enum** | Type qui peut prendre plusieurs formes | `enum Couleur { Rouge, Vert, Bleu }` |
| **Variant** | Une des formes possibles | `Couleur::Rouge` |
| **Enum avec données** | Variant qui contient des valeurs | `enum Message { Ecrire(String), Quitter }` |
| **`match`** | Pattern matching exhaustif | `match c { Rouge => ..., _ => ... }` |
| **`if let`** | Match rapide pour 1 cas | `if let Some(x) = option { ... }` |
| **`Option<T>`** | Présent ou absent | `Some(5)` / `None` |
| **`Result<T, E>`** | Succès ou échec | `Ok(5)` / `Err("erreur")` |

## Vocabulaire

- *Enum* = énumération (type à plusieurs formes)
- *Variant* = variante (une des formes possibles)
- *Exhaustive* = doit couvrir tous les cas
- *Pattern matching* = comparaison par motifs
- *Destructuring* = extraire des données d'un enum
- *Option* = valeur optionnelle (Some/None)
- *Result* = résultat avec erreur (Ok/Err)
