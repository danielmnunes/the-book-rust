fn main() {
    loop {
        println!("loop infinito");
        break;
    }

    let mut numero = 3;

    while numero != 0 {
        println!("usando while:{numero}");
        numero -= 1;
    }

    let lista = [1,2,3,4,5];
    let mut indice = 0;
    while indice < lista.len() {
        println!("usando while:{}", lista[indice] * 10);
        indice += 1;
    }

    for elemento in lista.iter() {
        println!("usando for: {elemento}");
    }

    for numero in 0..=4 {
        println!("usando range: {numero}");
    }
}
