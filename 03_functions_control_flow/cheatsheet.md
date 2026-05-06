# Cheatsheet - Module 03: Functions & Control Flow

## Fonctions
```rust
fn nom(param1: i32, param2: &str) -> i32 {
    // Corps de la fonction
    param1  // Retour implicite (dernière expression)
}

// Appel
let resultat = nom(5, "hello");
```

## if/else
```rust
if age >= 18 {
    println!("Majeur");
} else if age >= 12 {
    println!("Adolescent");
} else {
    println!("Enfant");
}

// Comme expression
let statut = if age >= 18 { "majeur" } else { "mineur" };
```

## Opérateurs logiques
| Opérateur | Signification | Exemple |
|-----------|---------------|---------|
| `&&` | ET (and) | `age >= 18 && actif` |
| `\|\|` | OU (or) | `age < 18 \|\| senior` |
| `!` | NON (not) | `!actif` |

## match
```rust
match jour {
    1 => println!("Lundi"),
    2 | 3 => println!("Mardi ou Mercredi"),
    4..=6 => println!("Entre 4 et 6"),
    _ => println!("Autre"),  // Catch-all obligatoire
}
```

## Boucles
```rust
// for sur range
for i in 0..5 { println!("{}", i); }   // 0,1,2,3,4
for i in 0..=5 { println!("{}", i); }  // 0,1,2,3,4,5

// for sur array
let notes = [85, 92, 78];
for note in notes { println!("{}", note); }

// while
while compteur > 0 {
    compteur -= 1;
}

// loop (infinie)
loop {
    if condition { break resultat; }
    // continue pour passer à l'itération suivante
}
```

## Array vs Slice
```rust
let tableau: [i32; 4] = [1, 2, 3, 4];    // Taille fixe
let tranche: &[i32] = &tableau;           // Référence (slice)

let vecteur = vec![1, 2, 3, 4];           // Taille dynamique
let tranche2: &[i32] = &vecteur;          // Fonctionne aussi
```

## Expressions vs Statements
| | Expression | Statement |
|---|------------|-----------|
| Retourne une valeur ? | Oui | Non |
| Exemple | `if x > 5 { 10 } else { 0 }` | `let x = 5;` |
| Peut être assigné ? | Oui | Non |

## Vocabulaire
- *Function* = bloc de code réutilisable
- *Expression* = retourne une valeur
- *Statement* = instruction (ne retourne rien)
- *Pattern matching* = comparaison de motifs
- *Branch* = branche d'un match
- *Catch-all* = attrape-tout (`_`)
- *Range* = plage (`0..5` ou `0..=5`)
- *Iterator* = itérateur
