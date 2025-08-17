// Ownership
// Solves the issue created by Garbage collections and manual memory handling

// Ownership
// [stopping/Resuming the program]
// OWNERSHIP introduced by Rust to solve memory safety issues and high performance at the same time.
// What is Ownership ?
// Every value has a single owner [every variable has one value, and it is its sole owner].

// Ownership Rules
// 1- Each value in Rust has a variable that's its owner.
// 2- There can be only one owner at a time.
// 3— When the owner goes out of scope, the value will be dropped.

fn main(){
    // s1 is the owner of this string value
    let s1: String = String::from("Hello");

    // Borrow reference to s1
    let len = calculate_str_length(&s1);
    println!("Length of '{}' is {}", s1, len);

    let s2 = s1;
    println!("Value: {}", s2);
    println!("Value: {}", s2);

    /*
    * There can only be one owner at a time
    * This will throw an error
    * println!("Value: {}", s1);
    */
}

fn calculate_str_length(s: &String) -> usize {
    s.len()
}