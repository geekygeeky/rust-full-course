// Functions
// Rust supports hoisting - creating & calling functions anywhere in your code

// Entry point
fn main(){
    hello_word();
    display_height(162);
    bio_data("Joe", 34, 182.0);

    // Calculate BMI
    let weight = 70.0;
    let height = 1.75;

    let bmi = bmi_calculator(weight, height);
    println!("BMI is: {:.2}", bmi);
}

fn hello_word(){
    println!("Hello, Rust!")
}

// Function params
fn display_height(height: u32){
    println!("The height is {} cm.", height);
}

// Multiple params
fn bio_data(name: &str, age: u32, height: f32){
    println!("{} is {} years old and he is {} cm tall.", name, age, height);
}

fn bmi_calculator(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

// Expressions and Statements
// Expression: Anything that returns a value.
/*
 * let X = {
 *  let a = 50;
 *  let b = 10;
 *  b + a
 * }
 */
// Statement: Anything that does not return a value.
// let x = 10;
// Other examples includes function declaration themselves and conditional statements