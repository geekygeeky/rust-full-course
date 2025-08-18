// Structs are used to name and package related values similar to tuples

struct Book {
    title: String,
    author: String,
    pages: u32,
    in_stock: bool,
}

fn main() {

    let mut book = Book {
        title: "Rust For Dummies".to_string(),
        author: "John doe".to_string(),
        pages: 132,
        in_stock: true,
    };

    println!("Book title: {}", book.title);

    // Tuple Structs
    struct Color(i32, i32, i32);

    let black_color = Color(0,0,0);
    let white_color = Color(255,255,255);

}