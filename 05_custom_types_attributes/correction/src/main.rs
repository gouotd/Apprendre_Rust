// 2. Structs
#[derive(Debug, Clone)]
struct Personne {
    nom: String,
    age: u32,
}

struct Point3D(u8, u8, u8);

// 4. CompteBancaire avec méthodes
#[derive(Debug)]
struct CompteBancaire {
    titulaire: String,
    solde: f64,
}

impl CompteBancaire {
    // 5. Méthode associée (constructeur)
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
    // 2. Struct avec champs nommés
    let moi = Personne {
        nom: String::from("Développeur"),
        age: 25,
    };
    println!("Nom: {}, Age: {}", moi.nom, moi.age);

    // Tuple struct
    let point = Point3D(10, 20, 30);
    println!("Point3D: x={}, y={}, z={}", point.0, point.1, point.2);

    // 3. #[derive(Debug, Clone)]
    let clone = moi.clone();
    println!("Clone (Debug): {:?}", clone);

    // 4. CompteBancaire avec méthodes
    let mut compte = CompteBancaire::new("Alice", 1000.0);
    compte.afficher_solde();

    compte.deposer(500.0);
    compte.afficher_solde();

    compte.retirer(200.0);
    compte.afficher_solde();

    compte.retirer(2000.0); // Fonds insuffisants

    // 5. Méthode associée new
    let compte2 = CompteBancaire::new("Bob", 500.0);
    println!("\nCompte de Bob: {:?}", compte2);

    // 6. Field Init Shorthand
    let nom = String::from("Charlie");
    let age: u32 = 30;
    let charlie = Personne { nom, age };
    println!("\nPersonne (shorthand): {:?}", charlie);
}
