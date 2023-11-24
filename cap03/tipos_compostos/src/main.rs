#[allow(unused_variables)]
fn main() {
    // tuplas
    let tup: (i32, f64, bool) = (500, 2.5, true);
    let (x, y, z) = tup;
    let x1 = tup.0;
    println!("{x} {x1}");

    // Array
    let array = [1, 2, 3, 4, 5];
    let primeiro = array[0];
    let segundo = array[1];
    println!("{:?}", array);
    println!("{primeiro}, {segundo}");
}
