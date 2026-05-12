// Étape 2 : Enum simple
#[derive(Debug)]
enum Direction {
    Nord,
    Sud,
    Est,
    Ouest,
}

// Étape 3 : Enum avec données
#[allow(dead_code)]
#[derive(Debug)]
enum CarteBancaire {
    Visa(u32),
    Mastercard(u32, u8),
    Inconnue,
}

// Étape 5 : Fonction avec Option
fn diviser(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

// Étape 7 : Fonction avec Result
#[allow(dead_code)]
fn diviser_result(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division par zéro"))
    } else {
        Ok(a / b)
    }
}

fn verifier_age(age: i32) -> Result<i32, String> {
    if age < 0 {
        Err(String::from("Âge négatif impossible"))
    } else if age > 150 {
        Err(String::from("Âge trop grand"))
    } else {
        Ok(age)
    }
}

// Étape 8 : Enum avec impl
impl Direction {
    fn oppose(&self) -> Direction {
        match self {
            Direction::Nord => Direction::Sud,
            Direction::Sud => Direction::Nord,
            Direction::Est => Direction::Ouest,
            Direction::Ouest => Direction::Est,
        }
    }
}

// FeuTricolore avec les deux versions de suivant()
#[derive(Debug, Clone, Copy)]
enum FeuTricolore {
    Rouge,
    Orange,
    Vert,
}

impl FeuTricolore {
    // Version CONSOMMANTE : retourne une nouvelle valeur
    fn suivant(&self) -> FeuTricolore {
        match self {
            FeuTricolore::Rouge => FeuTricolore::Vert,
            FeuTricolore::Vert => FeuTricolore::Orange,
            FeuTricolore::Orange => FeuTricolore::Rouge,
        }
    }

    // Version MUTATRICE : modifie self sur place
    fn suivant_mut(&mut self) {
        *self = match self {
            FeuTricolore::Rouge => FeuTricolore::Vert,
            FeuTricolore::Vert => FeuTricolore::Orange,
            FeuTricolore::Orange => FeuTricolore::Rouge,
        }
    }

    fn afficher(&self) {
        match self {
            FeuTricolore::Rouge => println!("  STOP"),
            FeuTricolore::Orange => println!("  ATTENTION"),
            FeuTricolore::Vert => println!("  PASSEZ"),
        }
    }
}

fn afficher_direction(d: Direction) {
    match d {
        Direction::Nord => println!("Direction: Nord"),
        Direction::Sud => println!("Direction: Sud"),
        _ => println!("Direction inconnue"),
    }
}

fn main() {
    // Étape 2 : Enum simple
    let ma_direction = Direction::Nord;
    println!("ma_direction: {:?}", ma_direction);

    // Étape 3 : Enum avec données
    let carte1 = CarteBancaire::Visa(123456);
    let carte2 = CarteBancaire::Mastercard(789012, 123);
    println!("Carte 1: {:?}", carte1);
    println!("Carte 2: {:?}", carte2);

    match carte1 {
        CarteBancaire::Visa(num) => println!("Visa n°{}", num),
        CarteBancaire::Mastercard(num, code) => {
            println!("Mastercard n°{} code {}", num, code)
        }
        CarteBancaire::Inconnue => println!("Carte inconnue"),
    }

    // Étape 4 : match sur enum
    afficher_direction(Direction::Nord);
    afficher_direction(Direction::Ouest);

    // Étape 5 : Option
    match diviser(10.0, 2.0) {
        Some(v) => println!("10 / 2 = {}", v),
        None => println!("Erreur: division par zéro"),
    }
    match diviser(10.0, 0.0) {
        Some(v) => println!("10 / 0 = {}", v),
        None => println!("Erreur: division par zéro"),
    }

    // Étape 6 : if let
    if let Some(v) = diviser(20.0, 4.0) {
        println!("20 / 4 = {} (avec if let)", v);
    }

    // Étape 7 : Result
    match verifier_age(-5) {
        Ok(age) => println!("Âge valide: {}", age),
        Err(e) => println!("Erreur: {}", e),
    }
    match verifier_age(200) {
        Ok(age) => println!("Âge valide: {}", age),
        Err(e) => println!("Erreur: {}", e),
    }
    match verifier_age(25) {
        Ok(age) => println!("Âge valide: {}", age),
        Err(e) => println!("Erreur: {}", e),
    }

    // Étape 8 : Enum avec impl
    let nord = Direction::Nord;
    println!("Nord opposé: {:?}", nord.oppose());
    println!("Sud opposé: {:?}", Direction::Sud.oppose());

    // FeuTricolore : démo des deux approches
    println!("\n--- Version CONSOMMANTE (retourne une nouvelle valeur) ---");
    let mut feu = FeuTricolore::Vert;
    feu.afficher();
    feu = feu.suivant(); // On doit réassigner
    feu.afficher();
    feu = feu.suivant();
    feu.afficher();

    println!("\n--- Version MUTATRICE (modifie sur place) ---");
    let mut feu2 = FeuTricolore::Vert;
    feu2.afficher();
    feu2.suivant_mut(); // Pas de réassignation
    feu2.afficher();
    feu2.suivant_mut();
    feu2.afficher();
}
