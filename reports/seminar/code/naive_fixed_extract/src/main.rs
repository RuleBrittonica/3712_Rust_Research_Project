fn main() {
    let text = String::from("alpha beta gamma");
    show_first(&text); // pass a shared borrow instead of moving
    println!("total_len: {}", text.len()); // ok
}

fn show_first(text: &String) {
    let first = text.split_whitespace().next().unwrap_or("");
    println!("first: {first}");
}
