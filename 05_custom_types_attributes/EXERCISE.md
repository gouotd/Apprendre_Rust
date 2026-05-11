# Exercice - Module 05

## Objectif

Maîtriser les structs, les méthodes et les attributs en Rust.

---

## Étape 1 : Préparation

```bash
cargo new custom_types
cd custom_types
```

Ouvre `src/main.rs` dans ton éditeur.

---

## Étape 2.1 : Struct avec champs nommés

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

### Créer et manipuler

```rust
struct Point {
    x: f64,
    y: f64,
}

// Création
let p = Point { x: 3.0, y: 4.0 };

// Accès aux champs
let distance_x = p.x;   // → 3.0
let distance_y = p.y;   // → 4.0

// Modification (struct doit être mut)
let mut p2 = Point { x: 1.0, y: 2.0 };
p2.x = 10.0;  // Modifie uniquement x
```

**Exercice :**
1. Crée une struct `Personne` avec champs `nom: String` et `age: u32`
2. Crée une instance avec ton nom et âge
3. Affiche les deux champs
4. Modifie l'âge (la struct doit être `mut`) et réaffiche

---

## Étape 2.2 : Tuple struct

Une **tuple struct** est un hybride entre un tuple et une struct. Elle a un **nom de type**, mais les champs n'ont **pas de noms**. On y accède par **index**.

```rust
struct Couleur(u8, u8, u8);
//      ^nom_type ^rouge, ^vert, ^bleu

let noir = Couleur(0, 0, 0);
let rouge = noir.0;   // premier champ → 0
let vert = noir.1;    // deuxième champ → 0
```

### Quand utiliser une tuple struct ?

| Cas | Struct nommée | Tuple struct |
|-----|---------------|--------------|
| Champs nombreux (> 3) | ✅ | ❌ Trop confus |
| Champs sémantiquement clairs | ✅ (`personne.age`) | ❌ (`personne.1` = ?) |
| Wrapper d'un seul type | ❌ Overkill | ✅ (`struct Id(u32)`) |
| Couple/Triple simple | ❌ Verbeux | ✅ (`struct Position(f64, f64)`) |

**Exemple : un ID qui n'est pas un entier nu**

```rust
struct UserId(u32);

let id = UserId(42);
// UserId(42) est un type DIFFÉRENT de u32
// Le compilateur refuse de mélanger les deux
```

**Exercice :**
1. Crée une struct `Point3D(u8, u8, u8)` et une instance
2. Affiche chaque coordonnée avec `.0`, `.1`, `.2`

---

## Étape 2.3 : Unit struct (struct vide)

Une **unit struct** est une struct **sans aucun champ**. Elle ne contient aucune donnée.

```rust
struct Admin;
//      ^juste un nom, pas de champs
```

### À quoi ça sert ?

**1. Garantie à la compilation (badge) :**

Imagine un système où on doit se connecter avant d'accéder à une ressource. Sans struct vide, tu aurais un booléen `est_connecte` et tu devrais vérifier à chaque appel :

```rust
// ❌ Vérification à l'exécution (runtime)
fn acceder(ressource: &str, est_connecte: bool) {
    if est_connecte {
        println!("Accès à {}", ressource);
    } else {
        println!("Accès refusé");
    }
}

acceder("/admin", true);  // Ça marche, mais le booléen peut être "true" par erreur
```

Le problème : rien n'empêche quelqu'un de passer `true` sans s'être vraiment connecté.

**Avec une struct vide**, le type `Authenticated` lui-même est le "badge" :

```rust
struct Authenticated;  // Pas de données, juste un type "badge"

// Cette fonction NE PEUT être appelée que si tu as un Authenticated
fn acceder(ressource: &str, _badge: Authenticated) {
    println!("Accès à {}", ressource);
}

// Pour obtenir le badge, il faut passer par login()
fn login(mdp: &str) -> Option<Authenticated> {
    if mdp == "secret" {
        Some(Authenticated)  // Seul moyen d'obtenir le badge
    } else {
        None
    }
}

// Utilisation
match login("secret") {
    Some(badge) => acceder("/admin", badge),  // OK: on a le badge
    None => println!("Accès refusé"),
}

// acceder("/admin", ???);  // IMPOSSIBLE sans badge!
// Le compilateur refuse car tu ne peux pas créer un Authenticated ailleurs
```

> **Pourquoi c'est puissant ?** Le `if est_connecte` était une vérification à l'exécution (runtime). Avec la struct vide, la vérification est faite par le **compilateur** : impossible d'appeler `acceder()` sans posséder un `Authenticated`. Tu ne peux pas "tricher" en passant `true`.

**2. Organiser les fonctions (namespacing) :**

Tu pourrais écrire une fonction libre sans struct :

```rust
fn log(message: &str) {
    println!("[LOG] {}", message);
}
log("Message");  // Fonction isolée
```

Mais avec une struct vide + `impl`, tu **groupes** les fonctions :

```rust
struct ConsoleLogger;

impl ConsoleLogger {
    fn log(message: &str) {
        println!("[LOG] {}", message);
    }
    fn clear() {
        println!("Écran effacé");
    }
}

ConsoleLogger::log("Message");    // Groupé sous ConsoleLogger
ConsoleLogger::clear();           // Toutes les fonctions ensemble
```

Et surtout, la struct vide devient **indispensable** pour implémenter des **traits**.

> **Rappel : qu'est-ce qu'un trait ?**
> Un **trait** définit un **comportement** (un ensemble de méthodes) qu'un type peut offrir. C'est comme une interface dans d'autres langages. Un trait dit "ce que sait faire" un type, sans préciser comment.
>
> Exemple : le trait `Debug` signifie "Je sais m'afficher en mode débogage". Le trait `Clone` signifie "Je sais me copier".
>
> On ne peut pas implémenter un trait sur une fonction libre. Il faut un **type**.

```rust
// Un trait définit un comportement
trait Logger {
    fn log(&self, message: &str);
}

// Il faut un TYPE pour implémenter un trait
struct ConsoleLogger;  // Vide car le logger n'a besoin d'aucune donnée

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[CONSOLE] {}", message);
    }
}

// Tu peux maintenant échanger les implémentations
traiter(&ConsoleLogger);  // Accepte tout type qui implémente Logger
```

**3. Empêcher des mélanges d'unités à la compilation :**

Imagine que tu manipules des distances. Tu veux empêcher d'additionner des mètres avec des kilomètres par erreur.

```rust
struct Metres(f64);
struct Kilometres(f64);

// Ces deux types sont DIFFÉRENTS pour le compilateur
let m = Metres(1500.0);
let km = Kilometres(1.5);

// fn additionner(a: Metres, b: Metres) -> Metres
// additionner(m, km);  // ERREUR : types incompatibles !
```

C'est très utile pour les problèmes physiques : mélanger des degrés et des radians, des newtons et des joules, etc. Le compilateur détecte l'erreur **avant** l'exécution.

**Exercice :**
1. Crée une struct vide `Connecte;`
2. Crée une fonction `se_connecter(cle: &str) -> Option<Connecte>` qui retourne `Some(Connecte)` si `cle == "admin"`
3. Crée une fonction `acceder_admin(_badge: Connecte)` qui affiche "Accès admin accordé"
4. Teste avec la bonne clé et une mauvaise clé

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

**Exercice :**
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

**Exercice :**
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

**Exercice :**
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

**Exercice :**
1. Crée des variables `nom` et `age`
2. Utilise la syntaxe courte pour créer une `Personne`
3. Affiche-la

---

## Checklist

- [x] Struct avec champs nommés : exercice fait
- [x] Tuple struct : exercice fait
- [x] Unit struct (badge) : exercice fait
- [x] `#[derive]` et attributs : exercice fait
- [x] Méthodes avec impl : exercice fait
- [x] Méthodes associées (new) : exercice fait
- [x] Field Init Shorthand : exercice fait
