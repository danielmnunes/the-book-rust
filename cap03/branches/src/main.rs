fn main() {
    let numero = 3;

    if numero < 5 {
        println!("verdade");
    } else if numero != 0 {
        println!("falso");
    } else {
        println!("falso");
    }

    let condicao = true;
    let numero = if condicao { 5 } else { 6 };
    println!("{numero}");
}
