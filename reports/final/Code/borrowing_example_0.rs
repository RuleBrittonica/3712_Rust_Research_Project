fn print_message(message: &str) {
    println!("{}", message);
}

fn main() {
    let greeting = String::from("Hello, world!");
    print_message(&greeting); // Borrows greeting immutably
    println!("Still own greeting: {}", greeting);
}