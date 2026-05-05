# Scoping Rules & Ownership - Module 04

## Objectifs

- Comprendre le scope et la durée de vie des variables
- Maîtriser l'ownership (possession)
- Comprendre le borrowing (emprunt) avec `&` et `&mut`
- Différence entre move et copy
- Introduction aux lifetimes (durées de vie)

## Résumé

| Concept | Description | Exemple |
|---------|-------------|----------|
| **Scope** | Bloc où la variable existe | `let x = 5;` dans `{ }` |
| **Ownership** | Une variable possède sa valeur | `let s = String::from("hi");` |
| **Move** | Transfert de possession | `let s2 = s;` (s n'est plus valide) |
| **Copy** | Copie automatique (types simples) | `let y = x;` (x reste valide) |
| **Borrow** | Emprunt avec `&` (lecture seule) | `fn f(x: &i32)` |
| **Mutable borrow** | Emprunt modifiable avec `&mut` | `fn f(x: &mut String)` |
| **Lifetime** | Durée de vie d'une référence | `fn f<'a>(x: &'a str) -> &'a str` |

## Vocabulaire

- **Ownership** = possession (qui "possède" la donnée)
- **Borrow** = emprunt (utiliser sans posséder)
- **Move** = transfert de possession
- **Copy** = copie automatique (types primitifs)
- **Lifetime** = durée de vie d'une référence (`'a`)
- **Scope** = bloc de visibilité (`{ }`)
- **Heap** = mémoire dynamique (allocation runtime)
- **Stack** = mémoire statique (allocation compile-time)