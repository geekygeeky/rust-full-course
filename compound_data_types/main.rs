// Compound Data Types

// Arrays, tuples, slices and strings (slice string)

fn main() {
    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array of numbers: {:?}", numbers);

    let fruits: [&str; 2] = ["Apple", "Orange"];
    println!("Array of fruits: {:?}", fruits);

    // Tuples
    let bio_data: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Bio data Tuple: {:?}", bio_data);

    let mixed_tuples = ("Joe", 23, true, [1, 2, 3, 4, 5]);
    println!("Mixed Tuple: {:?} \n", mixed_tuples);

    // Slices
    let number_slices: &[i32] = &[1, 2, 3, 4, 5, 6];
    println!("-----Slices------\nNumber slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Hyena", "crocodile"];
    println!("Animal slices: {:?}", animal_slices);

    let name_slices: &[&String] = &[&"Abike".to_string(), &"Serah".to_string()];
    println!("Name slices: {:?}", name_slices);

    // Strings
    // Strings are growable and mutable
    // They are owned string types -- they are not borrowed
    // Stored in a heap

    // immutable string
    let name: String = String::from("John");
    println!("Name is {}", name);

    // Mutable string
    let mut full_name: String = String::from("Joe");
    full_name.push_str(" Schmoe");
    println!("Name is {}", full_name);

    // &str (string slice)
    // Stored in the stack

    let city: String = String::from("Paris");
    let city_slice: &str = &city[0..3];
    println!("City slice: {}", city_slice);

}
