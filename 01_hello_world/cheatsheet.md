# Cheatsheet - Module 01: Hello World

## Installation
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
cargo --version
```

## Cargo
| Commande | Action |
|----------|--------|
| `cargo new nom` | Créer un projet |
| `cargo run` | Compiler et exécuter |
| `cargo build` | Compiler seulement |
| `cargo build --release` | Build optimisé |

## Affichage
| Macro | Usage |
|-------|-------|
| `println!("{}", x)` | Affiche avec Display |
| `println!("{:?}", x)` | Affiche avec Debug |
| `println!("{:#?}", x)` | Pretty print Debug |
| `format!("{}", x)` | Crée une String |

## Formatage
| Syntaxe | Résultat |
|---------|----------|
| `println!("{}", 31)` | `31` |
| `println!("{0} {1}", "A", "B")` | `A B` (positionnel) |
| `println!("{nom}", nom = "Alice")` | `Alice` (nommé) |
| `println!("{:b}", 255)` | `11111111` (binaire) |
| `println!("{:o}", 255)` | `377` (octal) |
| `println!("{:x}", 255)` | `ff` (hex) |
| `println!("{:.2}", 3.14159)` | `3.14` (2 décimales) |

## Debug sur struct
```rust
#[derive(Debug)]
struct Point(i32);
let p = Point(10);
println!("{:?}", p);   // Point(10)
println!("{:#?}", p);  // Pretty print
```

## Vocabulaire
- *Literal* = valeur écrite dans le code (`"hello"`, `42`)
- *Field* = champ d'une struct
- *Macro* = code qui génère du code (`println!`, `format!`)
- *Display* = trait d'affichage pour l'utilisateur
- *Debug* = trait d'affichage pour le développeur
