fn main() {
    // 3.1. let et mutabilité
    let mut compte = 100;
    compte = compte + 50;
    println!("Compte: {}", compte);

    // 3.2. Type explicite
    let age: i32 = 25;
    let nom: &str = "Bob";
    println!("Nom: {}, Age: {}", nom, age);

    // 3.3. Constantes
    const TAXE: f64 = 0.20;
    let prix_ht: f64 = 100.0;
    let prix_ttc = prix_ht * (1.0 + TAXE);
    println!("Prix TTC: {}€", prix_ttc);

    // 3.4. Shadowing
    let message = "Hello";
    let message = "World";
    println!("{}", message);

    // 3.5. Inference de type
    let a = 42;
    let b = 3.14;
    let c = true;
    println!("{:?}", (a, b, c));

    // 3.6. Casting
    let x: f64 = 100 as f64;
    println!("100 en f64: {}", x);

    let z: f64 = 5.9;
    let w: i32 = z as i32;
    println!("5.9 tronqué: {}", w);

    let pourcentage: i32 = (80 as f64 / 100.0 * 200.0) as i32;
    println!("80% de 200: {}", pourcentage);
}
