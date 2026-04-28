# Exercice - Module 01

## Objectif

Créer ton premier programme Rust et afficher du texte formaté.

---

## Étape 1 : Installer Rust

Ouvre un terminal et exécute :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Suis les instructions à l'écran (choisis les options par défaut).

Redémarre ton terminal ou exécute :

```bash
source $HOME/.cargo/env
```

Vérifie l'installation :

```bash
rustc --version
cargo --version
```

---

## Étape 2 : Créer le projet

Crée un nouveau projet Rust :

```bash
cargo new hello_world
cd hello_world
```

Esto devrait créer cette structure :

```
hello_world/
├── src/
│   └── main.rs      ← on va modifier ce fichier
└── Cargo.toml
```

---

## Étape 3 : Écrire le code

Ouvre le fichier `src/main.rs` avec ton éditeur.

**Supprime tout le contenu actuel** et copie ces lignes :

```rust
fn main() {
    println!("Hello World!");
}
```

---

## Étape 4 : Compiler et exécuter

Dans le terminal, lance la compilation et l'exécution :

```bash
cargo run
```

Si tout va bien, tu devrais voir :

```
Hello World!
```

---

## Étape 5 : Expérimente

Maintenant, tu vas tester différentes facons d'afficher du texte. **Pour chaque section, modifie le code, teste avec `cargo run`, puis passe à la suivante.**

---

### 5.1. Positional arguments

**Explication :** Dans une chaîne `{}` représente un argument. Le chiffre à l'intérieur indique la position (0 = premier argument, 1 = deuxième, etc.). Cela permet de réutiliser un même argument plusieurs fois.

```rust
println!("{0}, ton age est {1}", "Alice", 30);
```

**Exercice à faire :** Modifie le code pour afficher :

```
Alice, ton age est 30 ans, et tu mesures 175cm
```

175 doit être utilisé comme argument (dans l'ordre après "30").

---

### 5.2. Named arguments

**Explication :** Au lieu d'utiliser des chiffres, tu peux donner des noms aux arguments. C'est utile quand il y a beaucoup de variables pour que le code soit plus lisible. On utilise `nom=valeur` pour nommer.

```rust
println!("Le {operation} de {a} {op} {b} = {result}",
    operation = "produit",
    a = 5,
    op = "x", 
    b = 7,
    result = 35);
```

**Exercice à faire :** Modifie le code pour afficher :

```
La somme de 150 + 200 = 350
```

Utilise les noms : operation, a, op, b, result.

---

### 5.3. Formats numériques (:b, :o, :x)

**Explication :** Rust peut convertir des nombres dans différents formats :
- `:b` = binaire (base 2, seulement 0 et 1)
- `:o` = octal (base 8, de 0 à 7)
- `:x` = hexadécimal (base 16, 0-9 et a-f)

```rust
println!("Binaire: {:b}", 255);   // 11111111
println!("Octal: {:o}", 255);    // 377
println!("Hex: {:x}", 255);       // ff
```

**Exercice à faire :** Modifie le code pour afficher la même valeur 42 dans les trois formats, et vérifie que tu obtiens :
- Binaire: 101010
- Octal: 52
- Hex: 2a

---

### 5.4. Float avec précision

**Explication :** Pour les nombres décimaux, on peut contrôler le nombre de chiffres après la virgule avec `:.n` où n est le nombre de décimales.

```rust
let pi = 3.141592;
println!("Pi ≈ {:.2}", pi);      // 3.14
```

> **⚠️ Note importante : `let` et division**
> 
> `let` crée une variable. Quand tu divises deux entiers (ex: `10/3`), Rust fait une **division entière** → résultat = 3.
> 
> Pour avoir un résultat décimal, utilise des floats :
> 
> ```rust
> let un_tier = 10.0 / 3.0;  // float / float = float
> println!("1/3 ≈ {:.3}", un_tier);  // 3.333
> ```

**Exercice à faire :** Calcul le résultat de 37 / 3 et affiche-le avec 3 décimales. Le résultat devrait être 12.333.

---

### 5.5. Debug avec structure et #[derive(Debug)]

**Explication :** Pour afficher un type personnalisé (comme une struct), on doit implémenter le trait `Debug`. Grâce à l'attribut `#[derive(Debug)]`, Rust génère automatiquement le code nécessaire.

```rust
#[derive(Debug)]
struct Point(i32);

fn main() {
    let p = Point(10);
    println!("{:?}", p);
}
```

> **📚 Note : C'est quoi un trait ?**
> 
> Un **trait** en Rust décrit un ensemble de comportements qu'un type peut offrir. C'est comparable aux interfaces dans d'autres langages : ça définit "ce que sait faire" un type, sans préciser comment.
> 
> - Le trait `Debug` signifie : "Je sais m'afficher en mode débogage"
> - Le trait `Display` signifie : "Je sais m'afficher pour l'utilisateur"
> 
> L'attribut `#[derive(Debug)]` demande à Rust de générer automatiquement le code pour le trait Debug.
> 
> Nous verrons les traits en détail dans le **Module 07**.

**À propos de `{:?}` vs `{}` :**

Ces deux marqueurs indiquent comment afficher une valeur :
- `{}` utilise le trait **Display** (pour l'utilisateur final, format lisible)
- `{:?}` utilise le trait **Debug** (pour le développeur, format technique)

Display est plus élégant mais doit être implémenté manuellement. Debug peut être derives automatiquement par Rust.

**À propos de `struct Point(i32)` :**

Une **struct** (abréviation de "structure", *structure* en anglais) permet de créer ton propre type de données. Ici :
- `Point` = le nom du type
- `(i32)` = un champ (*field* en anglais) de type nombre entier (i32 = integer 32 bits)

C'est comme une classe avec seulement des données (pas de méthodes). Nous verrons les structs en détail dans le **Module 05**.

**Exercice à faire :** Crée une struct `Personne` avec un champ `nom` (String) et `age` (nombre). Affiche une personne avec `{:?}`.

Aide :
```rust
#[derive(Debug)]
struct Personne {
    nom: String,
    age: i32
}
```

---

### 5.6. format! pour créer une String

**Explication :** `format!` fonctionne comme `println!` mais au lieu d'afficher à l'écran, ça crée une String qu'on peut stocker dans une variable.

```rust
let message = format!("Bienvenue {}!", "Alice");
println!("{}", message);
```

**Exercice à faire :** Crée une String avec `format!` qui contient "Hello World, je sais print!" et affiche-la.

---

## Checklist

- [x] Rust installé
- [x] Projet créé avec `cargo new`
- [x] "Hello World!" affiché
- [x] positional arguments : exercice fait
- [x] named arguments : exercice fait
- [x] formats numériques : exercice fait
- [x] float avec précision : exercice fait
- [x] Debug avec structure : exercice fait
- [x] format! : exercice fait