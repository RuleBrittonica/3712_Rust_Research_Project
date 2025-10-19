fn main() {
    let text = String::from("alpha beta gamma");

    // Naïve extract: takes ownership of `text` (moves it)
    show_first(text);

    // Borrow *after* move - compile-time error
    println!("total_len: {}", text.len());
}

// Naïve extracted function: takes `String` by value, moving it.
fn show_first(text: String) {
    let first = text.split_whitespace().next().unwrap_or("");
    println!("first: {first}");
}
