fn print_first_word_block(text: &String) {
    // Only reads from `text`, so it only needs a shared reference.
    println!("First word is: {}", get_first_word(text));
}

fn main() {
    let text = String::from("Hello world!");
    let length = text.len();

    print_first_word_block(&text); // Pass a shared reference
    println!("Total length is: {}", length);
}
