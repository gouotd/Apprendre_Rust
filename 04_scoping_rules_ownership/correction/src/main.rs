// 3.a. Ownership - Move
fn exercice_move() {
    let nom = String::from("Alice");
    let nom2 = nom; // nom transfère sa possession à nom2

    // println!("{}", nom);  // ERREUR: nom n'est plus valide
    println!("Move: {}", nom2);
}

// 3.b. Clone pour garder les deux
fn exercice_clone() {
    let texte = String::from("Bonjour");
    let texte2 = texte.clone(); // Copie profonde: texte2 a ses propres données
    println!("Clone: texte={}, texte2={}", texte, texte2);
}

// 4. Copy vs Move
fn exercice_copy() {
    let a = 42;
    let b = a; // Copy: a reste valide
    println!("Copy: a={}, b={}", a, b);

    // Compare avec move
    let s1 = String::from("test");
    let s2 = s1; // Move: s1 n'est plus valide
    println!("Move: s2={}", s2);
}

// 5. Borrowing (&)
fn afficher(nom: &String) {
    println!("Borrowing: Nom: {}", nom);
}

fn exercice_borrowing() {
    let personne = String::from("Bob");
    afficher(&personne);
    println!("Borrowing: personne est toujours valide: {}", personne);
}

// 6. Mutable borrow (&mut)
fn mettre_en_majuscules(texte: &mut String) {
    texte.make_ascii_uppercase();
}

fn exercice_mut_borrow() {
    let mut texte = String::from("bonjour");
    mettre_en_majuscules(&mut texte);
    println!("Mutable borrow: {}", texte);
}

// 7. Lifetimes
fn plus_court<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() < s2.len() {
        s1
    } else {
        s2
    }
}

fn exercice_lifetimes() {
    let resultat = plus_court("Bonjour", "Salut");
    println!("Lifetime: le plus court est '{}'", resultat);
}

fn main() {
    // 2. Scope
    {
        let message = "Dans le scope";
        println!("Scope: {}", message);
    }
    // message n'existe plus ici

    // 3.a. Move
    exercice_move();

    // 3.b. Clone
    exercice_clone();

    // 4. Copy vs Move
    exercice_copy();

    // 5. Borrowing
    exercice_borrowing();

    // 6. Mutable borrow
    exercice_mut_borrow();

    // 7. Lifetimes
    exercice_lifetimes();
}
