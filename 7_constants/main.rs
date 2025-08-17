// Cannot use 'mut' with constants

fn main() {
    const PI: f64 = 3.142;
    println!("Value of PI is {}", PI);
    println!("App ID is {}", APP_ID);
}

// Declare a constant here

const APP_ID: i32 = 12345;