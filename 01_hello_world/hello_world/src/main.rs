#[derive(Debug)]
struct Point (i32);
#[derive(Debug)]
struct Personne {
    nom: String,
    age: i32}
fn main() {
    println!("Hello, world!");
    println!("{0}, ton age est {1}, et tu mesures {2}cm", "Alice", 30, 175);
    println!{"Le{operation} de {a} {op} {b} = {result}",
        operation = "produit",
        a = 5,
        op = "x",
        b = 7,
        result = 35}
    println!{"La somme de {a} {operation} {b} = {resultat}",
        a=150,
        operation="+",
        b=200,
        resultat=350}

    println!("Binaire: {:b}", 255);
    println!("Octal: {:o}",255);
    println!("Hex: {:x}",255);

    println!("Binaire: {:b}", 42);
    println!("Octal: {:o}",42);
    println!("Hex: {:x}",42);

    let pi = 3.141592;
    println!("Pi ≈ {:.2}", pi);

    let un_tier = 37.0/3.0;
    println!("37/3 ≈ {:.3}", un_tier);

    let p = Point(10);
    println!("{:?}", p);

    let dimitri = Personne{nom: "Dimitri".to_string(), age: 31};
    println!("{:?}", dimitri);

    let message = format!("Bienvenue {}!", "Alice");
    println!("{}", message);

    let hello = format!("Hello world ! Je sais {}!", "print");
    println!("{}", hello);
}
