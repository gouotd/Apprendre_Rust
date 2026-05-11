#[derive(Debug, Clone)]
struct Personne {
    nom: String,
    age: u32,
}
struct Couleur_tuple(u8, u8, u8);
struct Point3D (u8,u8,u8);

trait logger {
    fn log(&self, messag:&str);
}
struct ConsoleLogger;
impl ConsoleLogger{
    fn log(message: &str){
        println!("[LOG] {}", message);
    }
    fn clear(){
        println!("Écran effacé");
    }
}

struct Connecte;
fn se_connecter (cle: String)-> Option<Connecte>{
    if cle == "admin"{
        Some(Connecte)
    }
    else{
        None
    }
}
fn acceder_admin(_badge: Connecte){
    println!("Accès admin accordé");
}

struct CompteBancaire{titulaire: String, solde: f64}
impl CompteBancaire{
    fn deposer(&mut self, montant: f64){
        self.solde+=montant;
    }
    fn retirer(&mut self, montant: f64) -> bool{
        let mut retrait_valide =true;
        if montant > self.solde{
            println!("Retrait superieur à solde disponible");
            retrait_valide=false;
        }
        else{
            self.solde-=montant;
        }
        retrait_valide
        
    }
    fn afficher_solde(&self){
        println!("Le solde disponible est de : {} euro", self.solde);
    }
    fn new(titulaire: String, solde: f64) -> Self{
        CompteBancaire {titulaire, solde}
    }

}



fn main() {
    let mut Adulte = Personne{nom: String::from("Dupont"), 
        age: 45,
    };
    println!("Mon nom est : {}, et j'ai {} ans",Adulte.nom, Adulte.age);
    Adulte.age = 53;
    println!("J'ai modifié mon âge, et j'ai maintenant {} ans", Adulte.age);

    let noir = Couleur_tuple(0,0,0);
    println!("La couleur noir en RGB est composé comme suit : {}, {}, {}", noir.0,noir.1,noir.2);

    let A1 = Point3D(5,3,2);
    println!("Le point A1 a pour coordonnées : {}, {}, {}", A1.0,A1.1,A1.2);

    ConsoleLogger::log("Un message de test");
    ConsoleLogger::clear();

    let mut cle = String::from("admin");

    match se_connecter(cle){
        Some(badge)=> acceder_admin(badge),
        None => println!("Clé incorrecte"),
    }

    cle = String::from("not admin");
    match se_connecter(cle){
        Some(badge)=> acceder_admin(badge),
        None => println!("Clé incorrecte"),
    }

    let Adulte_clone = Adulte.clone();
    println!("{:?}", Adulte_clone);
    
    let mut MyCompte = CompteBancaire{titulaire: String::from("ano"),solde:1000.0};
    MyCompte.deposer(100.0);
    MyCompte.afficher_solde();
    MyCompte.retirer(300.0);
    MyCompte.afficher_solde();
    MyCompte.retirer(2300.0);

    let mut HerCompte = CompteBancaire::new(String::from("ano"), 1000.0);
    HerCompte.deposer(100.0);
    HerCompte.afficher_solde();
    HerCompte.retirer(300.0);
    HerCompte.afficher_solde();
    HerCompte.retirer(2300.0);

    let nom = String::from("Un nom");
    let age = 36;
    let personne = Personne{nom, age};
    println!("Mon nom est : {}, et j'ai {} ans",personne.nom, personne.age);

}
