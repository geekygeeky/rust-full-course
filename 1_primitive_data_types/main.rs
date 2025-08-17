// Primitive data types
// int, float, bool, char

fn main() {
    println!("Hello, world!");

    // Integer
    // Rust has signed (+ & -) and unsigned integer (only '+') types of different sizes
    // i8, i16, i32, i64, i128: Signed integers
    // u8, i16, u32, u64, u128: Unsigned integers

    let x = -42;
    let y: u8 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Floats [floating point types]
    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    // Boolean values: true , false
    let is_rust: bool = true;
    println!("Is this a rust crash course? {}", is_rust);

    // Character Type - char (represents a unicode scalar value)
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
}