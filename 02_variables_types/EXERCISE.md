# Exercice - Module 02

## Objectif

Maîtriser les variables, types primitifs et conversions en Rust.

---

## Étape 1 : Préparation

Si ce n'est pas déjà fait, crée un nouveau projet :

```bash
cargo new variables_types
cd variables_types
```

---

## Étape 2 : Types primitifs

Rust a plusieurs types primitifs. Voici les principaux :

| Type | Description | Exemple |
|------|-------------|---------|
| `i32` | Entier signé 32 bits | `-5`, `0`, `42` |
| `u32` | Entier non-signé 32 bits | `0`, `100` (pas de négatif) |
| `i64`, `u64` | Entiers 64 bits | Grands nombres |
| `f64` | Float 64 bits | `3.14` |
| `bool` | Booléen | `true`, `false` |
| `char` | Caractère Unicode | `'a'`, `'@'` |
| `&str` | Référence vers une chaîne | `"hello"` |

---

## Étape 3 : Expérimente

Pour chaque section, modifie `src/main.rs`, teste avec `cargo run`, puis passe à la suivante.

---

### 3.1. var et mutabilité

**Explication :** Par défaut, une variable en Rust est **immutable** (non-modifiable). Pour la rendre modifiable, on ajoute `mut`.

```rust
fn main() {
    let x = 5;      // immutable (fixe)
    let mut y = 5;  // mutable (modifiable)
    
    y = 10;         // OK car mut
    // x = 10;     // ERREUR: cannot assign twice to immutable variable
}
```

**Vocabulaire :** *mutable* = modifiable, *immutable* = non-modifiable.

**Exercice à faire :**
1. Crée une variable `-compte` avec `mut` valant 100
2. Ajoute 50 (compte = compte + 50)
3. Affiche le résultat

---

### 3.2. Type explicite

**Explication :** On peut指定 le type avec `: type`. Utile pour clarifier ou quand Rust ne devine pas.

```rust
let x: i32 = 5;
let name: &str = "Alice";
let actif: bool = true;
```

**Exercice à faire :**
1. Crée une variable `age` de type `i32` valant 25
2. Crée une variable `nom` de type `&str` valant "Bob"
3. Affiche-les

---

### 3.3. Constantes

**Explication :** Une `const` est définie à la compilation et ne peut jamais changer. Toujours en majuscules avec des underscores.

```rust
const MAX_SCORE: i32 = 100;
const PI: f64 = 3.14159;
```

**Vocabulaire :** *const* = constante (compile-time, immuable).

**Différence avec `let`** :
- `let` = peut être calculé à runtime
- `const` = doit être connu à la compilation

**Exercice à faire :**
1. Crée une constante `TAXE` de type `f64` valant 0.20 (20%)
2. Calcule le prix TTC d'un produit à 100€ avec cette taxe
3. Affiche le résultat

---

### 3.4. Shadowing

**Explication :** On peut réutiliser le même nom avec `let`. La nouvelle variable "Cache" l'ancienne.

```rust
fn main() {
    let x = 5;
    println!("{}", x);  // 5
    
    let x = x + 10;
    println!("{}", x);  // 15
}
```

**Vocabulaire :** *Shadowing* = masks en anglais = occulter/recouvrir.

**Exercice à faire :**
1. Crée une variable `message` valant "Hello"
2. Crée une nouvelle variable `message` valant "World" (shadowing)
3. Affiche la nouvelle valeur

---

### 3.5. Inferrence de type

**Explication :** Rust devine souvent le type automatiquement (type inference).

```rust
let x = 5;        // Rust devine i32
let pi = 3.14;     // Rust devine f64
let actif = true;   // Rust devine bool
```

**Exercice à faire :**
1. Laisse Rust deviner le type de ces valeurs :
   - `42` → deviné comme ?
   - `3.14` → deviné comme ?
   - `true` → deviné comme ?
2. Utilise `println!("{:?}", variable)` pour voir le type

Aide : Le format `{:?}` affiche la valeur avec son type.

---

### 3.6. Conversion (casting)

**Explication :** Pour convertir un type en un autre, on utilise `as`.

```rust
let x: i32 = 5;
let y: f64 = x as f64;  // i32 → f64

let z: f64 = 5.9;
let w: i32 = z as i32;  // f64 → i32 (tronque: 5)
```

**Vocabulaire :** *Cast* = conversion de type, *truncate* = couper les décimales.

**Attention :** La conversion float → int **tronque** (5.9 devient 5, pas 6).

**Exercice à faire :**
1. Convertis `100` (i32) en f64 et affiche
2. Convertis `5.9` (f64) en i32 et affiche - que vois-tu ?
3. Pourcentage : calcule 80% de 200 (utilise le cast)

---

## Checklist

- [ ] let et mutabilité compris
- [ ] Type explicite : exercice fait
- [ ] Constantes : exercice fait
- [ ] Shadowing : exercice fait
- [ ] Inference de type : exercice fait
- [ ] Casting (as) : exercice fait