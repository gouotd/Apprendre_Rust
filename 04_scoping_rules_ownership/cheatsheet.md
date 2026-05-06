# Cheatsheet - Module 04: Scoping & Ownership

## 3 Règles de l'Ownership
1. Chaque valeur a un propriétaire (une variable)
2. Il ne peut y avoir qu'un seul propriétaire à la fois
3. Quand le propriétaire sort du scope, la valeur est supprimée

## Move (transfert de possession)
```rust
let s1 = String::from("Bonjour");
let s2 = s1;        // s1 n'est plus valide
println!("{}", s2); // OK
// println!("{}", s1); // ERREUR: use of moved value
```

## Copy (copie automatique)
```rust
let x = 5;
let y = x;          // x reste valide (Copy)
println!("{}", x);  // OK
```

## Clone (copie explicite)
```rust
let s1 = String::from("Bonjour");
let s2 = s1.clone(); // s1 et s2 sont tous deux valides
println!("{}, {}", s1, s2);
```

## Copy vs Move
| Types **Copy** | Types **Move** |
|----------------|----------------|
| `i32`, `u32`, `f64` | `String` |
| `bool`, `char` | `Vec<T>` |
| `&T` (références) | `HashMap<K, V>` |

## Borrowing (emprunt)
```rust
let s1 = String::from("Bonjour");
let len = calculer(&s1);  // Emprunt (lecture seule)
println!("{}", s1);       // s1 toujours valide

fn calculer(s: &String) -> usize {
    s.len()
}
```

## Mutable borrow
```rust
let mut s = String::from("Bonjour");
modifier(&mut s);  // Emprunt modifiable
println!("{}", s); // "Bonjour;"

fn modifier(s: &mut String) {
    s.push_str(";");
}
```

## Règles du borrow checker
| Emprunt | Combien en même temps ? |
|---------|------------------------|
| `&` (lecture seule) | Autant que tu veux |
| `&mut` (modifiable) | **Un seul, et aucun autre** |

## Limiter le scope d'un borrow
```rust
let mut texte = String::from("bonjour");

{
    let borrow = &mut texte;
    borrow.make_ascii_uppercase();
} // borrow détruit ici

println!("{}", texte);  // OK maintenant
```

## Lifetimes
```rust
fn plus_long<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```
`'a` n'est pas une durée. C'est une **étiquette de scope** qui garantit qu'une référence ne vit pas plus longtemps que la donnée qu'elle pointe.

## 3 façons de contourner le Move
| Approche | Coût | Les deux valides ? | Usage |
|----------|------|-------------------|-------|
| Move | Gratuit | Non | Par défaut |
| Clone | Coûteux | Oui | Deux copies indépendantes |
| Borrow | Gratuit | Oui | Juste lire/modifier |

## Vocabulaire
- *Ownership* = possession (qui possède la donnée)
- *Borrow* = emprunt (utiliser sans posséder)
- *Move* = transfert de possession
- *Copy* = copie automatique (types primitifs)
- *Lifetime* = durée de vie d'une référence (`'a`)
- *Scope* = bloc de visibilité (`{ }`)
- *Heap* = mémoire dynamique
- *Stack* = mémoire statique
- *Dangling reference* = référence vers mémoire libérée
