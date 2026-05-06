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

**Explication :** En Rust, chaque valeur a un **propriétaire** (*owner*). Il ne peut y avoir qu'un seul propriétaire à la fois. Quand le propriétaire sort du scope, la valeur est détruite et la mémoire est libérée.

C'est le concept fondamental qui rend Rust sûr sans garbage collector.

### Les 3 règles de l'ownership

1. Chaque valeur a un propriétaire (une variable)
2. Il ne peut y avoir qu'un seul propriétaire à la fois
3. Quand le propriétaire sort du scope, la valeur est supprimée

```rust
{
    let s = String::from("Bonjour");  // s devient le propriétaire
    // s est utilisable ici
}                                      // s sort du scope → String est détruit
// println!("{}", s);  // ERREUR: s n'existe plus
```

---

### Move (Transfert de possession)

Pour les types complexes qui stockent des données sur le **heap** (mémoire dynamique), l'assignation **transfère** la possession :

```rust
let s1 = String::from("Bonjour");  // s1 possède le String
let s2 = s1;                        // s1 "donne" sa valeur à s2
                                     // → s1 n'est plus valide

println!("{}", s2);  // OK
// println!("{}", s1);  // ERREUR: use of moved value
```

**Pourquoi le Move ?**

Imagine deux variables pointant vers la même zone mémoire. Quand elles sortent du scope, Rust appelle `drop()` pour libérer la mémoire. Si deux variables libèrent la même zone → **double free** = crash mémoire.

Pour éviter ça, Rust transfère la possession. L'ancienne variable est "invalidée", seule la nouvelle libère la mémoire.

**Analogie :** C'est comme un titre de propriété. Tu ne peux pas donner une maison à deux personnes. Si tu la vends à B, A n'est plus propriétaire.

### Avantages du Move

| ✅ Avantages | ❌ Inconvénients |
|---|---|
| **Sécurité mémoire** : pas de double free | **Usage** : on ne peut pas réutiliser la variable après |
| **Pas de copie coûteuse** : on ne copie pas le contenu du String | **Apprentissage** : concept nouveau pour les débutants |
| **Garantie** : une seule variable libère la mémoire | **Code** : il faut repenser la façon de passer des données |
| **Performance** : transférer un pointeur est très rapide | |

### Quand le Move se produit ?

| Situation | Exemple | Se produit ? |
|---|---|---|
| Assignation | `let s2 = s1;` | ✅ Oui (String, Vec, etc.) |
| Passage à une fonction | `fn f(s: String)` → `f(s1);` | ✅ Oui |
| Retour de fonction | `fn f() -> String { s }` | ✅ Oui |

---

## Étape 4 : Copy vs Move

**Explication :** Certains types sont simples, ont une taille connue à la compilation et sont stockés sur le **stack** (mémoire rapide). Ils implémentent le trait `Copy` et sont copiés automatiquement lors de l'assignation.

### Comment savoir si un type implémente Copy ?

**Règle générale :** Si le type contient des données sur le **heap** (mémoire dynamique), il n'implémente **pas** `Copy`.

| Types **Copy** (stockés sur stack) | Types **Move** (stockés sur heap) |
|---|---|
| `i32`, `u32`, `i64`, `f64` | `String` (texte dynamique) |
| `bool` (`true` / `false`) | `Vec<T>` (liste dynamique) |
| `char` (`'a'`, `'@'`) | `HashMap<K, V>` |
| `&T` (références, même vers des types Move) | `Box<T>` (pointeur intelligent) |
| `[T; N]` (array de types Copy) | `VecDeque<T>`, etc. |

### Copy en détail

Quand un type implémente `Copy`, l'assignation fait une **copie bitwise** (copie bête des bits) :

```rust
let x: i32 = 42;
let y = x;  // Rust copie les 32 bits de x dans y
println!("{}", x);  // x existe toujours, il a été copié
println!("{}", y);  // y est une copie indépendante
```

**Analogie :** C'est comme photocopier un papier. L'original et la copie sont deux documents indépendants.

### Move en détail

Quand un type n'implémente pas `Copy`, l'assignation **transfère** la possession :

```rust
let s1 = String::from("Bonjour");  // s1 pointe vers des données sur le heap
let s2 = s1;                        // s2 prend possession, s1 est invalidé
println!("{}", s2);                 // OK
// println!("{}", s1);             // ERREUR: s1 n'existe plus
```

**Analogie :** C'est comme déménager. Tu ne peux pas être dans deux maisons en même temps. Quand tu déménages, tu ne possèdes plus l'ancienne maison.

### Relation Copy/Move avec l'Ownership

| | Copy | Move |
|---|------|------|
| **Ownership** | Chaque variable a sa propre copie | Une seule variable possède la donnée |
| **Mémoire** | Stack (rapide) | Heap (dynamique) |
| **Coût** | Négligeable (quelques octets) | Transfert de pointeur (très rapide) |
| **Après assignation** | Les deux variables sont valides | L'ancienne est invalidée |

### Comment contourner le Move ?

Si tu veux garder les deux variables, tu as deux options :

**1. Cloner explicitement (copier les données heap) :**
```rust
let s1 = String::from("Bonjour");
let s2 = s1.clone();  // Copie profonde : s2 a ses propres données
println!("{}", s1);   // OK
println!("{}", s2);   // OK
```

**2. Emprunter avec `&` (voir Étape 5) :**
```rust
let s1 = String::from("Bonjour");
let s2 = &s1;         // s2 emprunte s1, ne prend pas possession
println!("{}", s1);   // OK
println!("{}", s2);   // OK
```

### Comparaison des approches

| Approche | Coût | Les deux valides ? | Usage |
|---|---|---|---|
| **Move** (`let s2 = s1`) | Gratuit (pointeur) | Non | Par défaut, quand on transfère |
| **Clone** (`s1.clone()`) | Coûteux (copie heap) | Oui | Quand on veut deux copies indépendantes |
| **Borrow** (`&s1`) | Gratuit (référence) | Oui | Quand on veut juste lire/modifier |

> **💡 Note sur `&str` :** Les références (`&T`) implémentent `Copy` ! Donc `&str` est copiée, pas déplacée. C'est pourquoi `let nom2 = nom;` avec un `&str` laisse `nom` valide.

### Exercice 3a : Move

1. Crée un `String` `nom` valant "Alice"
2. Assigne-le à `nom2`
3. Essaye d'afficher `nom` → observe l'erreur
4. Affiche `nom2` à la place

### Exercice 3b : Cloner pour garder les deux

1. Crée un `String` `texte` valant "Bonjour"
2. Utilise `.clone()` pour créer `texte2`
3. Affiche les deux (prouve qu'ils sont tous les deux valides)

---

## Étape 4 : Copy vs Move

**Explication :** Certains types sont simples, ont une taille connue à la compilation et sont stockés sur le **stack** (mémoire rapide). Ils implémentent le trait `Copy` et sont copiés automatiquement lors de l'assignation.

### Exercice 4 : Comparer Copy et Move

1. Crée un `i32` `a = 42`
2. Assigne-le à `b`
3. Affiche les deux (prouve que la copie fonctionne)
4. Crée un `String` `s1 = String::from("test")`
5. Assigne-le à `s2`
6. Essaye d'afficher `s1` → observe l'erreur
7. Affiche `s2`

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

> **⚠️ Piège courant : conflit de borrows**
> 
> Tu ne peux pas utiliser la variable originale **pendant** qu'un emprunt mutable existe :
> 
> ```rust
> let mut texte = String::from("bonjour");
> let texte_borrow = &mut texte;
> 
> // ERREUR: cannot borrow `texte` as immutable because it is also borrowed as mutable
> println!("{}", texte);  // ← texte utilisé alors que texte_borrow existe
> ```
> 
> **Solution :** Limite le scope du borrow mutable avec `{ }` :
> 
> ```rust
> let mut texte = String::from("bonjour");
> 
> {
>     let texte_borrow = &mut texte;
>     texte_borrow.make_ascii_uppercase();
> } // ← texte_borrow est détruit ici, l'emprunt se termine
> 
> println!("{}", texte);  // OK maintenant !
> ```

**Exercice à faire :**
1. Crée une fonction `mettre_en_majuscules(texte: &mut String)`
2. Utilise `texte.make_ascii_uppercase()` pour modifier
3. Teste avec un `String mutable`

---

## Étape 7 : Lifetimes (Durées de vie)

### C'est quoi un lifetime ?

**Ce n'est pas une mesure de temps** (pas en secondes, pas en minutes). C'est une notion de **scope** (bloc de visibilité).

Un **lifetime** (`'a`) est une **étiquette** que le compilateur utilise pour vérifier qu'une **référence ne vit pas plus longtemps que la donnée qu'elle pointe**.

### Le problème que les lifetimes résolvent

Imagine ce scénario :

```rust
fn donner_reference() -> &i32 {
    let x = 5;
    &x  // ERREUR: x sera détruit à la fin de la fonction !
}
```

Quand la fonction se termine, `x` est détruit. Si on retournait `&x`, on aurait une **référence pendante** (*dangling reference*) : un pointeur vers de la mémoire qui n'existe plus. C'est un bug grave.

Rust **interdit ça à la compilation** grâce aux lifetimes.

### Que signifie `'a` ?

`'a` n'est **pas une durée**. C'est un **nom de variable** pour un scope. C'est comme un `T` pour les génériques, mais pour les références.

```rust
fn plus_long<'a>(s1: &'a str, s2: &'a str) -> &'a str
```

Cette signature dit au compilateur :
> "Je prends deux références qui vivent au moins `'a`, et je retourne une référence qui vit `'a`."

En pratique, ça signifie :
> "La référence retournée vivra **aussi longtemps que la plus courte** des deux références en entrée."

### Exemple concret : sans lifetime

```rust
fn plus_long(s1: &str, s2: &str) -> &str {
    // ERREUR: le compilateur ne sait pas quelle référence est retournée
    // Donc il ne sait pas combien de temps la référence retournée vit
}
```

Le compilateur ne peut pas deviner si la référence retournée vient de `s1` ou de `s2`. Il a besoin qu'on lui dise qu'elles ont la **même durée de vie**.

### Exemple concret : avec lifetime

```rust
fn plus_long<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1  // Retourne s1
    } else {
        s2  // Retourne s2
    }
}
```

Ici, `'a` dit au compilateur : "Les deux entrées et la sortie ont la même durée de vie."

### Comment Rust vérifie ?

Quand tu appelles la fonction, Rust vérifie que les références que tu passes **existent au moins aussi longtemps** que le résultat que tu utilises :

```rust
fn main() {
    let string1 = String::from("long string");
    //                    └─ string1 vit jusqu'à la fin de main

    let result;
    {
        let string2 = String::from("xyz");
        //              └─ string2 vit seulement dans ce bloc
        result = plus_long(&string1, &string2);
    }
    // string2 est détruit ici !

    println!("Résultat: {}", result);
    // ↑ ERREUR: result pointe vers string2 qui n'existe plus !
}
```

Rust refuse ce code car `result` pourrait pointer vers `string2`, mais `string2` est détruit avant que `result` ne soit utilisé.

> **❓ "Mais `result` devrait pointer vers `string1`, il est plus long !"**
> 
> C'est vrai du point de vue humain, mais le compilateur **ne sait pas** quelle branche sera exécutée à la compilation. La fonction `plus_long` décide au **runtime**, pas à la compilation.
> 
> Le compilateur ne peut pas deviner si `s1.len() > s2.len()` sera vrai ou faux. Donc il prend le **pire cas** : il suppose que `result` **pourrait** pointer vers `string2` (le plus court des deux).
> 
> ```
> ┌─────────────────────────────────────┐
> │ string1 ─────────────────────────── │ (vit longtemps)
> │         ┌──────┐                    │
> │ string2 │??????│                    │ (vit peu)
> │         └──────┘                    │
> │ result  ← pourrait pointer ici !    │
> └─────────────────────────────────────┘
> ```
> 
> **Rust est conservateur** : s'il y a un risque (même minime) de référence pendante, il refuse. C'est le prix de la sécurité mémoire sans garbage collector.

### Quand as-tu besoin d'annoter les lifetimes ?

**Cas 1 : Retourner une référence**
```rust
fn premier_element<'a>(tableau: &'a [i32]) -> &'a i32 {
    &tableau[0]
}
```

**Cas 2 : Struct avec des références**
```rust
struct Vue<'a> {
    donnees: &'a str,  // La référence doit vivre aussi longtemps que la struct
}
```

**Cas 3 : Méthode avec retour de référence**
```rust
impl<'a> Vue<'a> {
    fn obtenir(&self) -> &'a str {
        self.donnees
    }
}
```

### Dans combien de cas Rust le devine tout seul ?

Dans **~90% des cas**, le compilateur infère les lifetimes automatiquement. C'est ce qu'on appelle le **lifetime elision**. Tu n'as besoin de les écrire explicitement que quand le compilateur ne peut pas deviner.

### Résumé en une phrase

> Un **lifetime** (`'a`) est une **étiquette de scope** que le compilateur utilise pour garantir qu'une référence n'est jamais utilisée après que la donnée qu'elle pointe a été détruite.

**Exercice à faire :**
1. Crée une fonction `plus_court<'a>(s1: &'a str, s2: &'a str) -> &'a str`
2. Retourne la chaîne la plus courte
3. Teste avec "Bonjour" et "Salut"
4. Vérifie que le résultat est toujours utilisable après l'appel

---

## Checklist

- [x] Scope : exercice fait
- [x] Ownership (move) : exercice fait
- [x] Copy vs Move : exercice fait
- [x] Borrowing (&) : exercice fait
- [x] Mutable borrow (&mut) : exercice fait
- [x] Lifetimes : exercice fait