#[allow(dead_code)]
#[derive(Debug)]
struct Point(i32);

fn main() {
    println!("Hello World!");

    println!("{0}, ton age est {1}", "Alice", 30);

    println!(
        "Le {operation} de {a} {op} {b} = {result}",
        operation = "produit",
        a = 5,
        op = "x",
        b = 7,
        result = 35
    );

    println!("Binaire: {:b}", 255);
    println!("Octal: {:o}", 255);
    println!("Hex: {:x}", 255);

    let pi = 3.141592;
    println!("Pi ≈ {:.2}", pi);

    let p = Point(10);
    println!("Point: {:?}", p);
}
