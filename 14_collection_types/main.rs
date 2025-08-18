use std::collections::HashMap;
fn main() {

    // Vectors
    let mut names: Vec<&str> = Vec::new();
    let cities = vec!["Paris", "New York"];

    println!("{:?}", cities);

    names.push("Joe");
    println!("{:?}", names);

    let first_name_item = &mut names[0];

    println!("First name in vector {}", first_name_item);

    *first_name_item = "John";

    println!("First name in vector {}", first_name_item);
    println!("First name in vector {}", names[0]);

    // Hash maps
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    println!("Score: {}", score);
    println!("{:?}", scores);

}