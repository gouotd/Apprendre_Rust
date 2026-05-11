// Étape 2.1 : Struct avec champs nommés
#[derive(Debug, Clone)]
struct Personne {
    nom: String,
    age: u32,
}

// Étape 2.2 : Tuple struct
struct Point3D(u8, u8, u8);

// Étape 2.3 : Unit struct (badge)
struct Connecte;

fn se_connecter(cle: &str) -> Option<Connecte> {
    if cle == "admin" {
        Some(Connecte)
    } else {
        None
    }
}

fn acceder_admin(_badge: Connecte) {
    println!("Accès admin accordé");
}

// Étape 4 : CompteBancaire avec méthodes
#[derive(Debug)]
struct CompteBancaire {
    titulaire: String,
    solde: f64,
}

impl CompteBancaire {
    // Étape 5 : Méthode associée (constructeur)
    fn new(titulaire: &str, solde: f64) -> Self {
        CompteBancaire {
            titulaire: titulaire.to_string(),
            solde,
        }
    }

    fn deposer(&mut self, montant: f64) {
        self.solde += montant;
        println!("Dépôt de {}€ effectué", montant);
    }

    fn retirer(&mut self, montant: f64) -> bool {
        if montant <= self.solde {
            self.solde -= montant;
            println!("Retrait de {}€ effectué", montant);
            true
        } else {
            println!("Fonds insuffisants pour retirer {}€", montant);
            false
        }
    }

    fn afficher_solde(&self) {
        println!("Solde de {}: {:.2}€", self.titulaire, self.solde);
    }
}

fn main() {
    // Étape 2.1 : Struct avec champs nommés
    let moi = Personne {
        nom: String::from("Développeur"),
        age: 25,
    };
    println!("Nom: {}, Age: {}", moi.nom, moi.age);

    // Étape 2.2 : Tuple struct
    let point = Point3D(10, 20, 30);
    println!("Point3D: x={}, y={}, z={}", point.0, point.1, point.2);

    // Étape 2.3 : Unit struct (badge)
    match se_connecter("admin") {
        Some(badge) => acceder_admin(badge),
        None => println!("Clé incorrecte"),
    }

    match se_connecter("mauvaise") {
        Some(badge) => acceder_admin(badge),
        None => println!("Clé incorrecte"),
    }

    // Étape 3 : #[derive(Debug, Clone)]
    let clone = moi.clone();
    println!("Clone (Debug): {:?}", clone);

    // Étape 4 : CompteBancaire avec méthodes
    let mut compte = CompteBancaire::new("Alice", 1000.0);
    compte.afficher_solde();

    compte.deposer(500.0);
    compte.afficher_solde();

    compte.retirer(200.0);
    compte.afficher_solde();

    compte.retirer(2000.0); // Fonds insuffisants

    // Étape 5 : Méthode associée new
    let compte2 = CompteBancaire::new("Bob", 500.0);
    println!("\nCompte de Bob: {:?}", compte2);

    // Étape 6 : Field Init Shorthand
    let nom = String::from("Charlie");
    let age: u32 = 30;
    let charlie = Personne { nom, age };
    println!("\nPersonne (shorthand): {:?}", charlie);
}
