// 2. Fonctions
fn carre(x: i32) -> i32 {
    x * x
}

fn saluer(nom: &str) {
    println!("Bonjour, {}!", nom);
}

// 3. Expressions vs instructions
fn verifier_pair(nombre: i32) {
    let resultat = if nombre % 2 == 0 { "pair" } else { "impair" };
    println!("Le nombre {} est {}", nombre, resultat);
}

// 4.a. Accès avec && et ||
fn verifier_acces(age: i32, a_carte: bool) {
    if age >= 18 && a_carte {
        println!("Accès autorisé");
    } else {
        if age < 18 && !a_carte {
            println!("Accès refusé: âge insuffisant et pas de carte");
        } else if age < 18 {
            println!("Accès refusé: âge insuffisant");
        } else {
            println!("Accès refusé: pas de carte");
        }
    }
}

// 4.b. Catégorie d'âge
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

// 5. Match
fn afficher_mention(note: i32) {
    match note {
        90..=100 => println!("Excellent"),
        80..=89 => println!("Très bien"),
        70..=79 => println!("Bien"),
        _ => println!("À améliorer"),
    }
}

// 6.c. Fonction avec tableau/vecteur en paramètre
fn afficher_notes(notes: &[i32]) {
    for (i, &note) in notes.iter().enumerate() {
        println!("Note {}: {}", i + 1, note);
    }
}

fn moyenne(notes: &[i32]) -> f64 {
    let mut total = 0;
    for &note in notes {
        total += note;
    }
    total as f64 / notes.len() as f64
}

fn main() {
    // 2. Fonctions
    saluer("Alice");
    println!("3² = {}", carre(3));

    // 3. Expressions vs instructions
    verifier_pair(10);
    verifier_pair(7);

    // 4.a. Accès avec && et ||
    verifier_acces(20, true);
    verifier_acces(15, true);
    verifier_acces(20, false);
    verifier_acces(15, false);

    // 4.b. Catégorie d'âge
    println!("Age 25 → {}", categorie(25));

    // 5. Match
    afficher_mention(85);
    afficher_mention(95);
    afficher_mention(60);

    // 6. For loop
    let mut somme = 0;
    for i in 1..=10 {
        somme += i;
    }
    println!("Somme 1 à 10: {}", somme);

    // 6.b. Parcourir un array
    let notes = [85, 92, 78, 90, 88];
    let mut total = 0;
    for (i, &note) in notes.iter().enumerate() {
        println!("Note {}: {}", i + 1, note);
        total += note;
    }
    let moyenne_calc = total as f64 / notes.len() as f64;
    println!("Moyenne: {:.2}", moyenne_calc);

    // 6.c. Fonction avec tableau/vecteur en paramètre
    println!("\n--- Avec un array [15, 18, 12, 16] ---");
    let notes_array = [15, 18, 12, 16];
    afficher_notes(&notes_array);
    println!("Moyenne array: {:.2}", moyenne(&notes_array));

    println!("\n--- Avec un vec![10, 14, 17, 13] ---");
    let notes_vec = vec![10, 14, 17, 13];
    afficher_notes(&notes_vec);
    println!("Moyenne vec: {:.2}", moyenne(&notes_vec));

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
