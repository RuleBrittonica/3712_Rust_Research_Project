fn main() {
    let text = String::from("Hello world!");
    let length = text.len();

    // Some inline block that only reads from `text`:
    println!("First word is: {}", get_first_word(&text));
    println!("Total length is: {}", length);
}

fn get_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
