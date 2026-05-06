# Cheatsheet - Module 02: Variables & Types

## Variables
```rust
let x = 5;           // Immutable (fixe)
let mut x = 5;       // Mutable (modifiable)
let x: i32 = 5;      // Type explicite
const PI: f64 = 3.14; // Constante (compile-time)
```

## Types primitifs
| Type | Description | Exemple |
|------|-------------|---------|
| `i8`, `i16`, `i32`, `i64` | Entiers signés | `-5`, `0`, `42` |
| `u8`, `u16`, `u32`, `u64` | Entiers non-signés | `0`, `100` |
| `f32`, `f64` | Flottants | `3.14` |
| `bool` | Booléen | `true`, `false` |
| `char` | Caractère Unicode | `'a'`, `'@'` |
| `&str` | Référence vers chaîne | `"hello"` |

## Opérations
```rust
let somme = 5 + 3;        // Addition
let diff = 5 - 3;         // Soustraction
let produit = 5 * 3;      // Multiplication
let quotient = 10 / 3;     // Division entière → 3
let reste = 10 % 3;        // Modulo → 1
let decimal = 10.0 / 3.0;  // Division float → 3.333
```

## Shadowing
```rust
let x = 5;
let x = x + 1;    // x = 6 (nouvelle variable, même nom)
let x = "hello";  // x = "hello" (peut changer de type)
```

## Conversion (casting)
```rust
let x: i32 = 5;
let y: f64 = x as f64;  // i32 → f64

let z: f64 = 5.9;
let w: i32 = z as i32;  // f64 → i32 (tronque: 5)
```

## let vs const
| | `let` | `const` |
|---|-------|---------|
| Quand ? | À l'exécution (runtime) | À la compilation |
| Peut changer ? | Non (sauf si `mut`) | Jamais |
| Typage requis | Non (inféré) | Obligatoire |
| Exemple | `let x = input();` | `const PI: f64 = 3.14;` |

## Vocabulaire
- *Variable binding* = association nom ↔ valeur
- *Mutable* = modifiable
- *Immutable* = non-modifiable
- *Shadowing* = recouvrir une variable existante
- *Type inference* = déduction automatique du type
- *Casting* = conversion explicite entre types
