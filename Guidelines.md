# Guide d'apprentissage Rust

## Plan de progression

| # | Module | Contenu RBE intégré | [ ] Fait |
|---|--------|-------------------|----------|
| 01 | Hello World + Primitives + Debug/Display | Rust install, Cargo, println!, format!, {:?}, Debug vs Display, field | [x] |
| 02 | Variable Bindings + Types + Conversion | let, mut, shadowing, constantes, type inference, casting | [x] |
| 03 | Flow of Control + Expressions + Functions | if/else, match, for, while, loop, fn, closures | [ ] |
| 04 | Scoping rules + Ownership | ownership, borrowing, &mut &, lifetimes | [ ] |
| 05 | Custom Types + Attributes | struct, impl, new(), self, #[derive] | [ ] |
| 06 | Enums + Pattern Matching | enum, match, if let, Option, Result | [ ] |
| 07 | Traits + Generics | trait, impl Trait, dyn Trait, T: Bound | [ ] |
| 08 | Collections | Vec, HashMap, String, slice, &str | [ ] |
| 09 | Iterators + Closures | iterators, closures, functional style | [ ] |
| 10 | Modules + Crates | mod, use, visibility, crate structure | [ ] |
| 11 | Error Handling | Result, ? operator, Box<dyn Error> | [ ] |

## Commandes essentielles

```bash
# Nouvel projet
cargo new nom_projet

# Compiler et exécuter
cargo run

# Build de production
cargo build --release

# Tests
cargo test

# Doc
cargo doc --open
```

## Ressources

- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [The Rust Book](https://doc.rust-lang.org/stable/book/)
- [Rustlings](https://github.com/rust-lang/rustlings)