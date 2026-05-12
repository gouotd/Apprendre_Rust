# Exercice - Module 06

## Objectif

Maîtriser les enums et le pattern matching en Rust.

---

## Étape 1 : Préparation

```bash
cargo new enums_pattern_matching
cd enums_pattern_matching
```

Ouvre `src/main.rs` dans ton éditeur.

---

## Étape 2 : Qu'est-ce qu'une enum ?

### Définition

Une **enum** (énumération) est un type qui peut prendre **plusieurs formes possibles**, appelées **variants**.

**Analogie :** Imagine un feu tricolore. Il peut être Rouge, Orange ou Vert. C'est un seul concept (le feu) avec 3 états possibles.

```rust
// Définition de l'enum
enum FeuTricolore {
    Rouge,
    Orange,
    Vert,
}

// Utilisation
let feu = FeuTricolore::Rouge;
```

> **`::`** est l'opérateur de **chemin** en Rust. On l'utilise pour accéder à un variant d'une enum : `NomEnum::NomVariant`.

### Différence avec les constantes

Tu pourrais utiliser des constantes :

```rust
const ROUGE: u8 = 0;
const ORANGE: u8 = 1;
const VERT: u8 = 2;
```

Mais le problème : rien n'empêche de mélanger `ROUGE` avec un autre `u8` quelconque. Avec une **enum**, le type est **unique** et le compilateur refuse de le mélanger.

### Pourquoi utiliser une enum plutôt que des types séparés ?

| Approche | Problème |
|----------|----------|
| `struct Rouge; struct Orange; struct Vert;` | Trop de types différents, code lourd |
| `const ROUGE = 0;` (entiers) | On peut mélanger avec d'autres entiers |
| **`enum FeuTricolore { Rouge, Orange, Vert }`** | ✅ Type unique, clair, sécurisé |

### Exercice

1. Crée une enum `Direction` avec les variants : `Nord`, `Sud`, `Est`, `Ouest`
2. Crée une variable `ma_direction` valant `Direction::Nord`
3. Affiche-la avec `println!("{:?}", ma_direction)` (ajoute `#[derive(Debug)]`)

---

## Étape 3 : Enum avec données attachées

### Le concept

Les variants d'une enum peuvent **contenir des données**. C'est ce qui rend les enums plus puissantes que dans la plupart des langages.

```rust
enum Message {
    Quitter,                    // Aucune donnée
    Deplacer(i32, i32),         // Tuple (x, y)
    Ecrire(String),             // Une String
    ChangerCouleur(u8, u8, u8), // Tuple RGB
}
```

Chaque variant a sa propre "signature" :

| Variant | Données | Exemple |
|---------|---------|---------|
| `Quitter` | Rien | `Message::Quitter` |
| `Deplacer(i32, i32)` | Deux entiers | `Message::Deplacer(10, 20)` |
| `Ecrire(String)` | Une chaîne | `Message::Ecrire("hello".to_string())` |
| `ChangerCouleur(u8, u8, u8)` | Trois entiers | `Message::ChangerCouleur(255, 0, 0)` |

### Mémoire

Malgré des variants de tailles différentes, tous occupent la **même taille en mémoire** (la taille du plus grand variant). Rust réserve assez d'espace pour le plus grand cas.

### Pourquoi c'est utile ?

Ça permet de regrouper des concepts liés sous un **même type**, sans perdre l'information spécifique à chaque cas. C'est le cœur de la programmation par **type algébrique** (ADT).

### Exercice

1. Crée une enum `CarteBancaire` avec variants :
   - `Visa(numero: u32)`
   - `Mastercard(numero: u32, code: u8)`
   - `Inconnue`
2. Ajoute `#[derive(Debug)]`
3. Crée une instance `Visa(123456)` et une `Mastercard(789012, 123)`
4. Affiche-les avec `{:?}`

---

## Étape 4 : `match` sur les enums

### Pattern matching

Le vrai pouvoir des enums apparaît avec `match`. Chaque variant peut avoir une logique différente :

```rust
fn traiter(message: Message) {
    match message {
        Message::Quitter => println!("Au revoir !"),
        Message::Deplacer(x, y) => println!("Déplacement à ({}, {})", x, y),
        Message::Ecrire(texte) => println!("Texte: {}", texte),
        Message::ChangerCouleur(r, g, b) => {
            println!("Couleur changée en RGB({}, {}, {})", r, g, b);
        }
    }
}
```

> **⚠️ Le match doit être exhaustif** : tous les variants doivent être couverts. Si tu oublies un variant, le compilateur refuse.

### `_` (catch-all)

Pour ignorer certains variants, utilise `_` :

```rust
match message {
    Message::Ecrire(texte) => println!("{}", texte),
    _ => {}  // Ignore tous les autres variants
}
```

### Destructuring

Le `match` peut **extraire** les données contenues dans un variant :

```rust
match carte {
    CarteBancaire::Visa(num) => println!("Visa n°{}", num),
    CarteBancaire::Mastercard(num, code) => {
        println!("Mastercard n°{} code {}", num, code);
    }
    CarteBancaire::Inconnue => println!("Carte inconnue"),
}
```

### Exercice

1. Crée une fonction `afficher_direction(d: Direction)` qui utilise `match`
2. Pour `Nord` → affiche "Direction: Nord"
3. Pour `Sud` → "Direction: Sud"
4. Utilise `_` pour les autres → "Direction inconnue"
5. Teste avec `Direction::Nord` et `Direction::Ouest`

---

## Étape 5 : `Option<T>` (Présent ou absent)

### Le problème de `null`

Dans beaucoup de langages, une fonction peut retourner `null` si elle échoue. Mais `null` cause des crashs quand on l'utilise sans vérifier.

**Rust n'a pas de `null`**. À la place, il a `Option<T>` :

```rust
enum Option<T> {
    Some(T),   // Une valeur existe
    None,      // Aucune valeur
}
```

### Pourquoi `Option` plutôt que `null` ?

| Langage | Retour | Risque |
|---------|--------|-------|
| C | `int*` ou `NULL` | Crash si on oublie de vérifier |
| Java | `Integer` ou `null` | `NullPointerException` |
| **Rust** | `Option<i32>` | **Le compilateur force à vérifier** |

### Utilisation concrète

`Option` est une **enum comme les autres**. On utilise `match` pour extraire la valeur, exactement comme dans l'étape 4.

```rust
fn trouver_personne(nom: &str) -> Option<String> {
    if nom == "Alice" {
        Some(String::from("Alice a 30 ans"))
    } else {
        None  // Personne non trouvée
    }
}

let resultat = trouver_personne("Alice");

// Option est juste une enum ! Même syntaxe match qu'avec CarteBancaire
match resultat {
    Some(description) => println!("{}", description),
    //    ^^^^^^^^^^^^
    //    "description" est la valeur contenue dans Some
    //    C'est du destructuring : on extrait la donnée du variant
    
    None => println!("Personne non trouvée"),
}
```

**Explication du destructuring :** `Some(description)` signifie : "Si ce variant est `Some`, extrait la valeur qu'il contient et stocke-la dans la variable `description`".

C'est exactement le même principe que `CarteBancaire::Visa(num)` à l'étape 4 : on extrait les données du variant dans une variable nommée.

### Méthodes utiles sur Option

| Méthode | Description | Exemple |
|---------|-------------|---------|
| `.is_some()` | Vérifie si Some | `opt.is_some()` |
| `.is_none()` | Vérifie si None | `opt.is_none()` |
| `.unwrap()` | Extrait la valeur (panique si None) | À éviter |
| `.unwrap_or(default)` | Valeur par défaut si None | `opt.unwrap_or(0)` |

### Exercice

1. Crée une fonction `diviser(a: f64, b: f64) -> Option<f64>`
2. Si `b == 0.0`, retourne `None` (division par zéro)
3. Sinon, retourne `Some(a / b)`
4. Dans `main`, teste avec `diviser(10.0, 2.0)` et `diviser(10.0, 0.0)`
5. Utilise `match` pour afficher le résultat ou "Erreur: division par zéro"

---

## Étape 6 : `if let` (match rapide)

### Pourquoi `if let` ?

Parfois, tu ne veux vérifier qu'**un seul variant**, et ignorer tous les autres. Avec `match`, c'est verbeux :

```rust
// Verbeux avec match
match option {
    Some(valeur) => println!("Valeur: {}", valeur),
    _ => {}  // Boilerplate
}
```

`if let` simplifie ça :

```rust
// Court avec if let
if let Some(valeur) = option {
    println!("Valeur: {}", valeur);
}
```

### Syntaxe

```rust
if let MonEnum::MonVariant(variable) = valeur {
    // Exécuté seulement si c'est ce variant
}
```

### Exercice

1. Reprends la fonction `diviser` de l'étape 5
2. Utilise `if let Some(resultat) = ...` pour afficher le résultat
3. Pas de message si échec (on ignore `None`)

---

## Étape 7 : `Result<T, E>` (Succès ou échec avec message)

### Pourquoi un deuxième type ?

`Option<T>` dit "il y a une valeur ou pas". Mais parfois, tu veux savoir **pourquoi** ça a échoué. C'est le rôle de `Result<T, E>` :

```rust
enum Result<T, E> {
    Ok(T),    // Succès, contient la valeur
    Err(E),   // Échec, contient l'erreur
}
```

### Différence Option vs Result

| Scénario | Type | Exemple |
|----------|------|---------|
| Division par zéro | `Option<f64>` | `None` (pas de message) |
| Division par zéro | `Result<f64, String>` | `Err("Division par zéro".to_string())` |
| Trouver une personne | `Option<String>` | `None` (pas trouvée) |
| Ouvrir un fichier | `Result<String, io::Error>` | `Err(io::Error)` |

### Utilisation concrète

```rust
fn diviser_result(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division par zéro"))
    } else {
        Ok(a / b)
    }
}

match diviser_result(10.0, 0.0) {
    Ok(valeur) => println!("Résultat: {}", valeur),
    Err(message) => println!("Erreur: {}", message),
}
```

### Exercice

1. Crée une fonction `verifier_age(age: i32) -> Result<i32, String>`
2. Si `age < 0` → `Err("Âge négatif impossible")`
3. Si `age > 150` → `Err("Âge trop grand")`
4. Sinon → `Ok(age)`
5. Teste avec `-5`, `200`, et `25`
6. Affiche le résultat avec `match`

---

## Étape 8 : Enum avec `impl` (méthodes)

Comme les structs, les enums peuvent avoir des méthodes via `impl` :

```rust
enum FeuTricolore {
    Rouge,
    Orange,
    Vert,
}

impl FeuTricolore {
    fn suivant(&self) -> FeuTricolore {
        match self {
            FeuTricolore::Rouge => FeuTricolore::Vert,
            FeuTricolore::Vert => FeuTricolore::Orange,
            FeuTricolore::Orange => FeuTricolore::Rouge,
        }
    }

    fn afficher(&self) {
        match self {
            FeuTricolore::Rouge => println!("STOP"),
            FeuTricolore::Orange => println!("ATTENTION"),
            FeuTricolore::Vert => println!("PASSEZ"),
        }
    }
}
```

### Exercice

1. Ajoute `#[derive(Debug)]` à ton enum `Direction`
2. Implémente une méthode `oppose(&self) -> Direction` :
   - `Nord → Sud`, `Sud → Nord`, `Est → Ouest`, `Ouest → Est`
3. Teste avec `Direction::Nord.oppose()` et affiche avec `{:?}`

---

## Checklist

- [x] Enum simple : exercice fait
- [x] Enum avec données : exercice fait
- [x] match sur enums : exercice fait
- [x] Option<T> : exercice fait
- [x] if let : exercice fait
- [x] Result<T, E> : exercice fait
- [x ] Enum avec impl : exercice fait
