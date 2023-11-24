fn main() {
    println!("Hello, world!");
    outra_funcao(12, 13);


    let y = {
        let x = 3;
        println!("{x}");
        x * 11
    };
    println!("{y}");

    let x = cinco();
    println!("{x}");
}

fn outra_funcao(x: i32, y: i32) {
    println!("Valor de x: {x}-{y}");
}

fn cinco() -> i32 {
    5
}

