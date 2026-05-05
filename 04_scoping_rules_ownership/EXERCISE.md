# Exercice - Module 04

## Objectif

Comprendre l'ownership, le borrowing et les lifetimes en Rust.

---

## Étape 1 : Préparation

```bash
cargo new scoping_ownership
cd scoping_ownership
```

---

## Étape 2 : Scope

**Explication :** Le **scope** est le bloc `{ }` dans lequel une variable existe. Quand on quitte ce bloc, la variable est détruite et sa mémoire est libérée.

```rust
{
    let x = 5;
    println!("{}", x);  // OK, x existe
}
// println!("{}", x);  // ERREUR: x n'existe plus ici
```

**Exercice à faire :**
1. Crée un bloc `{ }` avec une variable `message` valant "Dans le scope"
2. Affiche-la à l'intérieur du bloc
3. Essaye de l'afficher après le bloc → observe l'erreur de compilation

---

## Étape 3 : Ownership (Possession)

**Explication :** En Rust, chaque valeur a un **propriétaire** (*owner*). Il ne peut y avoir qu'un seul propriétaire à la fois. Quand le propriétaire sort du scope, la valeur est détruite.

### Move (Transfert de possession)

Pour les types complexes (comme `String`), l'assignation transfère la possession :

```rust
let s1 = String::from("Bonjour");
let s2 = s1;  // s1 "donne" sa valeur à s2 → s1 n'est plus valide

// println!("{}", s1);  // ERREUR: s1 n'est plus valide
println!("{}", s2);  // OK
```

> **💡 Pourquoi ?** Pour éviter les problèmes de mémoire. Si deux variables pointaient vers la même zone mémoire, la libérer deux fois causerait un crash.

**Exercice à faire :**
1. Crée un `String` `nom` valant "Alice"
2. Assigne-le à `nom2`
3. Essaye d'afficher `nom` → observe l'erreur
4. Affiche `nom2` à la place

---

## Étape 4 : Copy vs Move

**Explication :** Certains types sont simples et ont une taille connue à la compilation. Ils implémentent le trait `Copy` et sont copiés automatiquement.

| Types **Copy** (copie automatique) | Types **Move** (transfert) |
|-----------------------------------|---------------------------|
| `i32`, `u32`, `f64` (nombres) | `String` |
| `bool` | `Vec<T>` |
| `char` | `HashMap<K, V>` |

```rust
// Types Copy
let x = 5;
let y = x;  // y = 5, x reste valide
println!("{}", x);  // OK

// Types Move
let s1 = String::from("hi");
let s2 = s1;  // s1 n'est plus valide
```

**Exercice à faire :**
1. Crée un `i32` `a = 42`
2. Assigne-le à `b`
3. Affiche les deux (prouve que la copie fonctionne)
4. Compare avec un `String` (move)

---

## Étape 5 : Borrowing (Emprunt) avec `&`

**Explication :** Au lieu de transférer la possession, on peut **emprunter** une variable avec `&`. Cela crée une **référence** (*reference*) vers la valeur.

```rust
let s1 = String::from("Bonjour");
let len = calculer_longueur(&s1);  // On emprunte s1, on ne le donne pas
println!("{} a {} caractères", s1, len);  // s1 est toujours valide !

fn calculer_longueur(s: &String) -> usize {
    s.len()
}
```

> **💡 Analogie :** Prêter un livre vs donner un livre. Avec le prêt (`&`), tu peux toujours lire ton livre après.

**Exercice à faire :**
1. Crée une fonction `afficher(nom: &String)` qui affiche "Nom: {nom}"
2. Crée un `String` `personne` valant "Bob"
3. Appelle `afficher(&personne)`
4. Affiche `personne` après (prouve qu'il est toujours valide)

---

## Étape 6 : Mutable Borrow (`&mut`)

**Explication :** Par défaut, les références sont en lecture seule. Pour modifier une valeur empruntée, il faut un **emprunt mutable** avec `&mut`.

```rust
let mut s = String::from("Bonjour");
ajouter_point_virgule(&mut s);  // Emprunt mutable
println!("{}", s);  // "Bonjour;"

fn ajouter_point_virgule(s: &mut String) {
    s.push_str(";");
}
```

**Règles importantes :**
1. Une seule référence mutable à la fois (pas de `&mut` + `&` en même temps)
2. La variable originale doit être déclarée avec `mut`

**Exercice à faire :**
1. Crée une fonction `mettre_en_majuscules(texte: &mut String)`
2. Utilise `texte.make_ascii_uppercase()` pour modifier
3. Teste avec un `String mutable`

---

## Étape 7 : Lifetimes (Durées de vie)

**Explication :** Un **lifetime** (`'a`) indique combien de temps une référence est valide. Le compilateur les infère souvent, mais parfois il faut les expliciter.

```rust
// Le compilateur infère que les deux références ont la même durée de vie
fn plus_long<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

> **💡 À retenir :** Un lifetime garantit que la référence retournée ne vivra pas plus longtemps que les données qu'elle pointe.

**Exercice à faire :**
1. Crée une fonction `plus_court<'a>(s1: &'a str, s2: &'a str) -> &'a str`
2. Retourne la chaîne la plus courte
3. Teste avec "Bonjour" et "Salut"

---

## Checklist

- [ ] Scope : exercice fait
- [ ] Ownership (move) : exercice fait
- [ ] Copy vs Move : exercice fait
- [ ] Borrowing (&) : exercice fait
- [ ] Mutable borrow (&mut) : exercice fait
- [ ] Lifetimes : exercice fait