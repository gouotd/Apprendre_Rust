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
1. Crée une variable `compte` avec `mut` valant 100
2. Ajoute 50 (compte = compte + 50)
3. Affiche le résultat

---

### 3.2. Type explicite

**Explication :** On peut spécifier le type avec `: type`. Utile pour clarifier ou quand Rust ne devine pas.

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

**Différence entre `let` et `const` :**

| | `let` | `const` |
|---|-------|---------|
| Quand est connu ? | À l'exécution (runtime) | À la compilation |
| Peut changer ? | Non (sauf si `mut`) | Jamais |
| Typage requis | Non (inféré) | Obligatoire |
| Exemple | `let x = input();` | `const PI: f64 = 3.14;` |

En résumé : `let` peut recevoir une valeur pendant que le programme tourne. `const` est gravé dans le binaire à la compilation et ne peut pas changer.

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

> **💡 À propos du shadowing :**
> 
> L'ancienne variable n'est pas supprimée, mais devient inaccessible. La mémoire sera libérée quand on sortira du scope.
> 
> **Qu'est-ce que "sortir du scope" ?**
> Le scope est le bloc `{ }` où la variable existe. Quand on quitte ce bloc (fin de la fonction, fin d'une boucle, etc.), la variable est détruite et sa mémoire est libérée.
> 
> **Pourquoi c'est utile ?**
> - **Changer de type** : `let x = "42"; let x = x.parse::<i32>();` (texte → nombre)
> - **Garder l'immutabilité** : `let x = 5; let x = x + 1;` au lieu de `let mut x = 5; x = x + 1;`
> - **Recycler un nom** : Pas besoin de `input1`, `input2`, `input_final`

**Vocabulaire :** *Shadowing* = occulter/recouvrir, *Scope* = bloc de visibilité, *Variable binding* = association nom ↔ valeur.

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

Pour vérifier le type, utilise cette fonction utilitaire :
```rust
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
```

Exemple d'utilisation :
```rust
let x = 42;
println!("Type: {}", type_of(&x));  // i32
```

Aide : `{:?}` affiche la valeur en mode debug, pas le type. Pour voir le type, on utilise `std::any::type_name`.

---

### 3.6. Conversion (casting)

**Explication :** Pour convertir un type en un autre, on utilise `as`. Le cast crée une **nouvelle valeur temporaire**, la variable originale conserve son type.

```rust
let x: i32 = 5;
let y: f64 = x as f64;  // y est f64, mais x reste i32
println!("{}", x);       // OK, x existe toujours

let z: f64 = 5.9;
let w: i32 = z as i32;  // w est i32, mais z reste f64
```

**Vocabulaire :** *Cast* = conversion de type, *truncate* = couper les décimales.

**Attention :** La conversion float → int **tronque** (5.9 devient 5, pas 6).

**Exercice à faire :**
1. Convertis `100` (i32) en f64 et affiche
2. Convertis `5.9` (f64) en i32 et affiche - que vois-tu ?
3. Pourcentage : calcule 80% de 200 (utilise le cast)

---

## Checklist

- [x] let et mutabilité compris
- [x] Type explicite : exercice fait
- [x] Constantes : exercice fait
- [x] Shadowing : exercice fait
- [x] Inference de type : exercice fait
- [x] Casting (as) : exercice fait