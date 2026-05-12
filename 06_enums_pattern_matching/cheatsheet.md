# Cheatsheet - Module 06: Enums & Pattern Matching

## Enum simple
```rust
#[derive(Debug)]
enum Direction { Nord, Sud, Est, Ouest }

let d = Direction::Nord;
```

## Enum avec données
```rust
enum Message {
    Quitter,
    Deplacer(i32, i32),
    Ecrire(String),
    ChangerCouleur(u8, u8, u8),
}
```

## match sur enum
```rust
match message {
    Message::Quitter => println!("Au revoir"),
    Message::Deplacer(x, y) => println!("({}, {})", x, y),
    _ => {}  // catch-all
}
```

## Option<T>
```rust
let some: Option<i32> = Some(5);
let none: Option<i32> = None;

match some {
    Some(v) => println!("{}", v),
    None => println!("rien"),
}
```

| Méthode | Description |
|---------|-------------|
| `.is_some()` | Est-ce Some ? |
| `.is_none()` | Est-ce None ? |
| `.unwrap()` | Extrait (panique si None) |
| `.unwrap_or(x)` | Valeur par défaut si None |

## if let
```rust
if let Some(v) = option {
    println!("{}", v);  // Exécuté seulement si Some
}
```

## Result<T, E>
```rust
fn div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err("Div par 0".to_string()) }
    else { Ok(a / b) }
}

match div(10.0, 0.0) {
    Ok(v) => println!("{}", v),
    Err(e) => println!("Erreur: {}", e),
}
```

## Enum avec impl
```rust
enum Feu { Rouge, Vert, Orange }

impl Feu {
    fn suivant(&self) -> Feu { ... }
    fn afficher(&self) { ... }
}
```

## Consommant (`-> Self`) vs Mutateur (`&mut self`)

```rust
// CONSOMMANT : retourne une nouvelle valeur
fn suivant(&self) -> Feu {
    match self {
        Feu::Rouge => Feu::Vert,
        Feu::Vert => Feu::Orange,
        Feu::Orange => Feu::Rouge,
    }
}
// Usage : x = x.suivant();  ← réassignation obligatoire

// MUTATEUR : modifie self sur place, pas de retour
fn suivant_mut(&mut self) {
    *self = match self {
        Feu::Rouge => Feu::Vert,
        Feu::Vert => Feu::Orange,
        Feu::Orange => Feu::Rouge,
    }
}
// Usage : x.suivant_mut();  ← modifie directement
```

| | Consommant | Mutateur |
|---|-----------|----------|
| Signature | `fn f(&self) -> Type` | `fn f(&mut self)` |
| Usage | `x = x.f()` | `x.f()` |
| Comportement | Crée une **nouvelle** valeur | **Modifie** la valeur existante |
| Readable | Oui | Non (a besoin de `&mut`) |
| Variable | `let mut` pour réassigner | `let mut` pour muter |

## Option vs Result
| | Option<T> | Result<T, E> |
|---|-----------|--------------|
| Échec | `None` | `Err(description)` |
| Succès | `Some(v)` | `Ok(v)` |
| Usage | Valeur peut être absente | Opération peut échouer |
| Exemple | Base de données | Ouverture fichier |

## Vocabulaire
- *Enum* = type à plusieurs formes
- *Variant* = une des formes possibles
- *Exhaustive* = tous les cas doivent être couverts
- *Destructuring* = extraire des données d'un variant
- *Catch-all* = attrape-tout (`_`)
