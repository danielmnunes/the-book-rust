#[allow(unused_variables)]
fn main() {
    // Tipos compostos

    // tuplas
    let tup: (u32, f64, bool) = (1, 3.14, true);
    let (x, y, z) = tup;
    let xx = tup.0;

    // Arrays
    let array = [1, 2, 3, 4, 5];
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array = [3; 5];
    let a1 = array[0];
    println!("Array: {:?}", array);
}