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

## Étape 3 : Expressions vs Instructions

**Explication :** En Rust, presque tout est une **expression** (retourne une valeur). Les `if/else` retournent une valeur comme les autres.

```rust
// if/else comme expression
let nombre = if true { 5 } else { 10 };
// nombre vaut 5
```

**Vocabulaire :** *Expression* = retourne une valeur, *Statement* = instruction (ne retourne rien, comme `let x = 5;`)

**Attention :** Chaque branche du `if/else` doit retourner le même type.

**Exercice à faire :**
1. Crée une variable `est_pair` avec `if 10 % 2 == 0 { "oui" } else { "non" }`
2. Affiche "10 est pair ? {est_pair}"

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

**Exercice à faire :**
1. Crée une fonction `categorie(age: i32) -> &'static str`
   - age >= 60 → "Senior"
   - age >= 18 → "Adulte"
   - age >= 12 → "Adolescent"
   - sinon → "Enfant"
2. Teste avec l'âge 25

> **💡 Note :** `&'static str` signifie une chaîne qui existe pendant toute la durée du programme (les littéraux comme "Adulte" ont cette durée de vie). On verra les lifetimes en détail au **Module 04**.

---

## Étape 5 : Pattern matching `match`

**Explication :** `match` est un `switch` puissant. Il compare une valeur à des patterns et exécute le code correspondant. Le `_` est le catch-all (tout le reste).

```rust
let jour = 1;

match jour {
    1 => println!("Lundi"),
    2 => println!("Mardi"),
    3 => println!("Mercredi"),
    _ => println!("Autre"),  // Toutes les autres valeurs
}
```

> **⚠️ Important :** Le `match` doit être exhaustif. Toutes les valeurs possibles doivent être couvertes (d'où le `_`).

**Vocabulaire :** *Pattern matching* = comparaison de motifs, *Branch* = branch du match, *Catch-all* = valeur par défaut.

**Exercice à faire :**
1. Crée une variable `note` valant 85
2. Utilise un `match` pour afficher la mention :
   - >= 90 → "Excellent"
   - >= 80 → "Très bien"
   - >= 70 → "Bien"
   - < 70 → "À améliorer"

Aide : Pour les ranges dans un match : `90..=100` veut dire "de 90 à 100 inclus".

---

## Étape 6 : Boucle `for`

**Explication :** La boucle `for` parcourt un **iterator**. Le plus courant est le range `0..5` (0, 1, 2, 3, 4). Le `..5` n'inclut pas 5.

```rust
for i in 0..5 {
    println!("i = {}", i);  // Affiche 0, 1, 2, 3, 4
}

// Range inclusif (inclut 5)
for i in 0..=5 {
    println!("i = {}", i);  // Affiche 0, 1, 2, 3, 4, 5
}
```

**Vocabulaire :** *Range* = plage, *Iterator* = itérateur, *Inclusive* = inclusif.

**Exercice à faire :**
1. Calcule la somme des nombres de 1 à 10 avec une boucle `for`
2. Affiche le résultat (devrait être 55)

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

- [ ] Fonctions (fn) : exercice fait
- [ ] Expressions vs instructions : exercice fait
- [ ] if/else : exercice fait
- [ ] match : exercice fait
- [ ] for : exercice fait
- [ ] while : exercice fait
- [ ] loop : exercice fait