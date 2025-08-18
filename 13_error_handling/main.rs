// Error handling techniques [2 approaches]

// Approach 1
// enum Option<T> { // Define the generic Option type
//     Some(T), // Represents a value
//     None, // Represents no value
// }

// Approach 2
// enum Result<T, E> { // Define the generic Option type
//     Ok(T), // Represents a value
//     Err(E), // Represents an error
// }

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    }else{
        Some(numerator / denominator)
    }
}

fn issue_license(id: &mut u32, age: u32) -> Result<u32, String> {
    if age < 18 {
        Err("You need to be 18 to be issued a license".to_string())
    }else {
        *id += 1;
        Ok(*id)
    }
}

fn main() {
    
    let mut id: u32 = 0;

    let result = divide(10.0, 2.0);

    match result {
        Some(x) => println!("Result: {x}"),
        None => println!("Error: Cannot divide by Zero!")
    }

    let license = issue_license(&mut id, 19);
    match license {
        Ok(result) => println!("License ID: {}", result),
        Err(err) => println!("Error: {}", err)
    }
    println!("Last generated ID: {id}");

}