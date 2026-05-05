# Exercice - Module 03

## Objectif

Maîtriser les fonctions, conditions et boucles en Rust.

---

## Étape 1 : Préparation

```bash
cargo new functions_control_flow
cd functions_control_flow
```

---

## Étape 2 : Les fonctions (`fn`)

**Explication :** En Rust, on définit une fonction avec `fn`. On précise les types des paramètres et le type de retour avec `->`.

```rust
fn addition(x: i32, y: i32) -> i32 {
    x + y  // Pas besoin de `return` en fin de fonction
}

fn main() {
    let resultat = addition(3, 5);
    println!("{}", resultat);  // 8
}
```

> **💡 Note :** La dernière expression d'une fonction est automatiquement retournée. Le `return` est optionnel sauf si on veut sortir prématurément.

**Vocabulaire :** *Return type* = type de retour, *Parameter* = paramètre.

**Exercice à faire :**
1. Crée une fonction `carre(x: i32) -> i32` qui retourne le carré d'un nombre
2. Crée une fonction `saluer(nom: &str)` qui affiche "Bonjour, {nom}!" (pas de retour)
3. Appelle les deux dans `main`

---

## Étape 3 : Expressions vs Instructions (Statements)

**Explication :** Cette distinction est fondamentale en Rust et différente d'autres langages.

Une **instruction** (*statement*) est une commande qui fait quelque chose mais ne retourne rien.
Une **expression** retourne toujours une valeur.

En Rust, presque tout est une expression, y compris `if/else`, `match`, et les blocs `{}`.

```rust
// INSTRUCTION : on assigne une valeur, mais cette ligne ne retourne rien
let x = 5;

// EXPRESSION : if/else retourne une valeur !
let y = if x > 3 { 10 } else { 0 };
// y vaut 10 car x (5) > 3
```

**Comparaison avec Python/JavaScript :**

| Langage | `if` retourne une valeur ? |
|---------|---------------------------|
| Python | Non → `y = 10 if x > 3 else 0` (syntaxe spéciale) |
| JavaScript | Non → `y = x > 3 ? 10 : 0` (opérateur ternaire) |
| **Rust** | **Oui → `let y = if x > 3 { 10 } else { 0 };` (naturel)** |

**Règles importantes :**
1. Chaque branche doit retourner le **même type**
2. Pas de point-virgule `;` à la fin de la dernière expression d'un bloc (sinon ça devient une instruction qui ne retourne rien)

```rust
// ✅ CORRECT : pas de ; après 10 et 0
let y = if x > 3 { 10 } else { 0 };

// ❌ ERREUR : le ; transforme l'expression en instruction (type = () )
let y = if x > 3 { 10; } else { 0; };  // Erreur de type
```

**Vocabulaire :**
- *Expression* = retourne une valeur (ex: `5`, `x + 1`, `if ...`)
- *Statement* = instruction qui ne retourne rien (ex: `let x = 5;`)

**Exercice à faire :**
1. Crée une fonction `verifier_pair(nombre: i32)` qui utilise un `if` comme expression
2. Elle doit assigner à une variable `resultat` le texte "pair" ou "impair" selon le nombre
3. Affiche le résultat

Exemple de signature :
```rust
fn verifier_pair(nombre: i32) {
    let resultat = if ... { ... } else { ... };
    println!("Le nombre {} est {}", nombre, resultat);
}
```

---

## Étape 4 : Conditionnel `if/else`

**Explication :** Le `if` en Rust est comme dans d'autres langages. Pas de parenthèses autour de la condition.

```rust
let age = 20;

if age >= 18 {
    println!("Majeur");
} else if age >= 12 {
    println!("Adolescent");
} else {
    println!("Enfant");
}
```

### Opérateurs logiques `&&` et `||`

Comme dans d'autres langages, on peut combiner des conditions :

| Opérateur | Signification | Exemple |
|-----------|---------------|---------|
| `&&` | **ET** (and) - les deux doivent être vrais | `age >= 18 && actif` |
| `\|\|` | **OU** (or) - au moins un doit être vrai | `age < 18 \|\| senior` |
| `!` | **NON** (not) - inverse la condition | `!actif` |

```rust
let age = 25;
let a_carte = true;

// Les deux conditions doivent être vraies
if age >= 18 && a_carte {
    println!("Peut entrer");
}

// Au moins une condition doit être vraie
if age < 18 || a_carte {
    println!("Tarif réduit");
}
```

**Exercices à faire :**

**Exercice 4a :**
1. Crée une fonction `verifier_acces(age: i32, a_carte: bool)`
2. Autorise l'entrée si : `age >= 18 && a_carte`
3. Sinon, refuse avec un message expliquant pourquoi (manque age, manque carte, ou les deux)

**Exercice 4b :**
1. Crée une fonction `categorie(age: i32) -> &'static str`
   - age >= 60 → "Senior"
   - age >= 18 → "Adulte"
   - age >= 12 → "Adolescent"
   - sinon → "Enfant"
2. Teste avec l'âge 25

> **💡 Note sur `&'static str` et les littéraux (*literals*) :**
> 
> Un **littéral** (*literal*) est une valeur écrite directement dans le code source. Exemples :
> - `"Adulte"` = littéral de chaîne (string literal)
> - `42` = littéral entier (integer literal)
> - `3.14` = littéral float (float literal)
> - `true` = littéral booléen (bool literal)
> 
> Ces valeurs sont **stockées dans le binaire** à la compilation. Comme elles font partie du programme compilé, elles existent pendant toute la durée d'exécution → c'est pourquoi elles ont la durée de vie `'static`.
> 
> À l'inverse, une chaîne créée dynamiquement pendant l'exécution n'est pas `'static` :
> ```rust
> // Littéral → &'static str (stocké dans le binaire, vit toujours)
> let x = "Adulte";
> 
> // Créé à l'exécution → String (stocké sur le heap, vit jusqu'à sa destruction)
> let y = format!("{} ans", 25);  // Pas 'static !
> ```
> 
> On verra les lifetimes en détail au **Module 04**.

---

## Étape 5 : Pattern matching `match`

**Explication :** Le `match` en Rust est l'équivalent du `switch` dans d'autres langages (C, Java, JavaScript), mais beaucoup plus puissant.

### C'est quoi un `switch` ?

Un `switch` compare une valeur à plusieurs cas possibles et exécute le code du cas correspondant :

```javascript
// En JavaScript
switch (jour) {
  case 1: console.log("Lundi"); break;
  case 2: console.log("Mardi"); break;
  default: console.log("Autre");
}
```

```rust
// En Rust avec match
let jour = 1;

match jour {
    1 => println!("Lundi"),
    2 => println!("Mardi"),
    _ => println!("Autre"),  // default
}
```

### C'est quoi un `pattern` (motif) ?

Un **pattern** (motif) est une forme que peut prendre une valeur. Le `match` compare la valeur à ces formes. Les patterns les plus simples sont les valeurs exactes, mais il y en a beaucoup d'autres :

| Pattern | Signification | Exemple |
|---------|---------------|---------|
| `1` | Valeur exacte | `1 => ...` |
| `x` | Capture la valeur | `x => println!("{}", x)` |
| `_` | Attrape-tout (ignore) | `_ => ...` |
| `a..=b` | Plage inclusive | `1..=5 => ...` |
| `A \| B` | Ou (l'un ou l'autre) | `1 \| 2 => ...` |

### Exemple complet

```rust
let chiffre = 3;

match chiffre {
    1 => println!("Un"),           // Pattern: valeur exacte
    2 | 3 => println!("Deux ou Trois"),  // Pattern: OU
    4..=6 => println!("Entre 4 et 6"),   // Pattern: plage
    _ => println!("Autre chose"),        // Pattern: attrape-tout
}
// Affiche: "Deux ou Trois"
```

### Différences avec le `switch` classique

| | `switch` (C/JS) | `match` (Rust) |
|---|---|---|
| Oubli de `break` | Bug fréquent | Impossible |
| Valeur retournée | Non | Oui (c'est une expression) |
| Exhaustivité | Pas vérifiée | Obligatoire |
| Patterns | Valeurs simples | Valeux avancés (tuples, structs, etc.) |

> **⚠️ Important :** Le `match` doit être **exhaustif**. Toutes les valeurs possibles doivent être couvertes. Si tu ne couvres pas tous les cas, le compilateur refuse. C'est pourquoi on utilise souvent `_` pour attraper "tout le reste".

**Vocabulaire :**
- *Pattern matching* = comparaison de motifs (*pattern*)
- *Branch* = branche du match (un cas)
- *Catch-all* = attrape-tout (`_`), valeur par défaut
- *Exhaustive* = tous les cas possibles sont couverts

**Exercice à faire :**
1. Crée une fonction `afficher_mention(note: i32)` avec un `match`
2. Affiche la mention selon la note :
   - 90 à 100 → "Excellent"
   - 80 à 89 → "Très bien"
   - 70 à 79 → "Bien"
   - en dessous → "À améliorer"
3. Teste avec `afficher_mention(85)`

Aide : Pour les ranges dans un match : `90..=100` veut dire "de 90 à 100 inclus".

> **💡 Quand utiliser `match` vs `if/else` ?**
> 
> Les deux font des comparaisons, mais ils ont des forces différentes :
> 
> | | `if/else` | `match` |
> |---|---|---|
> | **Meilleur pour** | Conditions booléennes | Comparer à des valeurs précises |
> | **Exemple** | `if x > 5` | `match x { 1 => ..., 2 => ... }` |
> | **Condition complexe** | ✅ `if age > 18 && actif` | ❌ Pas possible directement |
> | **Valeurs discrètes** | ❌ Verbeux | ✅ `match day { Lundi => ... }` |
> | **Retourne une valeur** | ✅ | ✅ |
> | **Vérification exhaustive** | ❌ | ✅ Le compilateur vérifie |
> | **Pattern avancé** | ❌ | ✅ Structs, enums, tuples |
> 
> **Règle simple :**
> - Utilise `if/else` pour des **conditions logiques** (`>`, `<`, `&&`, `||`)
> - Utilise `match` pour **choisir entre des valeurs connues** (énumérées)
> 
> Exemple :
> ```rust
> // if/else : condition logique
> if age >= 18 && actif {
>     println!("Peut voter");
> }
> 
> // match : valeurs discrètes
> match statut {
>     "étudiant" => println!("Tarif réduit"),
>     "senior" => println!("Tarif senior"),
>     _ => println!("Plein tarif"),
> }
> ```

---

## Étape 6 : Boucle `for`

**Explication :** La boucle `for` parcourt un **iterator**. Le plus courant est le `range` (`0..5`). Mais elle peut aussi parcourir des **tableaux** (arrays) et des **vecteurs** (Vec).

### 1. Parcourir un range

```rust
for i in 0..5 {
    println!("i = {}", i);  // Affiche 0, 1, 2, 3, 4
}

// Range inclusif (inclut 5)
for i in 0..=5 {
    println!("i = {}", i);  // Affiche 0, 1, 2, 3, 4, 5
}
```

### 2. Parcourir un tableau (*array*)

Un **array** (tableau) est une collection de taille fixe. Il se déclare avec `[T; N]` où T est le type et N la taille.

```rust
let nombres: [i32; 4] = [10, 20, 30, 40];
//         ^type  ^taille

for n in nombres {
    println!("Valeur: {}", n);
}
```

### 3. Parcourir un vecteur (*Vec*)

Un **vecteur** est une collection de taille dynamique. C'est le type le plus courant pour les listes.

```rust
let fruits = vec!["Pomme", "Banane", "Cerise"];

for fruit in &fruits {
    println!("Fruit: {}", fruit);
}
```

> **⚠️ Note sur le `&` :** On utilise `&fruits` pour emprunter le vecteur. Sans `&`, la boucle prendrait possession (*ownership*) du vecteur, ce qui le rendrait inutilisable après la boucle. C'est une notion d'**ownership** que nous verrons en détail au **Module 04**.

**Vocabulaire :** *Range* = plage, *Iterator* = itérateur, *Array* = tableau (taille fixe), *Vector* = vecteur (taille dynamique), *Ownership* = possession.

**Exercice à faire :**
1. Crée un tableau `notes` contenant `[85, 92, 78, 90, 88]`
2. Utilise une boucle `for` pour calculer la somme des notes
3. Affiche la moyenne (somme / nombre de notes)
4. Affiche chaque note avec "Note X: Y"

> **💡 Indice :** Pour connaître la taille d'un array, utilise `.len()` :
> ```rust
> let taille = notes.len();  // 5
> ```

### Exercice 2 : Fonction avec un tableau ou un vecteur en paramètre

**Explication :** On peut passer un tableau ou un vecteur à une fonction. Pour un tableau, on précise le type et la taille. Pour un vecteur, on utilise `&Vec<T>` ou `&[T]` (un *slice*).

**Signature d'une fonction avec un tableau :**
```rust
fn somme_tableau(nombres: [i32; 4]) -> i32 {
    // On prend un tableau de 4 entiers
    let mut total = 0;
    for n in nombres {
        total += n;
    }
    total
}
```

**Signature d'une fonction avec un vecteur (recommandé) :**
```rust
fn somme_vecteur(nombres: &[i32]) -> i32 {
    // On prend un "slice" (tranche) de i32
    // Le slice fonctionne avec un Vec ou un array !
    let mut total = 0;
    for &n in nombres {
        total += n;
    }
    total
}
```

> **💡 Note :** `&[T]` s'appelle un **slice**. C'est une référence vers une séquence de données. C'est plus flexible car il accepte à la fois un `Vec<T>` et un `[T; N]`. Nous verrons les slices en détail au **Module 08** (Collections).

**Exercice à faire :**
1. Crée une fonction `afficher_notes(notes: &[i32])` qui affiche toutes les notes
2. Crée une fonction `moyenne(notes: &[i32]) -> f64` qui retourne la moyenne
3. Dans `main`, appelle ces fonctions avec un tableau `[15, 18, 12, 16]` et un vecteur `vec![10, 14, 17, 13]`
4. Compare les résultats

---

## Étape 7 : Boucle `while`

**Explication :** Le `while` exécute tant que la condition est vraie. Attention aux boucles infinies !

```rust
let mut compteur = 0;

while compteur < 5 {
    println!("Compteur: {}", compteur);
    compteur += 1;
}
```

**Exercice à faire :**
1. Crée une boucle `while` qui compte de 10 à 1 (décompte)
2. Affiche chaque valeur

---

## Étape 8 : Boucle `loop`

**Explication :** `loop` est une boucle infinie. Elle ne s'arrête que si on utilise `break`. On peut aussi retourner une valeur avec `break`.

```rust
let mut i = 0;
let resultat = loop {
    i += 1;
    if i == 10 {
        break i * 2;  // Retourne 20 et sort
    }
};
println!("{}", resultat);  // 20
```

**Vocabulaire :** *Break* = sortir de la boucle, *Continue* = passer à l'itération suivante.

**Exercice à faire :**
1. Crée une boucle `loop` qui cherche le premier multiple de 7 supérieur à 50
2. Quand tu le trouves, affiche-le et sors avec `break`

Aide : Commence à 1, incrémente, vérifie si `i % 7 == 0 && i > 50`.

---

## Checklist

- [x] Fonctions (fn) : exercice fait
- [x] Expressions vs instructions : exercice fait
- [x] if/else : exercice fait
- [x] match : exercice fait
- [x] for : exercice fait
- [x] while : exercice fait
- [x] loop : exercice fait