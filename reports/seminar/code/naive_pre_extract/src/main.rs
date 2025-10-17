fn main() {
    let text = String::from("alpha beta gamma");

    // (This is the block we "extract")
    let first = text.split_whitespace().next().unwrap_or("");

    // Still allowed to use `text` here as it is only borrowed above
    println!("first: {first}, total_len: {}", text.len());
}
