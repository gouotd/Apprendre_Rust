fn main() {
    let resultat = addition(5,10);
    println!("resultat de l'addition : {}", resultat);

    let resultat = carre(5);
    println!("resultat de carre : {}", resultat);

    saluer("André");

    est_pair(10);
    est_pair(11);
    est_pair(0);

    let age = 20;
    if age >= 18{
        println!("Majeur");
    } else if age >= 12{
        println!("Adolescent");
    } else { 
        println!("Enfant");
    }

    let tranche_age= categorie(age);
    println!("Si tu as {} ans, alors tu es un {}",age,tranche_age);

    let chiffre =2;
    match chiffre {
        1 => println!("Un"),           // Pattern: valeur exacte
        2 | 3 => println!("Deux ou Trois"),  // Pattern: OU
        4..=6 => println!("Entre 4 et 6"),   // Pattern: plage
        _ => println!("Autre chose"),        // Pattern: attrape-tout
    }

    afficher_mention(85);

    for i in 0..5 {
    println!("i = {}", i);  // Affiche 0, 1, 2, 3, 4
    }
    // Range inclusif (inclut 5)
    for i in 0..=5 {
        println!("i = {}", i);  // Affiche 0, 1, 2, 3, 4, 5
    }
    let nombres:  [i32;4] = [10,20,30,40];
    for n in nombres {
        println!("Valeur : {}", n);
    }
    let fruits = vec!["Pomme","Cerise", "Banane"];
    for fruit in &fruits {
        println!("Fruit: {}", fruit);
    }
    let nombres: [i32; 5] = [85, 92 ,78, 90, 88];
    somme_table(&nombres);

    let mut compteur =0;
    while compteur <5{
        println!("Compteur: {}", compteur);
        compteur += 1;
    }
    compteur=10;
    while compteur >=0{
        println!("Compteur: {}", compteur);
        compteur -= 1;
    }
    let mut i = 0;
    let resultat = loop{
        i+= 1;
        if i == 10{
            break i *2;
        }
    };
    println!("la boucle loop retourne : {}", resultat);

    let mut j =0;
    let multiple7_sup50 = loop {
        if j>50 && j%7==0{
            break j;
        }
        j+=1;
    };
    println!("la boucle loop nous dit que le premier multiple de 7 superieuer à 50 est  : {}", multiple7_sup50);
}

fn addition(x: i32, y: i32) -> i32 {
    x + y  // Pas besoin de `return` en fin de fonction
}
fn carre(x: i32)-> i32{
    x*x
}
fn saluer(nom: &str){
    println!("Bonjour, {nom} !");
}

fn est_pair(nombre: i32) {
    let parite = if nombre % 2 ==0 {"pair"} else {"impair"};
    println!("Le nombre {}, est {}",nombre, parite);
}

fn categorie(age: i32) -> &'static str {
    if age >= 60{"Senior"
    } else if age >= 18{"Adulte"
    } else if age >= 12{"Adolescent"
    } else {"Enfant"
    }
}

fn afficher_mention(note: i32) {
    match note{
        90..=100 => println!("Excellent"),
        80..=89 => println!("Très bien"),
        70..=79 => println!("Bien"),
        _ => println!("À améliorer"),
    }
}

fn somme_table(nombres: &[i32]){
    let mut somme: i32 =0;
    for &i in nombres{
        somme += i;
    }
    let moyenne = somme / 5;
    println!("la moyenne de la somme du tableau est {}", moyenne);
}