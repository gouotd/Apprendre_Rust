// 2. Fonctions

fn carre(x: i32) -> i32 {
    x * x
}

fn saluer(nom: &str) {
    println!("Bonjour, {}!", nom);
}

// 4. Catégorie d'âge
fn categorie(age: i32) -> &'static str {
    if age >= 60 {
        "Senior"
    } else if age >= 18 {
        "Adulte"
    } else if age >= 12 {
        "Adolescent"
    } else {
        "Enfant"
    }
}

fn main() {
    // 2. Fonctions
    saluer("Alice");
    println!("3² = {}", carre(3));

    // 3. Expressions vs instructions
    let est_pair = if 10 % 2 == 0 { "oui" } else { "non" };
    println!("10 est pair ? {}", est_pair);

    // 4. Catégorie d'âge
    println!("Age 25 → {}", categorie(25));

    // 5. Match
    let note = 85;
    match note {
        90..=100 => println!("Excellent"),
        80..=89 => println!("Très bien"),
        70..=79 => println!("Bien"),
        _ => println!("À améliorer"),
    }

    // 6. For loop
    let mut somme = 0;
    for i in 1..=10 {
        somme += i;
    }
    println!("Somme 1 à 10: {}", somme);

    // 7. While loop
    let mut compteur = 10;
    while compteur > 0 {
        println!("Décompte: {}", compteur);
        compteur -= 1;
    }

    // 8. Loop
    let mut i = 1;
    let multiple = loop {
        if i % 7 == 0 && i > 50 {
            break i;
        }
        i += 1;
    };
    println!("Premier multiple de 7 > 50: {}", multiple);
}
