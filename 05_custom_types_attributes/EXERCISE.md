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

### Qu'est-ce qu'une struct ?

**Définition :** Une **struct** (contraction de *structure*) est un **type de données personnalisé** que tu crées toi-même. Elle regroupe plusieurs valeurs (appelées **champs** / *fields*) sous un même nom.

**Pourquoi utiliser une struct ?**

Imagine que tu veux représenter une personne. Sans struct, tu dois gérer des variables séparées :

```rust
// ❌ Sans struct : variables dispersées, pas de lien logique
let nom_personne = String::from("Alice");
let age_personne = 30;
let email_personne = String::from("alice@mail.com");
```

Le problème : rien ne lie ces variables entre elles. Si tu passes une personne à une fonction, tu dois passer 3 paramètres séparés.

```rust
// ✅ Avec struct : tout est groupé, logique et clair
struct Personne {
    nom: String,
    age: u32,
    email: String,
}

let alice = Personne {
    nom: String::from("Alice"),
    age: 30,
    email: String::from("alice@mail.com"),
};
```

Maintenant, `alice` est **une seule entité** avec 3 champs. Tu peux la passer à une fonction en un seul paramètre.

**Analogie :** Une struct, c'est comme une **fiche** dans un registre. Chaque fiche a des rubriques (nom, âge, email) et toutes les rubriques appartiennent à la même personne.

---

### 1. Struct avec champs nommés (la plus courante)

C'est le type le plus utilisé en Rust. Chaque champ a un **nom** et un **type**.

```rust
struct Point {
    x: f64,   // champ nommé "x" de type float 64 bits
    y: f64,   // champ nommé "y" de type float 64 bits
}
```

#### Comment créer une instance ?

```rust
let p = Point { x: 3.0, y: 4.0 };
//      ^nom_type ^champ: valeur, ^champ: valeur
```

#### Comment accéder aux champs ?

On utilise la **notation point** (`.`), comme dans la plupart des langages :

```rust
let distance_x = p.x;   // accède au champ x → 3.0
let distance_y = p.y;   // accède au champ y → 4.0
```

#### Comment modifier un champ ?

La struct doit être **mutable** (`mut`), et on utilise `.` pour assigner :

```rust
let mut p = Point { x: 3.0, y: 4.0 };
p.x = 10.0;  // Modifie uniquement le champ x
println!("x: {}", p.x);  // 10.0
```

#### Syntaxe de mise à jour

Rust permet de copier les champs d'une instance existante avec `..` :

```rust
let p1 = Point { x: 1.0, y: 2.0 };
let p2 = Point { x: 5.0, ..p1 };  // x = 5.0, y = 2.0 (copié de p1)
```

> **⚠️ Attention :** Après cette opération, `p1` est **déplacé** pour les champs non-Copy (String, Vec). Pour les types Copy (f64, i32), ça fonctionne.

---

### 2. Tuple struct (champs positionnels)

Une **tuple struct** est un hybride entre un tuple et une struct. Elle a un **nom de type**, mais les champs n'ont **pas de noms**. On y accède par **index**.

```rust
struct Couleur(u8, u8, u8);
//      ^nom_type ^type1, ^type2, ^type3
//      Les champs sont positionnels : 0, 1, 2
```

#### Comment créer une instance ?

```rust
let noir = Couleur(0, 0, 0);
//         ^appel comme une fonction
```

#### Comment accéder aux champs ?

On utilise la **notation point + index** (comme un tuple) :

```rust
let rouge = noir.0;   // premier champ → 0
let vert = noir.1;    // deuxième champ → 0
let bleu = noir.2;    // troisième champ → 0
```

#### Quand utiliser une tuple struct ?

| Cas | Struct nommée | Tuple struct |
|-----|---------------|--------------|
| Champs nombreux (> 3) | ✅ | ❌ Trop confus |
| Champs sémantiquement clairs | ✅ (`personne.age`) | ❌ (`personne.1` = ?) |
| Wrapper d'un seul type | ❌ Overkill | ✅ (`struct Id(u32)`) |
| Couple/Triple simple | ❌ Verbeux | ✅ (`struct Position(f64, f64)`) |

**Exemple concret : un ID qui n'est pas un entier nu**

```rust
struct UserId(u32);

let id = UserId(42);
// Maintenant, UserId(42) est différent de UserId(99)
// Et surtout, différent d'un simple u32 !
```

---

### 3. Unit struct (struct vide)

Une **unit struct** est une struct **sans aucun champ**. Elle ne contient aucune donnée.

```rust
struct Admin;
//      ^juste un nom, pas de champs
```

#### À quoi ça sert ?

C'est contre-intuitif, mais très utile :

**1. Marquer un type (type marker) :**
```rust
struct Authenticated;  // Juste un "badge"

fn acceder(ressource: &str, _badge: Authenticated) {
    println!("Accès à {}", ressource);
}

let badge = Authenticated;
acceder("/admin", badge);
```

**2. Implémenter un trait sans données :**
```rust
struct Logger;

impl Logger {
    fn log(message: &str) {
        println!("[LOG] {}", message);
    }
}
```

**3. Générique phantom type (avancé) :**
```rust
struct Metres(f64);
struct Kilometres(f64);
// On ne peut pas mélanger accidentellement les deux !
```

---

### Comparaison des 3 types

| Type | Syntaxe | Accès | Usage typique |
|------|---------|-------|---------------|
| **Champs nommés** | `struct P { x: i32 }` | `p.x` | Données structurées (personne, config) |
| **Tuple struct** | `struct P(i32, i32)` | `p.0` | Wrapper, couple simple |
| **Unit struct** | `struct P;` | N/A | Marqueur, trait, état |

---

**Vocabulaire :**
- *Field* = champ (donnée dans une struct)
- *Instance* = une valeur concrète d'un type struct
- *Tuple struct* = struct avec champs positionnels
- *Unit struct* = struct vide (sans données)
- *Named struct* = struct avec champs nommés

**Exercice à faire :**
1. Crée une struct `Personne` avec champs `nom: String` et `age: u32`
2. Crée une instance avec ton nom et âge
3. Affiche les deux champs
4. Modifie l'âge (la struct doit être `mut`)
5. Crée une struct `Point3D(u8, u8, u8)` et une instance
6. Affiche chaque coordonnée avec `.0`, `.1`, `.2`

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
