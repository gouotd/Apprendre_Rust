# Hello World - Module 01

## Objectifs

- Installer Rust via rustup
- Comprendre la structure d'un projet Cargo
- Maîtriser les macros de formattage : `println!`, `format!`
- Comprendre la différence entre `{:?}` (Debug) et `{}` (Display)

## Résumé

Ce module couvre les bases de l'affichage en Rust. Nous avons vu :
- L'installation de Rust avec `rustup`
- La création d'un projet avec `cargo new`
- Les macros `println!`, `print!`, `eprintln!`
- Le formattage avec positionnement, named arguments
- Les formats : `:b` (binary), `:o` (octal), `:x` (hex)
- Debug vs Display avec `{:?}` et `{:#?}`

## Connaissances clés

| Concept | Description |
|---------|-------------|
| `println!("{}", x)` | Affiche avec Display |
| `println!("{:?}", x)` | Affiche avec Debug |
| `println!("{:#?}", x)` | Pretty print Debug |
| `format!("{}", x)` | Crée une String |
| `cargo run` | Compile et exécute |
| `cargo build` | Compile seulement |