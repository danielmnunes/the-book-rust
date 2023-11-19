fn main() {
    // Definição com let
    let x = 5;

    // Shadowing
    let x = x + 1;
    let x = x * 2;

    println!("valor de x é: {x}");

    let espacos = "   ";
    // espacos = espacos.len(); // não funciona'
    let espacos = espacos.len();
    println!("espaços: {espacos}")
}