fn main() {
    let x = 5;
    let mut y =5;
    y = 10;
    println!("y vaut de base {xvar}, mais comme il est mutable il faut desormais : {yvar}",xvar=x,yvar=y);

    let mut compte =100;
    compte = compte+50;
    println!("compte vaut de base 50, mais comme il est mutable il faut desormais : {comptevar}",comptevar=compte);

    let x: i32 = 5;
    let name: &str = "Alice";
    let actif: bool = true;

    let age: i32 =25;
    let nom: &str = "Bob";

    const MAX_SCORE: i32 = 100;
    const PI: f64 = 3.14159;
    const TAXE: f64 =0.20;
    let mut price: f64=100.0;
    price = price *(1.0+TAXE) ;
    println!("Le prix HT est de 100€, le prix ttc est de : {price_var}€",price_var=price);

    let x = 5;
    println!("x valait {}", x);  // 5
    let x = x + 10;
    println!("x vaut desormais {}", x);  // 15
    let message: &str = "Hello";
    println!("message contenait : {message_var}", message_var=message);
    let message: &str = "World";
    println!("message contient desormais : {message_var}", message_var=message);

    let x = 42;
    println!("Type: {}", type_of(&x));  // i32
    let x = 5;        // Rust devine i32
    println!("Type de x : {} ", type_of(&x));
    let pi = 3.14;     // Rust devine f64
    println!("Type de pi : {} ", type_of(&pi));
    let actif = true;   // Rust devine bool
    println!("Type de actif : {} ", type_of(&actif));

    let x: i32 = 5;
    let y: f64 = x as f64;
    println!("x = {}",x);
    println!("Type de x : {} ", type_of(&x));
    println!("y = {}",y);
    println!("Type de y : {} ", type_of(&x));

}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

