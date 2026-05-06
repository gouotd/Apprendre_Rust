fn main() {
    {
    let x = 5;
    println!("{}", x);  // OK, x existe
    }
    // println!("{}", x);  // ERREUR: x n'existe plus ici
    {
        let message : &str="message";
        println!("message : {}",message);
    }
    // println!("message : {}",message); // ERREUR: message n'existe plus ici

    let s1 = String::from("Bonjour");
    let s2 = s1;  // s1 "donne" sa valeur à s2 → s1 n'est plus valide
    // println!("{}", s1);  // ERREUR: s1 n'est plus valide
    println!("{}", s2);  // OK

    let nom= String::from("Alice");
    // let nom2: String = nom;
    println!("{}", nom);
    // println!("{}", nom2);

    // Types Copy
    let x = 5;
    let y = x;  // y = 5, x reste valide
    println!("{}", x);  // OK

    // Types Move
    let s1 = String::from("hi");
    let s2 = s1;  // s1 n'est plus valide

    let a: i32= 42;
    let b = a;
    println!("on a la valeur de a :{}, on l'a copié dans b : {}, a existe encore car i32 n implemente copy",a,b);    
    let c = &a;
    println!("on a la valeur de a :{}, on l'a preté à c : {}, a existe encore et on a évité une copie couteuse",a,c);    

    let nom1: &str= "Alice";
    let nom2 = nom1;
    println!("on affiche nom1 :{}, on affiche nom2 : {}",nom1,nom2);   
    let nom3= String::from("Alice");
    let nom4 = nom3;
    println!("nom3 n existe plus mais on peut afficher nom4 : {}",nom4);   

    let s1 = String::from("Bonjour");
    let s2 = s1.clone();  // Copie profonde : s2 a ses propres données
    println!("{}", s1);   // OK
    println!("{}", s2);   // OK

    let s1 = String::from("Bonjour");
    let s2 = &s1;         // s2 emprunte s1, ne prend pas possession
    println!("{}", s1);   // OK
    println!("{}", s2);   // OK

    let mut s = String::from("Test string mutable borrow");
    mettre_en_majuscules_clone(&mut s);
    println!("on voit que s est modifié : {}", s);


    let mut s2 = String::from("Test string mutable borrow");
    mettre_en_borrow(&mut s2);


    let string1 = String::from("long string");
    //                    └─ string1 vit jusqu'à la fin de main
    let result;
    {
        let string2 = String::from("xyz");
        //              └─ string2 vit seulement dans ce bloc
        result = plus_long(&string1, &string2);
        println!("Résultat: {}", result);
    }
    // string2 est détruit ici !
    // println!("Résultat: {}", result);
    // ↑ ERREUR: result pourrait pointer vers string2 qui n'existe plus !

    let result;
    {
        let string2 = String::from("xyz");
        //              └─ string2 vit seulement dans ce bloc
        result = plus_court(&string1, &string2);
        println!("Résultat: {}", result);
    }

fn mettre_en_majuscules_clone(texte: &mut String){
    println!("Avant passage en majuscule : {}", texte);
    let mut texte_clone = texte.clone();
    // let mut texte_borrow = &mut texte_clone;

    texte.make_ascii_uppercase();
    println!("Après passage en majuscule en modifiant texte : {}", texte);
    println!("On a copié dans texte_clone avant la modif donc on dfoit encore etre en minuscule: {}", texte_clone);
    
    texte_clone.make_ascii_uppercase();
    println!("Après passage en majuscule test copie : {}", texte_clone);
    // println!("On a emprunté texte_clone dans texte_borrow donc on devrait etre en majuscule: {}", texte_borrow);

}

fn mettre_en_borrow(texte: &mut String){
    println!("Avant passage en majuscule : {}", texte);
    let mut texte_clone = texte.clone();
    let mut texte_borrow = &mut texte_clone;

    texte_borrow.make_ascii_uppercase();
    println!("Après passage en majuscule en modifiant texte_borrow, texte est en minuscule: {} mais texte_clone : {} et texte borrow n'estp lus disponible car borrow", texte, texte_clone);
    println!("On a copié dans texte_clone avant la modif donc on dfoit encore etre en minuscule: {}", texte_clone);
    
    texte_clone.make_ascii_uppercase();
    println!("Après passage en majuscule test copie : {}", texte_clone);
    // println!("On a emprunté texte_clone dans texte_borrow donc on devrait etre en majuscule: {}", texte_borrow);

}

fn plus_long<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1  // Retourne s1
    } else {
        s2  // Retourne s2
    }
}

fn plus_court<'a>(s1: &'a str, s2: &'a str) -> &'a str{
    if s1.len() < s2.len() {
        s1
    } else {
        s2
    }
}

}