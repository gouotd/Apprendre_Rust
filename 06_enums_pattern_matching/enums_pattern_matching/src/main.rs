#[derive(Debug)]
enum Direction {
    Nord,
    Sud,
    Est,
    Ouest,
}

#[derive(Debug)]
enum CarteBancaire {
    Visa(u32),
    Mastercard(u32, u8),
    Inconnue,
}

fn trouver_personne(nom: &str) -> Option<String> {
    if nom == "Alice"{
        Some(String::from("Alice a 30 ans"))
    } else {
        None
    }
}

fn diviser(a: f64, b: f64) -> Option<f64>{
    if b == 0.0{
        None
    } else {
        Some(a/b)
    }
}

fn diviser_result(a: f64, b : f64) -> Result<f64, String>{
    if b == 0.0{
        Err(String::from("Division par zéro"))
    } else {
        Ok(a/b)
    }
}
fn verifier_age(age: i32) -> Result<i32, String>{
    if age <0 {
        Err(String::from("Âge négatif impossible"))
    } else if age > 150 {
        Err(String::from("Âge trop grand"))
    } else {
        Ok(age)
    }
}

#[derive(Debug)]
enum FeuTricolore {
    Rouge,
    Orange,
    Vert,
}
impl FeuTricolore{
    fn suivant (&mut self) {
        *self = match self {
            FeuTricolore::Rouge => FeuTricolore::Vert,
            FeuTricolore::Vert => FeuTricolore::Orange,
            FeuTricolore::Orange => FeuTricolore::Rouge,
        }
    }
    fn afficher (&self) {
        match self {
            FeuTricolore::Rouge => println!("STOP"),
            FeuTricolore::Orange => println!("ATTENTION"),
            FeuTricolore::Vert => println!("PASSEZ"),
        }
    }
}
fn main() {
    let ma_direction = Direction::Nord;
    println!("{:?}",ma_direction);

    let ma_visa = CarteBancaire::Visa(123456);
    println!("{:?}",ma_visa);
    let ma_mastercard = CarteBancaire::Mastercard(789012, 123);
    println!("{:?}", ma_mastercard);

    let resultat = trouver_personne("Alice");
    match resultat {
        Some(description) => println!("{}", description),
        None => println!("Personne non trouvée"),
    }

    let a = 55.0;
    let b = 5.0;
    let mut division = diviser(a,b);
    match division{
        Some(resultat_division) => println!("Le resultat de la division est : {}",resultat_division),
        None => println!("Division apr 0 impossible"),
    }

    if let Some(resultat) = division{
        println!("Le resultat de la division est : {}",resultat)
    }

    let c = 128.3;
    let d = 0.0;
    division = diviser(c,d);
    match division{
        Some(resultat_division) => println!("Le resultat de la division est : {}",resultat_division),
        None => println!("Division par 0 impossible"),
    }

    if let Some(resultat) = division{
        println!("Le resultat de la division est : {}",resultat)
    }
    
    match diviser_result (a, b){
        Ok(valeur) => println!("Le resultat de la division est : {}",valeur),
        Err(message_erreur) => println!("Erreur : {}",message_erreur),
    }

    match diviser_result (c, d){
        Ok(valeur) => println!("Le resultat de la division est : {}",valeur),
        Err(message_erreur) => println!("Erreur : {}",message_erreur),
    }

    match verifier_age(-5){
        Ok(resultat) => println!("L'Âge est : {}",resultat),
        Err(message_erreur) => println!("Erreur : {}",message_erreur),
    }

    match verifier_age(205){
        Ok(resultat) => println!("L'Âge est : {}",resultat),
        Err(message_erreur) => println!("Erreur : {}",message_erreur),
    }
    match verifier_age(58){
        Ok(resultat) => println!("L'Âge est : {}",resultat),
        Err(message_erreur) => println!("Erreur : {}",message_erreur),
    }

    let mut MonFeuTricolor = FeuTricolore::Vert;
    MonFeuTricolor.afficher();
    MonFeuTricolor.suivant();
    MonFeuTricolor.afficher();
    MonFeuTricolor.suivant();
    MonFeuTricolor.afficher();
}
