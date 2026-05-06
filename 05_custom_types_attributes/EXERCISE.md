# Exercice - Module 05

## Objectif

Maîtriser les structs, les méthodes et les attributs en Rust.

---

## Étape 1 : Préparation

```bash
cargo new custom_types
cd custom_types
```

---

## Étape 2 : Les 3 types de structs

**Explication :** Rust propose 3 façons de définir une struct, selon le besoin.

### 1. Struct avec champs nommés (la plus courante)

C'est le type le plus utilisé. Chaque champ a un nom et un type.

```rust
struct Point {
    x: f64,
    y: f64,
}

// Création
let p = Point { x: 3.0, y: 4.0 };

// Accès aux champs
println!("x: {}, y: {}", p.x, p.y);
```

> **💡 Analogie :** C'est comme une fiche descriptive. Chaque champ a un label et une valeur.

### 2. Tuple struct (champs positionnels)

Comme un tuple, mais avec un nom de type. Utile quand les champs sont peu nombreux et que les noms ne sont pas essentiels.

```rust
struct Couleur(u8, u8, u8);  // Rouge, Vert, Bleu

let noir = Couleur(0, 0, 0);
let rouge = noir.0;  // Accès par index (comme un tuple)
```

### 3. Unit struct (struct vide)

Une struct sans aucun champ. Utile comme marqueur ou pour implémenter un trait.

```rust
struct Trace;  // Juste un type vide
```

**Vocabulaire :** *Field* = champ, *Tuple struct* = struct positionnelle, *Unit struct* = struct vide.

**Exercice à faire :**
1. Crée une struct `Personne` avec champs `nom: String` et `age: u32`
2. Crée une instance avec ton nom et âge
3. Affiche les deux champs
4. Crée aussi une struct `Point3D(u8, u8, u8)` et une instance

---

## Étape 3 : Le trait `#[derive]` et les attributs

**Explication :** Les **attributs** sont des métadonnées qui disent au compilateur de faire quelque chose. `#[derive(Trait)]` génère automatiquement le code pour un trait.

### Les dérivations courantes

| Attribut | Ce qu'il génère | Usage |
|----------|----------------|-------|
| `#[derive(Debug)]` | Permet d'afficher avec `{:?}` | Debug, logs |
| `#[derive(Clone)]` | Permet `.clone()` | Copier la struct |
| `#[derive(Copy)]` | Copie automatique (sans `.clone()`) | Types simples |
| `#[derive(PartialEq)]` | Permet `==` et `!=` | Comparaison |
| `#[derive(Default)]` | Permet `::default()` | Valeur par défaut |

```rust
#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

let p1 = Point { x: 1.0, y: 2.0 };
let p2 = p1.clone();  // Clone fonctionne grâce à #[derive(Clone)]
println!("{:?}", p2);  // Debug fonctionne grâce à #[derive(Debug)]
```

**Exercice à faire :**
1. Ajoute `#[derive(Debug, Clone)]` à ta struct `Personne`
2. Clone une instance et affiche-la avec `{:?}`

---

## Étape 4 : Implémenter des méthodes avec `impl`

**Explication :** Le bloc `impl` permet d'attacher des fonctions à une struct. Ces fonctions deviennent des **méthodes**.

```rust
struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

impl Rectangle {
    // Méthode qui lit les données (lecture seule)
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }

    // Méthode qui modifie les données
    fn redimensionner(&mut self, facteur: f64) {
        self.largeur *= facteur;
        self.hauteur *= facteur;
    }
}

fn main() {
    let mut rect = Rectangle { largeur: 10.0, hauteur: 5.0 };
    println!("Aire: {}", rect.aire());  // 50.0
    rect.redimensionner(2.0);
    println!("Nouvelle aire: {}", rect.aire());  // 200.0
}
```

### Les 3 formes de `self`

| Forme | Signification | Usage |
|-------|---------------|-------|
| `&self` | Emprunte en lecture | Méthode qui ne modifie pas |
| `&mut self` | Emprunte en écriture | Méthode qui modifie |
| `self` | Prend possession | Méthode qui consomme/détruit |

**Exercice à faire :**
1. Crée une struct `CompteBancaire` avec `titulaire: String` et `solde: f64`
2. Implémente :
   - `fn deposer(&mut self, montant: f64)`
   - `fn retirer(&mut self, montant: f64) -> bool` (retourne `true` si réussi)
   - `fn afficher_solde(&self)`
3. Teste avec un compte de 1000€

---

## Étape 5 : Méthodes associées (Constructeur)

**Explication :** Une méthode **sans `self`** s'appelle une **fonction associée** (*associated function*). Elle n'a pas besoin d'instance pour être appelée. On l'appelle avec `NomStruct::methode()`.

La plus courante est `new()`, qui sert de **constructeur**.

```rust
impl Point {
    // Fonction associée (pas de self)
    fn new(x: f64, y: f64) -> Self {
        // Self = le type actuel (Point)
        Point { x, y }
    }
}

// Utilisation
let p = Point::new(3.0, 4.0);
```

> **💡 Note :** `Self` (avec S majuscule) est un alias du type dans lequel on est. C'est plus court et ça évite de répéter le nom.

**Exercice à faire :**
1. Ajoute une fonction `new(nom: &str, solde: f64) -> Self` à `CompteBancaire`
2. Crée un compte avec `CompteBancaire::new("Alice", 500.0)`
3. Utilise les méthodes `deposer`, `retirer`, `afficher_solde`

---

## Étape 6 : Initialisation rapide (Field Init Shorthand)

**Explication :** Quand le nom d'un champ est le même que le nom de la variable, Rust permet d'écrire juste le nom.

```rust
let nom = String::from("Alice");
let age = 30;

// Au lieu de :
let p = Personne { nom: nom, age: age };

// On peut écrire :
let p = Personne { nom, age };  // Plus court !
```

**Exercice à faire :**
1. Crée des variables `nom` et `age`
2. Utilise la syntaxe courte pour créer une `Personne`
3. Affiche-la

---

## Checklist

- [ ] Struct avec champs nommés : exercice fait
- [ ] Tuple struct et Unit struct : exercice fait
- [ ] `#[derive]` et attributs : exercice fait
- [ ] Méthodes avec impl : exercice fait
- [ ] Méthodes associées (new) : exercice fait
- [ ] Field Init Shorthand : exercice fait
