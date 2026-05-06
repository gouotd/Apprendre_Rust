# Cheatsheet - Module 05: Custom Types & Attributes

## 3 Types de Structs

### 1. Struct avec champs nommés
```rust
struct Personne {
    nom: String,
    age: u32,
}

let p = Personne { nom: String::from("Alice"), age: 30 };
println!("{}", p.nom);  // Accès par nom
```

### 2. Tuple struct
```rust
struct Couleur(u8, u8, u8);

let c = Couleur(255, 0, 0);
let r = c.0;  // Accès par index
```

### 3. Unit struct
```rust
struct Admin;  // Vide, utile comme marqueur
let badge = Admin;
```

## Comparaison
| Type | Syntaxe | Accès | Usage |
|------|---------|-------|-------|
| Champs nommés | `struct P { x: i32 }` | `p.x` | Données structurées |
| Tuple struct | `struct P(i32, i32)` | `p.0` | Wrapper, couple |
| Unit struct | `struct P;` | N/A | Marqueur, état |

## Attributs #[derive]
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: f64, y: f64 }
```

| Attribut | Génère | Usage |
|----------|--------|-------|
| `Debug` | `{:?}` | Affichage debug |
| `Clone` | `.clone()` | Copie explicite |
| `Copy` | Copie auto | Types simples |
| `PartialEq` | `==`, `!=` | Comparaison |
| `Default` | `::default()` | Valeur par défaut |

## impl (méthodes)
```rust
impl Rectangle {
    // Méthode lecture seule
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }

    // Méthode modifiable
    fn redimensionner(&mut self, f: f64) {
        self.largeur *= f;
    }

    // Méthode qui consomme
    fn detruire(self) {
        // self n'est plus utilisable après
    }
}
```

## 3 formes de self
| Forme | Signification | Usage |
|-------|---------------|-------|
| `&self` | Emprunt lecture | Méthode qui ne modifie pas |
| `&mut self` | Emprunt écriture | Méthode qui modifie |
| `self` | Prise de possession | Méthode qui consomme |

## Méthodes associées (constructeur)
```rust
impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }  // Self = Point
    }
}

let p = Point::new(3.0, 4.0);  // Appel sans instance
```

## Field Init Shorthand
```rust
let nom = String::from("Alice");
let age = 30;

// Au lieu de :
let p = Personne { nom: nom, age: age };

// Plus court :
let p = Personne { nom, age };
```

## Update syntax
```rust
let p1 = Point { x: 1.0, y: 2.0 };
let p2 = Point { x: 5.0, ..p1 };  // x=5, y=2 (copié)
```

## Vocabulaire
- *Struct* = structure (type personnalisé)
- *Field* = champ (donnée dans une struct)
- *Instance* = valeur concrète d'un type
- *Method* = fonction attachée à une struct
- *Associated function* = fonction sans `self`
- *Attribute* = métadonnée (`#[derive]`)
- *Constructor* = convention `new()`
